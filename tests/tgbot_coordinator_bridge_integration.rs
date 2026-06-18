//! FM-016++: Telegram bot coordinator bridge (no teloxide / no bot token).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::services::virtual_node_telegram_binding_service::VirtualNodeTelegramBindingService;
use poolai::tgbot::coordinator::{
    fetch_telegram_seats, format_seats_reply, format_unbind_reply, forward_payload,
    message_to_webhook_payload, unbind_telegram, CoordinatorConfig,
};
use tower::ServiceExt;

#[tokio::test]
async fn webhook_accepts_bot_shaped_payload() {
    let peer = "tgbot-bridge-peer";
    VirtualNodeTelegramBindingService::clear_all();
    VirtualNodeTelegramBindingService::bind("88001", None, peer);

    let ctx = ApiContext::default();
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);

    let payload = message_to_webhook_payload(1, 88001, 100, "/status");
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/webhook")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["peer_id"], peer);

    VirtualNodeTelegramBindingService::clear_all();
}

#[tokio::test]
async fn forward_payload_posts_json_to_coordinator() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/v1/virtual-nodes/telegram/webhook")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_body(r#"{"ok":true,"peer_id":"mock-peer","task":{"task_type":"telegram_command"}}"#)
        .create_async()
        .await;

    let config = CoordinatorConfig {
        base_url: server.url(),
        webhook_secret: None,
    };
    let payload = message_to_webhook_payload(3, 42, 7, "/status");
    let result = forward_payload(&config, &payload).await.expect("forward");
    mock.assert_async().await;
    assert!(result.ok);
    assert_eq!(result.peer_id.as_deref(), Some("mock-peer"));
    assert_eq!(result.task_type.as_deref(), Some("telegram_command"));
}

#[tokio::test]
async fn fetch_telegram_seats_matches_api_ph_s514() {
    let ctx = ApiContext::default();
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let config = CoordinatorConfig {
        base_url: format!("http://{addr}"),
        webhook_secret: None,
    };
    let snapshot = fetch_telegram_seats(&config).await.expect("seats");
    let reply = format_seats_reply(&Ok(snapshot.clone()));
    assert!(reply.contains("Telegram seats"));
    assert!(reply.contains(&snapshot.seat_policy));
}

#[tokio::test]
async fn unbind_telegram_via_coordinator_ph_s515() {
    VirtualNodeTelegramBindingService::clear_all();
    VirtualNodeTelegramBindingService::bind("99001", None, "stop-peer");

    let ctx = ApiContext::default();
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let config = CoordinatorConfig {
        base_url: format!("http://{addr}"),
        webhook_secret: None,
    };
    let result = unbind_telegram(&config, 99001).await.expect("unbind");
    assert!(result.ok);
    let reply = format_unbind_reply(&Ok(result));
    assert!(reply.contains("unbound"));
    assert!(VirtualNodeTelegramBindingService::lookup("99001").is_none());
    VirtualNodeTelegramBindingService::clear_all();
}
