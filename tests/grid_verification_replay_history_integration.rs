//! PH-S478: verification replay history read API.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replay_metrics::{
    emit_verification_replay_record, reset_replay_pending_metrics_for_test,
    reset_verification_replay_history_for_test,
};
use poolai::network::api::create_api_routes;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
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
    let v = serde_json::from_slice(&bytes).expect("json");
    (status, v)
}

#[tokio::test]
async fn get_verification_replay_history_returns_records_ph_s478() {
    reset_replay_pending_metrics_for_test();
    emit_verification_replay_record("job-r1", None);
    emit_verification_replay_record("job-r2", None);

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/verification-replay/history?limit=10").await;
    assert_eq!(status, StatusCode::OK);
    let records = body
        .get("records")
        .and_then(|v| v.as_array())
        .expect("records array");
    assert_eq!(records.len(), 2);

    reset_replay_pending_metrics_for_test();
    reset_verification_replay_history_for_test();
}
