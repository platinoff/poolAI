//! Distributed RAID Protocol Client
//!
//! Provides client functionality for node-to-node communication
//! in the distributed RAID system.

use crate::core::error::AppError;
use crate::raid::circuit_breaker::CircuitBreakerManager;
use crate::raid::protocol::*;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;

/// Protocol client for distributed RAID communication
pub struct ProtocolClient {
    client: Client,
    base_url: String,
    node_id: String,
    #[allow(dead_code)] // Reserved for future use
    timeout: Duration,
    /// Circuit breaker manager for fault tolerance
    circuit_breaker_manager: Arc<RwLock<CircuitBreakerManager>>,
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
            circuit_breaker_manager: Arc::new(RwLock::new(CircuitBreakerManager::with_defaults())),
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
            circuit_breaker_manager: Arc::new(RwLock::new(CircuitBreakerManager::with_defaults())),
        }
    }

    /// Send a protocol message and get response
    ///
    /// This method integrates with circuit breaker for fault tolerance.
    /// Circuit breaker prevents cascading failures by rejecting requests
    /// when a node is detected as failing.
    async fn send_message<T>(&self, endpoint: &str, message: ProtocolMessage) -> Result<T, AppError>
    where
        T: serde::de::DeserializeOwned,
    {
        // Use base_url hash as node identifier for circuit breaker
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.base_url.hash(&mut hasher);
        let node_id = hasher.finish() as u64;

        // Get or create circuit breaker for this node
        let breaker = {
            let manager = self.circuit_breaker_manager.read().await;
            manager.get_or_create(node_id).await
        };

        // Check if request is allowed
        breaker.allow_request().await?;

        let url = format!("{}/api/v1/raid/distributed{}", self.base_url, endpoint);

        let json = message.to_json().map_err(|e| {
            AppError::ValidationError(format!(
                "Failed to serialize protocol message: {}. \
                Context: Unable to serialize the protocol message to JSON format. \
                Suggestion: Verify the message structure is valid and all required fields are present. \
                Message type: {}, Error: {}",
                e, message.message_type, e
            ))
        })?;

        info!(
            "Sending protocol message to {}: {}",
            url, message.message_type
        );

        // Attempt to send request
        let result = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await;

        let response = match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Record success
                    breaker.record_success().await;
                    resp
                } else {
                    // Record failure for non-success status
                    breaker.record_failure().await;
                    let status = resp.status();
                    let error_text = resp.text().await.unwrap_or_default();
                    return Err(AppError::NetworkError(format!(
                        "Request failed with status {}: {}",
                        status, error_text
                    )));
                }
            }
            Err(e) => {
                // Record failure for network errors
                breaker.record_failure().await;
                return Err(AppError::NetworkError(format!(
                    "Failed to send request: {}",
                    e
                )));
            }
        };

        let response_json: serde_json::Value = match response.json().await {
            Ok(json) => json,
            Err(e) => {
                breaker.record_failure().await;
                return Err(AppError::NetworkError(format!(
                    "Failed to parse response: {}",
                    e
                )));
            }
        };

        // Extract payload from response message
        let payload = match response_json.get("payload") {
            Some(p) => p,
            None => {
                breaker.record_failure().await;
                return Err(AppError::ValidationError(
                    "Response missing payload".to_string(),
                ));
            }
        };

        match serde_json::from_value(payload.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                breaker.record_failure().await;
                Err(AppError::ValidationError(format!(
                    "Failed to deserialize response: {}",
                    e
                )))
            }
        }
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
            use base64::{engine::general_purpose, Engine as _};
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
        let payload = JoinClusterPayload { address, node_info };

        let message = ProtocolMessage::join_cluster(self.node_id.clone(), payload)?;
        self.send_message("/cluster/join", message).await
    }

    /// Leave a cluster
    pub async fn leave_cluster(
        &self,
        reason: LeaveReason,
        graceful: bool,
    ) -> Result<LeaveClusterResponse, AppError> {
        let payload = LeaveClusterPayload { reason, graceful };

        let message = ProtocolMessage::leave_cluster(self.node_id.clone(), payload)?;
        self.send_message("/cluster/leave", message).await
    }

    /// Get circuit breaker manager reference
    pub fn circuit_breaker_manager(&self) -> &Arc<RwLock<CircuitBreakerManager>> {
        &self.circuit_breaker_manager
    }

    /// Get circuit breaker state for this client's node
    pub async fn circuit_breaker_state(&self) -> crate::raid::circuit_breaker::CircuitState {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.base_url.hash(&mut hasher);
        let node_id = hasher.finish() as u64;

        let manager = self.circuit_breaker_manager.read().await;
        if let Some(breaker) = manager.get(node_id).await {
            breaker.state().await
        } else {
            crate::raid::circuit_breaker::CircuitState::Closed
        }
    }
}

/// Helper function to encode data to base64
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_protocol_client_creation() {
        let client =
            ProtocolClient::new("http://localhost:8080".to_string(), "test-node".to_string());

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
