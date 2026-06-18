//! Virtual-node worker task API (FM-016 phase 3) and Telegram binding (FM-016+).
//!
//! **FM-017:** error responses stay **HTTP status only** (no `HttpAppError` JSON) so
//! `poolai-worker` can keep using `response.status().is_success()` without parsing bodies.
//! Discovery/admin routes use structured JSON errors per FM-005.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::core::discovery_types::PeerInfo;
use crate::core::state::ApiContext;
use crate::network::validation;
use crate::services::discovery_service::{DiscoveryNotReady, DiscoveryService};
use crate::services::virtual_node_task_service::{VirtualNodeTask, VirtualNodeTaskService};
use crate::services::virtual_node_telegram_binding_service::{
    TelegramBinding, VirtualNodeTelegramBindingService,
};
use crate::services::virtual_node_telegram_wallet_service::{
    TelegramWalletBinding, VirtualNodeTelegramWalletService, WalletBindError,
};
use crate::services::worker_pool_service::{AddWorkerError, CreateWorkerInput, WorkerPoolService};

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

#[derive(Deserialize)]
struct BindTelegramRequest {
    telegram_user_id: String,
    peer_id: String,
    #[serde(default)]
    chat_id: Option<String>,
}

#[derive(Serialize)]
struct BindTelegramResponse {
    binding: TelegramBinding,
}

#[derive(Serialize)]
struct TelegramBindingsListResponse {
    bindings: Vec<TelegramBinding>,
}

#[derive(Deserialize)]
struct BindTelegramWalletRequest {
    telegram_user_id: String,
    chat_id: String,
    payout_pubkey: String,
    #[serde(default)]
    chain: Option<String>,
}

#[derive(Serialize)]
struct BindTelegramWalletResponse {
    wallet: TelegramWalletBinding,
}

#[derive(Deserialize)]
struct TelegramWebhookUpdate {
    #[serde(default)]
    update_id: i64,
    #[serde(default)]
    message: Option<TelegramWebhookMessage>,
}

#[derive(Deserialize)]
struct TelegramWebhookMessage {
    #[serde(default)]
    from: Option<TelegramWebhookUser>,
    #[serde(default)]
    chat: Option<TelegramWebhookChat>,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Deserialize)]
struct TelegramWebhookUser {
    id: i64,
}

#[derive(Deserialize)]
struct TelegramWebhookChat {
    id: i64,
}

#[derive(Serialize)]
struct TelegramWebhookResponse {
    ok: bool,
    peer_id: Option<String>,
    task: Option<VirtualNodeTask>,
    detail: Option<String>,
}

#[derive(Deserialize, Default)]
struct PoolJoinRequest {
    max_memory_mb: Option<usize>,
    max_concurrent_requests: Option<usize>,
}

#[derive(Serialize)]
struct PoolJoinResponse {
    peer_id: String,
    joined: bool,
    message: String,
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
        .route("/virtual-nodes/telegram/bind", post(bind_telegram_handler))
        .route(
            "/virtual-nodes/telegram/wallet",
            post(bind_telegram_wallet_handler),
        )
        .route(
            "/virtual-nodes/telegram/wallets/{telegram_user_id}",
            get(get_telegram_wallet_handler),
        )
        .route(
            "/virtual-nodes/telegram/bindings",
            get(list_telegram_bindings_handler),
        )
        .route(
            "/virtual-nodes/telegram/bindings/{telegram_user_id}",
            get(get_telegram_binding_handler).delete(unbind_telegram_handler),
        )
        .route(
            "/virtual-nodes/telegram/webhook",
            post(telegram_webhook_handler),
        )
        .route(
            "/virtual-nodes/{peer_id}/pool/join",
            post(pool_join_handler),
        )
}

fn is_virtual_node_peer(peer: &PeerInfo) -> bool {
    peer.metadata.get("role").map(String::as_str) == Some("virtual_node")
}

async fn pool_join_handler(
    State(ctx): State<ApiContext>,
    Path(peer_id): Path<String>,
    Json(body): Json<PoolJoinRequest>,
) -> impl IntoResponse {
    if let Err(e) = validation::validate_worker_id(&peer_id) {
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }

    let peer = match DiscoveryService::get_peer(&ctx, &peer_id).await {
        Ok(Some(p)) if is_virtual_node_peer(&p) => p,
        Ok(Some(_)) => {
            return (
                StatusCode::FORBIDDEN,
                "peer is not a registered virtual node",
            )
                .into_response();
        }
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(DiscoveryNotReady) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };

    let max_memory_mb = body
        .max_memory_mb
        .unwrap_or(peer.capabilities.memory_mb.max(512));
    let max_concurrent_requests = body.max_concurrent_requests.unwrap_or(10);

    let input = CreateWorkerInput {
        worker_id: peer_id.clone(),
        max_concurrent_requests,
        request_timeout_ms: 5000,
        health_check_interval_ms: 1000,
        enable_caching: true,
        cache_size: 1000,
        max_memory_mb,
        cpu_priority: 5,
        gpu_device: None,
        auto_restart: true,
        resource_monitoring: true,
    };

    match WorkerPoolService::add_worker(&ctx, input).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(PoolJoinResponse {
                peer_id,
                joined: true,
                message: "virtual node joined worker pool".to_string(),
            }),
        )
            .into_response(),
        Err(AddWorkerError::PoolNotReady) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        Err(AddWorkerError::Operation(_)) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
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

