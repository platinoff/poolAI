//! PH-S730: network profile GET survives store reload (restart stub).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_store::{
    reload_network_profile_store_from_disk, reset_network_profile_store_for_test,
    ENV_NETWORK_PROFILE_DATA_DIR,
};
use poolai::network::api::create_api_routes;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
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

async fn get_json(app: &Router, uri: &str) -> (StatusCode, serde_json::Value) {
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
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({})),
    )
}

#[tokio::test]
async fn grid_network_profile_survives_store_reload_ph_s730() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("poolai-np-restart-{stamp}"));
    std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
    reset_network_profile_store_for_test();

    let app = grid_app();
    let peer = "peer-restart-s730";
    let profile = serde_json::json!({
        "network_profile": {
            "region": "ap-south",
            "latency_ms_p50": 88,
            "bandwidth_mbps": 250
        }
    });
    assert_eq!(
        put_json(
            &app,
            &format!("/api/v1/grid/network-profiles/{peer}"),
            profile,
        )
        .await,
        StatusCode::OK
    );

    reload_network_profile_store_from_disk().expect("reload from disk");

    let (status, body) = get_json(&app, &format!("/api/v1/grid/network-profiles/{peer}")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body.get("network_profile")
            .and_then(|p| p.get("region"))
            .and_then(|v| v.as_str()),
        Some("ap-south")
    );
    assert_eq!(
        body.get("network_profile")
            .and_then(|p| p.get("latency_ms_p50"))
            .and_then(|v| v.as_u64()),
        Some(88)
    );

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
    reset_network_profile_store_for_test();
}
