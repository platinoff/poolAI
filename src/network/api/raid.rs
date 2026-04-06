//! RAID management API endpoints
//!
//! Provides endpoints for managing RAID storage:
//! - List nodes and artifacts
//! - Create, delete artifacts
//! - Get quota and events
//! - Snapshot and GC operations
//! - Distributed RAID protocol endpoints

use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use chrono::DateTime;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::state::{ApiContext, AppState};
use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};
use crate::network::raid_distributed_handlers::*;
use crate::network::validation;
use crate::raid;
use crate::services::raid_service::{RaidService, RaidServiceError};

type RaidHttpErr = (StatusCode, AxumJson<serde_json::Value>);

fn raid_manager_unavailable() -> RaidHttpErr {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        AxumJson(serde_json::json!({
            "error": crate::services::raid_service::RAID_MANAGER_UNAVAILABLE_MESSAGE
        })),
    )
}

fn raid_service_http_err(e: RaidServiceError) -> RaidHttpErr {
    match e {
        RaidServiceError::ManagerUnavailable => raid_manager_unavailable(),
        RaidServiceError::ArtifactNotFound { id } => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!(
                    "Failed to delete artifact. Context: Cannot delete artifact from RAID storage. \
                    Suggestion: Verify artifact ID exists, check RAID manager status, and ensure artifact is not locked. \
                    Artifact ID: '{}', Error: Artifact {} not found",
                    id, id
                )
            })),
        ),
        RaidServiceError::Operation(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}

fn raid_http_manager(ctx: &AppState) -> Result<Arc<raid::RaidManager>, RaidHttpErr> {
    ctx.raid_manager
        .get()
        .cloned()
        .ok_or_else(raid_manager_unavailable)
}

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
struct RaidGcResponse {
    removed_count: usize,
}

#[derive(Deserialize)]
struct EventsRangeQuery {
    start: Option<String>, // ISO 8601 timestamp
    end: Option<String>,   // ISO 8601 timestamp
}

/// Create RAID management routes
pub fn create_raid_routes() -> Router<ApiContext> {
    Router::new()
        .route("/raid/nodes", get(raid_nodes_handler))
        // RAID Workers (nodes) management endpoints
        .route("/raid/workers", get(raid_workers_handler))
        .route(
            "/raid/workers",
            post(raid_worker_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/raid/workers/{id}", get(raid_worker_get_handler))
        .route(
            "/raid/workers/{id}",
            axum::routing::put(raid_worker_update_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/raid/workers/{id}",
            axum::routing::delete(raid_worker_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
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
        // Administrative Control Plane endpoints
        .route("/raid/strategies", get(raid_strategies_handler))
        .route("/raid/metrics", get(raid_metrics_handler))
        .route(
            "/raid/rebalance",
            post(raid_rebalance_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/raid/health", get(raid_health_handler))
}

async fn raid_nodes_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::list_nodes(&ctx).await {
        Ok(nodes) => AxumJson(nodes).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_artifacts_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::list_artifacts(&ctx).await {
        Ok(artifacts) => AxumJson(artifacts).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_artifact_create_handler(
    State(ctx): State<ApiContext>,
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

    let data_size = data.len();
    match RaidService::put_artifact(&ctx, &payload.name, &data).await {
        Ok(artifact) => {
            let response = CreateArtifactResponse {
                artifact_id: artifact.id.to_string(),
                name: artifact.name,
                stored_at: artifact.stored_at.to_rfc3339(),
                message: "Artifact created successfully".to_string(),
            };
            (StatusCode::CREATED, AxumJson(response)).into_response()
        }
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(RaidServiceError::ArtifactNotFound { .. }) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": "Unexpected RAID state while creating artifact."
            })),
        )
            .into_response(),
        Err(RaidServiceError::Operation(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create artifact. Context: Cannot store artifact in RAID storage. Suggestion: Check storage quota, verify RAID manager is initialized, and ensure sufficient disk space. Artifact name: '{}', Data size: {} bytes, Error: {}", artifact_name, data_size, e)
            })),
        )
            .into_response(),
    }
}

async fn raid_artifact_delete_handler(
    State(ctx): State<ApiContext>,
    Path(artifact_id): Path<String>,
) -> impl IntoResponse {
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

    match RaidService::delete_artifact(&ctx, id).await {
        Ok(()) => {
            let response = DeleteArtifactResponse {
                artifact_id,
                message: "Artifact deleted successfully".to_string(),
            };
            AxumJson(response).into_response()
        }
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(RaidServiceError::ArtifactNotFound { id }) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!(
                    "Failed to delete artifact. Context: Cannot delete artifact from RAID storage. \
                    Suggestion: Verify artifact ID exists, check RAID manager status, and ensure artifact is not locked. \
                    Artifact ID: '{}', Error: Artifact {} not found",
                    id, id
                )
            })),
        )
            .into_response(),
        Err(RaidServiceError::Operation(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!(
                    "Failed to delete artifact. Context: Cannot delete artifact from RAID storage. \
                    Suggestion: Verify artifact ID exists, check RAID manager status, and ensure artifact is not locked. \
                    Artifact ID: '{}', Error: {}",
                    artifact_id, e
                )
            })),
        )
            .into_response(),
    }
}

