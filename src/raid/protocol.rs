//! Distributed RAID Protocol
//!
//! Defines message types and structures for node-to-node communication
//! in the distributed RAID system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Protocol message wrapper
///
/// Encapsulates all messages sent between nodes in the distributed RAID system.
/// Messages include type, ID, timestamp, node ID, and a JSON payload.
///
/// # Example
///
/// ```rust
/// use poolai::raid::protocol::{ProtocolMessage, PutArtifactPayload, ArtifactMetadata, SyncMode};
/// use chrono::Utc;
///
/// let metadata = ArtifactMetadata {
///     name: "my-model".to_string(),
///     version: "1.0.0".to_string(),
///     size_bytes: 1024,
///     checksum: "sha256-hash".to_string(),
///     created_at: Utc::now(),
///     content_type: None,
///     tags: None,
/// };
///
/// let payload = PutArtifactPayload {
///     artifact_id: "artifact-123".to_string(),
///     source_node: "node-123".to_string(),
///     data: Some("base64data".to_string()),
///     metadata,
///     replication_factor: 3,
///     sync_mode: SyncMode::Sync,
/// };
///
/// let message = ProtocolMessage::put_artifact("node-123".to_string(), payload).unwrap();
/// let json = message.to_json().unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub node_id: String,
    pub payload: serde_json::Value,
}

impl ProtocolMessage {
    pub fn new(message_type: String, node_id: String, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            message_type,
            node_id,
            payload,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// PutArtifact message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutArtifactPayload {
    pub artifact_id: String,
    pub source_node: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>, // Base64-encoded data
    pub metadata: ArtifactMetadata,
    pub replication_factor: u32,
    pub sync_mode: SyncMode,
}

/// Artifact metadata
///
/// Contains descriptive information about an artifact including name, version,
/// size, checksum, and optional tags.
///
/// # Example
///
/// ```rust
/// use poolai::raid::protocol::ArtifactMetadata;
/// use chrono::Utc;
///
/// let metadata = ArtifactMetadata {
///     name: "llama-2-7b".to_string(),
///     version: "1.0.0".to_string(),
///     size_bytes: 13_000_000_000, // 13 GB
///     checksum: "sha256:abc123...".to_string(),
///     created_at: Utc::now(),
///     content_type: Some("application/octet-stream".to_string()),
///     tags: Some(vec!["model".to_string(), "llama".to_string()]),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    pub name: String,
    pub version: String,
    pub size_bytes: u64,
    pub checksum: String, // SHA-256 hash
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Synchronization mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncMode {
    Sync,
    Async,
}

/// PutArtifact response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutArtifactResponse {
    pub status: OperationStatus,
    pub artifact_id: String,
    pub stored_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// GetArtifact message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtifactPayload {
    pub artifact_id: String,
    pub include_data: bool,
}

/// GetArtifact response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtifactResponse {
    pub status: ArtifactStatus,
    pub artifact_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ArtifactMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>, // Base64-encoded data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// DeleteArtifact message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArtifactPayload {
    pub artifact_id: String,
    pub propagate: bool,
}

/// DeleteArtifact response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArtifactResponse {
    pub status: OperationStatus,
    pub artifact_id: String,
    pub deleted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// SyncArtifacts message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncArtifactsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_ids: Option<Vec<String>>,
    /// Optional peer versions (`artifact_id` -> `stored_at`) used to detect conflicts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_versions: Option<HashMap<String, DateTime<Utc>>>,
    pub direction: SyncDirection,
}

/// Synchronization direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncDirection {
    Pull,
    Push,
    Bidirectional,
}

/// SyncArtifacts response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncArtifactsResponse {
    pub status: OperationStatus,
    pub synced_count: u32,
    pub missing_artifacts: Vec<String>,
    pub conflicts: Vec<ArtifactConflict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Artifact conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactConflict {
    pub artifact_id: String,
    pub reason: String,
    pub local_version: DateTime<Utc>,
    pub remote_version: DateTime<Utc>,
}

/// HealthCheck message payload (empty)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckPayload {}

/// HealthCheck response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: HealthStatus,
    pub uptime_seconds: u64,
    pub storage_used_bytes: u64,
    pub storage_total_bytes: u64,
    pub artifact_count: u32,
    pub raft_role: RaftRole,
    pub raft_term: u64,
    pub last_heartbeat: DateTime<Utc>,
}

/// Health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Raft role
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RaftRole {
    Leader,
    Follower,
    Candidate,
}

/// JoinCluster message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinClusterPayload {
    pub address: String,
    pub node_info: NodeInfo,
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub storage_capacity_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// JoinCluster response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinClusterResponse {
    pub status: JoinStatus,
    pub cluster_id: String,
    pub member_nodes: Vec<ClusterNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Join status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JoinStatus {
    Accepted,
    Rejected,
}

