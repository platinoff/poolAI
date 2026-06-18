//! PH-S475: telegram seat cap on register-remote.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::services::telegram_seat_service::{
    reset_telegram_seats_for_test, ENV_TELEGRAM_SEAT_LIMIT,
};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

fn test_discovery() -> Arc<DiscoveryService> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18082));
    Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        addr,
        None,
    ))
}

async fn app_with_discovery() -> Router {
    let discovery = test_discovery();
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

async fn post_register(app: &Router, body: Value) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/discovery/register-remote")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

#[tokio::test]
async fn register_remote_telegram_edge_seat_exhausted_ph_s475() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_telegram_seats_for_test();
    std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "1");

    let app = app_with_discovery().await;
    let peer_a = format!("ph-s475-a-{}", uuid::Uuid::new_v4());
    let peer_b = format!("ph-s475-b-{}", uuid::Uuid::new_v4());
    let base = json!({
        "address": "127.0.0.1",
        "port": 9100,
        "protocol_version": "1.2",
        "metadata": { "origin": "telegram_edge" }
    });

    let mut body_a = base.clone();
    body_a["peer_id"] = json!(peer_a);
    let (status_a, _) = post_register(&app, body_a).await;
    assert_eq!(status_a, StatusCode::OK);

    let mut body_b = base.clone();
    body_b["peer_id"] = json!(peer_b);
    let (status_b, body_b) = post_register(&app, body_b).await;
    assert_eq!(status_b, StatusCode::CONFLICT, "body={body_b}");
    assert_eq!(
        body_b.get("error").and_then(|v| v.as_str()),
        Some("seat_exhausted")
    );

    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
    reset_telegram_seats_for_test();
}
