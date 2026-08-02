//! PH-S909: Galaxy horizon close band (PH-S900…S908) — pricing oracle live fetch hardening.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_depth::{
    current_pricing_depth, pricing_depth_stub, pricing_depth_wire_label, PricingDepth,
};
use poolai::grid::galaxy_pricing_metrics::pricing_metrics_snapshot;
use poolai::grid::galaxy_pricing_oracle::{
    bump_forced_fallback_for_test, bump_fresh_served_for_test, provider_http_timeout_ms_from_env,
    reset_forced_fallback_total_for_test, reset_fresh_served_total_for_test,
};
use poolai::grid::galaxy_pricing_provider_metrics::{
    record_provider_catalog_lookup, reset_provider_catalog_metrics_for_test,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_pricing_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::pricing::render_grid_pricing_freshness_strip_html;
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
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
async fn horizon_s900_band_pricing_live_fetch_production_ph_s909() {
    let _guard = env_lock();
    reset_fresh_served_total_for_test();
    reset_forced_fallback_total_for_test();
    reset_provider_catalog_metrics_for_test();

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"pricing_production": true}))),
        StandSmokeMetricsParityDepth::PricingProduction
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"grid_pricing_freshness_strip": true}))),
        AdminWasmSlimDepth::GridPricingFreshnessStrip
    );

    let strip = render_grid_pricing_freshness_strip_html(
        r#"{"freshness":"fresh","source":"cache","l1_cache":{"cache_age_secs":5,"cache_ttl_secs":300,"cache_fresh_until_secs":1718280300,"cache_stale_until_secs":1718283600}}"#,
        r#"{}"#,
    );
    assert!(strip.contains("grid-pricing-freshness-strip"));

    bump_forced_fallback_for_test();
    bump_fresh_served_for_test();
    record_provider_catalog_lookup(1);
    let snap = pricing_metrics_snapshot();
    assert_eq!(
        pricing_depth_stub(Some(&snap), provider_http_timeout_ms_from_env()),
        PricingDepth::FullProduction
    );
    assert_eq!(
        pricing_depth_wire_label(PricingDepth::FullProduction),
        "full_production"
    );
    assert_eq!(current_pricing_depth(), PricingDepth::FullProduction);

    let app = grid_app();
    let (status, pricing_json) = get_text(&app, "/api/v1/grid/pricing-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let body: Value = serde_json::from_str(&pricing_json).expect("json");
    assert_eq!(body["ok"], true);
    assert_eq!(body["pricing_depth"], "full_production");
    assert!(body["provider_http_timeout_ms"].as_u64().unwrap_or(0) >= 100);

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_pricing_metrics_parity(&prom, &body).expect("parity");

    reset_fresh_served_total_for_test();
    reset_forced_fallback_total_for_test();
    reset_provider_catalog_metrics_for_test();
}
