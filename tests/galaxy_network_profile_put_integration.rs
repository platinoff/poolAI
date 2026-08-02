//! PH-S506: network profile upsert API.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_store::{
    load_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::network::api::create_api_routes;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn put_json(
    app: &Router,
    uri: &str,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
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
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({})),
    )
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
async fn grid_network_profile_put_roundtrip_ph_s506() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_network_profile_store_for_test();

    let app = grid_app();
    let peer = "peer-np-s506";
    let profile = serde_json::json!({
        "network_profile": {
            "region": "us-east",
            "latency_ms_p50": 25
        }
    });
    let (status, body) = put_json(
        &app,
        &format!("/api/v1/grid/network-profiles/{peer}"),
        profile,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(
        body.get("network_profile")
            .and_then(|p| p.get("region"))
            .and_then(|v| v.as_str()),
        Some("us-east")
    );

    let stored = load_peer_network_profile(peer).expect("persisted");
    assert!(stored.contains("us-east"));

    let (status, body) = get_json(&app, &format!("/api/v1/grid/network-profiles/{peer}")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body.get("network_profile")
            .and_then(|p| p.get("latency_ms_p50"))
            .and_then(|v| v.as_u64()),
        Some(25)
    );
    reset_network_profile_store_for_test();
}
