//! PH-S900: Galaxy pricing live provider HTTP timeout hardening — metrics + fetch path.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_oracle::{
    clamp_provider_http_timeout_ms, fetch_live_provider_quotes, provider_http_timeout_ms_from_env,
    GalaxyPriceUnitKey, GalaxyPricingProviderCatalog, GalaxyPricingProviderEntry,
    MAX_PROVIDER_HTTP_TIMEOUT_MS, MIN_PROVIDER_HTTP_TIMEOUT_MS,
};
use poolai::grid::galaxy_pricing_provider_metrics::{
    provider_timeouts_total, reset_provider_catalog_metrics_for_test,
    METRIC_PROVIDER_TIMEOUTS_TOTAL,
};
use poolai::observability::{self, metrics_handler};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static TIMEOUT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn timeout_lock() -> std::sync::MutexGuard<'static, ()> {
    TIMEOUT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy pricing provider timeout integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[test]
fn provider_timeout_clamp_and_env_ph_s900() {
    let _lock = timeout_lock();
    assert_eq!(
        clamp_provider_http_timeout_ms(10),
        MIN_PROVIDER_HTTP_TIMEOUT_MS
    );
    assert_eq!(
        clamp_provider_http_timeout_ms(99_999),
        MAX_PROVIDER_HTTP_TIMEOUT_MS
    );
    std::env::set_var("POOLAI_GALAXY_PRICING_TIMEOUT_MS", "500");
    assert_eq!(provider_http_timeout_ms_from_env(), 500);
    std::env::remove_var("POOLAI_GALAXY_PRICING_TIMEOUT_MS");
}

#[tokio::test]
async fn provider_fetch_timeout_visible_on_metrics_scrape_ph_s900() {
    let _lock = timeout_lock();
    reset_provider_catalog_metrics_for_test();

    use poolai::grid::galaxy_pricing_provider_metrics::record_provider_fetch_timeout;
    record_provider_fetch_timeout();
    assert_eq!(provider_timeouts_total(), 1);

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
    assert!(body.contains(METRIC_PROVIDER_TIMEOUTS_TOTAL));
    assert!(body.contains(&format!("{METRIC_PROVIDER_TIMEOUTS_TOTAL} 1")));

    let catalog = GalaxyPricingProviderCatalog {
        providers: vec![GalaxyPricingProviderEntry {
            provider_id: "slow_live".into(),
            region: "us".into(),
            model_profile: Some("timeout-model".into()),
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
        "timeout-model",
        GalaxyPriceUnitKey::InferenceBlendedToken,
        provider_http_timeout_ms_from_env(),
    )
    .await;
    assert!(quotes.is_empty());

    reset_provider_catalog_metrics_for_test();
}
