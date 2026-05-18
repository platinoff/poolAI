//! FM-016+: Telegram user binding and webhook → virtual-node task enqueue.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::services::virtual_node_task_service::VirtualNodeTaskService;
use poolai::services::virtual_node_telegram_binding_service::VirtualNodeTelegramBindingService;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

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
