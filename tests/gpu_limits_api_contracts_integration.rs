//! PH-S1871: GPULimits API contracts (band 123).
//! Marker: gpu_limits_api_contracts_integration
//!
//! Verifies `GET /api/v1/gpu-limits` returns the durable store wire shape.

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::AppState;
use poolai::network::api::create_api_routes;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

async fn gpu_limits_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(Arc::new(AppState::default()))
}

#[tokio::test]
async fn gpu_limits_route_returns_200_ph_s1871() {
    let app = gpu_limits_app().await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/gpu-limits")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&bytes).expect("JSON body");
    for key in [
        "mode",
        "available",
        "max_gpus",
        "admission_enabled",
        "gpu_memory_mb_cap",
        "admission_active",
    ] {
        assert!(v.get(key).is_some(), "wire missing `{key}`: {v:?}");
    }
}

#[tokio::test]
async fn gpu_limits_route_requires_no_auth_for_read_ph_s1871() {
    let app = gpu_limits_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/gpu-limits")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn gpu_limits_wire_matches_durable_store_ph_s1871() {
    let app = gpu_limits_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/gpu-limits")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&bytes).expect("JSON body");
    let store = poolai_ui_core::gpu_limits_store::gpu_limits_store_wire_json();
    assert_eq!(v.get("max_gpus"), store.get("max_gpus"));
    assert_eq!(v.get("admission_enabled"), store.get("admission_enabled"));
    assert_eq!(v.get("admission_active"), store.get("admission_active"));
}
