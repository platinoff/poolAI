//! PH-S140: `metadata.network_profile` parsing on register-remote.

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

#[tokio::test]
async fn register_remote_parses_metadata_network_profile_object() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "np-object-worker",
                "address": "192.168.2.10",
                "port": 9092,
                "metadata": {
                    "role": "virtual_node",
                    "network_profile": {
                        "region": "eu-west",
                        "latency_ms_p50": 24,
                        "bandwidth_mbps": 500,
                        "egress_policy": "vpn_proxy"
                    }
                }
            }"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let peer_req = Request::builder()
        .uri("/api/v1/discovery/peers/np-object-worker")
        .body(Body::empty())
        .unwrap();
    let peer_resp = app.oneshot(peer_req).await.unwrap();
    assert_eq!(peer_resp.status(), StatusCode::OK);
    let peer_body = to_bytes(peer_resp.into_body(), usize::MAX).await.unwrap();
    let peer: Value = serde_json::from_slice(&peer_body).unwrap();
    let stored = peer["peer"]["metadata"]["network_profile"]
        .as_str()
        .expect("canonical network_profile JSON in peer metadata");
    let parsed: Value = serde_json::from_str(stored).unwrap();
    assert_eq!(parsed["region"], "eu-west");
    assert_eq!(parsed["latency_ms_p50"], 24);
    assert_eq!(parsed["bandwidth_mbps"], 500);
    assert_eq!(parsed["egress_policy"], "vpn_proxy");
}

#[tokio::test]
async fn register_remote_parses_metadata_network_profile_json_string() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "np-string-worker",
                "address": "10.0.0.2",
                "port": 9093,
                "metadata": {
                    "role": "virtual_node",
                    "network_profile": "{\"region\":\"us-east\",\"latency_ms_p50\":80}"
                }
            }"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let peer_req = Request::builder()
        .uri("/api/v1/discovery/peers/np-string-worker")
        .body(Body::empty())
        .unwrap();
    let peer_resp = app.oneshot(peer_req).await.unwrap();
    let peer_body = to_bytes(peer_resp.into_body(), usize::MAX).await.unwrap();
    let peer: Value = serde_json::from_slice(&peer_body).unwrap();
    let stored = peer["peer"]["metadata"]["network_profile"]
        .as_str()
        .expect("stored profile");
    let parsed: Value = serde_json::from_str(stored).unwrap();
    assert_eq!(parsed["region"], "us-east");
    assert_eq!(parsed["latency_ms_p50"], 80);
}

#[tokio::test]
async fn heartbeat_remote_retains_network_profile_ph_s440() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "np-hb-worker",
                "address": "192.168.3.20",
                "port": 9095,
                "metadata": {
                    "role": "virtual_node",
                    "network_profile": {
                        "region": "ap-south",
                        "latency_ms_p50": 110,
                        "egress_policy": "lan_only"
                    }
                }
            }"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let heartbeat = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/heartbeat-remote")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"peer_id":"np-hb-worker"}"#))
        .unwrap();

    let hb_resp = app.clone().oneshot(heartbeat).await.unwrap();
    assert_eq!(hb_resp.status(), StatusCode::OK);

    let peer_req = Request::builder()
        .uri("/api/v1/discovery/peers/np-hb-worker")
        .body(Body::empty())
        .unwrap();
    let peer_resp = app.oneshot(peer_req).await.unwrap();
    assert_eq!(peer_resp.status(), StatusCode::OK);
    let peer_body = to_bytes(peer_resp.into_body(), usize::MAX).await.unwrap();
    let peer: Value = serde_json::from_slice(&peer_body).unwrap();
    let stored = peer["peer"]["metadata"]["network_profile"]
        .as_str()
        .expect("network_profile retained after heartbeat");
    let parsed: Value = serde_json::from_str(stored).unwrap();
    assert_eq!(parsed["region"], "ap-south");
    assert_eq!(parsed["latency_ms_p50"], 110);
    assert_eq!(parsed["egress_policy"], "lan_only");
    assert!(
        parsed
            .get("last_measured_at")
            .and_then(|v| v.as_str())
            .is_some(),
        "heartbeat must refresh last_measured_at (PH-S519): {parsed}"
    );
}

#[tokio::test]
async fn register_remote_invalid_network_profile_returns_validation_error() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "np-bad-worker",
                "address": "127.0.0.1",
                "port": 9094,
                "metadata": {
                    "role": "virtual_node",
                    "network_profile": {
                        "region": "EU",
                        "latency_ms_p50": 10
                    }
                }
            }"#,
        ))
        .unwrap();

    let response = app.oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["error"]["code"], "VALIDATION_ERROR");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("network_profile"),
        "expected network_profile validation message: {v:?}"
    );
}
