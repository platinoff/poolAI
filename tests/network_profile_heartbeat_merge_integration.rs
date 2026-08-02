//! PH-S731: PUT base profile + heartbeat merge persist.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_store::{
    load_peer_network_profile, reset_network_profile_store_for_test, ENV_NETWORK_PROFILE_DATA_DIR,
};
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

async fn discovery_app() -> Router {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18083));
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

async fn put_json(app: &Router, uri: &str, body: serde_json::Value) -> StatusCode {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    response.status()
}

#[tokio::test]
async fn network_profile_put_heartbeat_merge_persist_ph_s731() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("poolai-np-merge-{stamp}"));
    std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
    reset_network_profile_store_for_test();

    let app = discovery_app().await;
    let peer = "peer-merge-s731";

    assert_eq!(
        put_json(
            &app,
            &format!("/api/v1/grid/network-profiles/{peer}"),
            serde_json::json!({
                "network_profile": {
                    "region": "eu-west",
                    "latency_ms_p50": 30,
                    "bandwidth_mbps": 500
                }
            }),
        )
        .await,
        StatusCode::OK
    );

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "peer_id": "{peer}",
                "address": "192.168.5.10",
                "port": 9100,
                "metadata": {{ "role": "virtual_node" }}
            }}"#
        )))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(register).await.unwrap().status(),
        StatusCode::OK
    );

    let heartbeat = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/heartbeat-remote")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "peer_id": "{peer}",
                "metadata": {{
                    "network_profile": {{
                        "region": "eu-west",
                        "latency_ms_p50": 18,
                        "egress_policy": "vpn_proxy"
                    }}
                }}
            }}"#
        )))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(heartbeat).await.unwrap().status(),
        StatusCode::OK
    );

    let peer_req = Request::builder()
        .uri(format!("/api/v1/discovery/peers/{peer}"))
        .body(Body::empty())
        .unwrap();
    let peer_resp = app.clone().oneshot(peer_req).await.unwrap();
    let peer_body = to_bytes(peer_resp.into_body(), usize::MAX).await.unwrap();
    let peer_json: Value = serde_json::from_slice(&peer_body).unwrap();
    let stored = peer_json["peer"]["metadata"]["network_profile"]
        .as_str()
        .expect("merged network_profile metadata");
    let parsed: Value = serde_json::from_str(stored).unwrap();
    assert_eq!(parsed["region"], "eu-west");
    assert_eq!(parsed["latency_ms_p50"], 18);
    assert_eq!(parsed["bandwidth_mbps"], 500);
    assert_eq!(parsed["egress_policy"], "vpn_proxy");

    let disk = load_peer_network_profile(peer).expect("persisted merge");
    let disk_parsed: Value = serde_json::from_str(&disk).unwrap();
    assert_eq!(disk_parsed["latency_ms_p50"], 18);
    assert_eq!(disk_parsed["bandwidth_mbps"], 500);

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
    reset_network_profile_store_for_test();
}
