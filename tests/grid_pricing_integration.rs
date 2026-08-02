//! PH-S144: Grid pricing API wire — migrated from `e2e/tests/grid_pricing.spec.ts`.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::api::grid::reset_pricing_oracle_for_tests;
use serde_json::Value;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PRICING_INTEGRATION_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn pricing_lock() -> std::sync::MutexGuard<'static, ()> {
    PRICING_INTEGRATION_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("grid pricing integration lock")
}

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

fn pricing_uri(model_profile: &str, unit_key: &str) -> String {
    format!(
        "/api/v1/grid/pricing?task_profile=inference:text&model_profile={model_profile}&unit_key={unit_key}"
    )
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
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
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

#[tokio::test]
async fn grid_pricing_l2_fallback_snapshot() {
    let _lock = pricing_lock();
    reset_pricing_oracle_for_tests(true, Some(470_000));
    let app = grid_app();
    let model = "ph-s144-pricing-fallback";

    let (status, body) = get_json(&app, &pricing_uri(model, "inference_blended_token")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["source"], "oracle");
    assert_eq!(body["freshness"], "fresh");
    assert_eq!(body["snapshot"]["task_profile"], "inference:text");
    assert_eq!(body["snapshot"]["model_profile"], model);
    assert_eq!(body["snapshot"]["unit_key"], "inference_blended_token");
    assert_eq!(body["snapshot"]["poolai_quote_usd_micro"], 470_000);
    assert_eq!(body["snapshot"]["provider_id_at_min"], "fallback_l2_config");
}

#[tokio::test]
async fn grid_pricing_second_get_serves_cache() {
    let _lock = pricing_lock();
    reset_pricing_oracle_for_tests(true, Some(470_000));
    let app = grid_app();
    let model = "ph-s144-pricing-cache";
    let uri = pricing_uri(model, "inference_blended_token");

    let (first_status, _) = get_json(&app, &uri).await;
    assert_eq!(first_status, StatusCode::OK);

    let (second_status, body) = get_json(&app, &uri).await;
    assert_eq!(second_status, StatusCode::OK);
    assert_eq!(body["source"], "cache");
    assert_eq!(body["snapshot"]["poolai_quote_usd_micro"], 470_000);
}

#[tokio::test]
async fn grid_pricing_rejects_invalid_unit_key() {
    let _lock = pricing_lock();
    reset_pricing_oracle_for_tests(false, None);
    let app = grid_app();

    let (status, _) = get_json(
        &app,
        &pricing_uri("ph-s144-invalid-unit", "not_a_valid_unit"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn grid_pricing_forced_fallback_stable_quote_ph_s901() {
    let _lock = pricing_lock();
    reset_pricing_oracle_for_tests(true, Some(470_000));
    let app = grid_app();
    let model = "ph-s901-pricing-forced-fallback";

    let (status, body) = get_json(&app, &pricing_uri(model, "inference_blended_token")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["source"], "oracle");
    assert_eq!(body["freshness"], "fresh");
    assert_eq!(body["snapshot"]["poolai_quote_usd_micro"], 470_000);
    assert_eq!(body["snapshot"]["provider_id_at_min"], "fallback_l2_config");
}
