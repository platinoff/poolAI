//! Distributed RAID Protocol API Handlers
//!
//! Handlers for processing protocol messages in the distributed RAID system.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::raid::protocol::*;
use crate::services::raid_service::{RaidService, RaidServiceError};
use crate::version;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;

/// Handle PutArtifact protocol message
pub async fn put_artifact_handler(
    State(ctx): State<ApiContext>,
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

    match artifact_data {
        Some(data) => match RaidService::put_artifact_versioned_name(
            &ctx,
            &payload.metadata.name,
            &payload.metadata.version,
            &data,
        )
        .await
        {
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
            Err(RaidServiceError::ManagerUnavailable) => create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                "RAID manager not initialized".to_string(),
            ),
            Err(RaidServiceError::Operation(e)) => {
                error!("Failed to store artifact: {}", e);
                create_error_response(
                    &message,
                    ErrorCode::ReplicationFailed,
                    format!("Storage failed: {}", e),
                )
            }
            Err(e) => {
                error!("Failed to store artifact: {:?}", e);
                create_error_response(
                    &message,
                    ErrorCode::ReplicationFailed,
                    format!("Storage failed: {:?}", e),
                )
            }
        },
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
    State(ctx): State<ApiContext>,
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

    let artifact = match RaidService::find_artifact_by_id(&ctx, &payload.artifact_id).await {
        Ok(a) => a,
        Err(RaidServiceError::ManagerUnavailable) => {
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                "RAID manager not initialized".to_string(),
            );
        }
        Err(e) => {
            error!("Failed to resolve artifact by id: {:?}", e);
            return create_error_response(
                &message,
                ErrorCode::ReplicationFailed,
                format!("Storage failed: {:?}", e),
            );
        }
    };

    match artifact {
        Some(ref artifact_ref) => {
            let mut response = GetArtifactResponse {
                status: ArtifactStatus::Success,
                artifact_id: payload.artifact_id.clone(),
                metadata: Some(ArtifactMetadata {
                    name: artifact_ref.name.clone(),
                    // Future improvement: Extract version from artifact metadata
                    // 1. Parse version from artifact name using regex or heuristics
                    //    - Example: "model-v1.2.3.bin" -> "1.2.3"
                    // 2. Or read version from metadata file (if stored separately)
                    //    - Check for <artifact_path>.meta or similar metadata file
                    // 3. Or parse from artifact content header (if format supports it)
                    //    - Some artifact formats (e.g., ONNX, TensorFlow) embed version info
                    // 4. Fallback to "unknown" if version cannot be determined
                    version: "unknown".to_string(),
                    // Future improvement: Get actual artifact file size
                    // 1. Use std::fs::metadata(&artifact_ref.path) to get file metadata
                    // 2. Extract metadata.len() for file size in bytes
                    // 3. Handle errors gracefully (file not found, permission denied)
                    //    - Log warning and use 0 as fallback
                    // Example: std::fs::metadata(&artifact_ref.path).map(|m| m.len()).unwrap_or(0)
                    size_bytes: 0,
                    // Future improvement: Calculate checksum for artifact
                    // 1. Read artifact file using manager.get_artifact(&artifact_ref.path).await
                    // 2. Calculate SHA-256 checksum using sha2 crate:
                    //    - use sha2::{Sha256, Digest};
                    //    - let mut hasher = Sha256::new();
                    //    - hasher.update(&artifact_data);
                    //    - let checksum = format!("{:x}", hasher.finalize());
                    // 3. Optionally cache checksum in artifact metadata to avoid recomputation
                    // 4. Handle errors gracefully (file read failures, I/O errors)
                    //    - Log warning and use "unknown" as fallback
                    checksum: "unknown".to_string(),
                    created_at: artifact_ref.stored_at,
                    content_type: None,
                    tags: None,
                }),
                data: None,
                error: None,
            };

            // Include data if requested
            if payload.include_data {
                match RaidService::get_artifact_data(&ctx, artifact_ref).await {
                    Ok(data) => {
                        use base64::{engine::general_purpose, Engine as _};
                        response.data = Some(general_purpose::STANDARD.encode(&data));
                    }
                    Err(e) => {
                        error!("Failed to read artifact file: {:?}", e);
                        response.status = ArtifactStatus::Error;
                        response.error = Some(format!("Failed to read artifact: {:?}", e));
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
    State(ctx): State<ApiContext>,
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

    let id = match Uuid::parse_str(&payload.artifact_id) {
        Ok(id) => id,
        Err(e) => {
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!("Invalid artifact id: {}", e),
            );
        }
    };

    match RaidService::delete_artifact(&ctx, id).await {
        Ok(()) => {
            info!("Artifact deleted successfully: {}", payload.artifact_id);
            let response = DeleteArtifactResponse {
                status: OperationStatus::Success,
                artifact_id: payload.artifact_id,
                deleted_at: Utc::now(),
                error: None,
            };
            create_success_response(&message, response)
        }
        Err(RaidServiceError::ManagerUnavailable) => create_error_response(
            &message,
            ErrorCode::InvalidRequest,
            "RAID manager not initialized".to_string(),
        ),
        Err(RaidServiceError::ArtifactNotFound { .. }) => {
            warn!("Artifact not found for deletion: {}", payload.artifact_id);
            let response = DeleteArtifactResponse {
                status: OperationStatus::Success, // Idempotent - already deleted
                artifact_id: payload.artifact_id,
                deleted_at: Utc::now(),
                error: None,
            };
            create_success_response(&message, response)
        }
        Err(RaidServiceError::Operation(e)) => {
            error!("Failed to delete artifact: {}", e);
            create_error_response(
                &message,
                ErrorCode::ReplicationFailed,
                format!("Delete failed: {}", e),
            )
        }
        Err(e) => {
            error!("Failed to delete artifact: {:?}", e);
            create_error_response(
                &message,
                ErrorCode::ReplicationFailed,
                format!("Delete failed: {:?}", e),
            )
        }
    }
}

/// Handle SyncArtifacts protocol message
pub async fn sync_artifacts_handler(
    State(ctx): State<ApiContext>,
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

    let synced_count = match RaidService::local_artifact_count(&ctx).await {
        Ok(n) => n,
        Err(RaidServiceError::ManagerUnavailable) => {
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                "RAID manager not initialized".to_string(),
            );
        }
        Err(e) => {
            error!("Failed to list artifacts: {:?}", e);
            return create_error_response(
                &message,
                ErrorCode::ReplicationFailed,
                format!("Storage failed: {:?}", e),
            );
        }
    };

    // Simplified sync implementation
    // In real implementation, would compare timestamps and sync accordingly
    let response = SyncArtifactsResponse {
        status: OperationStatus::Success,
        synced_count,
        // Future improvement: Implement proper sync logic
        // 1. Compare local artifacts with remote node's artifact list
        //    - Request artifact list from remote node using ProtocolMessage::ListArtifacts
        //    - Compare timestamps (stored_at) to determine which artifacts are missing
        // 2. Identify missing artifacts (present locally but not on remote)
        //    - Create missing_artifacts list with artifact IDs and metadata
        // 3. Optionally initiate replication for missing artifacts
        //    - Call replication engine to sync missing artifacts to remote node
        // 4. Track sync progress and handle failures
        //    - Retry failed syncs with exponential backoff
        //    - Report partial sync status if some artifacts fail to sync
        missing_artifacts: Vec::new(),
        // Future improvement: Implement conflict detection
        // 1. Compare artifact versions/timestamps between local and remote
        //    - If same artifact ID exists on both sides with different timestamps
        //    - If same artifact ID has different checksums (data divergence)
        // 2. Create conflict entries with:
        //    - artifact_id: conflicting artifact ID
        //    - local_version: local artifact version/timestamp
        //    - remote_version: remote artifact version/timestamp
        //    - resolution_strategy: "keep_latest", "merge", "manual_resolution"
        // 3. Apply resolution strategy automatically or flag for manual review
        // 4. Log conflicts for audit purposes
        conflicts: Vec::new(),
        error: None,
    };

    create_success_response(&message, response)
}

/// Handle HealthCheck protocol message
pub async fn health_check_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    info!("Received HealthCheck message: {}", message.id);

    let storage = match RaidService::distributed_health_storage(&ctx).await {
        Ok(s) => s,
        Err(RaidServiceError::ManagerUnavailable) => {
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                "RAID manager not initialized".to_string(),
            );
        }
        Err(e) => {
            error!("Failed to read RAID quota: {:?}", e);
            return create_error_response(
                &message,
                ErrorCode::ReplicationFailed,
                format!("Storage failed: {:?}", e),
            );
        }
    };

    // Get actual application uptime using version module
    // Note: version::initialize_start_time() must be called at startup (already done in main.rs)
    let uptime_seconds = version::get_uptime_seconds();

    // Future improvement: Get Raft role and term (when Raft is implemented)
    // 1. Access global Raft node using raid::raft::get_global_raft_node()
    //    - Returns Arc<Raft<RaidRaftStateMachine, RaidRaftNetwork, RaidRaftStorage>>
    // 2. Get current role using raft_node.current_leader() and raft_node.current_state()
    //    - current_state() returns RaftState (Leader, Candidate, Follower)
    //    - Map RaftState to RaftRole enum (Leader, Candidate, Follower)
    // 3. Get current term using raft_node.current_term() or from HardState
    //    - HardState contains current_term and voted_for
    // 4. Handle errors gracefully if Raft is not initialized or not enabled
    //    - Check for #[cfg(feature = "raft")] before accessing Raft APIs
    //    - Fallback to RaftRole::Follower if Raft is not available
    // Example (requires #[cfg(feature = "raft")]):
    //    let raft_role = if let Some(raft_node) = raid::raft::get_global_raft_node_opt() {
    //        match raft_node.current_state() {
    //            async_raft::raft::RaftState::Leader => RaftRole::Leader,
    //            async_raft::raft::RaftState::Candidate => RaftRole::Candidate,
    //            async_raft::raft::RaftState::Follower => RaftRole::Follower,
    //        }
    //    } else {
    //        RaftRole::Follower
    //    };
    let response = HealthCheckResponse {
        status: HealthStatus::Healthy,
        uptime_seconds,
        storage_used_bytes: storage.storage_used_bytes,
        storage_total_bytes: storage.storage_total_bytes,
        artifact_count: storage.artifact_count,
        raft_role: RaftRole::Follower, // Placeholder until Raft integration is complete
        // Future improvement: Get actual Raft term from Raft node
        // 1. Access global Raft node using raid::raft::get_global_raft_node()
        // 2. Get current term using raft_node.current_term() or from HardState
        //    - HardState contains current_term and voted_for
        // 3. Handle errors gracefully if Raft is not initialized or not enabled
        //    - Check for #[cfg(feature = "raft")] before accessing Raft APIs
        //    - Fallback to 0 if Raft is not available
        // Example (requires #[cfg(feature = "raft")]):
        //    let raft_term = if let Some(raft_node) = raid::raft::get_global_raft_node_opt() {
        //        raft_node.current_term()
        //    } else {
        //        0
        //    };
        raft_term: 0,
        last_heartbeat: Utc::now(),
    };

    create_success_response(&message, response)
}

