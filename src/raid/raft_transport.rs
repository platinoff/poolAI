//! Raft Network Transport for Distributed RAID
//!
//! This module provides HTTP/HTTPS transport for async-raft,
//! allowing Raft nodes to communicate over the existing REST API.

#[cfg(feature = "raft")]
use crate::raid::raft::{RaidRaftOperation, RaidRaftResponse};
#[cfg(feature = "raft")]
use async_raft::{
    network::RaftNetworkError, raft::RaftRequest, raft::RaftResponse, NodeId, RaftNetwork,
};
#[cfg(feature = "raft")]
use reqwest::Client;
#[cfg(feature = "raft")]
use serde_json;
#[cfg(feature = "raft")]
use std::sync::Arc;
#[cfg(feature = "raft")]
use tokio::sync::RwLock;
#[cfg(feature = "raft")]
use tracing::{info, warn};

/// Node address mapping
#[cfg(feature = "raft")]
type NodeAddress = String; // e.g., "http://192.168.1.100:8080"

/// HTTP/HTTPS transport for Raft network communication
#[cfg(feature = "raft")]
pub struct HttpRaftTransport {
    /// HTTP client for making requests
    client: Client,
    /// Mapping of node IDs to their addresses
    node_addresses: Arc<RwLock<std::collections::HashMap<NodeId, NodeAddress>>>,
}

#[cfg(feature = "raft")]
impl HttpRaftTransport {
    /// Create a new HTTP Raft transport
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            node_addresses: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Add or update a node address
    pub async fn add_node(&self, node_id: NodeId, address: NodeAddress) {
        let mut addresses = self.node_addresses.write().await;
        addresses.insert(node_id, address);
        info!("Added Raft node {} at address {}", node_id, address);
    }

    /// Remove a node address
    pub async fn remove_node(&self, node_id: NodeId) {
        let mut addresses = self.node_addresses.write().await;
        addresses.remove(&node_id);
        info!("Removed Raft node {}", node_id);
    }

    /// Get node address
    pub async fn get_node_address(&self, node_id: NodeId) -> Option<NodeAddress> {
        let addresses = self.node_addresses.read().await;
        addresses.get(&node_id).cloned()
    }
}

#[cfg(feature = "raft")]
impl RaftNetwork<RaidRaftOperation, RaidRaftResponse> for HttpRaftTransport {
    async fn append_entries(
        &self,
        target: NodeId,
        rpc: RaftRequest<RaidRaftOperation>,
    ) -> Result<RaftResponse<RaidRaftResponse>, RaftNetworkError> {
        let address = match self.get_node_address(target).await {
            Some(addr) => addr,
            None => {
                return Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Node {} address not found", target),
                });
            }
        };

        let url = format!("{}/raft/append-entries", address);

        // Serialize the Raft request
        let body = match serde_json::to_vec(&rpc) {
            Ok(b) => b,
            Err(e) => {
                return Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Failed to serialize request: {}", e),
                });
            }
        };

        // Send HTTP POST request
        match self.client.post(&url).body(body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<RaftResponse<RaidRaftResponse>>().await {
                        Ok(raft_response) => Ok(raft_response),
                        Err(e) => Err(RaftNetworkError::Unreachable {
                            target,
                            reason: format!("Failed to deserialize response: {}", e),
                        }),
                    }
                } else {
                    Err(RaftNetworkError::Unreachable {
                        target,
                        reason: format!("HTTP error: {}", response.status()),
                    })
                }
            }
            Err(e) => {
                warn!("Failed to send append entries to node {}: {}", target, e);
                Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Network error: {}", e),
                })
            }
        }
    }

    async fn install_snapshot(
        &self,
        target: NodeId,
        rpc: RaftRequest<RaidRaftOperation>,
    ) -> Result<RaftResponse<RaidRaftResponse>, RaftNetworkError> {
        // Similar to append_entries but for snapshots
        // For now, delegate to append_entries
        // TODO: Implement proper snapshot transfer
        self.append_entries(target, rpc).await
    }

    async fn vote(
        &self,
        target: NodeId,
        rpc: RaftRequest<RaidRaftOperation>,
    ) -> Result<RaftResponse<RaidRaftResponse>, RaftNetworkError> {
        let address = match self.get_node_address(target).await {
            Some(addr) => addr,
            None => {
                return Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Node {} address not found", target),
                });
            }
        };

        let url = format!("{}/raft/vote", address);

        let body = match serde_json::to_vec(&rpc) {
            Ok(b) => b,
            Err(e) => {
                return Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Failed to serialize request: {}", e),
                });
            }
        };

        match self.client.post(&url).body(body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<RaftResponse<RaidRaftResponse>>().await {
                        Ok(raft_response) => Ok(raft_response),
                        Err(e) => Err(RaftNetworkError::Unreachable {
                            target,
                            reason: format!("Failed to deserialize response: {}", e),
                        }),
                    }
                } else {
                    Err(RaftNetworkError::Unreachable {
                        target,
                        reason: format!("HTTP error: {}", response.status()),
                    })
                }
            }
            Err(e) => {
                warn!("Failed to send vote to node {}: {}", target, e);
                Err(RaftNetworkError::Unreachable {
                    target,
                    reason: format!("Network error: {}", e),
                })
            }
        }
    }
}
