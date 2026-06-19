//! PH-S523: Galaxy horizon wire integration band (PH-S514…S522).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
};
use poolai::grid::galaxy_worker_health::{
    galaxy_worker_unhealthy_total, reset_worker_health_for_test,
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
async fn horizon_s514_band_read_apis_and_metrics_ph_s523() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_telegram_seats_for_test();
    reset_worker_health_for_test();
    reset_last_payout_batch_ledger_entry_for_test();

    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "horizon-s514".into(),
        cleared_at: "2026-06-18T20:00:00Z".into(),
        gross_lamports: Some(100_000),
        primary_dev_lamports: Some(100),
        secondary_admin_lamports: Some(1_000),
        gross_usd_micro: None,
        ..PayoutBatchLedgerEntry::minimal("", "")
    });

    let app = grid_app();

    let (status, body) = get_json(&app, "/api/v1/grid/telegram-seats").await;
    assert_eq!(status, StatusCode::OK, "telegram-seats: {body}");
    assert!(body.get("seat_policy").is_some());

    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "payout-batch: {body}");
    assert_eq!(body["entry"]["job_id"].as_str(), Some("horizon-s514"));

    let (status, metrics) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(metrics.contains("galaxy_worker_unhealthy_total"));
    assert_eq!(galaxy_worker_unhealthy_total(), 0);

    reset_last_payout_batch_ledger_entry_for_test();
    reset_worker_health_for_test();
    reset_telegram_seats_for_test();
}
