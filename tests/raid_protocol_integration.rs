//! Integration tests for Distributed RAID Protocol

use chrono::Utc;
use poolai::raid::protocol::*;

#[test]
fn test_put_artifact_message_flow() {
    let node_id = "test-node-1".to_string();
    let artifact_id = "artifact-123".to_string();

    let metadata = ArtifactMetadata {
        name: "test-library".to_string(),
        version: "1.0.0".to_string(),
        size_bytes: 1024,
        checksum: "sha256-hash-123".to_string(),
        created_at: Utc::now(),
        content_type: Some("application/octet-stream".to_string()),
        tags: Some(vec!["test".to_string(), "library".to_string()]),
    };

    let payload = PutArtifactPayload {
        artifact_id: artifact_id.clone(),
        source_node: node_id.clone(),
        data: Some("base64encodeddata".to_string()),
        metadata: metadata.clone(),
        replication_factor: 3,
        sync_mode: SyncMode::Sync,
    };

    let message = ProtocolMessage::put_artifact(node_id.clone(), payload).unwrap();

    // Verify message structure
    assert_eq!(message.message_type, "put_artifact");
    assert_eq!(message.node_id, node_id);

    // Extract and verify payload
    let extracted = message.extract_put_artifact().unwrap();
    assert_eq!(extracted.artifact_id, artifact_id);
    assert_eq!(extracted.metadata.name, metadata.name);
    assert_eq!(extracted.replication_factor, 3);
    assert_eq!(extracted.sync_mode, SyncMode::Sync);
}

#[test]
fn test_get_artifact_message_flow() {
    let node_id = "test-node-1".to_string();
    let artifact_id = "artifact-456".to_string();

    let payload = GetArtifactPayload {
        artifact_id: artifact_id.clone(),
        include_data: true,
    };

    let message = ProtocolMessage::get_artifact(node_id.clone(), payload).unwrap();

    assert_eq!(message.message_type, "get_artifact");

    let extracted = message.extract_get_artifact().unwrap();
    assert_eq!(extracted.artifact_id, artifact_id);
    assert!(extracted.include_data);
}

#[test]
fn test_delete_artifact_message_flow() {
    let node_id = "test-node-1".to_string();
    let artifact_id = "artifact-789".to_string();

    let payload = DeleteArtifactPayload {
        artifact_id: artifact_id.clone(),
        propagate: true,
    };

    let message = ProtocolMessage::delete_artifact(node_id.clone(), payload).unwrap();

    assert_eq!(message.message_type, "delete_artifact");

    let extracted = message.extract_delete_artifact().unwrap();
    assert_eq!(extracted.artifact_id, artifact_id);
    assert!(extracted.propagate);
}

#[test]
fn test_sync_artifacts_message_flow() {
    let node_id = "test-node-1".to_string();
    let last_sync = Utc::now();

    let payload = SyncArtifactsPayload {
        last_sync_timestamp: Some(last_sync),
        artifact_ids: Some(vec!["artifact-1".to_string(), "artifact-2".to_string()]),
        remote_versions: None,
        direction: SyncDirection::Bidirectional,
    };

    let message = ProtocolMessage::sync_artifacts(node_id.clone(), payload).unwrap();

    assert_eq!(message.message_type, "sync_artifacts");

    let extracted = message.extract_sync_artifacts().unwrap();
    assert_eq!(extracted.direction, SyncDirection::Bidirectional);
    assert!(extracted.artifact_ids.is_some());
    assert_eq!(extracted.artifact_ids.unwrap().len(), 2);
}

#[test]
fn test_join_cluster_message_flow() {
    let node_id = "new-node".to_string();

    let node_info = NodeInfo {
        storage_capacity_bytes: 107374182400, // 100 GB
        region: Some("us-east-1".to_string()),
        tags: Some(vec!["storage".to_string(), "compute".to_string()]),
    };

    let payload = JoinClusterPayload {
        address: "https://new-node.example.com:8080".to_string(),
        node_info: node_info.clone(),
    };

    let message = ProtocolMessage::join_cluster(node_id.clone(), payload).unwrap();

    assert_eq!(message.message_type, "join_cluster");

    let extracted = message.extract_join_cluster().unwrap();
    assert_eq!(extracted.address, "https://new-node.example.com:8080");
    assert_eq!(
        extracted.node_info.storage_capacity_bytes,
        node_info.storage_capacity_bytes
    );
}

#[test]
fn test_leave_cluster_message_flow() {
    let node_id = "leaving-node".to_string();

    let payload = LeaveClusterPayload {
        reason: LeaveReason::Maintenance,
        graceful: true,
    };

    let message = ProtocolMessage::leave_cluster(node_id.clone(), payload).unwrap();

    assert_eq!(message.message_type, "leave_cluster");

    let extracted = message.extract_leave_cluster().unwrap();
    assert_eq!(extracted.reason, LeaveReason::Maintenance);
    assert!(extracted.graceful);
}

#[test]
fn test_message_serialization_roundtrip() {
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
        data: Some("data".to_string()),
        metadata,
        replication_factor: 3,
        sync_mode: SyncMode::Async,
    };

    let message = ProtocolMessage::put_artifact(node_id.clone(), payload).unwrap();

    // Serialize to JSON
    let json = message.to_json().unwrap();

    // Deserialize from JSON
    let deserialized = ProtocolMessage::from_json(&json).unwrap();

    // Verify roundtrip
    assert_eq!(message.message_type, deserialized.message_type);
    assert_eq!(message.node_id, deserialized.node_id);

    // Verify payload
    let original_payload = message.extract_put_artifact().unwrap();
    let deserialized_payload = deserialized.extract_put_artifact().unwrap();
    assert_eq!(
        original_payload.artifact_id,
        deserialized_payload.artifact_id
    );
    assert_eq!(original_payload.sync_mode, deserialized_payload.sync_mode);
}

#[test]
fn test_health_check_response() {
    let response = HealthCheckResponse {
        status: HealthStatus::Healthy,
        uptime_seconds: 3600,
        storage_used_bytes: 10737418240,   // 10 GB
        storage_total_bytes: 107374182400, // 100 GB
        artifact_count: 150,
        raft_role: RaftRole::Leader,
        raft_term: 5,
        last_heartbeat: Utc::now(),
    };

    // Test serialization
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("healthy"));
    assert!(json.contains("leader"));

    // Test deserialization
    let deserialized: HealthCheckResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.status, HealthStatus::Healthy);
    assert_eq!(deserialized.raft_role, RaftRole::Leader);
}

#[test]
fn test_error_codes() {
    let error = ProtocolError {
        error_code: ErrorCode::ArtifactNotFound,
        error_message: "Artifact not found".to_string(),
        details: Some(serde_json::json!({"artifact_id": "missing-123"})),
    };

    let json = serde_json::to_string(&error).unwrap();
    assert!(json.contains("ARTIFACT_NOT_FOUND"));

    let deserialized: ProtocolError = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.error_code, ErrorCode::ArtifactNotFound);
}
