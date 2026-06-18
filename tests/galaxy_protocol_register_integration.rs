//! PH-S65: Galaxy protocol_version negotiation on register-remote.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use serde_json::Value;
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

fn register_body(protocol_version: &str) -> String {
    format!(
        r#"{{
            "peer_id": "proto-{protocol_version}",
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "{protocol_version}",
            "build_id": "test-build",
            "metadata": {{ "role": "virtual_node" }}
        }}"#
    )
}

#[tokio::test]
async fn register_remote_protocol_1_2_accepted() {
    let app = app_with_discovery().await;
    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(register_body("1.2")))
        .unwrap();

    let response = app.oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["compat_status"], "accepted");
    assert_eq!(v["registered"], true);
    assert_eq!(v["worker_protocol_version"], "1.2");
}

async fn register_remote_protocol_1_0_unsupported() {
    let app = app_with_discovery().await;
    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(register_body("1.0")))
        .unwrap();

    let response = app.oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["compat_status"], "unsupported");
    assert_eq!(v["registered"], false);
}

#[tokio::test]
async fn register_remote_build_id_allowlist_ph_s520() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("POOLAI_ALLOWED_BUILD_IDS", "ci-build,prod");

    let app = app_with_discovery().await;
    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "build-reject",
                "address": "10.0.0.9",
                "port": 9091,
                "protocol_version": "1.2",
                "build_id": "unknown-build",
                "metadata": { "role": "virtual_node" }
            }"#,
        ))
        .unwrap();

    let response = app.oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["error"], "build_id_rejected");

    std::env::remove_var("POOLAI_ALLOWED_BUILD_IDS");
}
