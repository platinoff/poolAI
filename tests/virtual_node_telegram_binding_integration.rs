//! FM-016+: Telegram user binding and webhook → virtual-node task enqueue.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::services::virtual_node_task_service::VirtualNodeTaskService;
use poolai::services::virtual_node_telegram_binding_service::VirtualNodeTelegramBindingService;
use poolai::services::virtual_node_telegram_wallet_service::VirtualNodeTelegramWalletService;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

static WEBHOOK_SECRET_ENV_LOCK: Mutex<()> = Mutex::new(());
static TELEGRAM_WALLET_TEST_LOCK: Mutex<()> = Mutex::new(());

struct WebhookSecretEnvGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl WebhookSecretEnvGuard {
    fn install(secret: Option<&str>) -> Self {
        let lock = WEBHOOK_SECRET_ENV_LOCK.lock().unwrap();
        if let Some(value) = secret {
            std::env::set_var("POOLAI_TELEGRAM_WEBHOOK_SECRET", value);
        } else {
            std::env::remove_var("POOLAI_TELEGRAM_WEBHOOK_SECRET");
        }
        Self { _lock: lock }
    }
}

impl Drop for WebhookSecretEnvGuard {
    fn drop(&mut self) {
        std::env::remove_var("POOLAI_TELEGRAM_WEBHOOK_SECRET");
    }
}

async fn app_with_discovery() -> Router {
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18080)),
        None,
    ));
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

#[tokio::test]
async fn register_remote_auto_binds_telegram_metadata() {
    VirtualNodeTelegramBindingService::clear_all();
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "tg-bind-peer",
                "address": "127.0.0.1",
                "port": 19092,
                "metadata": {
                    "role": "virtual_node",
                    "channel": "telegram",
                    "telegram_id": "9001"
                }
            }"#,
        ))
        .unwrap();
    let reg = app.clone().oneshot(register).await.unwrap();
    assert_eq!(reg.status(), StatusCode::OK);

    let get = Request::builder()
        .uri("/api/v1/virtual-nodes/telegram/bindings/9001")
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(get).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["peer_id"], "tg-bind-peer");

    VirtualNodeTelegramBindingService::clear_all();
}

#[tokio::test]
async fn telegram_webhook_enqueues_task_for_bound_peer() {
    let _env = WebhookSecretEnvGuard::install(None);
    let peer = "tg-webhook-peer";
    VirtualNodeTelegramBindingService::clear_all();
    VirtualNodeTaskService::clear_peer(peer);

    let app = app_with_discovery().await;

    VirtualNodeTelegramBindingService::bind("4242", None, peer);

    let webhook = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/webhook")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "update_id": 7,
                "message": {
                    "from": { "id": 4242 },
                    "chat": { "id": 999 },
                    "text": "/status"
                }
            }"#,
        ))
        .unwrap();
    let res = app.clone().oneshot(webhook).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["peer_id"], peer);
    assert_eq!(v["task"]["task_type"], "telegram_command");

    let poll = Request::builder()
        .uri(format!("/api/v1/virtual-nodes/{peer}/tasks/poll"))
        .body(Body::empty())
        .unwrap();
    let poll_res = app.oneshot(poll).await.unwrap();
    let poll_body = to_bytes(poll_res.into_body(), usize::MAX).await.unwrap();
    let poll_v: Value = serde_json::from_slice(&poll_body).unwrap();
    let task = poll_v["task"].as_object().expect("telegram task");
    assert_eq!(task["task_type"], "telegram_command");

    VirtualNodeTelegramBindingService::clear_all();
    VirtualNodeTaskService::clear_peer(peer);
}

#[tokio::test]
async fn telegram_webhook_rejects_missing_secret_when_configured() {
    let _env = WebhookSecretEnvGuard::install(Some("integration-test-secret"));
    let app = app_with_discovery().await;
    VirtualNodeTelegramBindingService::bind("555", None, "secret-peer");

    let webhook = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/webhook")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"update_id":1,"message":{"from":{"id":555},"text":"hi"}}"#,
        ))
        .unwrap();
    let res = app.clone().oneshot(webhook).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    VirtualNodeTelegramBindingService::clear_all();
}

