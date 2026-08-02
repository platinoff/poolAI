//! PH-S669: Galaxy horizon close band (PH-S660…S668).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_store::{
    load_peer_network_profile, reset_network_profile_store_for_test, ENV_NETWORK_PROFILE_DATA_DIR,
};
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::observability::{self, metrics_handler};
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

async fn discovery_app() -> Router {
    observability::init_prometheus();
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18081));
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
        .route("/metrics", get(metrics_handler))
        .with_state(ctx)
}

#[tokio::test]
async fn horizon_s660_band_network_profile_heartbeat_ph_s669() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("poolai-horizon-s660-{stamp}"));
    std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
    reset_network_profile_store_for_test();

    let app = discovery_app().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "horizon-s664-worker",
                "address": "192.168.4.40",
                "port": 9097,
                "metadata": { "role": "virtual_node" }
            }"#,
        ))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(register).await.unwrap().status(),
        StatusCode::OK
    );

    // PH-S664: heartbeat metadata network_profile → in-memory + disk persist stub.
    let heartbeat = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/heartbeat-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "horizon-s664-worker",
                "metadata": {
                    "network_profile": {
                        "region": "eu-central",
                        "latency_ms_p50": 35,
                        "egress_policy": "vpn_proxy"
                    }
                }
            }"#,
        ))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(heartbeat).await.unwrap().status(),
        StatusCode::OK
    );

    let peer_req = Request::builder()
        .uri("/api/v1/discovery/peers/horizon-s664-worker")
        .body(Body::empty())
        .unwrap();
    let peer_resp = app.clone().oneshot(peer_req).await.unwrap();
    let peer_body = to_bytes(peer_resp.into_body(), usize::MAX).await.unwrap();
    let peer: Value = serde_json::from_slice(&peer_body).unwrap();
    let stored = peer["peer"]["metadata"]["network_profile"]
        .as_str()
        .expect("network_profile on peer metadata");
    let parsed: Value = serde_json::from_str(stored).unwrap();
    assert_eq!(parsed["region"], "eu-central");
    assert_eq!(parsed["latency_ms_p50"], 35);
    assert_eq!(parsed["egress_policy"], "vpn_proxy");
    assert!(load_peer_network_profile("horizon-s664-worker").is_some());

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
    reset_network_profile_store_for_test();
}
