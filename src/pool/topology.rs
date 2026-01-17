//! Network topology management for load balancing
//!
//! This module provides:
//! - Latency matrix between nodes
//! - Network bandwidth measurement
//! - Topology-aware placement strategies
//!
//! Inspired by exo's topology-aware load balancing feature.

use crate::core::error::AppError;
use crate::network::discovery::{get_global_discovery_service, PeerInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// Network topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    /// Latency matrix between nodes (in milliseconds)
    /// Key format: "node_id1:node_id2"
    pub latency_matrix: HashMap<String, f64>,
    /// Bandwidth matrix between nodes (in Mbps)
    /// Key format: "node_id1:node_id2"
    pub bandwidth_matrix: HashMap<String, f64>,
    /// Resource information per node
    pub node_resources: HashMap<String, NodeResources>,
    /// Last topology update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Resource information for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    /// Node ID
    pub node_id: String,
    /// Available GPU memory (MB)
    pub available_gpu_memory_mb: u64,
    /// Total GPU memory (MB)
    pub total_gpu_memory_mb: u64,
    /// Available CPU cores
    pub available_cpu_cores: usize,
    /// Total CPU cores
    pub total_cpu_cores: usize,
    /// Available system memory (MB)
    pub available_memory_mb: u64,
    /// Total system memory (MB)
    pub total_memory_mb: u64,
    /// Current load (0.0-1.0)
    pub current_load: f32,
}

/// Topology manager
pub struct TopologyManager {
    topology: Arc<RwLock<Topology>>,
    /// Latency measurement timeout (seconds)
    latency_timeout_secs: u64,
    /// Interval for topology updates (seconds)
    update_interval_secs: u64,
}

impl TopologyManager {
    /// Create a new topology manager
    pub fn new() -> Self {
        Self {
            topology: Arc::new(RwLock::new(Topology {
                latency_matrix: HashMap::new(),
                bandwidth_matrix: HashMap::new(),
                node_resources: HashMap::new(),
                last_updated: Utc::now(),
            })),
            latency_timeout_secs: 5,
            update_interval_secs: 30,
        }
    }

    /// Get latency between two nodes (in milliseconds)
    pub async fn get_latency(&self, node_id1: &str, node_id2: &str) -> Option<f64> {
        let topology = self.topology.read().await;
        
        // Try direct path
        let key = format!("{}:{}", node_id1, node_id2);
        if let Some(&latency) = topology.latency_matrix.get(&key) {
            return Some(latency);
        }

        // Try reverse path (latency is symmetric)
        let reverse_key = format!("{}:{}", node_id2, node_id1);
        if let Some(&latency) = topology.latency_matrix.get(&reverse_key) {
            return Some(latency);
        }

        None
    }

    /// Get bandwidth between two nodes (in Mbps)
    pub async fn get_bandwidth(&self, node_id1: &str, node_id2: &str) -> Option<f64> {
        let topology = self.topology.read().await;
        
        let key = format!("{}:{}", node_id1, node_id2);
        topology.bandwidth_matrix.get(&key).copied().or_else(|| {
            // Try reverse path
            let reverse_key = format!("{}:{}", node_id2, node_id1);
            topology.bandwidth_matrix.get(&reverse_key).copied()
        })
    }

    /// Get resource information for a node
    pub async fn get_node_resources(&self, node_id: &str) -> Option<NodeResources> {
        let topology = self.topology.read().await;
        topology.node_resources.get(node_id).cloned()
    }

    /// Measure latency to a peer (simplified ping-based)
    async fn measure_latency(&self, peer: &PeerInfo) -> Option<f64> {
        // Simplified latency measurement using TCP connect time
        // In real implementation, this would use ICMP ping or HTTP ping
        let addr = format!("{}:{}", peer.address, peer.port);
        
        // Try to establish a TCP connection and measure time
        let start = std::time::Instant::now();
        let result = timeout(
            Duration::from_secs(self.latency_timeout_secs),
            tokio::net::TcpStream::connect(&addr),
        ).await;

        match result {
            Ok(Ok(_stream)) => {
                let elapsed = start.elapsed();
                Some(elapsed.as_secs_f64() * 1000.0) // Convert to milliseconds
            }
            Ok(Err(_)) => {
                warn!("Failed to connect to {} for latency measurement", addr);
                None
            }
            Err(_) => {
                warn!("Latency measurement timeout for {}", addr);
                None
            }
        }
    }

