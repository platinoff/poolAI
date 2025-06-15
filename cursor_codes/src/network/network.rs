use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub network_type: String,
    pub max_connections: u32,
    pub max_bandwidth: u64,
    pub max_latency: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_connections: u64,
    pub active_connections: u32,
    pub total_bandwidth: u64,
    pub current_bandwidth: u64,
    pub total_latency: u64,
    pub average_latency: f64,
    pub uptime: u64,
    pub last_connection_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub config: NetworkConfig,
    pub stats: NetworkStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub network_id: String,
    pub peer_id: String,
    pub bandwidth: u64,
    pub latency: u64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

pub struct NetworkSystem {
    networks: Arc<Mutex<HashMap<String, NetworkMetrics>>>,
    connections: Arc<Mutex<HashMap<String, Connection>>>,
}

impl NetworkSystem {
    pub fn new() -> Self {
        Self {
            networks: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_network(&self, config: NetworkConfig) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        
        if networks.contains_key(&config.id) {
            return Err(format!("Network '{}' already exists", config.id));
        }

        let metrics = NetworkMetrics {
            config,
            stats: NetworkStats {
                total_connections: 0,
                active_connections: 0,
                total_bandwidth: 0,
                current_bandwidth: 0,
                total_latency: 0,
                average_latency: 0.0,
                uptime: 0,
                last_connection_time: None,
                last_error: None,
            },
        };

        networks.insert(metrics.config.id.clone(), metrics);
        info!("Added new network: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_network(&self, id: &str) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        let mut connections = self.connections.lock().await;
        
        if !networks.contains_key(id) {
            return Err(format!("Network '{}' not found", id));
        }

        // Remove associated connections
        connections.retain(|_, c| c.network_id != id);
        
        networks.remove(id);
        info!("Removed network: {}", id);
        Ok(())
    }

    pub async fn establish_connection(
        &self,
        network_id: &str,
        peer_id: &str,
        bandwidth: u64,
    ) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        let mut connections = self.connections.lock().await;
        
        let network = networks
            .get_mut(network_id)
            .ok_or_else(|| format!("Network '{}' not found", network_id))?;

        if !network.config.active {
            return Err("Network is not active".to_string());
        }

        if network.stats.active_connections >= network.config.max_connections {
            return Err("Network has reached maximum connections".to_string());
        }

        if bandwidth > network.config.max_bandwidth {
            return Err("Bandwidth exceeds maximum".to_string());
        }

        let connection = Connection {
            id: uuid::Uuid::new_v4().to_string(),
            network_id: network_id.to_string(),
            peer_id: peer_id.to_string(),
            bandwidth,
            latency: 0,
            timestamp: Utc::now(),
            status: "pending".to_string(),
        };

        connections.insert(connection.id.clone(), connection.clone());
        network.stats.active_connections += 1;
        network.stats.total_connections += 1;
        network.stats.current_bandwidth += bandwidth;

        info!(
            "Established connection: {} on network: {} (peer: {}, bandwidth: {})",
            connection.id, network_id, peer_id, bandwidth
        );
        Ok(())
    }

    pub async fn process_connection(&self, connection_id: &str) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        let mut connections = self.connections.lock().await;
        
        let connection = connections
            .get_mut(connection_id)
            .ok_or_else(|| format!("Connection '{}' not found", connection_id))?;

        let network = networks
            .get_mut(&connection.network_id)
            .ok_or_else(|| format!("Network '{}' not found", connection.network_id))?;

        if !network.config.active {
            return Err("Network is not active".to_string());
        }

        let start_time = Utc::now();

        match self.validate_connection(connection, &network.config).await {
            Ok(latency) => {
                connection.status = "active".to_string();
                connection.latency = latency;
                network.stats.total_latency += latency;
                network.stats.average_latency = network.stats.total_latency as f64
                    / network.stats.active_connections as f64;
            }
            Err(e) => {
                connection.status = "failed".to_string();
                network.stats.active_connections -= 1;
                network.stats.current_bandwidth -= connection.bandwidth;
                network.stats.last_error = Some(e);
            }
        }

        network.stats.last_connection_time = Some(start_time);
        info!("Processed connection: {}", connection_id);
        Ok(())
    }

    async fn validate_connection(
        &self,
        connection: &Connection,
        config: &NetworkConfig,
    ) -> Result<u64, String> {
        // Simulate connection validation
        let latency = (connection.bandwidth as f64 / config.max_bandwidth as f64 * config.max_latency as f64) as u64;
        
        // Simulate network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        if latency > config.max_latency {
            return Err("Connection latency too high".to_string());
        }

        info!(
            "Validated connection: {} on network: {} (peer: {}, latency: {})",
            connection.id, connection.network_id, connection.peer_id, latency
        );
        Ok(latency)
    }

    pub async fn update_bandwidth(&self, id: &str, bandwidth: u64) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        
        let network = networks
            .get_mut(id)
            .ok_or_else(|| format!("Network '{}' not found", id))?;

        if bandwidth > network.config.max_bandwidth {
            return Err("Bandwidth exceeds maximum".to_string());
        }

        network.stats.current_bandwidth = bandwidth;
        info!("Updated bandwidth for network: {} to {}", id, bandwidth);
        Ok(())
    }

    pub async fn get_network(&self, id: &str) -> Result<NetworkMetrics, String> {
        let networks = self.networks.lock().await;
        
        networks
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Network '{}' not found", id))
    }

    pub async fn get_all_networks(&self) -> Vec<NetworkMetrics> {
        let networks = self.networks.lock().await;
        networks.values().cloned().collect()
    }

    pub async fn get_active_networks(&self) -> Vec<NetworkMetrics> {
        let networks = self.networks.lock().await;
        networks
            .values()
            .filter(|n| n.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_connections(&self, network_id: &str) -> Vec<Connection> {
        let connections = self.connections.lock().await;
        connections
            .values()
            .filter(|c| c.network_id == network_id)
            .cloned()
            .collect()
    }

    pub async fn set_network_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        
        let network = networks
            .get_mut(id)
            .ok_or_else(|| format!("Network '{}' not found", id))?;

        network.config.active = active;
        info!(
            "Network '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_network_config(&self, id: &str, new_config: NetworkConfig) -> Result<(), String> {
        let mut networks = self.networks.lock().await;
        
        let network = networks
            .get_mut(id)
            .ok_or_else(|| format!("Network '{}' not found", id))?;

        network.config = new_config;
        info!("Updated network configuration: {}", id);
        Ok(())
    }
} 