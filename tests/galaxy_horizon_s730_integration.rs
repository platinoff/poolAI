//! PH-S739: Galaxy horizon close band (PH-S730…S738).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_depth::{network_profile_depth_stub, NetworkProfileDepth};
use poolai::grid::galaxy_network_profile_store::{
    reload_network_profile_store_from_disk, reset_network_profile_store_for_test,
    ENV_NETWORK_PROFILE_DATA_DIR,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn put_json(app: &Router, uri: &str, body: Value) -> StatusCode {
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

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap_or(json!({})))
}

#[tokio::test]
async fn horizon_s730_band_network_profile_persist_ph_s739() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("poolai-horizon-s730-{stamp}"));
    std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
    reset_network_profile_store_for_test();

    let app = grid_app();
    let peer = "horizon-s730-peer";

    assert_eq!(
        put_json(
            &app,
            &format!("/api/v1/grid/network-profiles/{peer}"),
            json!({
                "network_profile": {
                    "region": "eu-central",
                    "latency_ms_p50": 22,
                    "bandwidth_mbps": 800,
                    "egress_policy": "direct",
                    "last_measured_at": "2026-06-20T10:00:00Z"
                }
            }),
        )
        .await,
        StatusCode::OK
    );

    reload_network_profile_store_from_disk().expect("reload");

    let (list_status, list) = get_json(&app, "/api/v1/grid/network-profiles").await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list
        .get("peer_ids")
        .and_then(|v| v.as_array())
        .map(|ids| ids.iter().any(|id| id.as_str() == Some(peer)))
        .unwrap_or(false));

    let (get_status, profile) =
        get_json(&app, &format!("/api/v1/grid/network-profiles/{peer}")).await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(
        profile
            .get("network_profile")
            .and_then(|p| p.get("region"))
            .and_then(|v| v.as_str()),
        Some("eu-central")
    );

    assert_eq!(
        network_profile_depth_stub(profile.get("network_profile")),
        NetworkProfileDepth::FullTelemetry
    );
    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"network_profile_persist": true}))),
        StandSmokeMetricsParityDepth::NetworkProfile
    );

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
    reset_network_profile_store_for_test();
}