/// Handle JoinCluster protocol message
pub async fn join_cluster_handler(
    State(ctx): State<ApiContext>,
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

    let nodes =
        match RaidService::join_cluster_nodes_after_register(&ctx, payload.address.clone()).await {
            Ok(n) => n,
            Err(RaidServiceError::ManagerUnavailable) => {
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    "RAID manager not initialized".to_string(),
                );
            }
            Err(e) => {
                error!("Failed to list cluster nodes: {:?}", e);
                return create_error_response(
                    &message,
                    ErrorCode::ReplicationFailed,
                    format!("Cluster state failed: {:?}", e),
                );
            }
        };
    let member_nodes: Vec<ClusterNode> = nodes
        .iter()
        .map(|n| {
            ClusterNode {
                node_id: n.id.to_string(),
                address: n.address.clone(),
                // Future improvement: Get actual Raft role for each node
                // 1. Access global Raft node using raid::raft::get_global_raft_node()
                // 2. Get current leader using raft_node.current_leader()
                // 3. Compare node_id with current_leader to determine role:
                //    - If node_id == current_leader: RaftRole::Leader
                //    - If node_id in candidates: RaftRole::Candidate
                //    - Otherwise: RaftRole::Follower
                // 4. Handle errors gracefully if Raft is not initialized or not enabled
                //    - Check for #[cfg(feature = "raft")] before accessing Raft APIs
                //    - Fallback to RaftRole::Follower if Raft is not available
                // Example (requires #[cfg(feature = "raft")]):
                //    let role = if let Some(raft_node) = raid::raft::get_global_raft_node_opt() {
                //        if raft_node.current_leader() == Some(node_id) {
                //            RaftRole::Leader
                //        } else {
                //            RaftRole::Follower
                //        }
                //    } else {
                //        RaftRole::Follower
                //    };
                role: RaftRole::Follower,
                status: Some(HealthStatus::Healthy),
                last_seen: Some(n.last_seen),
            }
        })
        .collect();

    let response = JoinClusterResponse {
        status: JoinStatus::Accepted,
        // Future improvement: Use actual cluster ID from configuration or Raft
        // 1. Get cluster ID from RaidConfig (if stored in config)
        //    - Add cluster_id field to RaidConfig
        //    - Read from config file or environment variable
        // 2. Or generate cluster ID on first node and persist it
        //    - Store cluster ID in metadata file (e.g., cluster.json)
        //    - Share cluster ID with all nodes during join
        // 3. Or derive cluster ID from Raft membership configuration
        //    - Use MembershipConfig to generate stable cluster ID
        //    - Persist cluster ID with membership changes
        // 4. Ensure cluster ID is consistent across all nodes
        //    - Reject join requests if cluster ID mismatch
        // Example: let cluster_id = manager.get_cluster_id().await.unwrap_or_else(|| Uuid::new_v4().to_string());
        cluster_id: Uuid::new_v4().to_string(),
        member_nodes,
        error: None,
    };

    create_success_response(&message, response)
}

