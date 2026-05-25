//! WebSocket hub (connections, broadcast). Used by `network::ws` handlers and `AppState`.

use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::pool::topology::{NodeResources, Topology};

/// WebSocket message structure
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

/// Live metrics structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveMetrics {
    pub active_workers: u32,
    pub total_requests: u64,
    pub avg_response_time: f64,
    pub memory_usage: f64,
    pub gpu_temperature: f64,
    pub timestamp: u64,
}

/// System event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: u64,
}

/// Live topology snapshot pushed to WebSocket subscribers (PH-S22).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyLiveUpdate {
    pub node_count: usize,
    pub latency_measurements: usize,
    pub last_updated: String,
    pub node_ids: Vec<String>,
    pub nodes: HashMap<String, NodeResources>,
    pub latency_matrix: HashMap<String, f64>,
}

impl TopologyLiveUpdate {
    pub fn from_topology(topology: &Topology) -> Self {
        let node_ids: Vec<String> = topology.node_resources.keys().cloned().collect();
        Self {
            node_count: topology.node_resources.len(),
            latency_measurements: topology.latency_matrix.len(),
            last_updated: topology.last_updated.to_rfc3339(),
            node_ids,
            nodes: topology.node_resources.clone(),
            latency_matrix: topology.latency_matrix.clone(),
        }
    }
}

/// WebSocket connection information
#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub user_id: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub last_heartbeat: u64,
}

/// WebSocket connection manager
pub struct WebSocketManager {
    pub(crate) connections: Arc<RwLock<HashMap<String, WebSocketConnection>>>,
    pub(crate) senders: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    metrics_subscriptions: Arc<RwLock<HashMap<String, bool>>>,
    events_subscriptions: Arc<RwLock<HashMap<String, bool>>>,
    topology_subscriptions: Arc<RwLock<HashMap<String, bool>>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    pub fn new() -> Self {
        let manager = Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            senders: Arc::new(RwLock::new(HashMap::new())),
            metrics_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            events_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            topology_subscriptions: Arc::new(RwLock::new(HashMap::new())),
        };

        let metrics_subs = manager.metrics_subscriptions.clone();
        let events_subs = manager.events_subscriptions.clone();
        let senders_metrics = manager.senders.clone();
        let senders_events = manager.senders.clone();

