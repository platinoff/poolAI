//! RAID management API endpoints
//!
//! Provides endpoints for managing RAID storage:
//! - List nodes and artifacts
//! - Create, delete artifacts
//! - Get quota and events
//! - Snapshot and GC operations
//! - Distributed RAID protocol endpoints

use axum::{
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use chrono::DateTime;
use serde::Deserialize;
use uuid::Uuid;

use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};
use crate::network::raid_distributed_handlers::*;
use crate::network::validation;
use crate::raid;

#[derive(Deserialize)]
struct CreateArtifactRequest {
    name: String,
    data: String, // base64-encoded data
}

#[derive(serde::Serialize)]
struct CreateArtifactResponse {
    artifact_id: String,
    name: String,
    stored_at: String,
    message: String,
}

#[derive(serde::Serialize)]
struct DeleteArtifactResponse {
    artifact_id: String,
    message: String,
}

#[derive(serde::Serialize)]
struct RaidQuotaResponse {
    total_size_bytes: u64,
    quota_bytes: Option<u64>,
    usage_percent: Option<f64>,
    artifact_count: usize,
}

#[derive(serde::Serialize)]
struct RaidGcResponse {
    removed_count: usize,
}

#[derive(serde::Serialize)]
struct RaidStatusResponse {
    cluster_status: String, // "healthy", "degraded", "unhealthy"
    node_count: usize,
    artifact_count: usize,
    storage: RaidStorageStatus,
    mode: String, // "Local", "BurstRaid", "SmallWorld"
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raft_status: Option<RaftStatus>,
}

#[derive(serde::Serialize)]
struct RaidStorageStatus {
    total_size_bytes: u64,
    quota_bytes: Option<u64>,
    usage_percent: Option<f64>,
    available_bytes: Option<u64>,
}

#[derive(serde::Serialize)]
struct RaftStatus {
    role: String, // "leader", "follower", "candidate"
    term: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    leader_id: Option<String>,
}

#[derive(Deserialize)]
struct EventsRangeQuery {
    start: Option<String>, // ISO 8601 timestamp
    end: Option<String>,   // ISO 8601 timestamp
}

