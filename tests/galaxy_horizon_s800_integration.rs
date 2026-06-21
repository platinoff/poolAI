//! PH-S809: Galaxy horizon close band (PH-S800…S808).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::stand_smoke_metrics_parity::validate_settlement_trust_metrics_parity;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::ml::render_ml_pipeline_metrics_panel_html;
use poolai_ui_core::payout_batch::render_payout_batch_panel_html;
use poolai_ui_core::stand_smoke_metrics::render_grid_settlement_trust_metrics_strip_html;
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
async fn horizon_s800_band_admin_wasm_slim_monitoring_payout_ph_s809() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"ml_pipeline_panel": true}))),
        AdminWasmSlimDepth::MlPipelinePanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"payout_batch_panel": true}))),
        AdminWasmSlimDepth::PayoutBatchPanel
    );

    let ml_html = render_ml_pipeline_metrics_panel_html(
        "[]",
        "ML Pipeline Step Metrics",
        "No ML pipeline step metrics yet",
        "Run the demo pipeline",
        r#"["Pipeline","Step","Kind","Status","Metrics"]"#,
        "Avg: ",
    );
    assert!(ml_html.contains("ml-pipeline-metrics-panel"));

    let payout_html = render_payout_batch_panel_html(
        r#"{"entry":{"job_id":"job-1"}}"#,
        r#"{"entries":[]}"#,
        r#"{}"#,
    );
    assert!(payout_html.contains("admin-card"));

    let strip_html = render_grid_settlement_trust_metrics_strip_html(
        r#"{"metrics":{"cleared_total":1,"payout_batch_total":0}}"#,
        r#"{"metrics":{"payout_eligible_total":2,"last_trust_score":50}}"#,
        50,
    );
    assert!(strip_html.contains("admin-metrics-strip"));

    let app = grid_app();

    let (status, settlement_json) = get_text(&app, "/api/v1/grid/settlement-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let settlement_body: serde_json::Value =
        serde_json::from_str(&settlement_json).expect("settlement json");

    let (status, trust_json) = get_text(&app, "/api/v1/grid/trust-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let trust_body: serde_json::Value = serde_json::from_str(&trust_json).expect("trust json");

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_settlement_trust_metrics_parity(&prom, &settlement_body, &trust_body)
        .expect("settlement/trust parity");
}
