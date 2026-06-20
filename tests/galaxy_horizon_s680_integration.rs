//! PH-S689: Galaxy horizon close band (PH-S680…S688).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement::{settlement_gate_depth_stub, SettlementGateDepth};
use poolai::grid::galaxy_settlement_metrics::{
    record_settlement_cleared, reset_settlement_metrics_for_test, settlement_metrics_snapshot,
};
use poolai::grid::galaxy_trust_score::{
    evaluate_result_settlement_gate, reset_settlement_gate_metrics_for_test,
    trust_metrics_snapshot, TrustScoreGateConfig,
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
async fn horizon_s680_band_settlement_trust_metrics_ph_s689() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_settlement_metrics_for_test();
    reset_settlement_gate_metrics_for_test();

    let app = grid_app();

    // PH-S680: settlement metrics HTTP snapshot.
    record_settlement_cleared();
    let (settlement_status, settlement_body) =
        request_json(&app, "GET", "/api/v1/grid/settlement-metrics", None).await;
    assert_eq!(settlement_status, StatusCode::OK);
    assert_eq!(settlement_body["ok"], true);
    assert!(
        settlement_body["metrics"]["cleared_total"]
            .as_u64()
            .unwrap()
            >= 1
    );
    assert_eq!(
        settlement_body["metrics"]["cleared_total"],
        settlement_metrics_snapshot().cleared_total
    );

    // PH-S681: trust metrics HTTP snapshot.
    let cfg = TrustScoreGateConfig::default_stub();
    evaluate_result_settlement_gate(Some("tg-peer-1"), Some(55), &cfg);
    let (trust_status, trust_body) =
        request_json(&app, "GET", "/api/v1/grid/trust-metrics", None).await;
    assert_eq!(trust_status, StatusCode::OK);
    assert_eq!(trust_body["ok"], true);
    assert!(
        trust_body["metrics"]["payout_eligible_total"]
            .as_u64()
            .unwrap()
            >= 1
    );
    assert_eq!(
        trust_body["metrics"]["payout_eligible_total"],
        trust_metrics_snapshot().payout_eligible_total
    );

    // PH-S684: concept settlement depth stub.
    assert_eq!(
        settlement_gate_depth_stub(Some(&json!({
            "settlement_gate_verdict": "payout_held",
            "verification_sample": "not_selected",
        }))),
        SettlementGateDepth::PendingVerification
    );

    reset_settlement_metrics_for_test();
    reset_settlement_gate_metrics_for_test();
}
