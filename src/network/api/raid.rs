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
use uuid::Uuid;

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::network::api::common::{api_error_response, api_json_error, check_permission};
use crate::network::auth::{auth_middleware, Claims};
use crate::network::raid_distributed_handlers::*;
use crate::network::validation;
use crate::services::raid_service::{RaidService, RaidServiceError};

type RaidHttpErr = (StatusCode, AxumJson<serde_json::Value>);

fn raid_api_err(
    code: impl AsRef<str>,
    message: impl Into<String>,
    ctx: Option<ErrorContext>,
    status: StatusCode,
) -> RaidHttpErr {
    let (s, j) = api_json_error(code, message, ctx, status);
    (s, AxumJson(j.0))
}

fn raid_event_store_unavailable(operation: impl Into<String>) -> RaidHttpErr {
    raid_api_err(
        "EVENT_STORE_UNAVAILABLE",
        "Event store is not initialized or accessible.",
        Some(ErrorContext::new(operation.into()).with_hint(
            "Enable event store in configuration and ensure initialization at startup.",
        )),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

fn raid_manager_unavailable() -> RaidHttpErr {
    raid_api_err(
        "RAID_MANAGER_UNAVAILABLE",
        crate::services::raid_service::RAID_MANAGER_UNAVAILABLE_MESSAGE,
        Some(
            ErrorContext::new("raid")
                .with_hint("Ensure RAID manager is initialized during application startup."),
        ),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

fn raid_service_http_err(e: RaidServiceError) -> RaidHttpErr {
    match e {
        RaidServiceError::ManagerUnavailable => raid_manager_unavailable(),
        RaidServiceError::ArtifactNotFound { id } => raid_api_err(
            "ARTIFACT_NOT_FOUND",
            format!("Artifact {} not found", id),
            Some(ErrorContext::new("raid_artifact").with_resource("artifact_id", id.to_string())),
            StatusCode::NOT_FOUND,
        ),
        RaidServiceError::WorkerNotFound { id } => raid_api_err(
            "RAID_WORKER_NOT_FOUND",
            format!("RAID worker not found: {}", id),
            Some(ErrorContext::new("raid_worker_get").with_resource("worker_id", id.to_string())),
            StatusCode::NOT_FOUND,
        ),
        RaidServiceError::EventStoreUnavailable { operation } => {
            raid_event_store_unavailable(operation)
        }
        RaidServiceError::Operation(ref err) => {
            let (s, j) = api_error_response(
                err,
                Some(ErrorContext::new("raid")),
                Some(StatusCode::INTERNAL_SERVER_ERROR),
            );
            (s, AxumJson(j.0))
        }
    }
}

/// Worker update/delete historically return HTTP 404 with a RAID_* code (even for non-not-found errors).
fn raid_worker_mutation_err(
    code: &'static str,
    err: AppError,
    operation: &'static str,
    worker_id: &str,
) -> RaidHttpErr {
    let (s, j) = api_json_error(
        code,
        err.to_string(),
        Some(ErrorContext::new(operation).with_resource("worker_id", worker_id.to_string())),
        StatusCode::NOT_FOUND,
    );
    (s, AxumJson(j.0))
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
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("raid_artifact_create").with_resource("field", "name")),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    // Validate base64 data size (max 100MB)
    const MAX_ARTIFACT_SIZE: usize = 100 * 1024 * 1024; // 100MB
    if let Err(e) = validation::validate_base64_data(&payload.data, MAX_ARTIFACT_SIZE) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("raid_artifact_create").with_resource("field", "data")),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    // Decode base64 data
    let artifact_name = payload.name.clone();
    let data = match base64_engine.decode(&payload.data) {
        Ok(d) => d,
        Err(e) => {
            let (s, j) = api_json_error(
                "INVALID_BASE64",
                format!("Cannot decode base64 artifact data: {}", e),
                Some(
                    ErrorContext::new("raid_artifact_create")
                        .with_resource("artifact_name", artifact_name.clone())
                        .with_hint("Verify payload is valid base64 and not corrupted."),
                ),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    // Validate decoded data size
    if let Err(e) = validation::validate_artifact_data_size(data.len(), MAX_ARTIFACT_SIZE) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("raid_artifact_create").with_resource("field", "decoded_size")),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
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
        Err(RaidServiceError::ArtifactNotFound { .. }) => {
            let (s, j) = api_json_error(
                "RAID_UNEXPECTED_STATE",
                "Unexpected RAID state while creating artifact.",
                Some(ErrorContext::new("raid_artifact_create")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(RaidServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "RAID_ARTIFACT_CREATE_FAILED",
                format!("Failed to store artifact in RAID: {}", e),
                Some(
                    ErrorContext::new("raid_artifact_create")
                        .with_resource("artifact_name", artifact_name.clone())
                        .with_details(format!("data_size_bytes={}", data_size))
                        .with_hint("Check storage quota, RAID manager status, and disk space."),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_artifact_delete_handler(
    State(ctx): State<ApiContext>,
    Path(artifact_id): Path<String>,
) -> impl IntoResponse {
    // Validate UUID format
    if let Err(e) = validation::validate_uuid(&artifact_id) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(
                ErrorContext::new("raid_artifact_delete")
                    .with_resource("artifact_id", &artifact_id),
            ),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
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
        Err(RaidServiceError::ArtifactNotFound { id }) => {
            let (s, j) = api_json_error(
                "ARTIFACT_NOT_FOUND",
                format!("Artifact {} not found", id),
                Some(
                    ErrorContext::new("raid_artifact_delete")
                        .with_resource("artifact_id", id.to_string()),
                ),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(RaidServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "RAID_ARTIFACT_DELETE_FAILED",
                format!("Failed to delete artifact: {}", e),
                Some(
                    ErrorContext::new("raid_artifact_delete")
                        .with_resource("artifact_id", artifact_id.clone()),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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
    match RaidService::load_all_events(&ctx).await {
        Ok(events) => AxumJson(serde_json::json!({
            "events": events,
            "count": events.len()
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_EVENTS_LOAD_FAILED",
                format!("Failed to load events: {}", err),
                Some(ErrorContext::new("raid_events")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_events_for_artifact_handler(
    State(ctx): State<ApiContext>,
    Path(artifact_id): Path<String>,
) -> impl IntoResponse {
    match RaidService::load_events_for_artifact(&ctx, &artifact_id).await {
        Ok(events) => AxumJson(serde_json::json!({
            "artifact_id": artifact_id,
            "events": events,
            "count": events.len()
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_EVENTS_LOAD_FAILED",
                format!("Failed to load events: {}", err),
                Some(
                    ErrorContext::new("raid_events_for_artifact")
                        .with_resource("artifact_id", artifact_id.clone()),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_events_range_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<EventsRangeQuery>,
) -> impl IntoResponse {
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

    match RaidService::load_events_in_range(&ctx, start, end).await {
        Ok(events) => AxumJson(serde_json::json!({
            "start": start.to_rfc3339(),
            "end": end.to_rfc3339(),
            "events": events,
            "count": events.len()
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_EVENTS_LOAD_FAILED",
                format!("Failed to load events: {}", err),
                Some(ErrorContext::new("raid_events_range")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_snapshot_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::load_snapshot(&ctx).await {
        Ok(Some(snapshot)) => AxumJson(snapshot).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "RAID_SNAPSHOT_NOT_FOUND",
                "No snapshot available",
                Some(ErrorContext::new("raid_snapshot_get")),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_SNAPSHOT_LOAD_FAILED",
                format!("Failed to load snapshot: {}", err),
                Some(ErrorContext::new("raid_snapshot_get")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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

    match RaidService::create_snapshot(&ctx).await {
        Ok(()) => AxumJson(serde_json::json!({
            "status": "success",
            "message": "Snapshot created successfully"
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_SNAPSHOT_CREATE_FAILED",
                format!("Failed to create snapshot: {}", err),
                Some(ErrorContext::new("raid_snapshot_create")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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

    match RaidService::restore_from_snapshot(&ctx).await {
        Ok(()) => AxumJson(serde_json::json!({
            "status": "success",
            "message": "RAID state restored from snapshot successfully"
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_SNAPSHOT_RESTORE_FAILED",
                format!("Failed to restore from snapshot: {}", err),
                Some(ErrorContext::new("raid_snapshot_restore")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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

    match RaidService::gc_old_artifacts(&ctx).await {
        Ok(removed) => AxumJson(RaidGcResponse {
            removed_count: removed,
        })
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_GC_FAILED",
                format!("Garbage collection failed: {}", err),
                Some(ErrorContext::new("raid_gc")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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
    match RaidService::list_workers(&ctx).await {
        Ok(nodes) => {
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
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

async fn raid_worker_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format: {}", id),
                Some(ErrorContext::new("raid_worker_get").with_resource("worker_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match RaidService::get_worker(&ctx, uuid).await {
        Ok(node) => {
            let response = RaidWorkerResponse {
                id: node.id.to_string(),
                address: node.address,
                last_seen: node.last_seen.to_rfc3339(),
            };
            AxumJson(response).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            "Address cannot be empty",
            Some(ErrorContext::new("raid_worker_create").with_resource("field", "address")),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    match RaidService::register_worker(&ctx, payload.address.clone()).await {
        Ok(node) => {
            let response = RaidWorkerResponse {
                id: node.id.to_string(),
                address: node.address,
                last_seen: node.last_seen.to_rfc3339(),
            };
            (StatusCode::CREATED, AxumJson(response)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
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

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format: {}", id),
                Some(ErrorContext::new("raid_worker_update").with_resource("worker_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match RaidService::update_worker(&ctx, uuid, Some(payload.address)).await {
        Ok(node) => {
            let response = RaidWorkerResponse {
                id: node.id.to_string(),
                address: node.address,
                last_seen: node.last_seen.to_rfc3339(),
            };
            AxumJson(response).into_response()
        }
        Err(RaidServiceError::Operation(e)) => {
            raid_worker_mutation_err("RAID_WORKER_UPDATE_FAILED", e, "raid_worker_update", &id)
                .into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
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

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format: {}", id),
                Some(ErrorContext::new("raid_worker_delete").with_resource("worker_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match RaidService::delete_worker(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("RAID worker {} deleted successfully", id)
        }))
        .into_response(),
        Err(RaidServiceError::Operation(e)) => {
            raid_worker_mutation_err("RAID_WORKER_DELETE_FAILED", e, "raid_worker_delete", &id)
                .into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

// ============================================================================
// Administrative Control Plane handlers
// ============================================================================

/// Get strategy status for all available strategies
async fn raid_strategies_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::strategies_overview(&ctx).await {
        Ok(body) => AxumJson(body).into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_STRATEGY_STATUS_FAILED",
                format!("Failed to get strategy status: {}", err),
                Some(ErrorContext::new("raid_strategies")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

/// Get metrics for the active RAID strategy
async fn raid_metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::metrics_overview(&ctx).await {
        Ok(body) => AxumJson(body).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
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

    match RaidService::trigger_rebalance(&ctx).await {
        Ok(result) => AxumJson(serde_json::json!({
            "success": result.success,
            "artifacts_moved": result.artifacts_moved,
            "message": format!(
                "Rebalancing completed successfully. {} artifacts moved.",
                result.artifacts_moved
            ),
        }))
        .into_response(),
        Err(RaidServiceError::Operation(ref err)) => {
            let (s, j) = api_json_error(
                "RAID_REBALANCE_FAILED",
                format!("Failed to trigger rebalancing: {}", err),
                Some(ErrorContext::new("raid_rebalance").with_hint(
                    "Local RAID mode does not support rebalancing; verify strategy configuration.",
                )),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => raid_service_http_err(e).into_response(),
    }
}

/// Health check for RAID strategies
async fn raid_health_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::health_overview(&ctx).await {
        Ok(body) => AxumJson(body).into_response(),
        Err(e) => raid_service_http_err(e).into_response(),
    }
}
