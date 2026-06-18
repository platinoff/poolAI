//! PH-S460: verification replay read API.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replay_metrics::{
    emit_verification_replay_record, reset_last_verification_replay_record_for_test,
    reset_replay_pending_metrics_for_test,
};
use poolai::network::api::create_api_routes;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
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
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

#[tokio::test]
async fn get_verification_replay_returns_last_record_ph_s460() {
    reset_replay_pending_metrics_for_test();
    emit_verification_replay_record(
        "job-replay-1",
        Some(&json!({
            "verification_id": "v-ph-s460",
            "verification_verdict": "mismatch"
        })),
    );

    let app = app().await;
    let (status, body) = get_json(&app, "/api/v1/grid/verification-replay").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    let record = body.get("record").expect("record");
    assert_eq!(
        record.get("primary_job_id").and_then(|v| v.as_str()),
        Some("job-replay-1")
    );
    assert_eq!(
        record.get("verification_id").and_then(|v| v.as_str()),
        Some("v-ph-s460")
    );
    reset_replay_pending_metrics_for_test();
    reset_last_verification_replay_record_for_test();
}

#[tokio::test]
async fn get_verification_replay_empty_when_no_record_ph_s460() {
    reset_replay_pending_metrics_for_test();
    reset_last_verification_replay_record_for_test();
    let app = app().await;
    let (status, body) = get_json(&app, "/api/v1/grid/verification-replay").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    assert!(body.get("record").is_none() || body.get("record").unwrap().is_null());
    reset_replay_pending_metrics_for_test();
    reset_last_verification_replay_record_for_test();
}
