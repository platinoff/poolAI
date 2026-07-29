//! PH-S1590…S1592: Monitoring live stand-smoke contracts (band 95).
//! Marker: monitoring_stand_smoke_integration
//!
//! CI canon uses in-process axum (no live stand). Live HTTP runners live in
//! `poolai-http-stand-smoke --monitoring-stand-smoke`.

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::monitoring_stand_smoke_depth::{
    monitoring_stand_smoke_criteria_total, monitoring_stand_smoke_depth_stub,
    MonitoringStandSmokeDepth, MONITORING_STAND_SMOKE_CASES, MONITORING_STAND_SMOKE_CRITERIA,
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

#[test]
fn monitoring_stand_smoke_depth_registry_ph_s1589() {
    assert_eq!(MONITORING_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(monitoring_stand_smoke_criteria_total(), 10);
    assert!(MONITORING_STAND_SMOKE_CASES.contains(&"live_store"));
    assert!(MONITORING_STAND_SMOKE_CASES.contains(&"live_alerts_query"));
    assert_eq!(
        monitoring_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
        MonitoringStandSmokeDepth::LiveStore
    );
}

#[tokio::test]
async fn monitoring_stand_smoke_store_wire_ph_s1590() {
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
}

#[tokio::test]
async fn monitoring_stand_smoke_alerts_query_ph_s1591() {
    let app = enterprise_app().await;

    let (status, body) = request_json(
        &app,
        "GET",
        "/api/enterprise/monitoring/alerts?limit=5",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "query: {body}");
    assert!(body.is_array(), "expected alerts array: {body}");

    let (status_filter, filtered) = request_json(
        &app,
        "GET",
        "/api/enterprise/monitoring/alerts?severity=WARNING&limit=2",
        None,
    )
    .await;
    assert_eq!(status_filter, StatusCode::OK, "filtered: {filtered}");
    let arr = filtered.as_array().expect("filtered array");
    for item in arr {
        let sev = item.get("severity").and_then(|s| s.as_str()).unwrap_or("");
        assert!(
            sev.eq_ignore_ascii_case("WARNING") || sev.eq_ignore_ascii_case("warning"),
            "severity={sev}"
        );
    }

    assert_eq!(
        monitoring_stand_smoke_depth_stub(Some(&json!({"live_alerts_query": true}))),
        MonitoringStandSmokeDepth::LiveAlertsQuery
    );
}

#[tokio::test]
async fn monitoring_stand_smoke_field_fixtures_ph_s1592() {
    let app = enterprise_app().await;

    let (ok_status, ok_body) = request_json(
        &app,
        "POST",
        "/api/enterprise/monitoring/alert-rules/validate",
        Some(json!({
            "name": "stand-smoke-ok",
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
    assert!(
        missing_name.to_string().contains("MONITORING_MISSING_NAME")
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
    assert!(
        bad_op.to_string().contains("MONITORING_INVALID_OPERATOR")
            || bad_op.to_string().contains("operator"),
        "unexpected bad-operator body: {bad_op}"
    );

    assert_eq!(
        monitoring_stand_smoke_depth_stub(Some(&json!({"live_monitoring_field_fixtures": true}))),
        MonitoringStandSmokeDepth::LiveMonitoringFieldFixtures
    );
}
