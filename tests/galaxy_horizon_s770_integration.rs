//! PH-S779: Galaxy horizon close band (PH-S770…S778).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::{
    resolve_settlement_status, PayoutBatchLedgerEntry, SettlementStatus,
};
use poolai::grid::galaxy_settlement_metrics::{
    evaluate_result_settlement_cleared, record_payout_batch_ledger_entry,
    reset_settlement_metrics_for_test, settlement_metrics_snapshot,
};
use poolai::grid::galaxy_settlement_mode::{
    current_settlement_mode, offline_batch_payout_enabled, settlement_mode_gate_label,
};
use poolai::grid::galaxy_settlement_payout_batch_queue::{
    enqueue_offline_payout_batch_on_cleared, payout_batch_queue_depth,
    reset_payout_batch_queue_for_test,
};
use poolai::grid::galaxy_settlement_payout_depth::{
    settlement_payout_depth_stub, SettlementPayoutDepth,
};
use poolai::grid::galaxy_settlement_payout_metrics::payout_batch_metrics_snapshot;
use poolai::grid::galaxy_trust_score::SettlementGateVerdict;
use poolai::grid::galaxy_verify_sampling::VerifySamplingVerdict;
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_payout_batch_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::json;
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
    let text = String::from_utf8(bytes.to_vec()).unwrap_or_default();
    (status, text)
}

#[tokio::test]
async fn horizon_s770_band_payout_batch_settlement_depth_ph_s779() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_settlement_metrics_for_test();
    reset_payout_batch_queue_for_test();
    std::env::remove_var(poolai::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN);

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"payout_batch_settlement": true}))),
        StandSmokeMetricsParityDepth::PayoutBatchSettlement
    );
    assert!(offline_batch_payout_enabled());
    assert_eq!(settlement_mode_gate_label(), "offline_batch_queue");
    assert_eq!(current_settlement_mode(), "offline_batch");

    let status = resolve_settlement_status(
        SettlementGateVerdict::PayoutEligible,
        VerifySamplingVerdict::NotSelected,
    );
    assert_eq!(status, SettlementStatus::Cleared);
    evaluate_result_settlement_cleared(status);
    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry::minimal(
        "job-s770",
        "2026-06-21T00:00:00Z",
    ));
    enqueue_offline_payout_batch_on_cleared("job-s770");
    assert!(payout_batch_queue_depth() >= 1);

    let settlement_snap = settlement_metrics_snapshot();
    assert_eq!(
        settlement_payout_depth_stub(Some(&settlement_snap)),
        SettlementPayoutDepth::FullDepth
    );

    let app = grid_app();
    let (status, payout_json) = get_text(&app, "/api/v1/grid/payout-batch-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&payout_json).expect("json");
    assert_eq!(body["ok"], true);
    assert_eq!(body["metrics"]["settlement_mode"], "offline_batch");

    let (hist_status, hist_json) =
        get_text(&app, "/api/v1/grid/payout-batch/history?limit=5").await;
    assert_eq!(hist_status, StatusCode::OK);
    let hist: serde_json::Value = serde_json::from_str(&hist_json).expect("hist json");
    assert_eq!(hist["ok"], true);

    let (latest_status, latest_json) = get_text(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(latest_status, StatusCode::OK);
    let latest: serde_json::Value = serde_json::from_str(&latest_json).expect("latest json");
    assert_eq!(latest["settlement_mode"], "offline_batch");

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_payout_batch_metrics_parity(&prom, &body).expect("parity");
    assert!(prom.contains("galaxy_settlement_payout_batch_queue_depth"));

    let metrics_snap = payout_batch_metrics_snapshot();
    assert_eq!(
        metrics_snap.payout_batch_queue_depth,
        payout_batch_queue_depth()
    );

    reset_settlement_metrics_for_test();
    reset_payout_batch_queue_for_test();
}
