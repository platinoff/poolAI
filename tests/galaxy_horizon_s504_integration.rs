//! PH-S500: Galaxy horizon wire integration band (PH-S504…S511 metrics + read APIs).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_verification_metrics::{
    reset_verification_checker_tasks_for_test, reset_verification_metrics_for_test,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::telegram_seat_service::reset_telegram_seats_for_test;
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn get_text(app: &Router, uri: &str) -> (StatusCode, String) {
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
        String::from_utf8(bytes.to_vec()).unwrap_or_default(),
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
async fn metrics_export_horizon_s504_band_ph_s513() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_network_profile_store_for_test();
    reset_telegram_seats_for_test();

    persist_peer_network_profile("peer-s504", r#"{"region":"eu","latency_ms_p50":9}"#)
        .expect("persist");

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("galaxy_verification_checker_pending_total"));
    assert!(body.contains("galaxy_verification_match_total"));

    let (status, json) = get_json(&app, "/api/v1/grid/telegram-seats").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.get("ok").and_then(|v| v.as_bool()), Some(true));
    assert!(json.get("seat_policy").and_then(|v| v.as_str()).is_some());

    let (status, json) = get_json(&app, "/api/v1/grid/network-profiles/peer-s504").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        json.get("network_profile")
            .and_then(|p| p.get("region"))
            .and_then(|v| v.as_str()),
        Some("eu")
    );

    let (status, json) = get_json(&app, "/api/v1/grid/verification-checker/tasks").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json.get("tasks").and_then(|v| v.as_array()).is_some());

    reset_network_profile_store_for_test();
    reset_telegram_seats_for_test();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
}
