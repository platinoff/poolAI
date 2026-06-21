//! PH-S879: Galaxy horizon close band (PH-S870…S878) — Solana on-chain cleared depth.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_onchain::{
    emit_settlement_job_rewarded, last_onchain_rpc_signature_len, onchain_submit_total,
    reset_onchain_submit_metrics_for_test,
};
use poolai::grid::galaxy_settlement_onchain_depth::{
    settlement_onchain_depth_stub, settlement_onchain_depth_wire_label, SettlementOnchainDepth,
};
use poolai::grid::solana_depth::{solana_depth_stub, solana_depth_wire_label, SolanaDepth};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use serde_json::json;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

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
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!(null));
    (status, body)
}

#[tokio::test]
async fn horizon_s870_band_solana_onchain_cleared_ph_s879() {
    let _guard = env_lock();
    reset_onchain_submit_metrics_for_test();

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"on_chain_settlement": true}))),
        StandSmokeMetricsParityDepth::OnChainSettlement
    );
    assert_eq!(
        solana_depth_stub(true, true, true, false),
        SolanaDepth::MockRpc
    );
    assert_eq!(solana_depth_wire_label(SolanaDepth::MockRpc), "mock_rpc");
    assert_eq!(
        settlement_onchain_depth_stub(true, true, 1, 12),
        SettlementOnchainDepth::FullDepth
    );
    assert_eq!(
        settlement_onchain_depth_wire_label(SettlementOnchainDepth::NdjsonSink),
        "ndjson_sink"
    );

    let tmp = tempfile::TempDir::new().expect("tempdir");
    std::env::set_var(
        "POOLAI_ONCHAIN_EVENTS_DIR",
        tmp.path().to_string_lossy().as_ref(),
    );
    std::env::set_var("POOLAI_SETTLEMENT_ON_CHAIN", "1");

    let entry = PayoutBatchLedgerEntry {
        job_id: "job-horizon-s870".into(),
        cleared_at: "2026-06-21T12:00:00Z".into(),
        gross_lamports: Some(3_000),
        payout_pubkey: Some("pk-horizon".into()),
        ..PayoutBatchLedgerEntry::minimal("", "")
    };
    emit_settlement_job_rewarded(&entry, "peer-horizon");
    assert_eq!(onchain_submit_total(), 1);
    assert!(last_onchain_rpc_signature_len() > 0);

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["settlement_mode"], "on_chain");
    assert_eq!(body["on_chain_pending"], true);
    assert!(body["onchain_depth"].is_string());
    assert!(body["solana_depth"].is_string());
    assert_eq!(body["onchain_events_configured"], true);

    let (metrics_status, metrics) = get_json(&app, "/api/v1/grid/payout-batch-metrics").await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert_eq!(metrics["ok"], true);
    assert!(metrics["metrics"]["onchain_submit_total"].is_number());

    std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    std::env::remove_var("POOLAI_SETTLEMENT_ON_CHAIN");
    reset_onchain_submit_metrics_for_test();
}
