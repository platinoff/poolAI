//! FM-016: HTTP registration of virtual-node workers with discovery.

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
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18080));
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

#[tokio::test]
async fn register_remote_peer_lists_in_discovery_peers() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "tg-worker-test",
                "address": "192.168.1.50",
                "port": 9090,
                "metadata": { "channel": "telegram", "role": "virtual_node" }
            }"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["peer_id"], "tg-worker-test");
    assert_eq!(v["registered"], true);

    let peers_req = Request::builder()
        .uri("/api/v1/discovery/peers")
        .body(Body::empty())
        .unwrap();
    let peers_resp = app.oneshot(peers_req).await.unwrap();
    assert_eq!(peers_resp.status(), StatusCode::OK);
    let peers_body = to_bytes(peers_resp.into_body(), usize::MAX).await.unwrap();
    let peers: Value = serde_json::from_slice(&peers_body).unwrap();
    let arr = peers["peers"].as_array().expect("peers array");
    assert!(
        arr.iter().any(|p| p["peer_id"] == "tg-worker-test"),
        "expected tg-worker-test in peers: {arr:?}"
    );
}

#[tokio::test]
async fn heartbeat_remote_and_virtual_nodes_list() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "tg-hb-node",
                "address": "127.0.0.1",
                "port": 19090,
                "metadata": { "channel": "telegram", "role": "virtual_node" }
            }"#,
        ))
        .unwrap();
    let reg = app.clone().oneshot(register).await.unwrap();
    assert_eq!(reg.status(), StatusCode::OK);

    let hb = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/heartbeat-remote")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"peer_id":"tg-hb-node"}"#))
        .unwrap();
    let hb_resp = app.clone().oneshot(hb).await.unwrap();
    assert_eq!(hb_resp.status(), StatusCode::OK);

    let vn_req = Request::builder()
        .uri("/api/v1/discovery/virtual-nodes")
        .body(Body::empty())
        .unwrap();
    let vn_resp = app.oneshot(vn_req).await.unwrap();
    assert_eq!(vn_resp.status(), StatusCode::OK);
    let vn_body = to_bytes(vn_resp.into_body(), usize::MAX).await.unwrap();
    let vn: Value = serde_json::from_slice(&vn_body).unwrap();
    let nodes = vn["nodes"].as_array().expect("nodes");
    assert!(
        nodes
            .iter()
            .any(|n| n["peer"]["peer_id"] == "tg-hb-node" && n["stale"] == false),
        "virtual node list: {vn:?}"
    );
}
