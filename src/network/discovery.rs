//! Network discovery module for automatic device/worker detection
//!
//! This module provides:
//! - Automatic worker discovery via UDP broadcast (mDNS/Bonjour fallback)
//! - Worker registration protocol with heartbeat
//! - Discovery API endpoints for peer discovery
//!
//! Inspired by exo's automatic device discovery feature.

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Broadcast port for discovery messages
    pub broadcast_port: u16,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_secs: u64,
    /// Timeout for considering a worker offline (seconds)
    pub worker_timeout_secs: u64,
    /// Enable discovery
    pub enabled: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            broadcast_port: 8888,
            heartbeat_interval_secs: 5,
            worker_timeout_secs: 15,
            enabled: true,
        }
    }
}

/// Information about a discovered peer/worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Unique peer identifier
    pub peer_id: String,
    /// Peer hostname or IP address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Peer capabilities (GPU, CPU, etc.)
    pub capabilities: PeerCapabilities,
    /// Peer metadata
    pub metadata: HashMap<String, String>,
}

/// Peer capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerCapabilities {
    /// Number of CPU cores
    pub cpu_cores: usize,
    /// Available GPU devices
    pub gpu_devices: Vec<usize>,
    /// Available memory in MB
    pub memory_mb: usize,
    /// Whether peer supports tensor parallelism
    pub supports_tensor_parallelism: bool,
    /// Whether peer supports pipeline parallelism
    pub supports_pipeline_parallelism: bool,
}

/// Discovery message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiscoveryMessage {
    /// Announcement from a peer
    Announce {
        peer_id: String,
        address: String,
        port: u16,
        capabilities: PeerCapabilities,
        metadata: HashMap<String, String>,
    },
    /// Heartbeat from an existing peer
    Heartbeat {
        peer_id: String,
    },
    /// Query for available peers
    Query,
    /// Response to a query
    Response {
        peers: Vec<PeerInfo>,
    },
}

/// Discovery service for automatic peer/worker detection
pub struct DiscoveryService {
    config: DiscoveryConfig,
    /// Local peer ID
    local_peer_id: String,
    /// Local address and port
    local_address: SocketAddr,
    /// Discovered peers
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    /// Whether the service is running
    running: Arc<RwLock<bool>>,
}

impl DiscoveryService {
    /// Creates a new discovery service
    pub fn new(config: DiscoveryConfig, local_address: SocketAddr) -> Self {
        let local_peer_id = format!("poolai-{}", Uuid::new_v4().to_string()[..8].to_string());
        
        Self {
            config,
            local_peer_id,
            local_address,
            peers: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Starts the discovery service
    pub async fn start(&self) -> Result<(), AppError> {
        if !self.config.enabled {
            info!("Discovery service is disabled");
            return Ok(());
        }

        let mut running = self.running.write().await;
        if *running {
            warn!("Discovery service is already running");
            return Ok(());
        }
        *running = true;
        drop(running);

        info!(
            "Starting discovery service (peer_id: {}, address: {})",
            self.local_peer_id, self.local_address
        );

        // Spawn background tasks
        let peers = Arc::clone(&self.peers);
        let config = self.config.clone();
        let local_peer_id = self.local_peer_id.clone();
        let local_address = self.local_address;

        // Broadcast task
        tokio::spawn(Self::broadcast_task(
            peers.clone(),
            config.clone(),
            local_peer_id.clone(),
            local_address,
        ));

        // Cleanup task
        tokio::spawn(Self::cleanup_task(peers, config));

        // Send initial announcement
        self.send_announcement().await?;

        Ok(())
    }

    /// Stops the discovery service
    pub async fn stop(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        *running = false;
        drop(running);

        info!("Stopping discovery service");
        Ok(())
    }

    /// Broadcasts an announcement message
    pub async fn send_announcement(&self) -> Result<(), AppError> {
        // Get local capabilities (placeholder for now)
        let capabilities = PeerCapabilities::default();
        let metadata = HashMap::new();

        let message = DiscoveryMessage::Announce {
            peer_id: self.local_peer_id.clone(),
            address: self.local_address.ip().to_string(),
            port: self.local_address.port(),
            capabilities,
            metadata,
        };

        self.send_broadcast(&message).await?;
        Ok(())
    }

    /// Sends a broadcast message to all peers on the network
    async fn send_broadcast(&self, message: &DiscoveryMessage) -> Result<(), AppError> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| AppError::NetworkError(format!("Failed to bind UDP socket: {}", e)))?;
        
        socket.set_broadcast(true)
            .map_err(|e| AppError::NetworkError(format!("Failed to enable broadcast: {}", e)))?;

        let data = serde_json::to_vec(message)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize discovery message: {}", e)))?;

        let broadcast_addr = SocketAddr::new(
            IpAddr::from([255, 255, 255, 255]),
            self.config.broadcast_port,
        );

        socket.send_to(&data, broadcast_addr)
            .map_err(|e| AppError::NetworkError(format!("Failed to send broadcast: {}", e)))?;

        debug!("Sent discovery broadcast: {:?}", message);
        Ok(())
    }

    /// Background task for periodic broadcasting
    async fn broadcast_task(
        _peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
        config: DiscoveryConfig,
        local_peer_id: String,
        _local_address: SocketAddr,
    ) {
        let mut interval = interval(Duration::from_secs(config.heartbeat_interval_secs));

        loop {
            interval.tick().await;

            // Send heartbeat
            let message = DiscoveryMessage::Heartbeat {
                peer_id: local_peer_id.clone(),
            };

            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to bind UDP socket for broadcast: {}", e);
                    continue;
                }
            };

