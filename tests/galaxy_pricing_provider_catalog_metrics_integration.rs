//! PH-S172: Galaxy pricing provider catalog metrics — allow-list hits → Prometheus scrape.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_pricing_oracle::bundled_pricing_provider_catalog;
use poolai::grid::galaxy_pricing_provider_metrics::{
    reset_provider_catalog_metrics_for_test, METRIC_PROVIDER_CATALOG_HITS_TOTAL,
    METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL,
};
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PROVIDER_CATALOG_METRICS_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn provider_catalog_metrics_lock() -> std::sync::MutexGuard<'static, ()> {
    PROVIDER_CATALOG_METRICS_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy pricing provider catalog metrics integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn provider_catalog_hits_visible_on_metrics_scrape() {
    let _lock = provider_catalog_metrics_lock();
    reset_provider_catalog_metrics_for_test();
    let catalog = bundled_pricing_provider_catalog();
    let hits = catalog.matching_entries("inference:text", "gpt-4o-mini");
    assert!(!hits.is_empty());

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
    assert!(body.contains(METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL));
    assert!(body.contains(&format!("{METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL} 1")));
    assert!(body.contains(METRIC_PROVIDER_CATALOG_HITS_TOTAL));
    assert!(body.contains(&format!(
        "{METRIC_PROVIDER_CATALOG_HITS_TOTAL} {}",
        hits.len()
    )));

    reset_provider_catalog_metrics_for_test();
}
