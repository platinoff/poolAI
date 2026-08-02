//! PH-S181: Galaxy pricing market min gauge on GET /metrics.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_oracle::{
    observe_last_market_min_usd_micro, reset_last_market_min_usd_micro_for_test,
    METRIC_MARKET_MIN_USD_MICRO,
};
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PRICING_MARKET_MIN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn pricing_market_min_lock() -> std::sync::MutexGuard<'static, ()> {
    PRICING_MARKET_MIN_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy pricing market min usd_micro integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn pricing_market_min_usd_micro_gauge_on_metrics_scrape() {
    let _lock = pricing_market_min_lock();
    reset_last_market_min_usd_micro_for_test();
    observe_last_market_min_usd_micro(500_000);

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
    assert!(body.contains(METRIC_MARKET_MIN_USD_MICRO));
    assert!(body.contains(&format!("{METRIC_MARKET_MIN_USD_MICRO} 500000")));

    reset_last_market_min_usd_micro_for_test();
}
