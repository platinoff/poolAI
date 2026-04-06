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
use crate::pool;

#[derive(Serialize)]
pub struct WorkerInfo {
    id: String,
    /// High-level state for dashboards: `idle`, `busy`, or `error`.
    status: String,
    current_task: Option<String>,
    /// Matches admin UI and detailed panels (pool `WorkerStatus`).
    is_healthy: bool,
    total_requests_processed: u64,
    queue_size: usize,
    active_connections: usize,
    average_response_time_ms: f64,
}

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
    // Try to get real workers from pool, fallback to mock data
    if let Some(pool) = ctx.pool.get() {
        let worker_statuses = {
            let pool_guard = pool.read().await;
            pool_guard.get_worker_status().await
        };

        if !worker_statuses.is_empty() {
            let worker_infos: Vec<WorkerInfo> = worker_statuses
                .iter()
                .map(|(id, status)| {
                    let status_label = match status.is_healthy {
                        true => {
                            if status.active_connections > 0 {
                                "busy".to_string()
                            } else {
                                "idle".to_string()
                            }
                        }
                        false => "error".to_string(),
                    };
                    WorkerInfo {
                        id: id.clone(),
                        status: status_label,
                        current_task: status.current_task.clone(),
                        is_healthy: status.is_healthy,
                        total_requests_processed: status.total_requests_processed,
                        queue_size: status.queue_size,
                        active_connections: status.active_connections,
                        average_response_time_ms: status.average_response_time_ms,
                    }
                })
                .collect();

            return AxumJson(worker_infos).into_response();
        }
    }

    // Fallback to mock data
    let workers = vec![
        WorkerInfo {
            id: "worker-1".to_string(),
            status: "busy".to_string(),
            current_task: Some("text-generation".to_string()),
            is_healthy: true,
            total_requests_processed: 128,
            queue_size: 0,
            active_connections: 1,
            average_response_time_ms: 24.5,
        },
        WorkerInfo {
            id: "worker-2".to_string(),
            status: "idle".to_string(),
            current_task: None,
            is_healthy: true,
            total_requests_processed: 64,
            queue_size: 0,
            active_connections: 0,
            average_response_time_ms: 18.0,
        },
    ];
    AxumJson(workers).into_response()
}

async fn worker_create_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<CreateWorkerRequest>,
) -> impl IntoResponse {
    // Validate worker ID format
    if let Err(e) = validation::validate_worker_id(&payload.worker_id) {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            e.to_string(),
            Some(ErrorContext::new("create_worker").with_resource("worker_id", &payload.worker_id)),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    // Get pool from application context
    let pool = match ctx.pool.get() {
        Some(p) => p,
        None => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Pool not initialized; worker pool manager is not available",
                Some(ErrorContext::new("create_worker").with_resource("pool", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    // Prepare worker config values with defaults
    let max_concurrent_requests = payload.max_concurrent_requests.unwrap_or(10);
    let request_timeout_ms = payload.request_timeout_ms.unwrap_or(5000);
    let health_check_interval_ms = payload.health_check_interval_ms.unwrap_or(1000);
    let cache_size = payload.cache_size.unwrap_or(1000);
    let max_memory_mb = payload.max_memory_mb.unwrap_or(2048);
    let cpu_priority = payload.cpu_priority.unwrap_or(5);

    // Validate worker configuration values
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

    // Create worker config
    let worker_config = pool::worker::WorkerConfig {
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

    // Create worker
    let worker = pool::worker::Worker::new(worker_config);

    // Add worker to pool
    let pool_guard = pool.write().await;
    match pool_guard
        .add_worker(payload.worker_id.clone(), worker)
        .await
    {
        Ok(_) => {
            let response = CreateWorkerResponse {
                worker_id: payload.worker_id,
                message: "Worker created successfully".to_string(),
            };
            (StatusCode::CREATED, AxumJson(response)).into_response()
        }
        Err(e) => {
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
    let pool = match ctx.pool.get() {
        Some(p) => p,
        None => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Pool not initialized; worker pool manager is not available",
                Some(ErrorContext::new("delete_worker").with_resource("pool", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    // Remove worker from pool
    let pool_guard = pool.write().await;
    match pool_guard.remove_worker(&worker_id).await {
        Ok(_) => {
            let response = DeleteWorkerResponse {
                worker_id,
                message: "Worker deleted successfully".to_string(),
            };
            AxumJson(response).into_response()
        }
        Err(e) => {
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
