//! Worker management API endpoints
//!
//! Provides endpoints for managing workers in the pool:
//! - List workers
//! - Create worker
//! - Delete worker

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json as AxumJson, Router,
};
use serde::{Deserialize, Serialize};

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::network::auth::auth_middleware;
use crate::network::validation;
use crate::services::worker_pool_service::{
    AddWorkerError, CreateWorkerInput, RemoveWorkerError, WorkerPoolService,
};

/// Re-export for callers that imported `WorkerInfo` from this module.
pub use crate::services::worker_pool_service::WorkerInfo;

#[derive(Deserialize)]
struct CreateWorkerRequest {
    worker_id: String,
    max_concurrent_requests: Option<usize>,
    request_timeout_ms: Option<u64>,
    health_check_interval_ms: Option<u64>,
    enable_caching: Option<bool>,
    cache_size: Option<usize>,
    max_memory_mb: Option<usize>,
    cpu_priority: Option<u8>,
    gpu_device: Option<usize>,
    auto_restart: Option<bool>,
    resource_monitoring: Option<bool>,
}

#[derive(Serialize)]
struct CreateWorkerResponse {
    worker_id: String,
    message: String,
}

#[derive(Serialize)]
struct DeleteWorkerResponse {
    worker_id: String,
    message: String,
}

/// Create worker management routes
pub fn create_workers_routes() -> Router<ApiContext> {
    Router::new()
        .route("/workers", get(workers_handler))
        .route(
            "/workers",
            post(worker_create_handler).layer(axum::middleware::from_fn(auth_middleware)),
        )
        .route(
            "/workers/{id}",
            delete(worker_delete_handler).layer(axum::middleware::from_fn(auth_middleware)),
        )
}

async fn workers_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let workers = WorkerPoolService::list_workers(&ctx).await;
    AxumJson(workers).into_response()
}

async fn worker_create_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<CreateWorkerRequest>,
) -> impl IntoResponse {
    if let Err(e) = validation::validate_worker_id(&payload.worker_id) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("create_worker").with_resource("worker_id", &payload.worker_id)),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    let max_concurrent_requests = payload.max_concurrent_requests.unwrap_or(10);
    let request_timeout_ms = payload.request_timeout_ms.unwrap_or(5000);
    let health_check_interval_ms = payload.health_check_interval_ms.unwrap_or(1000);
    let cache_size = payload.cache_size.unwrap_or(1000);
    let max_memory_mb = payload.max_memory_mb.unwrap_or(2048);
    let cpu_priority = payload.cpu_priority.unwrap_or(5);

    if let Err(e) = validation::validate_worker_config(
        max_concurrent_requests,
        request_timeout_ms,
        health_check_interval_ms,
        cache_size,
        max_memory_mb,
        cpu_priority,
    ) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("create_worker").with_resource("worker_id", &payload.worker_id)),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    let input = CreateWorkerInput {
        worker_id: payload.worker_id.clone(),
        max_concurrent_requests,
        request_timeout_ms,
        health_check_interval_ms,
        enable_caching: payload.enable_caching.unwrap_or(true),
        cache_size,
        max_memory_mb,
        cpu_priority,
        gpu_device: payload.gpu_device,
        auto_restart: payload.auto_restart.unwrap_or(true),
        resource_monitoring: payload.resource_monitoring.unwrap_or(true),
    };

    match WorkerPoolService::add_worker(&ctx, input).await {
        Ok(()) => {
            let response = CreateWorkerResponse {
                worker_id: payload.worker_id,
                message: "Worker created successfully".to_string(),
            };
            (StatusCode::CREATED, AxumJson(response)).into_response()
        }
        Err(AddWorkerError::PoolNotReady) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Pool not initialized; worker pool manager is not available",
                Some(ErrorContext::new("create_worker").with_resource("pool", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(AddWorkerError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!(
                    "Failed to create worker: {} (worker_id='{}')",
                    e, payload.worker_id
                ),
                Some(
                    ErrorContext::new("create_worker")
                        .with_resource("worker_id", &payload.worker_id),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn worker_delete_handler(
    State(ctx): State<ApiContext>,
    Path(worker_id): Path<String>,
) -> impl IntoResponse {
    match WorkerPoolService::remove_worker(&ctx, &worker_id).await {
        Ok(()) => {
            let response = DeleteWorkerResponse {
                worker_id,
                message: "Worker deleted successfully".to_string(),
            };
            AxumJson(response).into_response()
        }
        Err(RemoveWorkerError::PoolNotReady) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Pool not initialized; worker pool manager is not available",
                Some(ErrorContext::new("delete_worker").with_resource("pool", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(RemoveWorkerError::Operation(e)) => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                format!("Failed to delete worker: {} (worker_id='{}')", e, worker_id),
                Some(ErrorContext::new("delete_worker").with_resource("worker_id", &worker_id)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}
