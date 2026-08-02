//! PH-S168: Galaxy pricing cache age gauge on GET /metrics.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_oracle::{
    observe_l1_cache_age_secs, reset_pricing_cache_age_for_test, METRIC_CACHE_AGE_SECONDS,
};
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PRICING_AGE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn pricing_age_lock() -> std::sync::MutexGuard<'static, ()> {
    PRICING_AGE_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy pricing cache age integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn pricing_cache_age_gauge_on_metrics_scrape() {
    let _lock = pricing_age_lock();
    reset_pricing_cache_age_for_test();
    observe_l1_cache_age_secs(512);

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
    assert!(body.contains(METRIC_CACHE_AGE_SECONDS));
    assert!(body.contains(&format!("{METRIC_CACHE_AGE_SECONDS} 512")));

    reset_pricing_cache_age_for_test();
}