/// Create RAID management routes
pub fn create_raid_routes() -> Router {
    Router::new()
        .route("/raid/nodes", get(raid_nodes_handler))
        .route("/raid/artifacts", get(raid_artifacts_handler))
        .route(
            "/raid/artifacts",
            post(raid_artifact_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/raid/artifacts/{id}",
            axum::routing::delete(raid_artifact_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route("/raid/quota", get(raid_quota_handler))
        .route("/raid/status", get(raid_status_handler))
        .route("/raid/events", get(raid_events_handler))
        .route(
            "/raid/events/{artifact_id}",
            get(raid_events_for_artifact_handler),
        )
        .route("/raid/events/range", get(raid_events_range_handler))
        .route("/raid/snapshot", get(raid_snapshot_handler))
        .route(
            "/raid/snapshot/create",
            post(raid_snapshot_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/raid/snapshot/restore",
            post(raid_snapshot_restore_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/raid/gc",
            post(raid_gc_handler).layer(middleware::from_fn(auth_middleware)),
        )
        // Distributed RAID Protocol endpoints
        .route(
            "/raid/distributed/artifacts/replicate",
            post(put_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/get",
            post(get_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/delete",
            post(delete_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/sync",
            post(sync_artifacts_handler),
        )
        .route("/raid/distributed/health", post(health_check_handler))
        .route("/raid/distributed/cluster/join", post(join_cluster_handler))
        .route(
            "/raid/distributed/cluster/leave",
            post(leave_cluster_handler),
        )
}

async fn raid_nodes_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let nodes = manager.list_nodes().await;
    AxumJson(nodes).into_response()
}

async fn raid_artifacts_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    AxumJson(artifacts).into_response()
}

async fn raid_artifact_create_handler(
    Json(payload): Json<CreateArtifactRequest>,
) -> impl IntoResponse {
    // Validate artifact name
    if let Err(e) = validation::validate_artifact_name(&payload.name) {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Validate base64 data size (max 100MB)
    const MAX_ARTIFACT_SIZE: usize = 100 * 1024 * 1024; // 100MB
    if let Err(e) = validation::validate_base64_data(&payload.data, MAX_ARTIFACT_SIZE) {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Decode base64 data
    let artifact_name = payload.name.clone();
    let data = match base64_engine.decode(&payload.data) {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid base64 data. Context: Cannot decode base64-encoded artifact data. Suggestion: Verify data is properly base64-encoded and not corrupted. Artifact name: '{}', Error: {}", artifact_name, e)
                })),
            )
                .into_response();
        }
    };

    // Validate decoded data size
    if let Err(e) = validation::validate_artifact_data_size(data.len(), MAX_ARTIFACT_SIZE) {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Create artifact
    let manager = raid::get_global_manager();
    let data_size = data.len();
    match manager.put_artifact(&payload.name, &data).await {
        Ok(artifact) => {
            let response = CreateArtifactResponse {
                artifact_id: artifact.id.to_string(),
                name: artifact.name,
                stored_at: artifact.stored_at.to_rfc3339(),
                message: "Artifact created successfully".to_string(),
            };
            (StatusCode::CREATED, AxumJson(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create artifact. Context: Cannot store artifact in RAID storage. Suggestion: Check storage quota, verify RAID manager is initialized, and ensure sufficient disk space. Artifact name: '{}', Data size: {} bytes, Error: {}", artifact_name, data_size, e)
            })),
        )
            .into_response(),
    }
}

async fn raid_artifact_delete_handler(Path(artifact_id): Path<String>) -> impl IntoResponse {
    // Validate UUID format
    if let Err(e) = validation::validate_uuid(&artifact_id) {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Parse artifact ID
    let id = Uuid::parse_str(&artifact_id).unwrap(); // Safe after validation

    // Delete artifact
    let manager = raid::get_global_manager();
    match manager.delete_artifact(id).await {
        Ok(_) => {
            let response = DeleteArtifactResponse {
                artifact_id: artifact_id,
                message: "Artifact deleted successfully".to_string(),
            };
            AxumJson(response).into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!("Failed to delete artifact. Context: Cannot delete artifact from RAID storage. Suggestion: Verify artifact ID exists, check RAID manager status, and ensure artifact is not locked. Artifact ID: '{}', Error: {}", artifact_id, e)
            })),
        )
            .into_response(),
    }
}

async fn raid_quota_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let total_size = manager.get_total_size().await.unwrap_or(0);
    let artifacts = manager.list_artifacts().await;
    let artifact_count = artifacts.len();

    // Get quota from config
    let quota_bytes = manager.get_quota_bytes().await;

    let usage_percent = quota_bytes.map(|quota| {
        if quota > 0 {
            (total_size as f64 / quota as f64) * 100.0
        } else {
            0.0
        }
    });

    AxumJson(RaidQuotaResponse {
        total_size_bytes: total_size,
        quota_bytes,
        usage_percent,
        artifact_count,
    })
    .into_response()
}

async fn raid_status_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    
    // Get basic information
    let nodes = manager.list_nodes().await;
    let artifacts = manager.list_artifacts().await;
    let node_count = nodes.len();
    let artifact_count = artifacts.len();
    
    // Get storage information
    let total_size = manager.get_total_size().await.unwrap_or(0);
    let quota_bytes = manager.get_quota_bytes().await;
    let usage_percent = quota_bytes.map(|quota| {
        if quota > 0 {
            (total_size as f64 / quota as f64) * 100.0
        } else {
            0.0
        }
    });
    let available_bytes = quota_bytes.map(|quota| quota.saturating_sub(total_size));
    
    // Get mode from config
    let mode = format!("{:?}", manager.get_mode().await);
    
    // Determine cluster status
    // Healthy: nodes > 0, usage < 90%, no errors
    // Degraded: usage >= 90%, or some nodes unavailable
    // Unhealthy: no nodes, or critical errors
    let cluster_status = if node_count == 0 {
        "unhealthy".to_string()
    } else if let Some(usage) = usage_percent {
        if usage >= 95.0 {
            "unhealthy".to_string()
        } else if usage >= 90.0 {
            "degraded".to_string()
        } else {
            "healthy".to_string()
        }
    } else {
        "healthy".to_string()
    };
    
    // Check replication status (if in distributed mode)
    let replication_status = if mode != "Local" {
        // For distributed modes, check if replication is active
        // Placeholder: in future, check actual replication status
        Some("active".to_string())
    } else {
        None
    };
    
    // Check Raft status (if enabled)
    // TODO: Implement Raft status query when Raft integration is complete
    // Placeholder: Raft status would be queried from the global Raft node
    #[cfg(feature = "raft")]
    let raft_status: Option<RaftStatus> = {
        // Future implementation:
        // - Query Raft node for current role (leader/follower/candidate)
        // - Get current term from Raft state
        // - Get leader ID from Raft state
        None
    };
    
    #[cfg(not(feature = "raft"))]
    let raft_status: Option<RaftStatus> = None;
    
    let response = RaidStatusResponse {
        cluster_status,
        node_count,
        artifact_count,
        storage: RaidStorageStatus {
            total_size_bytes: total_size,
            quota_bytes,
            usage_percent,
            available_bytes,
        },
        mode,
        replication_status,
        raft_status,
    };
    
    AxumJson(response).into_response()
}