#[tokio::test]
async fn telegram_webhook_truncates_oversized_message_text() {
    let _env = WebhookSecretEnvGuard::install(None);
    VirtualNodeTelegramBindingService::clear_all();
    let peer = "tg-truncate-peer";
    VirtualNodeTaskService::clear_peer(peer);
    let app = app_with_discovery().await;
    VirtualNodeTelegramBindingService::bind("7777", None, peer);

    let long_text = "x".repeat(5000);
    let body =
        format!(r#"{{"update_id":9,"message":{{"from":{{"id":7777}},"text":"{long_text}"}}}}"#);
    let webhook = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/webhook")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.clone().oneshot(webhook).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let poll = Request::builder()
        .uri(format!("/api/v1/virtual-nodes/{peer}/tasks/poll"))
        .body(Body::empty())
        .unwrap();
    let poll_res = app.oneshot(poll).await.unwrap();
    let poll_body = to_bytes(poll_res.into_body(), usize::MAX).await.unwrap();
    let poll_v: Value = serde_json::from_slice(&poll_body).unwrap();
    let text = poll_v["task"]["payload"]["text"].as_str().expect("text");
    assert_eq!(text.chars().count(), 4096);

    VirtualNodeTelegramBindingService::clear_all();
    VirtualNodeTaskService::clear_peer(peer);
}

#[tokio::test]
async fn telegram_wallet_bind_stub_creates_verified_binding() {
    let _lock = TELEGRAM_WALLET_TEST_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = app_with_discovery().await;
    let pubkey = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

    let bind = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/wallet")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "telegram_user_id": "wallet-user-1",
                "chat_id": "-1001234567890",
                "payout_pubkey": "{pubkey}",
                "chain": "solana"
            }}"#
        )))
        .unwrap();
    let res = app.oneshot(bind).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["wallet"]["telegram_user_id"], "wallet-user-1");
    assert_eq!(v["wallet"]["payout_pubkey"], pubkey);
    assert_eq!(v["wallet"]["chain"], "solana");
    assert_eq!(v["wallet"]["verified"], true);

    let row = VirtualNodeTelegramWalletService::lookup("wallet-user-1").expect("stored");
    assert_eq!(row.payout_pubkey, pubkey);
    VirtualNodeTelegramWalletService::clear_all();
}

#[tokio::test]
async fn telegram_wallet_bind_rejects_invalid_pubkey() {
    let _lock = TELEGRAM_WALLET_TEST_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = app_with_discovery().await;

    let bind = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/wallet")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "telegram_user_id": "wallet-user-2",
                "chat_id": "-10099",
                "payout_pubkey": "not-valid!",
                "chain": "solana"
            }"#,
        ))
        .unwrap();
    let res = app.oneshot(bind).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    VirtualNodeTelegramWalletService::clear_all();
}

#[tokio::test]
async fn telegram_wallet_get_lookup_returns_bound_wallet() {
    let _lock = TELEGRAM_WALLET_TEST_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = app_with_discovery().await;
    let pubkey = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

    let bind = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/wallet")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "telegram_user_id": "wallet-get-1",
                "chat_id": "-1001234567890",
                "payout_pubkey": "{pubkey}",
                "chain": "solana"
            }}"#
        )))
        .unwrap();
    let bind_res = app.clone().oneshot(bind).await.unwrap();
    assert_eq!(bind_res.status(), StatusCode::OK);

    let lookup = Request::builder()
        .method("GET")
        .uri("/api/v1/virtual-nodes/telegram/wallets/wallet-get-1")
        .body(Body::empty())
        .unwrap();
    let lookup_res = app.clone().oneshot(lookup).await.unwrap();
    assert_eq!(lookup_res.status(), StatusCode::OK);
    let body = to_bytes(lookup_res.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["wallet"]["telegram_user_id"], "wallet-get-1");
    assert_eq!(v["wallet"]["payout_pubkey"], pubkey);

    VirtualNodeTelegramWalletService::clear_all();
}

#[tokio::test]
async fn telegram_wallet_get_lookup_missing_returns_404() {
    let _lock = TELEGRAM_WALLET_TEST_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = app_with_discovery().await;

    let lookup = Request::builder()
        .method("GET")
        .uri("/api/v1/virtual-nodes/telegram/wallets/no-such-user")
        .body(Body::empty())
        .unwrap();
    let res = app.oneshot(lookup).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    VirtualNodeTelegramWalletService::clear_all();
}
