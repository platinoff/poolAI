//! Distributed RAID Protocol Client
//!
//! Provides client functionality for node-to-node communication
//! in the distributed RAID system.

use crate::core::error::AppError;
use crate::raid::protocol::*;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::info;

/// Protocol client for distributed RAID communication
pub struct ProtocolClient {
    client: Client,
    base_url: String,
    node_id: String,
    timeout: Duration,
}

impl ProtocolClient {
    /// Create a new protocol client
    pub fn new(base_url: String, node_id: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            node_id,
            timeout: Duration::from_secs(30),
        }
    }

    /// Create a client with custom timeout
    pub fn with_timeout(base_url: String, node_id: String, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            node_id,
            timeout,
        }
    }

    /// Send a protocol message and get response
    async fn send_message<T>(
        &self,
        endpoint: &str,
        message: ProtocolMessage,
    ) -> Result<T, AppError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/api/v1/raid/distributed{}", self.base_url, endpoint);
        
        let json = message.to_json()
            .map_err(|e| AppError::ValidationError(format!("Failed to serialize message: {}", e)))?;

        info!("Sending protocol message to {}: {}", url, message.message_type);

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::NetworkError(format!(
                "Request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await
            .map_err(|e| AppError::NetworkError(format!("Failed to parse response: {}", e)))?;

        // Extract payload from response message
        let payload = response_json.get("payload")
            .ok_or_else(|| AppError::ValidationError("Response missing payload".to_string()))?;

        serde_json::from_value(payload.clone())
            .map_err(|e| AppError::ValidationError(format!("Failed to deserialize response: {}", e)))
    }

    /// Replicate an artifact to another node
    pub async fn put_artifact(
        &self,
        artifact_id: String,
        data: Option<Vec<u8>>,
        metadata: ArtifactMetadata,
        replication_factor: u32,
        sync_mode: SyncMode,
    ) -> Result<PutArtifactResponse, AppError> {
        let data_base64 = data.map(|d| {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.encode(&d)
        });
        
        let payload = PutArtifactPayload {
            artifact_id: artifact_id.clone(),
            source_node: self.node_id.clone(),
            data: data_base64,
            metadata,
            replication_factor,
            sync_mode,
        };

        let message = ProtocolMessage::put_artifact(self.node_id.clone(), payload)?;
        self.send_message("/artifacts/replicate", message).await
    }

    /// Get an artifact from another node
    pub async fn get_artifact(
        &self,
        artifact_id: String,
        include_data: bool,
    ) -> Result<GetArtifactResponse, AppError> {
        let payload = GetArtifactPayload {
            artifact_id,
            include_data,
        };

        let message = ProtocolMessage::get_artifact(self.node_id.clone(), payload)?;
        self.send_message("/artifacts/get", message).await
    }

    /// Delete an artifact from another node
    pub async fn delete_artifact(
        &self,
        artifact_id: String,
        propagate: bool,
    ) -> Result<DeleteArtifactResponse, AppError> {
        let payload = DeleteArtifactPayload {
            artifact_id,
            propagate,
        };

        let message = ProtocolMessage::delete_artifact(self.node_id.clone(), payload)?;
        self.send_message("/artifacts/delete", message).await
    }

    /// Synchronize artifacts with another node
    pub async fn sync_artifacts(
        &self,
        last_sync_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        artifact_ids: Option<Vec<String>>,
        direction: SyncDirection,
    ) -> Result<SyncArtifactsResponse, AppError> {
        let payload = SyncArtifactsPayload {
            last_sync_timestamp,
            artifact_ids,
            direction,
        };

        let message = ProtocolMessage::sync_artifacts(self.node_id.clone(), payload)?;
        self.send_message("/artifacts/sync", message).await
    }

    /// Check health status of another node
    pub async fn health_check(&self) -> Result<HealthCheckResponse, AppError> {
        let message = ProtocolMessage::health_check(self.node_id.clone());
        self.send_message("/health", message).await
    }

    /// Join a cluster
    pub async fn join_cluster(
        &self,
        address: String,
        node_info: NodeInfo,
    ) -> Result<JoinClusterResponse, AppError> {
        let payload = JoinClusterPayload {
            address,
            node_info,
        };

        let message = ProtocolMessage::join_cluster(self.node_id.clone(), payload)?;
        self.send_message("/cluster/join", message).await
    }

    /// Leave a cluster
    pub async fn leave_cluster(
        &self,
        reason: LeaveReason,
        graceful: bool,
    ) -> Result<LeaveClusterResponse, AppError> {
        let payload = LeaveClusterPayload {
            reason,
            graceful,
        };

        let message = ProtocolMessage::leave_cluster(self.node_id.clone(), payload)?;
        self.send_message("/cluster/leave", message).await
    }
}

/// Helper function to encode data to base64
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_protocol_client_creation() {
        let client = ProtocolClient::new(
            "http://localhost:8080".to_string(),
            "test-node".to_string(),
        );
        
        assert_eq!(client.node_id, "test-node");
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_protocol_message_creation() {
        let node_id = "test-node".to_string();
        let metadata = ArtifactMetadata {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            size_bytes: 1024,
            checksum: "sha256-hash".to_string(),
            created_at: Utc::now(),
            content_type: None,
            tags: None,
        };

        let payload = PutArtifactPayload {
            artifact_id: "artifact-123".to_string(),
            source_node: node_id.clone(),
            data: Some("base64data".to_string()),
            metadata,
            replication_factor: 3,
            sync_mode: SyncMode::Sync,
        };

        let message = ProtocolMessage::put_artifact(node_id, payload).unwrap();
        assert_eq!(message.message_type, "put_artifact");
    }
}

