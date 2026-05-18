//! Virtual-node worker task API (FM-016 phase 3).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::state::ApiContext;
use crate::services::virtual_node_task_service::{VirtualNodeTask, VirtualNodeTaskService};

#[derive(Serialize)]
struct PollTasksResponse {
    task: Option<VirtualNodeTask>,
    pending: usize,
}

#[derive(Deserialize)]
struct EnqueueTaskRequest {
    task_type: String,
    #[serde(default)]
    payload: Value,
}

#[derive(Serialize)]
struct EnqueueTaskResponse {
    task: VirtualNodeTask,
}

#[derive(Deserialize)]
struct CompleteTaskRequest {
    status: String,
    detail: Option<String>,
}

#[derive(Serialize)]
struct CompleteTaskResponse {
    task_id: String,
    recorded: bool,
}

#[derive(Serialize)]
struct VirtualNodeTaskStatusResponse {
    peer_id: String,
    pending: usize,
    completed: usize,
}

pub fn create_virtual_node_routes() -> Router<ApiContext> {
    Router::new()
        .route(
            "/virtual-nodes/{peer_id}/tasks/poll",
            get(poll_tasks_handler),
        )
        .route("/virtual-nodes/{peer_id}/tasks", post(enqueue_task_handler))
        .route(
            "/virtual-nodes/{peer_id}/tasks/{task_id}/complete",
            post(complete_task_handler),
        )
        .route(
            "/virtual-nodes/{peer_id}/tasks/status",
            get(task_status_handler),
        )
}

async fn poll_tasks_handler(Path(peer_id): Path<String>) -> impl IntoResponse {
    let task = VirtualNodeTaskService::poll(&peer_id);
    let pending = VirtualNodeTaskService::pending_count(&peer_id);
    (StatusCode::OK, Json(PollTasksResponse { task, pending }))
}

async fn enqueue_task_handler(
    Path(peer_id): Path<String>,
    Json(body): Json<EnqueueTaskRequest>,
) -> impl IntoResponse {
    if body.task_type.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let task = VirtualNodeTaskService::enqueue(&peer_id, &body.task_type, body.payload);
    (StatusCode::CREATED, Json(EnqueueTaskResponse { task })).into_response()
}

async fn complete_task_handler(
    Path((peer_id, task_id)): Path<(String, String)>,
    Json(body): Json<CompleteTaskRequest>,
) -> impl IntoResponse {
    if body.status.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    VirtualNodeTaskService::complete(&peer_id, &task_id, &body.status, body.detail);
    (
        StatusCode::OK,
        Json(CompleteTaskResponse {
            task_id,
            recorded: true,
        }),
    )
        .into_response()
}

async fn task_status_handler(Path(peer_id): Path<String>) -> impl IntoResponse {
    let pending = VirtualNodeTaskService::pending_count(&peer_id);
    let completed = VirtualNodeTaskService::completed_count(&peer_id);
    (
        StatusCode::OK,
        Json(VirtualNodeTaskStatusResponse {
            peer_id,
            pending,
            completed,
        }),
    )
}
