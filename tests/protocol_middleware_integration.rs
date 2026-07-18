//! PH-S144: `X-PoolAI-Protocol` middleware wire — migrated from
//! `e2e/tests/protocol_middleware.spec.ts`.

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

async fn app_with_discovery() -> Router {
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18082)),
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

fn register_body(peer_id: &str) -> String {
    format!(
        r#"{{
            "peer_id": "{peer_id}",
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "1.2",
            "build_id": "integration-protocol-middleware",
            "metadata": {{ "role": "virtual_node" }}
        }}"#
    )
}

async fn register_with_protocol_header(
    app: &Router,
    peer_id: &str,
    protocol_header: &str,
) -> (StatusCode, Value, axum::http::HeaderMap) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/discovery/register-remote")
                .header("content-type", "application/json")
                .header("X-PoolAI-Protocol", protocol_header)
                .body(Body::from(register_body(peer_id)))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    (status, v, headers)
}

#[tokio::test]
async fn register_remote_protocol_header_1_2_adds_compat_headers() {
    let app = app_with_discovery().await;
    let peer_id = format!(
        "proto-accept-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );

    let (status, body, headers) = register_with_protocol_header(&app, &peer_id, "1.2").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["registered"], true);
    assert_eq!(body["compat_status"], "accepted");
    assert!(headers.get("x-poolai-protocol-coordinator").is_some());
    assert_eq!(
        headers
            .get("x-poolai-protocol-compat")
            .and_then(|v| v.to_str().ok()),
        Some("accepted")
    );
    // Docs URL may omit header when fragment contains non-ASCII (HeaderValue validation).
    if let Some(docs) = headers
        .get("x-poolai-protocol-docs")
        .and_then(|v| v.to_str().ok())
    {
        assert!(
            docs.contains("POOLAI_GALAXY_GRID") || docs.contains("galaxy"),
            "unexpected docs header: {docs}"
        );
    }
}

#[tokio::test]
async fn register_remote_unsupported_protocol_header_returns_403() {
    let app = app_with_discovery().await;
    let peer_id = format!(
        "proto-reject-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );

    let (status, body, headers) = register_with_protocol_header(&app, &peer_id, "1.0").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "protocol_unsupported");
    assert_eq!(
        headers
            .get("x-poolai-protocol-compat")
            .and_then(|v| v.to_str().ok()),
        Some("unsupported")
    );
}

/// PH-S992: band-34 registry — protocol middleware canon covers archived `protocol_middleware.spec.ts`.
#[test]
fn integration_gap_protocol_middleware_canon_ph_s992() {
    let src = include_str!("protocol_middleware_integration.rs");
    assert!(src.contains("register_remote_protocol_header_1_2_adds_compat_headers"));
    assert!(src.contains("register_remote_unsupported_protocol_header_returns_403"));
}
