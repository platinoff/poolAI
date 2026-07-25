//! PH-S1470…S1473: Policies HTTP API contracts (band 83).
//! Marker: policy_api_contracts_integration

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::policy_api_contracts_depth::{
    policy_api_contracts_depth_stub, policy_api_criteria_total, PolicyApiContractsDepth,
};
use serde_json::{json, Value};
use tower::ServiceExt;

async fn enterprise_app() -> Router {
    let ctx = ApiContext::default();
    ctx.security_manager
        .initialize()
        .await
        .expect("security manager init");
    Router::new()
        .nest("/api/enterprise", create_enterprise_api_routes())
        .with_state(ctx)
}

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let req_body = if let Some(v) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&v).unwrap())
    } else {
        Body::empty()
    };
    let response = app
        .clone()
        .oneshot(builder.body(req_body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("JSON body")
    };
    (status, v)
}

#[tokio::test]
async fn policy_query_http_lifecycle_ph_s1470() {
    let app = enterprise_app().await;

    let (status, body) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/policies?limit=5",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "query: {body}");
    assert!(body.is_array(), "expected policies array: {body}");

    let (status_filter, filtered) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/policies?name=default&limit=2",
        None,
    )
    .await;
    assert_eq!(status_filter, StatusCode::OK, "filtered: {filtered}");
    assert!(filtered.is_array());
    if let Some(arr) = filtered.as_array() {
        for item in arr {
            let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
            assert!(name.contains("default"), "name={name}");
        }
    }

    assert_eq!(
        policy_api_contracts_depth_stub(Some(&json!({"query_http_lifecycle": true}))),
        PolicyApiContractsDepth::QueryHttpLifecycle
    );
    assert_eq!(policy_api_criteria_total(), 9);
}

#[tokio::test]
async fn policy_store_wire_http_ph_s1471() {
    std::env::remove_var("POOLAI_POLICY_STORE");
    std::env::remove_var("POOLAI_POLICY_DATA_DIR");

    let app = enterprise_app().await;
    let (status, wire) = request_json(&app, "GET", "/api/enterprise/policy/store", None).await;
    assert_eq!(status, StatusCode::OK, "store wire: {wire}");
    let obj = wire.as_object().expect("wire object");
    for key in ["mode", "durable_path", "configured"] {
        assert!(obj.contains_key(key), "wire missing `{key}`: {obj:?}");
    }
    assert_eq!(obj.get("mode").and_then(|m| m.as_str()), Some("memory"));
    assert_eq!(obj.get("configured").and_then(|c| c.as_bool()), Some(false));

    std::env::set_var("POOLAI_POLICY_STORE", "sqlite");
    std::env::set_var("POOLAI_POLICY_DATA_DIR", "data/dev/policy");
    let (status2, wire2) = request_json(&app, "GET", "/api/enterprise/policy/store", None).await;
    assert_eq!(status2, StatusCode::OK, "sqlite wire: {wire2}");
    assert_eq!(wire2.get("mode").and_then(|m| m.as_str()), Some("sqlite"));
    assert_eq!(
        wire2.get("configured").and_then(|c| c.as_bool()),
        Some(true)
    );
    let path = wire2
        .get("durable_path")
        .and_then(|p| p.as_str())
        .expect("durable_path");
    assert!(path.contains("policy"), "path={path}");

    std::env::remove_var("POOLAI_POLICY_STORE");
    std::env::remove_var("POOLAI_POLICY_DATA_DIR");

    assert_eq!(
        policy_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
        PolicyApiContractsDepth::StoreWireHttp
    );
}

#[tokio::test]
async fn policy_field_fixtures_http_ph_s1473() {
    let app = enterprise_app().await;

    let (ok_status, ok_body) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/policies/validate",
        Some(json!({
            "name": "fixture-ok",
            "description": "valid",
            "session_timeout": 3600,
            "require_mfa": false,
            "max_failed_attempts": 5
        })),
    )
    .await;
    assert_eq!(ok_status, StatusCode::OK, "valid: {ok_body}");
    assert_eq!(ok_body.get("valid").and_then(|v| v.as_bool()), Some(true));

    let (missing_name_status, missing_name) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/policies/validate",
        Some(json!({
            "name": "",
            "description": "x",
            "session_timeout": 3600
        })),
    )
    .await;
    assert_eq!(
        missing_name_status,
        StatusCode::BAD_REQUEST,
        "missing name: {missing_name}"
    );
    let name_code = missing_name
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| missing_name.get("code").and_then(|c| c.as_str()));
    assert!(
        name_code == Some("POLICY_MISSING_NAME")
            || missing_name.to_string().contains("POLICY_MISSING_NAME")
            || missing_name.to_string().contains("name must be non-empty"),
        "unexpected missing-name body: {missing_name}"
    );

    let (bad_timeout_status, bad_timeout) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/policies/validate",
        Some(json!({
            "name": "timeout",
            "description": "x",
            "session_timeout": 0
        })),
    )
    .await;
    assert_eq!(
        bad_timeout_status,
        StatusCode::BAD_REQUEST,
        "bad timeout: {bad_timeout}"
    );
    let timeout_code = bad_timeout
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| bad_timeout.get("code").and_then(|c| c.as_str()));
    assert!(
        timeout_code == Some("POLICY_INVALID_TIMEOUT")
            || bad_timeout.to_string().contains("POLICY_INVALID_TIMEOUT")
            || bad_timeout.to_string().contains("session_timeout"),
        "unexpected timeout body: {bad_timeout}"
    );

    assert_eq!(
        policy_api_contracts_depth_stub(Some(&json!({"policy_field_fixtures": true}))),
        PolicyApiContractsDepth::PolicyFieldFixtures
    );
}
