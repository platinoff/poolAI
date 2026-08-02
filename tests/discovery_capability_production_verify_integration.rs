//! PH-S561: production capability verify key rejects invalid signatures.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_capability_doc::ENV_CAPABILITY_VERIFY_KEY;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

async fn app_with_discovery() -> Router {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18082));
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        addr,
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
    (status, serde_json::from_slice(&bytes).unwrap_or(json!({})))
}

#[tokio::test]
async fn register_remote_invalid_capability_sig_forbidden_ph_s561() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::set_var(
        ENV_CAPABILITY_VERIFY_KEY,
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
    );

    let app = app_with_discovery().await;
    let peer_id = format!("ph-s561-cap-{}", uuid::Uuid::new_v4());
    let (status, _body) = post_register(
        &app,
        json!({
            "peer_id": peer_id,
            "address": "127.0.0.1",
            "port": 9101,
            "protocol_version": "1.2",
            "capability_document": {
                "peer_id": "edge-worker-1",
                "capabilities": ["inference:gpu"],
                "signature": "00".repeat(64)
            }
        }),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    std::env::remove_var(ENV_CAPABILITY_VERIFY_KEY);
}