        tokio::spawn(async move {
            let mut metrics_interval = tokio::time::interval(std::time::Duration::from_secs(5));
            loop {
                metrics_interval.tick().await;

                let subs = metrics_subs.read().await;
                let senders = senders_metrics.read().await;
                for (connection_id, _) in subs.iter() {
                    if let Some(sender) = senders.get(connection_id) {
                        let metrics = get_current_metrics().await;
                        let ws_message = WebSocketMessage {
                            message_type: "live_metrics".to_string(),
                            data: serde_json::to_value(metrics).unwrap_or_default(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };
                        if let Ok(json) = serde_json::to_string(&ws_message) {
                            let _ = sender.send(Message::Text(json.into()));
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            let mut events_interval = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                events_interval.tick().await;

                let subs = events_subs.read().await;
                let senders = senders_events.read().await;
                for (connection_id, _) in subs.iter() {
                    if senders.get(connection_id).is_some() {
                        // Placeholder: real impl would drain event store
                    }
                }
            }
        });

        manager
    }

    pub async fn register_sender(
        &self,
        connection_id: String,
        sender: mpsc::UnboundedSender<Message>,
    ) {
        let mut senders = self.senders.write().await;
        senders.insert(connection_id, sender);
    }

    pub async fn add_connection(&self, connection_id: String, connection: WebSocketConnection) {
        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection);
    }

    pub async fn remove_connection(&self, connection_id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(connection_id);
        let mut senders = self.senders.write().await;
        senders.remove(connection_id);
        let mut metrics_subs = self.metrics_subscriptions.write().await;
        metrics_subs.remove(connection_id);
        let mut events_subs = self.events_subscriptions.write().await;
        events_subs.remove(connection_id);
        let mut topology_subs = self.topology_subscriptions.write().await;
        topology_subs.remove(connection_id);
    }

    pub async fn subscribe_topology(&self, connection_id: &str) {
        let mut subs = self.topology_subscriptions.write().await;
        subs.insert(connection_id.to_string(), true);
    }

    pub async fn subscribe_metrics(&self, connection_id: &str) {
        let mut subs = self.metrics_subscriptions.write().await;
        subs.insert(connection_id.to_string(), true);
    }

    pub async fn subscribe_events(&self, connection_id: &str) {
        let mut subs = self.events_subscriptions.write().await;
        subs.insert(connection_id.to_string(), true);
    }

    pub async fn broadcast(&self, message: WebSocketMessage) {
        let senders = self.senders.read().await;
        let message_json = match serde_json::to_string(&message) {
            Ok(json) => json,
            Err(e) => {
                tracing::warn!("Failed to serialize WebSocket message: {}", e);
                return;
            }
        };

        for (connection_id, sender) in senders.iter() {
            if let Err(e) = sender.send(Message::Text(message_json.clone().into())) {
                tracing::warn!(
                    "Failed to send message to connection {}: {}",
                    connection_id,
                    e
                );
            }
        }
    }

    pub async fn send_to_user(&self, user_id: &str, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        let senders = self.senders.read().await;

        if let Some(connection) = connections.get(user_id) {
            let message_json = match serde_json::to_string(&message) {
                Ok(json) => json,
                Err(e) => {
                    tracing::warn!("Failed to serialize WebSocket message: {}", e);
                    return;
                }
            };

            if let Some(sender) = senders.get(user_id) {
                if let Err(e) = sender.send(Message::Text(message_json.into())) {
                    tracing::warn!(
                        "Failed to send message to user {}: {}",
                        connection.user_id,
                        e
                    );
                }
            }
        }
    }

    pub async fn cleanup_inactive(&self) {
        let mut connections = self.connections.write().await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        connections.retain(|_, connection| current_time - connection.last_heartbeat < 300);
    }

    pub async fn broadcast_system_update(&self, message: &str) {
        let ws_message = WebSocketMessage {
            message_type: "system_update".to_string(),
            data: serde_json::json!({
                "message": message,
                "severity": "info"
            }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.broadcast(ws_message).await;
    }

    pub async fn broadcast_live_metrics_payload(&self, metrics: LiveMetrics) {
        let ws_message = WebSocketMessage {
            message_type: "live_metrics".to_string(),
            data: serde_json::to_value(metrics).unwrap(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.broadcast(ws_message).await;
    }

    pub async fn broadcast_system_event_payload(&self, event: SystemEvent) {
        let ws_message = WebSocketMessage {
            message_type: "system_event".to_string(),
            data: serde_json::to_value(event).unwrap(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.broadcast(ws_message).await;
    }

    pub async fn broadcast_topology_update(&self, update: TopologyLiveUpdate) {
        let subs = self.topology_subscriptions.read().await;
        if subs.is_empty() {
            return;
        }

        let ws_message = WebSocketMessage {
            message_type: "topology_update".to_string(),
            data: serde_json::to_value(&update).unwrap_or_default(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        let message_json = match serde_json::to_string(&ws_message) {
            Ok(json) => json,
            Err(e) => {
                tracing::warn!("Failed to serialize topology WebSocket message: {}", e);
                return;
            }
        };

        let senders = self.senders.read().await;
        for (connection_id, _) in subs.iter() {
            if let Some(sender) = senders.get(connection_id) {
                if let Err(e) = sender.send(Message::Text(message_json.clone().into())) {
                    tracing::warn!(
                        "Failed to send topology update to connection {}: {}",
                        connection_id,
                        e
                    );
                }
            }
        }
    }
}

async fn get_current_metrics() -> LiveMetrics {
    let active_workers = if let Some(pool) = crate::pool::get_global_pool() {
        let pool_guard = pool.read().await;
        pool_guard.get_worker_count().await as u32
    } else {
        0
    };

    LiveMetrics {
        active_workers,
        total_requests: 0,
        avg_response_time: 0.0,
        memory_usage: 0.0,
        gpu_temperature: 0.0,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}