async fn bind_telegram_wallet_handler(
    Json(body): Json<BindTelegramWalletRequest>,
) -> impl IntoResponse {
    match VirtualNodeTelegramWalletService::bind(
        &body.telegram_user_id,
        &body.chat_id,
        &body.payout_pubkey,
        body.chain.as_deref(),
    ) {
        Ok(wallet) => (StatusCode::OK, Json(BindTelegramWalletResponse { wallet })).into_response(),
        Err(WalletBindError::RebindCooldown { retry_after_secs }) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "wallet_rebind_cooldown",
                "message": "wallet rebind cooldown active",
                "retry_after_secs": retry_after_secs,
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.as_status_message()).into_response(),
    }
}

async fn get_telegram_wallet_handler(Path(telegram_user_id): Path<String>) -> impl IntoResponse {
    match VirtualNodeTelegramWalletService::lookup(&telegram_user_id) {
        Some(wallet) => {
            (StatusCode::OK, Json(BindTelegramWalletResponse { wallet })).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn bind_telegram_handler(Json(body): Json<BindTelegramRequest>) -> impl IntoResponse {
    if body.telegram_user_id.trim().is_empty() || body.peer_id.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let binding = VirtualNodeTelegramBindingService::bind(
        body.telegram_user_id.trim(),
        body.chat_id,
        body.peer_id.trim(),
    );
    (StatusCode::OK, Json(BindTelegramResponse { binding })).into_response()
}

async fn list_telegram_bindings_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(TelegramBindingsListResponse {
            bindings: VirtualNodeTelegramBindingService::list(),
        }),
    )
}

async fn get_telegram_binding_handler(Path(telegram_user_id): Path<String>) -> impl IntoResponse {
    match VirtualNodeTelegramBindingService::lookup(&telegram_user_id) {
        Some(binding) => (StatusCode::OK, Json(binding)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn unbind_telegram_handler(Path(telegram_user_id): Path<String>) -> impl IntoResponse {
    if VirtualNodeTelegramBindingService::unbind(&telegram_user_id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

/// Telegram message text cap (DoS guard for task queue payloads).
const TELEGRAM_WEBHOOK_MAX_TEXT: usize = 4096;

fn truncate_webhook_text(text: String) -> String {
    if text.chars().count() <= TELEGRAM_WEBHOOK_MAX_TEXT {
        text
    } else {
        text.chars().take(TELEGRAM_WEBHOOK_MAX_TEXT).collect()
    }
}

fn webhook_secret_ok(headers: &HeaderMap) -> bool {
    let expected = match std::env::var("POOLAI_TELEGRAM_WEBHOOK_SECRET") {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return true,
    };
    headers
        .get("x-telegram-webhook-secret")
        .and_then(|v| v.to_str().ok())
        == Some(expected.as_str())
}

async fn telegram_webhook_handler(
    headers: HeaderMap,
    Json(update): Json<TelegramWebhookUpdate>,
) -> impl IntoResponse {
    if !webhook_secret_ok(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let message = match update.message {
        Some(m) => m,
        None => {
            return (
                StatusCode::OK,
                Json(TelegramWebhookResponse {
                    ok: true,
                    peer_id: None,
                    task: None,
                    detail: Some("ignored: no message".into()),
                }),
            )
                .into_response();
        }
    };

    let user_id = message
        .from
        .as_ref()
        .map(|u| u.id.to_string())
        .or_else(|| message.chat.as_ref().map(|c| c.id.to_string()));

    let Some(telegram_user_id) = user_id else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let binding = match VirtualNodeTelegramBindingService::lookup(&telegram_user_id) {
        Some(b) => b,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(TelegramWebhookResponse {
                    ok: false,
                    peer_id: None,
                    task: None,
                    detail: Some(format!("no binding for telegram user {telegram_user_id}")),
                }),
            )
                .into_response();
        }
    };

    let text = truncate_webhook_text(message.text.unwrap_or_default());
    let task_type = if text.starts_with('/') {
        "telegram_command"
    } else {
        "telegram_message"
    };
    let payload = json!({
        "telegram_user_id": telegram_user_id,
        "chat_id": message.chat.as_ref().map(|c| c.id),
        "text": text,
        "update_id": update.update_id,
    });
    let task = VirtualNodeTaskService::enqueue(&binding.peer_id, task_type, payload);

    (
        StatusCode::OK,
        Json(TelegramWebhookResponse {
            ok: true,
            peer_id: Some(binding.peer_id),
            task: Some(task),
            detail: None,
        }),
    )
        .into_response()
}