            if let Err(e) = socket.set_broadcast(true) {
                warn!("Failed to enable broadcast: {}", e);
                continue;
            }

            let data = match serde_json::to_vec(&message) {
                Ok(d) => d,
                Err(e) => {
                    warn!("Failed to serialize heartbeat: {}", e);
                    continue;
                }
            };

            let broadcast_addr = SocketAddr::new(
                IpAddr::from([255, 255, 255, 255]),
                config.broadcast_port,
            );

            if let Err(e) = socket.send_to(&data, broadcast_addr) {
                warn!("Failed to send heartbeat: {}", e);
            } else {
                debug!("Sent heartbeat: {}", local_peer_id);
            }
        }
    }

    /// Background task for cleaning up stale peers
    async fn cleanup_task(
        peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
        config: DiscoveryConfig,
    ) {
        let mut interval = interval(Duration::from_secs(config.worker_timeout_secs));

        loop {
            interval.tick().await;

            let now = Utc::now();
            let timeout_duration = Duration::from_secs(config.worker_timeout_secs);

            let mut peers_guard = peers.write().await;
            peers_guard.retain(|peer_id, peer_info| {
                let elapsed = now.signed_duration_since(peer_info.last_seen);
                let is_valid = elapsed.num_seconds() < timeout_duration.as_secs() as i64;
                
                if !is_valid {
                    info!("Removing stale peer: {} (last seen: {})", peer_id, peer_info.last_seen);
                }
                
                is_valid
            });
        }
    }

    /// Gets all discovered peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Gets a specific peer by ID
    pub async fn get_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        let peers = self.peers.read().await;
        peers.get(peer_id).cloned()
    }

    /// Gets the local peer ID
    pub fn local_peer_id(&self) -> &str {
        &self.local_peer_id
    }

    /// Handles an incoming discovery message (called by UDP listener)
    pub async fn handle_message(&self, message: DiscoveryMessage, source: SocketAddr) -> Result<(), AppError> {
        // Ignore our own messages
        if source == self.local_address {
            return Ok(());
        }

        match message {
            DiscoveryMessage::Announce { peer_id, address, port, capabilities, metadata } => {
                let peer_info = PeerInfo {
                    peer_id: peer_id.clone(),
                    address,
                    port,
                    last_seen: Utc::now(),
                    capabilities,
                    metadata,
                };

                let mut peers = self.peers.write().await;
                if peers.contains_key(&peer_id) {
                    debug!("Updated peer: {} (last seen: {})", peer_id, peer_info.last_seen);
                } else {
                    info!("Discovered new peer: {} at {}:{}", peer_id, peer_info.address, port);
                }
                peers.insert(peer_id, peer_info);
            }
            DiscoveryMessage::Heartbeat { peer_id } => {
                let mut peers = self.peers.write().await;
                if let Some(peer_info) = peers.get_mut(&peer_id) {
                    peer_info.last_seen = Utc::now();
                    debug!("Received heartbeat from: {}", peer_id);
                }
            }
            DiscoveryMessage::Query => {
                // Respond with list of peers
                let peers_list = self.get_peers().await;
                let response = DiscoveryMessage::Response {
                    peers: peers_list,
                };
                
                // Send response back to source
                let socket = UdpSocket::bind("0.0.0.0:0")
                    .map_err(|e| AppError::NetworkError(format!("Failed to bind UDP socket: {}", e)))?;
                
                let data = serde_json::to_vec(&response)
                    .map_err(|e| AppError::ConfigError(format!("Failed to serialize response: {}", e)))?;

                socket.send_to(&data, source)
                    .map_err(|e| AppError::NetworkError(format!("Failed to send response: {}", e)))?;
            }
            DiscoveryMessage::Response { peers } => {
                // Update peer list from response
                let mut peers_guard = self.peers.write().await;
                for peer in peers {
                    peers_guard.insert(peer.peer_id.clone(), peer);
                }
            }
        }

        Ok(())
    }
}

/// Global discovery service instance
static GLOBAL_DISCOVERY: OnceLock<Arc<DiscoveryService>> = OnceLock::new();

/// Get or initialize the global discovery service
pub fn get_global_discovery_service() -> Option<&'static Arc<DiscoveryService>> {
    GLOBAL_DISCOVERY.get()
}

/// Initialize the global discovery service
pub fn initialize_global_discovery(
    config: DiscoveryConfig,
    local_address: SocketAddr,
) -> Result<Arc<DiscoveryService>, AppError> {
    let service = Arc::new(DiscoveryService::new(config, local_address));
    
    GLOBAL_DISCOVERY.set(service.clone())
        .map_err(|_| AppError::ConfigError(
            "Global discovery service already initialized".to_string()
        ))?;
    
    Ok(service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[tokio::test]
    async fn test_discovery_service_creation() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));
        let config = DiscoveryConfig::default();
        let service = DiscoveryService::new(config, addr);

        assert!(!service.local_peer_id().is_empty());
        assert!(service.local_peer_id().starts_with("poolai-"));
    }

    #[tokio::test]
    async fn test_discovery_message_serialization() {
        let message = DiscoveryMessage::Announce {
            peer_id: "test-peer".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            capabilities: PeerCapabilities::default(),
            metadata: HashMap::new(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: DiscoveryMessage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            DiscoveryMessage::Announce { peer_id, address, port, .. } => {
                assert_eq!(peer_id, "test-peer");
                assert_eq!(address, "127.0.0.1");
                assert_eq!(port, 8080);
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
