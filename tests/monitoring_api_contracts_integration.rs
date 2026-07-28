//! PH-S1570…S1573: Monitoring HTTP API contracts (band 93).
//! Marker: monitoring_api_contracts_integration

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::monitoring_api_contracts_depth::{
    monitoring_api_contracts_depth_stub, monitoring_api_criteria_total, MonitoringApiContractsDepth,
};
use serde_json::{json, Value};
use tower::ServiceExt;

async fn enterprise_app() -> Router {
    let ctx = ApiContext::default();
    let _ = ctx.enterprise_monitoring_manager.initialize().await;
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
async fn monitoring_query_http_lifecycle_ph_s1570() {
    let app = enterprise_app().await;

    let (status, body) = request_json(
        &app,
        "GET",
        "/api/enterprise/monitoring/alerts?limit=5",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "alerts query: {body}");
    assert!(body.is_array(), "expected alerts array: {body}");

    let (status_rules, rules) =
        request_json(&app, "GET", "/api/enterprise/monitoring/alert-rules", None).await;
    assert_eq!(status_rules, StatusCode::OK, "rules: {rules}");
    assert!(rules.is_array(), "expected rules array: {rules}");

    let (status_metrics, metrics) = request_json(
        &app,
        "GET",
        "/api/enterprise/monitoring/metrics?limit=5",
        None,
    )
    .await;
    assert_eq!(status_metrics, StatusCode::OK, "metrics: {metrics}");
    assert!(metrics.is_array(), "expected metrics array: {metrics}");

    assert_eq!(
        monitoring_api_contracts_depth_stub(Some(&json!({"query_http_lifecycle": true}))),
        MonitoringApiContractsDepth::QueryHttpLifecycle
    );
    assert_eq!(monitoring_api_criteria_total(), 9);
}

#[tokio::test]
async fn monitoring_store_wire_http_ph_s1571() {
    std::env::remove_var("POOLAI_MONITORING_STORE");
    std::env::remove_var("POOLAI_MONITORING_DATA_DIR");

    let app = enterprise_app().await;
    let (status, wire) = request_json(&app, "GET", "/api/enterprise/monitoring/store", None).await;
    assert_eq!(status, StatusCode::OK, "store wire: {wire}");
    let obj = wire.as_object().expect("wire object");
    for key in ["mode", "durable_path", "configured"] {
        assert!(obj.contains_key(key), "wire missing `{key}`: {obj:?}");
    }
    assert_eq!(obj.get("mode").and_then(|m| m.as_str()), Some("memory"));
    assert_eq!(obj.get("configured").and_then(|c| c.as_bool()), Some(false));

    std::env::set_var("POOLAI_MONITORING_STORE", "sqlite");
    std::env::set_var("POOLAI_MONITORING_DATA_DIR", "data/dev/monitoring");
    let (status2, wire2) =
        request_json(&app, "GET", "/api/enterprise/monitoring/store", None).await;
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
    assert!(path.contains("monitoring"), "path={path}");

    std::env::remove_var("POOLAI_MONITORING_STORE");
    std::env::remove_var("POOLAI_MONITORING_DATA_DIR");

    assert_eq!(
        monitoring_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
        MonitoringApiContractsDepth::StoreWireHttp
    );
}

#[tokio::test]
async fn monitoring_field_fixtures_http_ph_s1573() {
    let app = enterprise_app().await;

    let (ok_status, ok_body) = request_json(
        &app,
        "POST",
        "/api/enterprise/monitoring/alert-rules/validate",
        Some(json!({
            "name": "fixture-ok",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": ">",
            "severity": "WARNING",
            "enabled": true
        })),
    )
    .await;
    assert_eq!(ok_status, StatusCode::OK, "valid: {ok_body}");
    assert_eq!(ok_body.get("valid").and_then(|v| v.as_bool()), Some(true));

    let (missing_name_status, missing_name) = request_json(
        &app,
        "POST",
        "/api/enterprise/monitoring/alert-rules/validate",
        Some(json!({
            "name": "",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": ">"
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
        name_code == Some("MONITORING_MISSING_NAME")
            || missing_name.to_string().contains("MONITORING_MISSING_NAME")
            || missing_name.to_string().contains("name"),
        "unexpected missing-name body: {missing_name}"
    );

    let (bad_op_status, bad_op) = request_json(
        &app,
        "POST",
        "/api/enterprise/monitoring/alert-rules/validate",
        Some(json!({
            "name": "bad-op",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": "!="
        })),
    )
    .await;
    assert_eq!(
        bad_op_status,
        StatusCode::BAD_REQUEST,
        "bad operator: {bad_op}"
    );
    let op_code = bad_op
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| bad_op.get("code").and_then(|c| c.as_str()));
    assert!(
        op_code == Some("MONITORING_INVALID_OPERATOR")
            || bad_op.to_string().contains("MONITORING_INVALID_OPERATOR")
            || bad_op.to_string().contains("operator"),
        "unexpected bad-operator body: {bad_op}"
    );

    assert_eq!(
        monitoring_api_contracts_depth_stub(Some(&json!({"monitoring_field_fixtures": true}))),
        MonitoringApiContractsDepth::MonitoringFieldFixtures
    );
}
