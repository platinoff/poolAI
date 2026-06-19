//! PH-S533: Galaxy horizon wire integration band (PH-S524…S532).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_governance_metrics::{
    record_release_verify_fail, record_release_verify_success, reset_governance_metrics_for_test,
};
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
};
use poolai::grid::galaxy_worker_health::reset_worker_health_for_test;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
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
async fn horizon_s524_band_payout_settlement_and_governance_metrics_ph_s533() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_worker_health_for_test();
    reset_governance_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
    record_release_verify_success();
    record_release_verify_fail();

    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "horizon-s524".into(),
        cleared_at: "2026-06-18T21:00:00Z".into(),
        gross_lamports: Some(50_000),
        primary_dev_lamports: Some(50),
        secondary_admin_lamports: Some(500),
        gross_usd_micro: None,
        ..PayoutBatchLedgerEntry::minimal("", "")
    });

    let app = grid_app();

    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "payout-batch: {body}");
    assert_eq!(body["settlement_mode"].as_str(), Some("offline_batch"));
    assert_eq!(body["on_chain_pending"].as_bool(), Some(false));
    assert_eq!(body["entry"]["job_id"].as_str(), Some("horizon-s524"));

    let (status, metrics) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(metrics.contains("poolai_release_verify_total"));
    assert!(metrics.contains("poolai_release_verify_fail_total"));
    assert!(metrics.contains("poolai_update_notify_pending"));

    reset_governance_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
    reset_worker_health_for_test();
}
