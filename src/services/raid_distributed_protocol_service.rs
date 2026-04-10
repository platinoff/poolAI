//! Distributed RAID wire-protocol orchestration (Put/Get/Delete artifact, sync, health, cluster).
//!
//! HTTP handlers in `network::raid_distributed_handlers` delegate here.
//!
//! **LeaveCluster (FM-008):** when [`crate::services::raid_service::RaidService::list_nodes`] returns a
//! non-empty list, the leaving `node_id` must match a registered member (otherwise `InvalidRequest`
//! before replication). Empty membership keeps prior behaviour (delete may still return not-found).

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::raid::protocol::*;
use crate::raid::ArtifactRef;
use crate::services::raid_service::{RaidService, RaidServiceError};
use crate::version;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct RaidDistributedProtocolService;

impl RaidDistributedProtocolService {
    pub async fn put_artifact(ctx: &ApiContext, message: ProtocolMessage) -> Response {
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
                ctx,
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

    pub async fn get_artifact(ctx: &ApiContext, message: ProtocolMessage) -> Response {
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

        let artifact = match RaidService::find_artifact_by_id(ctx, &payload.artifact_id).await {
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
                    match RaidService::get_artifact_data(ctx, artifact_ref).await {
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

    pub async fn delete_artifact(ctx: &ApiContext, message: ProtocolMessage) -> Response {
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

        match RaidService::delete_artifact(ctx, id).await {
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

    pub async fn sync_artifacts(ctx: &ApiContext, message: ProtocolMessage) -> Response {
        info!("Received SyncArtifacts message: {}", message.id);

        let payload = match message.extract_sync_artifacts() {
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

        let local = match RaidService::list_artifacts(ctx).await {
            Ok(a) => a,
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

        let remote_slice = payload.artifact_ids.as_deref();
        let (synced_count, missing_artifacts) =
            diff_sync_catalog(payload.direction, &local, remote_slice);
        let conflicts = build_sync_conflicts(&local, payload.remote_versions.as_ref());

        let response = SyncArtifactsResponse {
            status: OperationStatus::Success,
            synced_count,
            missing_artifacts,
            conflicts,
            error: None,
        };

        create_success_response(&message, response)
    }

    pub async fn health_check(ctx: &ApiContext, message: ProtocolMessage) -> Response {
        info!("Received HealthCheck message: {}", message.id);

        let storage = match RaidService::distributed_health_storage(ctx).await {
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

    pub async fn join_cluster(ctx: &ApiContext, message: ProtocolMessage) -> Response {
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

        let nodes = match RaidService::join_cluster_nodes_after_register(
            ctx,
            payload.address.clone(),
        )
        .await
        {
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

    pub async fn leave_cluster(ctx: &ApiContext, message: ProtocolMessage) -> Response {
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

        let node_id = match Uuid::parse_str(&message.node_id) {
            Ok(id) => id,
            Err(e) => {
                error!("Invalid node_id in LeaveCluster: {}", e);
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    format!("Invalid node_id: {}", e),
                );
            }
        };

        // FM-008: if membership is non-empty, only registered nodes may leave (avoid pointless replication).
        let cluster_nodes = match RaidService::list_nodes(ctx).await {
            Ok(ns) => ns,
            Err(RaidServiceError::ManagerUnavailable) => {
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    "RAID manager not initialized".to_string(),
                );
            }
            Err(e) => {
                error!("LeaveCluster failed to list cluster nodes: {:?}", e);
                return create_error_response(
                    &message,
                    ErrorCode::ReplicationFailed,
                    format!("Failed to read cluster membership: {:?}", e),
                );
            }
        };
        if !cluster_nodes.is_empty() && !cluster_nodes.iter().any(|n| n.id == node_id) {
            return create_error_response(
                &message,
                ErrorCode::InvalidRequest,
                format!(
                    "Node {} is not a cluster member ({} nodes registered)",
                    node_id,
                    cluster_nodes.len()
                ),
            );
        }

        let (replication_complete, artifacts_moved) = if payload.graceful {
            match RaidService::list_artifacts(ctx).await {
                Ok(artifacts) => {
                    let total = artifacts.len() as u32;
                    // FM-008: graceful leave is considered incomplete when there are artifacts
                    // but no peer nodes to replicate to.
                    let can_replicate_to_peers = cluster_nodes.len() > 1;
                    if total > 0 && !can_replicate_to_peers {
                        warn!(
                            "Graceful leave requested with {} artifacts but no peer nodes available",
                            total
                        );
                        (false, 0)
                    } else {
                        let mut moved = 0u32;
                        for a in artifacts {
                            if RaidService::replicate_stored_artifact(ctx, a.id)
                                .await
                                .is_ok()
                            {
                                moved = moved.saturating_add(1);
                            }
                        }
                        let complete = total == 0 || moved == total;
                        (complete, moved)
                    }
                }
                Err(RaidServiceError::ManagerUnavailable) => {
                    return create_error_response(
                        &message,
                        ErrorCode::InvalidRequest,
                        "RAID manager not initialized".to_string(),
                    );
                }
                Err(e) => {
                    error!("Graceful leave failed listing artifacts: {:?}", e);
                    return create_error_response(
                        &message,
                        ErrorCode::ReplicationFailed,
                        format!("Failed to list artifacts for graceful leave: {:?}", e),
                    );
                }
            }
        } else {
            (true, 0)
        };

        match RaidService::delete_worker(ctx, node_id).await {
            Ok(()) => {}
            Err(RaidServiceError::ManagerUnavailable) => {
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    "RAID manager not initialized".to_string(),
                );
            }
            Err(RaidServiceError::Operation(AppError::ResourceError(ref msg)))
                if msg.contains("not found") =>
            {
                return create_error_response(
                    &message,
                    ErrorCode::InvalidRequest,
                    format!("RAID node not found: {}", node_id),
                );
            }
            Err(e) => {
                error!("LeaveCluster delete_node failed: {:?}", e);
                return create_error_response(
                    &message,
                    ErrorCode::ReplicationFailed,
                    format!("Failed to remove node from membership: {:?}", e),
                );
            }
        }

        let response = LeaveClusterResponse {
            status: OperationStatus::Success,
            replication_complete,
            artifacts_moved,
            details: if payload.graceful && !replication_complete {
                Some(
                    "No peer nodes available for graceful replication; artifacts remain local"
                        .to_string(),
                )
            } else {
                None
            },
        };

        create_success_response(&message, response)
    }
}

/// Compare local artifact IDs with the peer catalog from [`SyncArtifactsPayload::artifact_ids`].
///
/// - **Pull** — IDs present locally but absent on the peer (peer should pull from this node).
/// - **Push** — IDs the peer has that we lack (this node should receive them).
/// - **Bidirectional** — symmetric difference.
///
/// If `remote_ids` is `None`, returns `(local.len(), [])` (no peer catalog to diff).
fn diff_sync_catalog(
    direction: SyncDirection,
    local: &[ArtifactRef],
    remote_ids: Option<&[String]>,
) -> (u32, Vec<String>) {
    let local_set: HashSet<String> = local.iter().map(|a| a.id.to_string()).collect();
    let Some(remote_slice) = remote_ids else {
        return (local.len() as u32, Vec::new());
    };
    let remote_set: HashSet<String> = remote_slice.iter().cloned().collect();
    let synced = local_set.intersection(&remote_set).count() as u32;
    let mut missing: Vec<String> = match direction {
        SyncDirection::Pull => local_set.difference(&remote_set).cloned().collect(),
        SyncDirection::Push => remote_set.difference(&local_set).cloned().collect(),
        SyncDirection::Bidirectional => local_set
            .symmetric_difference(&remote_set)
            .cloned()
            .collect(),
    };
    missing.sort();
    (synced, missing)
}

/// Detect conflicts for artifacts that exist on both sides with different timestamps.
fn build_sync_conflicts(
    local: &[ArtifactRef],
    remote_versions: Option<&HashMap<String, chrono::DateTime<Utc>>>,
) -> Vec<ArtifactConflict> {
    let Some(remote) = remote_versions else {
        return Vec::new();
    };
    let mut conflicts = Vec::new();
    for a in local {
        let id = a.id.to_string();
        let Some(remote_ts) = remote.get(&id) else {
            continue;
        };
        if a.stored_at != *remote_ts {
            let reason = if a.stored_at > *remote_ts {
                "local_newer_than_remote"
            } else {
                "remote_newer_than_local"
            };
            conflicts.push(ArtifactConflict {
                artifact_id: id,
                reason: reason.to_string(),
                local_version: a.stored_at,
                remote_version: *remote_ts,
            });
        }
    }
    conflicts.sort_by(|l, r| l.artifact_id.cmp(&r.artifact_id));
    conflicts
}

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

#[cfg(test)]
mod sync_catalog_tests {
    use super::{build_sync_conflicts, diff_sync_catalog};
    use crate::raid::protocol::SyncDirection;
    use crate::raid::ArtifactRef;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use uuid::Uuid;

    fn artifact(id: &str) -> ArtifactRef {
        ArtifactRef {
            id: Uuid::parse_str(id).unwrap(),
            name: "n".into(),
            stored_at: Utc::now(),
            path: PathBuf::from("/tmp/x"),
        }
    }

    #[test]
    fn pull_reports_local_only_ids_as_missing() {
        let local = vec![
            artifact("00000000-0000-0000-0000-000000000001"),
            artifact("00000000-0000-0000-0000-000000000002"),
        ];
        let remote = vec!["00000000-0000-0000-0000-000000000001".to_string()];
        let (synced, missing) = diff_sync_catalog(SyncDirection::Pull, &local, Some(&remote));
        assert_eq!(synced, 1);
        assert_eq!(
            missing,
            vec!["00000000-0000-0000-0000-000000000002".to_string()]
        );
    }

    #[test]
    fn push_reports_remote_only_ids_as_missing() {
        let local = vec![artifact("00000000-0000-0000-0000-000000000001")];
        let remote = vec![
            "00000000-0000-0000-0000-000000000001".to_string(),
            "00000000-0000-0000-0000-000000000003".to_string(),
        ];
        let (synced, missing) = diff_sync_catalog(SyncDirection::Push, &local, Some(&remote));
        assert_eq!(synced, 1);
        assert_eq!(
            missing,
            vec!["00000000-0000-0000-0000-000000000003".to_string()]
        );
    }

    #[test]
    fn no_remote_catalog_keeps_missing_empty() {
        let local = vec![artifact("00000000-0000-0000-0000-00000000000a")];
        let (synced, missing) = diff_sync_catalog(SyncDirection::Bidirectional, &local, None);
        assert_eq!(synced, 1);
        assert!(missing.is_empty());
    }

    #[test]
    fn conflicts_report_timestamp_divergence_when_remote_versions_present() {
        let local = vec![artifact("00000000-0000-0000-0000-00000000000b")];
        let mut remote_versions = HashMap::new();
        let local_ts = local[0].stored_at;
        remote_versions.insert(
            "00000000-0000-0000-0000-00000000000b".to_string(),
            local_ts - chrono::Duration::seconds(1),
        );
        let conflicts = build_sync_conflicts(&local, Some(&remote_versions));
        assert_eq!(conflicts.len(), 1);
        assert_eq!(
            conflicts[0].artifact_id,
            "00000000-0000-0000-0000-00000000000b"
        );
        assert_eq!(conflicts[0].reason, "local_newer_than_remote");
    }
}
