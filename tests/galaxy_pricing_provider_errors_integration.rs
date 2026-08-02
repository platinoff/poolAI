//! PH-S173: Galaxy pricing provider fetch errors — Prometheus scrape snapshot.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_oracle::{
    fetch_live_provider_quotes, GalaxyPriceUnitKey, GalaxyPricingProviderCatalog,
    GalaxyPricingProviderEntry,
};
use poolai::grid::galaxy_pricing_provider_metrics::{
    reset_provider_catalog_metrics_for_test, METRIC_PROVIDER_ERRORS_TOTAL,
};
use poolai::observability::{self, metrics_handler};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PROVIDER_ERRORS_METRICS_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn provider_errors_metrics_lock() -> std::sync::MutexGuard<'static, ()> {
    PROVIDER_ERRORS_METRICS_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy pricing provider errors integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn provider_fetch_errors_visible_on_metrics_scrape() {
    let _lock = provider_errors_metrics_lock();
    reset_provider_catalog_metrics_for_test();

    let catalog = GalaxyPricingProviderCatalog {
        providers: vec![GalaxyPricingProviderEntry {
            provider_id: "bad_live".into(),
            region: "us".into(),
            model_profile: Some("fail-model".into()),
            task_profiles: vec!["inference:text".into()],
            units: HashMap::from([(
                GalaxyPriceUnitKey::InferenceBlendedToken.as_str().into(),
                500_000,
            )]),
            endpoint: Some("http://127.0.0.1:1/unreachable".into()),
            enabled: true,
        }],
    };
    let quotes = fetch_live_provider_quotes(
        &catalog,
        "inference:text",
        "fail-model",
        GalaxyPriceUnitKey::InferenceBlendedToken,
        500,
    )
    .await;
    assert!(quotes.is_empty());

    let app = metrics_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert!(body.contains(METRIC_PROVIDER_ERRORS_TOTAL));
    assert!(body.contains(&format!("{METRIC_PROVIDER_ERRORS_TOTAL} 1")));

    reset_provider_catalog_metrics_for_test();
}
