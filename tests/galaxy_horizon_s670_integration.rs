//! PH-S679: Galaxy horizon close band (PH-S670…S678).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replay_metrics::{
    record_replay_pending_scheduled, replay_metrics_snapshot, reset_replay_pending_metrics_for_test,
};
use poolai::grid::galaxy_verification_metrics::{
    record_verification_match, reset_verification_metrics_for_test, verification_metrics_snapshot,
};
use poolai::grid::galaxy_verification_replay::{
    verification_replay_depth_stub, VerificationReplayDepth,
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

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let builder = Request::builder().method(method).uri(uri);
    let req = if let Some(json_body) = body {
        builder
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&json_body).unwrap()))
            .unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(json!({ "raw": String::from_utf8_lossy(&bytes) })),
    )
}

#[tokio::test]
async fn horizon_s670_band_verification_replay_metrics_ph_s679() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_verification_metrics_for_test();
    reset_replay_pending_metrics_for_test();

    let app = grid_app();

    // PH-S670: verification metrics HTTP snapshot.
    record_verification_match();
    let (verify_status, verify_body) =
        request_json(&app, "GET", "/api/v1/grid/verification-metrics", None).await;
    assert_eq!(verify_status, StatusCode::OK);
    assert_eq!(verify_body["ok"], true);
    assert!(verify_body["metrics"]["match_total"].as_u64().unwrap() >= 1);
    assert_eq!(
        verify_body["metrics"]["match_total"],
        verification_metrics_snapshot().match_total
    );

    // PH-S671: replay metrics HTTP snapshot.
    record_replay_pending_scheduled();
    let (replay_status, replay_body) =
        request_json(&app, "GET", "/api/v1/grid/replay-metrics", None).await;
    assert_eq!(replay_status, StatusCode::OK);
    assert_eq!(replay_body["ok"], true);
    assert!(replay_body["metrics"]["replay_pending"].as_u64().unwrap() >= 1);
    assert_eq!(
        replay_body["metrics"]["replay_pending"],
        replay_metrics_snapshot().replay_pending
    );

    // PH-S674: concept replay depth stub.
    assert_eq!(
        verification_replay_depth_stub(Some(&json!({"verification_verdict": "mismatch"}))),
        VerificationReplayDepth::FullReplay
    );

    reset_verification_metrics_for_test();
    reset_replay_pending_metrics_for_test();
}
