//! Raft Network Transport for Distributed RAID
//!
//! This module provides HTTP/HTTPS transport for async-raft,
//! allowing Raft nodes to communicate over the existing REST API.

#[cfg(feature = "raft")]
use anyhow::Result;
#[cfg(feature = "raft")]
use async_raft::{
    network::RaftNetwork,
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    AppData, NodeId,
};
#[cfg(feature = "raft")]
use async_trait::async_trait;
#[cfg(feature = "raft")]
use reqwest::Client;
#[cfg(feature = "raft")]
use std::sync::Arc;
#[cfg(feature = "raft")]
use tokio::sync::RwLock;
#[cfg(feature = "raft")]
use tracing::info;

/// Node address mapping
#[cfg(feature = "raft")]
type NodeAddress = String; // e.g., "http://192.168.1.100:8080"

/// HTTP/HTTPS transport for Raft network communication
#[cfg(feature = "raft")]
#[derive(Clone)]
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
        let address_clone = address.clone();
        let mut addresses = self.node_addresses.write().await;
        addresses.insert(node_id, address);
        info!("Added Raft node {} at address {}", node_id, address_clone);
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

/// Implement RaftNetwork trait for HTTP/HTTPS transport
#[cfg(feature = "raft")]
#[async_trait]
impl<D> RaftNetwork<D> for HttpRaftTransport
where
    D: AppData + Send + Sync + 'static,
{
    /// Send an AppendEntries RPC to the target Raft node
    async fn append_entries(
        &self,
        target: NodeId,
        rpc: AppendEntriesRequest<D>,
    ) -> Result<AppendEntriesResponse> {
        let address = self
            .get_node_address(target)
            .await
            .ok_or_else(|| anyhow::anyhow!("Node {} not found in cluster", target))?;

        let url = format!("{}/raft/append-entries", address);
        let response = self
            .client
            .post(&url)
            .json(&rpc)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send append_entries request: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "AppendEntries request failed with status: {}",
                response.status()
            ));
        }

        let result: AppendEntriesResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse AppendEntriesResponse: {}", e))?;

        Ok(result)
    }

    /// Send an InstallSnapshot RPC to the target Raft node
    async fn install_snapshot(
        &self,
        target: NodeId,
        rpc: InstallSnapshotRequest,
    ) -> Result<InstallSnapshotResponse> {
        let address = self
            .get_node_address(target)
            .await
            .ok_or_else(|| anyhow::anyhow!("Node {} not found in cluster", target))?;

        let url = format!("{}/raft/install-snapshot", address);
        let response = self
            .client
            .post(&url)
            .json(&rpc)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send install_snapshot request: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "InstallSnapshot request failed with status: {}",
                response.status()
            ));
        }

        let result: InstallSnapshotResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse InstallSnapshotResponse: {}", e))?;

        Ok(result)
    }

    /// Send a RequestVote RPC to the target Raft node
    async fn vote(&self, target: NodeId, rpc: VoteRequest) -> Result<VoteResponse> {
        let address = self
            .get_node_address(target)
            .await
            .ok_or_else(|| anyhow::anyhow!("Node {} not found in cluster", target))?;

        let url = format!("{}/raft/vote", address);
        let response = self
            .client
            .post(&url)
            .json(&rpc)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send vote request: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Vote request failed with status: {}",
                response.status()
            ));
        }

        let result: VoteResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse VoteResponse: {}", e))?;

        Ok(result)
    }
}
