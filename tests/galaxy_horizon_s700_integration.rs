//! PH-S709: Galaxy horizon close band (PH-S700…S708).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_metrics::pricing_metrics_snapshot;
use poolai::grid::galaxy_pricing_oracle::{
    bump_fresh_served_for_test, reset_fresh_served_total_for_test,
};
use poolai::grid::galaxy_replication::REPLICATION_STRICT;
use poolai::grid::galaxy_replication_metrics::{
    evaluate_job_replication_strict, replication_metrics_snapshot,
    reset_replication_strict_metrics_for_test,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::grid_replication_pricing::{
    admin_wasm_slim_depth_stub, render_grid_replication_pricing_panel_html, AdminWasmSlimDepth,
};
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
    let mut builder = Request::builder().method(method).uri(uri);
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
async fn horizon_s700_band_admin_wasm_slim_ph_s709() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_replication_strict_metrics_for_test();
    reset_fresh_served_total_for_test();

    let app = grid_app();

    // PH-S700: ui-core panel renderer produces metrics strip HTML.
    evaluate_job_replication_strict(REPLICATION_STRICT);
    let (repl_status, repl_body) =
        request_json(&app, "GET", "/api/v1/grid/replication-metrics", None).await;
    assert_eq!(repl_status, StatusCode::OK);
    bump_fresh_served_for_test();
    let (price_status, price_body) =
        request_json(&app, "GET", "/api/v1/grid/pricing-metrics", None).await;
    assert_eq!(price_status, StatusCode::OK);

    let panel_html = render_grid_replication_pricing_panel_html(
        &serde_json::to_string(&repl_body).unwrap(),
        &serde_json::to_string(&price_body).unwrap(),
        replication_metrics_snapshot().strict_total,
        r#"{}"#,
    );
    assert!(panel_html.contains("admin-metrics-strip"));
    assert!(panel_html.contains("Fresh served"));

    // PH-S704: admin wasm slim depth stub.
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
        AdminWasmSlimDepth::PanelRenderer
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"charts_glue": true}))),
        AdminWasmSlimDepth::ChartsGlue
    );

    assert_eq!(
        repl_body["metrics"]["strict_total"],
        replication_metrics_snapshot().strict_total
    );
    assert_eq!(
        price_body["metrics"]["fresh_served_total"],
        pricing_metrics_snapshot().fresh_served_total
    );

    reset_replication_strict_metrics_for_test();
    reset_fresh_served_total_for_test();
}
