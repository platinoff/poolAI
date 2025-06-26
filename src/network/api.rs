// network/api.rs
use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};
use serde::Serialize;
use crate::platform;

#[derive(Serialize)]
struct StatusResponse {
    status: &'static str,
    version: &'static str,
    uptime: u64,
}

#[derive(Serialize)]
struct MetricsResponse {
    active_workers: u32,
    total_requests: u64,
    avg_response_time: f64,
}

#[derive(Serialize)]
struct ModelInfo {
    name: &'static str,
    status: &'static str,
    memory_usage: u64,
}

#[derive(Serialize)]
struct WorkerInfo {
    id: &'static str,
    status: &'static str,
    current_task: Option<&'static str>,
}

pub fn create_api_routes() -> Router {
    Router::new()
        .route("/status", get(status_handler))
        .route("/metrics", get(metrics_handler))
        .route("/models", get(models_handler))
        .route("/workers", get(workers_handler))
        .route("/gpu", get(gpu_info))
}

async fn status_handler() -> impl IntoResponse {
    let status = StatusResponse {
        status: "running",
        version: "0.1.0",
        uptime: 3600,
    };
    Json(status)
}

async fn metrics_handler() -> impl IntoResponse {
    let metrics = MetricsResponse {
        active_workers: 5,
        total_requests: 1234,
        avg_response_time: 0.045,
    };
    Json(metrics)
}

async fn models_handler() -> impl IntoResponse {
    let models = vec![
        ModelInfo {
            name: "llama-2-7b",
            status: "loaded",
            memory_usage: 8192,
        },
        ModelInfo {
            name: "gpt-3.5-turbo",
            status: "available",
            memory_usage: 4096,
        },
    ];
    Json(models)
}

async fn workers_handler() -> impl IntoResponse {
    let workers = vec![
        WorkerInfo {
            id: "worker-1",
            status: "busy",
            current_task: Some("text-generation"),
        },
        WorkerInfo {
            id: "worker-2",
            status: "idle",
            current_task: None,
        },
    ];
    Json(workers)
}

async fn gpu_info() -> impl IntoResponse {
    let info = platform::get_gpu_info();
    Json(info)
} 