    /// Update topology with discovered peers
    pub async fn update_topology(&self) -> Result<(), AppError> {
        info!("Updating network topology");

        // Get discovered peers from discovery service
        let peers = if let Some(discovery) = get_global_discovery_service() {
            discovery.get_peers().await
        } else {
            Vec::new()
        };

        let mut topology = self.topology.write().await;

        // Update node resources from peers
        for peer in &peers {
            let resources = NodeResources {
                node_id: peer.peer_id.clone(),
                available_gpu_memory_mb: peer.capabilities.memory_mb as u64, // Simplified
                total_gpu_memory_mb: peer.capabilities.memory_mb as u64,
                available_cpu_cores: peer.capabilities.cpu_cores,
                total_cpu_cores: peer.capabilities.cpu_cores,
                available_memory_mb: peer.capabilities.memory_mb as u64,
                total_memory_mb: peer.capabilities.memory_mb as u64,
                current_load: 0.0, // TODO: Get from peer status
            };

            topology.node_resources.insert(peer.peer_id.clone(), resources);
        }

        // Measure latency between nodes
        let local_peer_id = get_global_discovery_service()
            .map(|d| d.local_peer_id().to_string())
            .unwrap_or_else(|| "local".to_string());

        for peer in &peers {
            if peer.peer_id == local_peer_id {
                continue;
            }

            let key = format!("{}:{}", local_peer_id, peer.peer_id);
            
            // Measure latency (non-blocking, don't fail if measurement fails)
            if let Some(latency) = self.measure_latency(peer).await {
                topology.latency_matrix.insert(key.clone(), latency);
                debug!("Latency to {}: {:.2}ms", peer.peer_id, latency);
            }

            // Estimate bandwidth (simplified - in real implementation would measure)
            // For now, use default values based on network type
            let bandwidth = 1000.0; // Default 1 Gbps for local network
            topology.bandwidth_matrix.insert(key, bandwidth);
        }

        topology.last_updated = Utc::now();
        
        info!(
            "Topology updated: {} nodes, {} latency measurements",
            topology.node_resources.len(),
            topology.latency_matrix.len()
        );

        Ok(())
    }

    /// Find best nodes for placement based on topology
    pub async fn find_best_nodes(
        &self,
        required_memory_mb: u64,
        required_cpu_cores: usize,
        node_count: usize,
    ) -> Vec<String> {
        let topology = self.topology.read().await;

        // Filter nodes by resource availability
        let mut candidates: Vec<_> = topology
            .node_resources
            .iter()
            .filter(|(_, resources)| {
                resources.available_gpu_memory_mb >= required_memory_mb
                    && resources.available_cpu_cores >= required_cpu_cores
                    && resources.current_load < 0.9 // Not overloaded
            })
            .collect();

        // Sort by current load (prefer less loaded nodes)
        candidates.sort_by(|a, b| {
            a.1.current_load
                .partial_cmp(&b.1.current_load)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top N nodes
        candidates
            .into_iter()
            .take(node_count)
            .map(|(node_id, _)| node_id.clone())
            .collect()
    }

    /// Get average latency between a set of nodes
    pub async fn get_average_latency(&self, node_ids: &[String]) -> Option<f64> {
        if node_ids.len() < 2 {
            return None;
        }

        let topology = self.topology.read().await;
        let mut total_latency = 0.0;
        let mut count = 0;

        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                let key = format!("{}:{}", node_ids[i], node_ids[j]);
                if let Some(&latency) = topology.latency_matrix.get(&key) {
                    total_latency += latency;
                    count += 1;
                }
            }
        }

        if count > 0 {
            Some(total_latency / count as f64)
        } else {
            None
        }
    }

    /// Get topology snapshot (for API access)
    pub async fn get_topology_snapshot(&self) -> Topology {
        let topology = self.topology.read().await;
        topology.clone()
    }

}

impl Default for TopologyManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global topology manager instance
static GLOBAL_TOPOLOGY_MANAGER: OnceLock<Arc<RwLock<TopologyManager>>> = OnceLock::new();

/// Initialize global topology manager
pub fn initialize_global_topology_manager() -> Result<(), AppError> {
    let manager = TopologyManager::new();
    GLOBAL_TOPOLOGY_MANAGER
        .set(Arc::new(RwLock::new(manager)))
        .map_err(|_| AppError::ConfigError(
            "Topology manager already initialized".to_string()
        ))?;
    Ok(())
}

/// Get global topology manager
pub fn get_global_topology_manager() -> Option<&'static Arc<RwLock<TopologyManager>>> {
    GLOBAL_TOPOLOGY_MANAGER.get()
}
