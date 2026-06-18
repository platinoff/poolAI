//! PH-S477: payout batch history read API.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_payout_batch_history_for_test,
    reset_settlement_metrics_for_test,
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
async fn get_payout_batch_history_returns_entries_ph_s477() {
    reset_settlement_metrics_for_test();
    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "job-h1".into(),
        cleared_at: "2026-06-18T00:00:00Z".into(),
    });
    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "job-h2".into(),
        cleared_at: "2026-06-18T01:00:00Z".into(),
    });

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch/history?limit=10").await;
    assert_eq!(status, StatusCode::OK);
    let entries = body
        .get("entries")
        .and_then(|v| v.as_array())
        .expect("entries array");
    assert_eq!(entries.len(), 2);
    assert_eq!(
        entries[0].get("job_id").and_then(|v| v.as_str()),
        Some("job-h1")
    );

    reset_settlement_metrics_for_test();
    reset_payout_batch_history_for_test();
}
