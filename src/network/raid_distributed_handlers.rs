//! Distributed RAID Protocol API Handlers
//!
//! Handlers for processing protocol messages in the distributed RAID system.

use axum::{Json, response::{IntoResponse, Response}, http::StatusCode};
use crate::raid::protocol::*;
use crate::raid;
use crate::core::error::AppError;
use tracing::{info, warn, error};
use chrono::Utc;
use uuid::Uuid;

/// Handle PutArtifact protocol message
pub async fn put_artifact_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received PutArtifact message: {}", message.id);

    let payload = match message.extract_put_artifact() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract PutArtifact payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    let manager = raid::get_global_manager();
    
    // Decode base64 data if present
    let artifact_data = if let Some(data_base64) = payload.data {
        match base64_decode(&data_base64) {
            Ok(data) => Some(data),
            Err(e) => {
                error!("Failed to decode base64 data: {}", e);
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    format!("Invalid base64 data: {}", e),
                );
            }
        }
    } else {
        None
    };

    // Store artifact locally
    let artifact_name = format!("{}-{}", payload.metadata.name, payload.metadata.version);
    match artifact_data {
        Some(data) => {
            match manager.put_artifact(&artifact_name, &data).await {
                Ok(artifact_ref) => {
                    info!("Artifact stored successfully: {}", artifact_ref.id);
                    let response = PutArtifactResponse {
                        status: OperationStatus::Success,
                        artifact_id: payload.artifact_id,
                        stored_at: Utc::now(),
                        error: None,
                    };
                    create_success_response(&message, response)
                }
                Err(e) => {
                    error!("Failed to store artifact: {}", e);
                    create_error_response(
                        &message,
                        ErrorCode::ReplicationFailed,
                        format!("Storage failed: {}", e),
                    )
                }
            }
        }
        None => {
            // Metadata-only replication (data will be fetched separately)
            warn!("PutArtifact without data - metadata-only replication");
            let response = PutArtifactResponse {
                status: OperationStatus::Success,
                artifact_id: payload.artifact_id,
                stored_at: Utc::now(),
                error: None,
            };
            create_success_response(&message, response)
        }
    }
}

/// Handle GetArtifact protocol message
pub async fn get_artifact_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received GetArtifact message: {}", message.id);

    let payload = match message.extract_get_artifact() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract GetArtifact payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    
    // Find artifact by ID (simplified - in real implementation, would use proper ID mapping)
    let artifact = artifacts.iter().find(|a| a.id.to_string() == payload.artifact_id);

    match artifact {
        Some(artifact_ref) => {
            let mut response = GetArtifactResponse {
                status: ArtifactStatus::Success,
                artifact_id: payload.artifact_id.clone(),
                metadata: Some(ArtifactMetadata {
                    name: artifact_ref.name.clone(),
                    version: "unknown".to_string(), // TODO: extract from artifact
                    size_bytes: 0, // TODO: get actual size
                    checksum: "unknown".to_string(), // TODO: calculate checksum
                    created_at: artifact_ref.stored_at,
                    content_type: None,
                    tags: None,
                }),
                data: None,
                error: None,
            };

            // Include data if requested
            if payload.include_data {
                match tokio::fs::read(&artifact_ref.path).await {
                    Ok(data) => {
                        use base64::{Engine as _, engine::general_purpose};
                        response.data = Some(general_purpose::STANDARD.encode(&data));
                    }
                    Err(e) => {
                        error!("Failed to read artifact file: {}", e);
                        response.status = ArtifactStatus::Error;
                        response.error = Some(format!("Failed to read artifact: {}", e));
                    }
                }
            }

            create_success_response(&message, response)
        }
        None => {
            warn!("Artifact not found: {}", payload.artifact_id);
            let response = GetArtifactResponse {
                status: ArtifactStatus::NotFound,
                artifact_id: payload.artifact_id,
                metadata: None,
                data: None,
                error: Some("Artifact not found".to_string()),
            };
            create_success_response(&message, response)
        }
    }
}

/// Handle DeleteArtifact protocol message
pub async fn delete_artifact_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received DeleteArtifact message: {}", message.id);

    let payload = match message.extract_delete_artifact() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract DeleteArtifact payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    
    // Find and delete artifact (simplified - in real implementation, would use proper ID mapping)
    let artifact = artifacts.iter().find(|a| a.id.to_string() == payload.artifact_id);

    match artifact {
        Some(artifact_ref) => {
            match tokio::fs::remove_file(&artifact_ref.path).await {
                Ok(_) => {
                    info!("Artifact deleted successfully: {}", payload.artifact_id);
                    let response = DeleteArtifactResponse {
                        status: OperationStatus::Success,
                        artifact_id: payload.artifact_id,
                        deleted_at: Utc::now(),
                        error: None,
                    };
                    create_success_response(&message, response)
                }
                Err(e) => {
                    error!("Failed to delete artifact: {}", e);
                    create_error_response(
                        &message,
                        ErrorCode::ReplicationFailed,
                        format!("Delete failed: {}", e),
                    )
                }
            }
        }
        None => {
            warn!("Artifact not found for deletion: {}", payload.artifact_id);
            let response = DeleteArtifactResponse {
                status: OperationStatus::Success, // Idempotent - already deleted
                artifact_id: payload.artifact_id,
                deleted_at: Utc::now(),
                error: None,
            };
            create_success_response(&message, response)
        }
    }
}