async fn raid_quota_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::quota(&ctx).await {
        Ok(body) => AxumJson(body).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_status_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::cluster_status(&ctx).await {
        Ok(body) => AxumJson(body).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_events_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

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

async fn raid_events_for_artifact_handler(
    State(ctx): State<ApiContext>,
    Path(artifact_id): Path<String>,
) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

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

async fn raid_events_range_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<EventsRangeQuery>,
) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

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

async fn raid_snapshot_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

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

async fn raid_snapshot_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };

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

async fn raid_snapshot_restore_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
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

async fn raid_gc_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
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

// ============================================================================
// RAID Workers (nodes) handlers
// ============================================================================

#[derive(Deserialize)]
struct CreateRaidWorkerRequest {
    address: String, // Network address (e.g., "192.168.1.100:8080")
}

#[derive(serde::Serialize)]
struct RaidWorkerResponse {
    id: String,
    address: String,
    last_seen: String,
}

async fn raid_workers_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let nodes = manager.list_nodes().await;
    let workers: Vec<RaidWorkerResponse> = nodes
        .into_iter()
        .map(|node| RaidWorkerResponse {
            id: node.id.to_string(),
            address: node.address,
            last_seen: node.last_seen.to_rfc3339(),
        })
        .collect();
    AxumJson(workers).into_response()
}

async fn raid_worker_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format: {}", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_node(uuid).await {
        Some(node) => {
            let response = RaidWorkerResponse {
                id: node.id.to_string(),
                address: node.address,
                last_seen: node.last_seen.to_rfc3339(),
            };
            AxumJson(response).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!("RAID worker not found: {}", id)
            })),
        )
            .into_response(),
    }
}

async fn raid_worker_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateRaidWorkerRequest>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    if payload.address.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": "Address cannot be empty".to_string()
            })),
        )
            .into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let node = manager.register_node(payload.address.clone()).await;
    let response = RaidWorkerResponse {
        id: node.id.to_string(),
        address: node.address,
        last_seen: node.last_seen.to_rfc3339(),
    };
    (StatusCode::CREATED, AxumJson(response)).into_response()
}

async fn raid_worker_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<CreateRaidWorkerRequest>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format: {}", id)
                })),
            )
                .into_response();
        }
    };

    match manager.update_node(uuid, Some(payload.address)).await {
        Ok(node) => {
            let response = RaidWorkerResponse {
                id: node.id.to_string(),
                address: node.address,
                last_seen: node.last_seen.to_rfc3339(),
            };
            AxumJson(response).into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response(),
    }
}

async fn raid_worker_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format: {}", id)
                })),
            )
                .into_response();
        }
    };

    match manager.delete_node(uuid).await {
        Ok(_) => AxumJson(serde_json::json!({
            "message": format!("RAID worker {} deleted successfully", id)
        }))
        .into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response(),
    }
}

// ============================================================================
// Administrative Control Plane handlers
// ============================================================================

#[derive(serde::Serialize)]
struct RaidStrategiesResponse {
    strategies: Vec<raid::StrategyStatus>,
    current_mode: String,
}

#[derive(serde::Serialize)]
struct RaidMetricsResponse {
    mode: String,
    total_artifacts: usize,
    total_size_bytes: u64,
    quota_bytes: Option<u64>,
    usage_percent: Option<f64>,
    node_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clustering_coefficient: Option<f64>,
}

#[derive(serde::Serialize)]
struct RaidRebalanceResponse {
    success: bool,
    artifacts_moved: usize,
    message: String,
}

#[derive(serde::Serialize)]
struct RaidHealthResponse {
    status: String, // "healthy", "degraded", "unhealthy"
    mode: String,
    strategy_initialized: bool,
    storage_available: bool,
    replication_active: bool,
}

