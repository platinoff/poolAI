//! PH-S842: OpenAPI band contract tests — top grid/admin/virtual-nodes routes.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_security_advisory::reset_security_advisory_for_test;
use poolai::network::api::create_api_routes;
use serde_json::Value;
use tower::ServiceExt;

fn api_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[tokio::test]
async fn grid_network_profiles_list_openapi_shape_ph_s842() {
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/grid/network-profiles").await;
    assert_eq!(status, StatusCode::OK);
    let o = v.as_object().expect("network-profiles list object");
    for key in ["ok", "peer_ids", "count"] {
        assert!(o.contains_key(key), "missing `{key}`: {o:?}");
    }
    assert!(o["peer_ids"].is_array());
}

#[tokio::test]
async fn admin_security_advisories_list_openapi_shape_ph_s842() {
    reset_security_advisory_for_test();
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/admin/security-advisories").await;
    assert_eq!(status, StatusCode::OK);
    let rows = v.as_array().expect("security advisories array");
    assert_eq!(rows.len(), 3);
    for row in rows {
        let o = row.as_object().expect("advisory object");
        for key in ["id", "severity", "summary", "acknowledged"] {
            assert!(o.contains_key(key), "advisory missing `{key}`: {o:?}");
        }
    }
    reset_security_advisory_for_test();
}

#[tokio::test]
async fn virtual_nodes_wallet_rebind_override_requires_admin_ph_s842() {
    let app = api_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/virtual-nodes/telegram/wallet/rebind-override")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"telegram_user_id":"1","chat_id":"2","payout_pubkey":"abc"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("error json");
    assert!(v.get("error").is_some(), "expected structured error: {v:?}");
}

#[tokio::test]
async fn grid_seed_inventory_openapi_shape_ph_s1060() {
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/grid/seed-inventory").await;
    assert_eq!(status, StatusCode::OK);
    let o = v.as_object().expect("seed-inventory object");
    for key in [
        "ok",
        "entries",
        "generated_at",
        "memory_store_depth",
        "memory_layer_depth",
    ] {
        assert!(o.contains_key(key), "seed-inventory missing `{key}`: {o:?}");
    }
    assert!(o["entries"].is_array());
}

#[tokio::test]
async fn grid_verification_replay_openapi_shape_ph_s1060() {
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/grid/verification-replay").await;
    assert_eq!(status, StatusCode::OK);
    let o = v.as_object().expect("verification-replay object");
    assert_eq!(o.get("ok"), Some(&Value::Bool(true)));
    match o.get("record") {
        None | Some(Value::Null) => {}
        Some(record) => assert!(record.is_object(), "record must be object when set: {v:?}"),
    }
}

#[tokio::test]
async fn grid_verification_checker_tasks_openapi_shape_ph_s1060() {
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/grid/verification-checker/tasks").await;
    assert_eq!(status, StatusCode::OK);
    let o = v.as_object().expect("verification-checker tasks object");
    assert_eq!(o.get("ok"), Some(&Value::Bool(true)));
    assert!(o["tasks"].is_array(), "tasks must be array: {v:?}");
}

#[tokio::test]
async fn grid_metrics_band_openapi_shape_ph_s842() {
    let app = api_app();
    for path in [
        "/api/v1/grid/verification-metrics",
        "/api/v1/grid/replay-metrics",
        "/api/v1/grid/settlement-metrics",
        "/api/v1/grid/trust-metrics",
        "/api/v1/grid/replication-metrics",
        "/api/v1/grid/pricing-metrics",
        "/api/v1/grid/prefetch-metrics",
        "/api/v1/grid/locality-metrics",
        "/api/v1/grid/fee-split-metrics",
        "/api/v1/grid/governance-metrics",
        "/api/v1/grid/payout-batch-metrics",
    ] {
        let (status, v) = get_json(&app, path).await;
        assert_eq!(status, StatusCode::OK, "path {path}");
        let o = v.as_object().expect("metrics response object");
        assert_eq!(o.get("ok"), Some(&Value::Bool(true)), "path {path}");
        assert!(
            o.get("metrics").and_then(|m| m.as_object()).is_some(),
            "path {path} missing metrics object: {v:?}"
        );
    }
}
