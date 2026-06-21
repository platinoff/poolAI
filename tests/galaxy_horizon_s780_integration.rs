//! PH-S789: Galaxy horizon close band (PH-S780…S788).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_fee_split::{
    PRIMARY_DEV_FEE_BPS, SECONDARY_ADMIN_FEE_MAX_BPS, SECONDARY_ADMIN_FEE_MIN_BPS,
};
use poolai::grid::galaxy_fee_split_depth::{galaxy_fee_split_depth_stub, FeeSplitDepth};
use poolai::grid::galaxy_fee_split_metrics::{
    evaluate_result_fee_split, fee_split_metrics_snapshot, record_fee_split_applied,
    reset_fee_split_metrics_for_test,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_fee_split_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::json;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static FEE_SPLIT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn fee_split_lock() -> std::sync::MutexGuard<'static, ()> {
    FEE_SPLIT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy fee split integration lock")
}

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
async fn horizon_s780_band_fee_split_production_depth_ph_s789() {
    let _lock = fee_split_lock();
    reset_fee_split_metrics_for_test();

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"fee_split_production": true}))),
        StandSmokeMetricsParityDepth::FeeSplitProduction
    );

    evaluate_result_fee_split(Some(
        &json!({ "gross_lamports": 1_000_000, "secondary_admin_bps": 200 }),
    ));
    record_fee_split_applied();

    let snap = fee_split_metrics_snapshot();
    assert_eq!(snap.primary_dev_fee_bps, PRIMARY_DEV_FEE_BPS);
    assert_eq!(
        snap.secondary_admin_fee_min_bps,
        SECONDARY_ADMIN_FEE_MIN_BPS
    );
    assert_eq!(
        snap.secondary_admin_fee_max_bps,
        SECONDARY_ADMIN_FEE_MAX_BPS
    );
    assert!(snap.fee_split_applied_total >= 1);
    assert_eq!(
        galaxy_fee_split_depth_stub(Some(&snap)),
        FeeSplitDepth::FullDepth
    );

    let app = grid_app();
    let (status, fee_json) = get_text(&app, "/api/v1/grid/fee-split-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&fee_json).expect("json");
    assert_eq!(body["ok"], true);
    assert_eq!(body["metrics"]["primary_dev_fee_bps"], PRIMARY_DEV_FEE_BPS);

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_fee_split_metrics_parity(&prom, &body).expect("parity");
    assert!(prom.contains("galaxy_fee_split_applied_total"));

    assert_eq!(
        fee_split_metrics_snapshot().fee_split_applied_total,
        body["metrics"]["fee_split_applied_total"]
            .as_u64()
            .unwrap_or(0)
    );

    reset_fee_split_metrics_for_test();
}