/// Get strategy status for all available strategies
async fn raid_strategies_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let current_mode = manager.get_mode().await;

    // Get status for current strategy
    match manager.get_strategy_status().await {
        Ok(status) => {
            let response = RaidStrategiesResponse {
                strategies: vec![status],
                current_mode: format!("{:?}", current_mode),
            };
            AxumJson(response).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to get strategy status. Context: Cannot retrieve strategy status from RAID manager. Suggestion: Verify RAID manager is initialized and strategy is properly configured. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

/// Get metrics for the active RAID strategy
async fn raid_metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let mode = manager.get_mode().await;
    let mode_str = format!("{:?}", mode);

    let artifacts = manager.list_artifacts().await;
    let total_artifacts = artifacts.len();
    let total_size = manager.get_total_size().await.unwrap_or(0);
    let quota_bytes = manager.get_quota_bytes().await;
    let usage_percent = quota_bytes.map(|quota| {
        if quota > 0 {
            (total_size as f64 / quota as f64) * 100.0
        } else {
            0.0
        }
    });
    let nodes = manager.list_nodes().await;
    let node_count = nodes.len();

    // Strategy-specific metrics
    let replication_factor = match mode {
        raid::RaidMode::BurstRaid => {
            // Get actual replication factor from BurstRAID strategy
            if let Some(metrics) = manager.get_burst_raid_metrics().await {
                Some(metrics.base_replication_factor)
            } else {
                Some(2) // Default base_replication_factor
            }
        }
        raid::RaidMode::SmallWorld => {
            // Get actual replication factor from SmallWorld strategy
            if let Some(metrics) = manager.get_small_world_metrics().await {
                Some(metrics.base_replication_factor)
            } else {
                Some(3) // Default base_replication_factor
            }
        }
        _ => None,
    };

    let clustering_coefficient = match mode {
        raid::RaidMode::SmallWorld => {
            // Get actual clustering coefficient from SmallWorld strategy
            if let Some(metrics) = manager.get_small_world_metrics().await {
                Some(metrics.avg_clustering_coefficient)
            } else {
                None
            }
        }
        _ => None,
    };

    let response = RaidMetricsResponse {
        mode: mode_str,
        total_artifacts,
        total_size_bytes: total_size,
        quota_bytes,
        usage_percent,
        node_count,
        replication_factor,
        clustering_coefficient,
    };

    AxumJson(response).into_response()
}

/// Trigger manual rebalancing for the active strategy
async fn raid_rebalance_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }

    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    match manager.trigger_rebalance().await {
        Ok(result) => {
            let response = RaidRebalanceResponse {
                success: result.success,
                artifacts_moved: result.artifacts_moved,
                message: format!(
                    "Rebalancing completed successfully. {} artifacts moved.",
                    result.artifacts_moved
                ),
            };
            AxumJson(response).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to trigger rebalancing. Context: Cannot trigger rebalancing for RAID strategy. Suggestion: Verify RAID manager is initialized, check strategy mode (Local mode does not support rebalancing), and ensure strategy is properly configured. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

/// Health check for RAID strategies
async fn raid_health_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = match raid_http_manager(&ctx) {
        Ok(m) => m,
        Err(e) => return e.into_response(),
    };
    let mode = manager.get_mode().await;
    let mode_str = format!("{:?}", mode);

    // Get strategy status
    let strategy_status = manager.get_strategy_status().await.ok();
    let strategy_initialized = strategy_status
        .as_ref()
        .map(|s| s.initialized)
        .unwrap_or(false);

    // Check storage availability
    let total_size = manager.get_total_size().await.unwrap_or(0);
    let quota_bytes = manager.get_quota_bytes().await;
    let storage_available = quota_bytes
        .map(|quota| {
            let usage_percent = if quota > 0 {
                (total_size as f64 / quota as f64) * 100.0
            } else {
                0.0
            };
            usage_percent < 95.0 // Consider available if usage < 95%
        })
        .unwrap_or(true);

    // Check replication status
    let replication_active = match mode {
        raid::RaidMode::Local => false,
        raid::RaidMode::BurstRaid | raid::RaidMode::SmallWorld => strategy_status
            .as_ref()
            .map(|s| s.active && s.rebalancing_enabled)
            .unwrap_or(false),
    };

    // Determine overall health status
    let status = if !strategy_initialized && mode != raid::RaidMode::Local {
        "unhealthy"
    } else if !storage_available {
        "degraded"
    } else if !replication_active && mode != raid::RaidMode::Local {
        "degraded"
    } else {
        "healthy"
    };

    let response = RaidHealthResponse {
        status: status.to_string(),
        mode: mode_str,
        strategy_initialized,
        storage_available,
        replication_active,
    };

    AxumJson(response).into_response()
}