async fn raid_events_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();

    if let Some(event_store) = manager.event_store() {
        match event_store.read().await.load_events().await {
            Ok(events) => AxumJson(serde_json::json!({
                "events": events,
                "count": events.len()
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to load events. Context: Cannot retrieve events from event store. Suggestion: Verify event store is accessible, check file permissions, and ensure event store is properly initialized. Error: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Event store not available. Context: Event store is not initialized or accessible. Suggestion: Ensure event store is enabled in configuration and properly initialized during system startup."
            })),
        )
            .into_response()
    }
}

async fn raid_events_for_artifact_handler(Path(artifact_id): Path<String>) -> impl IntoResponse {
    let manager = raid::get_global_manager();

    if let Some(event_store) = manager.event_store() {
        match event_store
            .read()
            .await
            .get_events_for_artifact(&artifact_id)
            .await
        {
            Ok(events) => AxumJson(serde_json::json!({
                "artifact_id": artifact_id,
                "events": events,
                "count": events.len()
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to load events. Context: Cannot retrieve events from event store. Suggestion: Verify event store is accessible, check file permissions, and ensure event store is properly initialized. Error: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Event store not available. Context: Event store is not initialized or accessible. Suggestion: Ensure event store is enabled in configuration and properly initialized during system startup."
            })),
        )
            .into_response()
    }
}

async fn raid_events_range_handler(Query(params): Query<EventsRangeQuery>) -> impl IntoResponse {
    let manager = raid::get_global_manager();

    if let Some(event_store) = manager.event_store() {
        let start = params
            .start
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(7));

        let end = params
            .end
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        match event_store
            .read()
            .await
            .get_events_in_range(start, end)
            .await
        {
            Ok(events) => AxumJson(serde_json::json!({
                "start": start.to_rfc3339(),
                "end": end.to_rfc3339(),
                "events": events,
                "count": events.len()
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to load events. Context: Cannot retrieve events from event store. Suggestion: Verify event store is accessible, check file permissions, and ensure event store is properly initialized. Error: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Event store not available. Context: Event store is not initialized or accessible. Suggestion: Ensure event store is enabled in configuration and properly initialized during system startup."
            })),
        )
            .into_response()
    }
}

async fn raid_snapshot_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();

    if let Some(event_store) = manager.event_store() {
        match event_store.read().await.load_snapshot().await {
            Ok(Some(snapshot)) => AxumJson(snapshot).into_response(),
            Ok(None) => (
                StatusCode::NOT_FOUND,
                AxumJson(serde_json::json!({
                    "error": "No snapshot available"
                })),
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to load snapshot. Context: Cannot retrieve snapshot from event store. Suggestion: Verify snapshot exists, check file permissions, and ensure event store is properly initialized. Error: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Event store not available. Context: Event store is not initialized or accessible. Suggestion: Ensure event store is enabled in configuration and properly initialized during system startup."
            })),
        )
            .into_response()
    }
}

async fn raid_snapshot_create_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = raid::get_global_manager();

    match manager.create_snapshot().await {
        Ok(_) => AxumJson(serde_json::json!({
            "status": "success",
            "message": "Snapshot created successfully"
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create snapshot. Context: Cannot create new snapshot in event store. Suggestion: Verify event store is accessible, check disk space, and ensure event store is properly initialized. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn raid_snapshot_restore_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = raid::get_global_manager();
    match manager.restore_from_snapshot().await {
        Ok(_) => AxumJson(serde_json::json!({
            "status": "success",
            "message": "RAID state restored from snapshot successfully"
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to restore from snapshot. Context: Cannot restore RAID state from snapshot. Suggestion: Verify snapshot exists, check event store is accessible, and ensure event store is properly initialized. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn raid_gc_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = raid::get_global_manager();
    match manager.gc_old_artifacts().await {
        Ok(removed) => AxumJson(RaidGcResponse {
            removed_count: removed,
        })
        .into_response(),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": format!("GC failed. Context: Cannot perform garbage collection on old artifacts. Suggestion: Verify RAID manager is accessible, check file permissions, and ensure storage is not locked. Error: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, AxumJson(error_response)).into_response()
        }
    }
}