/// Handle LeaveCluster protocol message
pub async fn leave_cluster_handler(Json(message): Json<ProtocolMessage>) -> impl IntoResponse {
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

    // Future improvement: Implement graceful leave logic
    // 1. If graceful=true:
    //    a. Replicate all local artifacts to other nodes before leaving
    //       - Get list of all local artifacts using manager.list_artifacts()
    //       - For each artifact, use replication engine to sync to target nodes
    //       - Wait for replication to complete (with timeout)
    //    b. Update cluster membership via Raft
    //       - Create Raft operation to remove this node from membership
    //       - Wait for membership update to be applied
    //    c. Handle graceful shutdown
    //       - Stop accepting new requests
    //       - Complete in-flight requests
    //       - Close connections to other nodes
    // 2. If graceful=false:
    //    - Simply remove node from membership (other nodes will detect failure)
    //    - Don't wait for artifact replication
    // 3. Track artifact migration progress
    //    - Count artifacts successfully replicated
    //    - Return artifacts_moved count in response
    // 4. Handle errors gracefully
    //    - Log warnings for failed replications
    //    - Continue with leave even if some artifacts fail to replicate

    let response = LeaveClusterResponse {
        status: OperationStatus::Success,
        replication_complete: payload.graceful,
        // Future improvement: Implement artifact migration tracking
        // 1. If graceful=true, count artifacts successfully replicated during leave
        //    - Track artifacts_moved during graceful leave logic
        //    - Return actual count of artifacts migrated
        // 2. If graceful=false, return 0 (no migration performed)
        // 3. Include failed artifacts in error response if any
        // Example: artifacts_moved: if payload.graceful { migrated_count } else { 0 }
        artifacts_moved: 0,
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
        // Future improvement: Use actual node ID instead of copying from request
        // 1. Get node ID from RaidManager using manager.get_node_id().await
        //    - This returns the actual node ID from Raft configuration
        //    - More reliable than trusting client-provided node_id
        // 2. Handle errors gracefully if node ID cannot be retrieved
        //    - Fallback to original_message.node_id if get_node_id fails
        // Example: node_id: manager.get_node_id().await.to_string()
        // Note: For now, using original_message.node_id for compatibility
        node_id: original_message.node_id.clone(),
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
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| AppError::ValidationError(format!(
            "Base64 decode error: {}. Context: Failed to decode base64-encoded artifact data. \
            Suggestion: Verify the data is valid base64 encoding. Ensure no whitespace or invalid characters are present. \
            Error details: {}, Data length: {}",
            e, e, data.len()
        )))
}
