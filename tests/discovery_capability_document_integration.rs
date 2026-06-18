//! PH-S448: capability_document wire on register-remote.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

fn test_discovery() -> Arc<DiscoveryService> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18081));
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
async fn register_remote_capability_document_accepted_ph_s448() {
    let app = app_with_discovery().await;
    let peer_id = format!("ph-s448-cap-{}", uuid::Uuid::new_v4());
    let (status, body) = post_register(
        &app,
        json!({
            "peer_id": peer_id,
            "address": "127.0.0.1",
            "port": 9100,
            "protocol_version": "1.2",
            "capability_document": {
                "peer_id": "edge-worker-1",
                "capabilities": ["inference:gpu"]
            }
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(body.get("registered").and_then(|v| v.as_bool()), Some(true));
}

#[tokio::test]
async fn register_remote_invalid_capability_document_returns_400_ph_s448() {
    let app = app_with_discovery().await;
    let peer_id = format!("ph-s448-bad-{}", uuid::Uuid::new_v4());
    let (status, body) = post_register(
        &app,
        json!({
            "peer_id": peer_id,
            "address": "127.0.0.1",
            "port": 9101,
            "protocol_version": "1.2",
            "capability_document": {
                "peer_id": "  ",
                "capabilities": []
            }
        }),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "body={body}");
}
