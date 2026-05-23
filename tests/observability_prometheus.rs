//! PH-S07 / FM-043: Prometheus `/metrics` scrape endpoint.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use poolai::core::state::AppState;
use poolai::observability::{
    apply_prometheus_http_layer, init_prometheus, metrics_handler, record_http_request,
};
use std::sync::Arc;
use tower::ServiceExt;

fn test_app() -> Router {
    let state = Arc::new(AppState::new());
    init_prometheus();
    apply_prometheus_http_layer(
        Router::new()
            .route("/metrics", get(metrics_handler))
            .route("/api/v1/health", get(|| async { "ok" }))
            .with_state(state),
    )
}

#[tokio::test]
async fn metrics_endpoint_returns_prometheus_text() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(content_type.contains("text/plain"));
    assert!(content_type.contains("version=0.0.4"));

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("poolai_build_info"));
    assert!(text.contains("poolai_uptime_seconds"));
    assert!(text.contains("poolai_workers_active"));
}

#[tokio::test]
async fn http_middleware_records_request_counters() {
    record_http_request("GET", 200, 0.002);
    record_http_request("POST", 500, 0.05);

    let app = test_app();
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let scrape = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(scrape.into_body(), usize::MAX)
        .await
        .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("poolai_http_requests_total"));
}