/// Handle SyncArtifacts protocol message
pub async fn sync_artifacts_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received SyncArtifacts message: {}", message.id);

    let _payload = match message.extract_sync_artifacts() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract SyncArtifacts payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    
    // Simplified sync implementation
    // In real implementation, would compare timestamps and sync accordingly
    let response = SyncArtifactsResponse {
        status: OperationStatus::Success,
        synced_count: artifacts.len() as u32,
        missing_artifacts: Vec::new(), // TODO: implement proper sync logic
        conflicts: Vec::new(), // TODO: implement conflict detection
        error: None,
    };

    create_success_response(&message, response)
}

/// Handle HealthCheck protocol message
pub async fn health_check_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received HealthCheck message: {}", message.id);

    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    let total_size = manager.get_total_size().await.unwrap_or(0);
    
    // TODO: Get actual storage capacity from config
    let storage_total_bytes = 107374182400u64; // 100 GB default
    let storage_used_bytes = total_size;
    
    // TODO: Get actual uptime
    let uptime_seconds = 3600u64;
    
    // TODO: Get Raft role and term (when Raft is implemented)
    let response = HealthCheckResponse {
        status: HealthStatus::Healthy,
        uptime_seconds,
        storage_used_bytes,
        storage_total_bytes,
        artifact_count: artifacts.len() as u32,
        raft_role: RaftRole::Follower, // TODO: get actual role
        raft_term: 0, // TODO: get actual term
        last_heartbeat: Utc::now(),
    };

    create_success_response(&message, response)
}

/// Handle JoinCluster protocol message
pub async fn join_cluster_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received JoinCluster message: {}", message.id);

    let payload = match message.extract_join_cluster() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract JoinCluster payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    let manager = raid::get_global_manager();
    
    // Register the new node
    let _node = manager.register_node(payload.address.clone()).await;
    
    // Get existing nodes
    let nodes = manager.list_nodes().await;
    let member_nodes: Vec<ClusterNode> = nodes.iter().map(|n| {
        ClusterNode {
            node_id: n.id.to_string(),
            address: n.address.clone(),
            role: RaftRole::Follower, // TODO: get actual role
            status: Some(HealthStatus::Healthy),
            last_seen: Some(n.last_seen),
        }
    }).collect();

    let response = JoinClusterResponse {
        status: JoinStatus::Accepted,
        cluster_id: Uuid::new_v4().to_string(), // TODO: use actual cluster ID
        member_nodes,
        error: None,
    };

    create_success_response(&message, response)
}

/// Handle LeaveCluster protocol message
pub async fn leave_cluster_handler(
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received LeaveCluster message: {}", message.id);

    let payload = match message.extract_leave_cluster() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to extract LeaveCluster payload: {}", e);
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid payload: {}", e),
            );
        }
    };

    // TODO: Implement graceful leave logic
    // - Replicate artifacts to other nodes
    // - Update cluster membership
    // - Handle graceful shutdown

    let response = LeaveClusterResponse {
        status: OperationStatus::Success,
        replication_complete: payload.graceful,
        artifacts_moved: 0, // TODO: implement artifact migration
    };

    create_success_response(&message, response)
}

// Helper functions

fn create_success_response<T: serde::Serialize>(
    original_message: &ProtocolMessage,
    payload: T,
) -> Response {
    let response_message = ProtocolMessage {
        message_type: format!("{}_response", original_message.message_type),
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        node_id: original_message.node_id.clone(), // TODO: use actual node ID
        payload: serde_json::to_value(payload).unwrap_or(serde_json::Value::Null),
    };

    (StatusCode::OK, Json(response_message)).into_response()
}

fn create_error_response(
    original_message: &ProtocolMessage,
    error_code: ErrorCode,
    error_message: String,
) -> Response {
    let error = ProtocolError {
        error_code,
        error_message,
        details: None,
    };

    let response_message = ProtocolMessage {
        message_type: "error".to_string(),
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        node_id: original_message.node_id.clone(),
        payload: serde_json::to_value(error).unwrap_or(serde_json::Value::Null),
    };

    (StatusCode::BAD_REQUEST, Json(response_message)).into_response()
}

fn base64_decode(data: &str) -> Result<Vec<u8>, AppError> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.decode(data)
        .map_err(|e| AppError::ValidationError(format!("Base64 decode error: {}", e)))
}

