//! PH-S467: payout batch read API.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
    reset_settlement_metrics_for_test,
};
use poolai::network::api::create_api_routes;
use serde_json::Value;
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
async fn get_payout_batch_returns_last_entry_ph_s467() {
    reset_settlement_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "job-payout-1".into(),
        cleared_at: "2026-06-18T12:00:00Z".into(),
    });

    let app = app().await;
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    let entry = body.get("entry").expect("entry");
    assert_eq!(
        entry.get("job_id").and_then(|v| v.as_str()),
        Some("job-payout-1")
    );
    reset_last_payout_batch_ledger_entry_for_test();
}

#[tokio::test]
async fn get_payout_batch_empty_when_no_entry_ph_s467() {
    reset_last_payout_batch_ledger_entry_for_test();
    let app = app().await;
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    let entry = body.get("entry");
    assert!(entry.is_none() || entry.unwrap().is_null());
    reset_last_payout_batch_ledger_entry_for_test();
}
