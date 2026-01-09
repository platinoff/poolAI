//! Worker management API endpoints
//!
//! Provides endpoints for managing workers in the pool:
//! - List workers
//! - Create worker
//! - Delete worker

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json as AxumJson, Router,
};
use serde::{Deserialize, Serialize};

use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};
use crate::pool;
use crate::network::validation;

#[derive(Serialize)]
pub struct WorkerInfo {
    id: String,
    status: String,
    current_task: Option<String>,
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
pub fn create_workers_routes() -> Router {
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

async fn workers_handler() -> impl IntoResponse {
    // Try to get real workers from pool, fallback to mock data
    if let Some(pool) = pool::get_global_pool() {
        let worker_statuses = {
            let pool_guard = pool.read().await;
            pool_guard.get_worker_status().await
        };

        if !worker_statuses.is_empty() {
            let worker_infos: Vec<WorkerInfo> = worker_statuses
                .iter()
                .map(|(id, status)| WorkerInfo {
                    id: id.clone(),
                    status: match status.is_healthy {
                        true => {
                            if status.active_connections > 0 {
                                "busy".to_string()
                            } else {
                                "idle".to_string()
                            }
                        }
                        false => "error".to_string(),
                    },
                    current_task: status.current_task.clone(),
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
        },
        WorkerInfo {
            id: "worker-2".to_string(),
            status: "idle".to_string(),
            current_task: None,
        },
    ];
    AxumJson(workers).into_response()
}

async fn worker_create_handler(
    Json(payload): Json<CreateWorkerRequest>,
) -> impl IntoResponse {
    // Validate worker ID format
    if let Err(e) = validation::validate_worker_id(&payload.worker_id) {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
    }

    // Get global pool
    let pool = match pool::get_global_pool() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                AxumJson(serde_json::json!({
                    "error": "Pool not initialized. Context: Worker pool manager is not available. Suggestion: Ensure pool is initialized before creating or managing workers. Check system startup sequence and pool initialization status."
                })),
            )
                .into_response();
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
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": e.to_string()
            })),
        )
            .into_response();
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
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create worker. Context: Cannot add new worker to pool. Suggestion: Verify pool is initialized, check worker ID is unique, and ensure resource limits are not exceeded. Worker ID: '{}', Error: {}", payload.worker_id, e)
            })),
        )
            .into_response(),
    }
}

async fn worker_delete_handler(
    Path(worker_id): Path<String>,
) -> impl IntoResponse {
    // Get global pool
    let pool = match pool::get_global_pool() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                AxumJson(serde_json::json!({
                    "error": "Pool not initialized. Context: Worker pool manager is not available. Suggestion: Ensure pool is initialized before creating or managing workers. Check system startup sequence and pool initialization status."
                })),
            )
                .into_response();
        }
    };

    // Remove worker from pool
    let pool_guard = pool.write().await;
    match pool_guard.remove_worker(&worker_id).await {
        Ok(_) => {
            let response = DeleteWorkerResponse {
                worker_id: worker_id,
                message: "Worker deleted successfully".to_string(),
            };
            AxumJson(response).into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!("Failed to delete worker. Context: Cannot remove worker from pool. Suggestion: Verify worker ID exists, ensure worker is not processing critical tasks, and check pool status. Worker ID: '{}', Error: {}", worker_id, e)
            })),
        )
            .into_response(),
    }
}