/// Cluster node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNode {
    pub node_id: String,
    pub address: String,
    pub role: RaftRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HealthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<Utc>>,
}

/// LeaveCluster message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveClusterPayload {
    pub reason: LeaveReason,
    pub graceful: bool,
}

/// Leave reason
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LeaveReason {
    Shutdown,
    Maintenance,
    Error,
}

/// LeaveCluster response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveClusterResponse {
    pub status: OperationStatus,
    pub replication_complete: bool,
    pub artifacts_moved: u32,
}

/// Operation status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OperationStatus {
    Success,
    Error,
}

/// Artifact status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatus {
    Success,
    NotFound,
    Error,
}

/// Protocol error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolError {
    pub error_code: ErrorCode,
    pub error_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Error codes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    ArtifactNotFound,
    NodeUnavailable,
    ReplicationFailed,
    InsufficientStorage,
    AuthenticationFailed,
    AuthorizationFailed,
    InvalidRequest,
    ClusterFull,
    RaftError,
}

/// Helper functions for creating protocol messages
impl ProtocolMessage {
    pub fn put_artifact(
        node_id: String,
        payload: PutArtifactPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new("put_artifact".to_string(), node_id, payload_json))
    }

    pub fn get_artifact(
        node_id: String,
        payload: GetArtifactPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new("get_artifact".to_string(), node_id, payload_json))
    }

    pub fn delete_artifact(
        node_id: String,
        payload: DeleteArtifactPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new(
            "delete_artifact".to_string(),
            node_id,
            payload_json,
        ))
    }

    pub fn sync_artifacts(
        node_id: String,
        payload: SyncArtifactsPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new(
            "sync_artifacts".to_string(),
            node_id,
            payload_json,
        ))
    }

    pub fn health_check(node_id: String) -> Self {
        let payload = serde_json::json!({});
        Self::new("health_check".to_string(), node_id, payload)
    }

    pub fn join_cluster(
        node_id: String,
        payload: JoinClusterPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new("join_cluster".to_string(), node_id, payload_json))
    }

    pub fn leave_cluster(
        node_id: String,
        payload: LeaveClusterPayload,
    ) -> Result<Self, serde_json::Error> {
        let payload_json = serde_json::to_value(payload)?;
        Ok(Self::new(
            "leave_cluster".to_string(),
            node_id,
            payload_json,
        ))
    }
}

/// Helper functions for extracting payloads
impl ProtocolMessage {
    pub fn extract_put_artifact(&self) -> Result<PutArtifactPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }

    pub fn extract_get_artifact(&self) -> Result<GetArtifactPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }

    pub fn extract_delete_artifact(&self) -> Result<DeleteArtifactPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }

    pub fn extract_sync_artifacts(&self) -> Result<SyncArtifactsPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }

    pub fn extract_join_cluster(&self) -> Result<JoinClusterPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }

    pub fn extract_leave_cluster(&self) -> Result<LeaveClusterPayload, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_message_serialization() {
        let node_id = "node-123".to_string();
        let payload = PutArtifactPayload {
            artifact_id: "artifact-456".to_string(),
            source_node: "node-123".to_string(),
            data: Some("base64data".to_string()),
            metadata: ArtifactMetadata {
                name: "test-library".to_string(),
                version: "1.0.0".to_string(),
                size_bytes: 1024,
                checksum: "sha256-hash".to_string(),
                created_at: Utc::now(),
                content_type: None,
                tags: None,
            },
            replication_factor: 3,
            sync_mode: SyncMode::Sync,
        };

        let msg = ProtocolMessage::put_artifact(node_id.clone(), payload).unwrap();
        let json = msg.to_json().unwrap();
        let deserialized = ProtocolMessage::from_json(&json).unwrap();

        assert_eq!(msg.message_type, deserialized.message_type);
        assert_eq!(msg.node_id, deserialized.node_id);
    }

    #[test]
    fn test_health_check_message() {
        let node_id = "node-123".to_string();
        let msg = ProtocolMessage::health_check(node_id.clone());

        assert_eq!(msg.message_type, "health_check");
        assert_eq!(msg.node_id, node_id);
    }

    #[test]
    fn test_sync_mode_serialization() {
        let sync = SyncMode::Sync;
        let json = serde_json::to_string(&sync).unwrap();
        assert_eq!(json, "\"sync\"");

        let async_mode = SyncMode::Async;
        let json = serde_json::to_string(&async_mode).unwrap();
        assert_eq!(json, "\"async\"");
    }

    #[test]
    fn test_health_status_serialization() {
        let healthy = HealthStatus::Healthy;
        let json = serde_json::to_string(&healthy).unwrap();
        assert_eq!(json, "\"healthy\"");
    }

    #[test]
    fn test_raft_role_serialization() {
        let leader = RaftRole::Leader;
        let json = serde_json::to_string(&leader).unwrap();
        assert_eq!(json, "\"leader\"");
    }
}
