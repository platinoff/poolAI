//! PH-S1370…S1373: Audit HTTP API contracts (band 73).
//! Marker: audit_api_contracts_integration

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::audit_api_contracts_depth::{
    audit_api_contracts_depth_stub, audit_api_criteria_total, AuditApiContractsDepth,
};
use serde_json::{json, Value};
use tower::ServiceExt;

async fn enterprise_app() -> Router {
    let ctx = ApiContext::default();
    let _ = ctx.audit_logger.initialize().await;
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
async fn audit_query_http_lifecycle_ph_s1370() {
    let app = enterprise_app().await;

    let (status, body) =
        request_json(&app, "GET", "/api/enterprise/audit/events?limit=5", None).await;
    assert_eq!(status, StatusCode::OK, "query: {body}");
    assert!(body.is_array(), "expected events array: {body}");

    let (status_filter, filtered) = request_json(
        &app,
        "GET",
        "/api/enterprise/audit/events?action=create_instance&limit=2",
        None,
    )
    .await;
    assert_eq!(status_filter, StatusCode::OK, "filtered: {filtered}");
    assert!(filtered.is_array());

    assert_eq!(
        audit_api_contracts_depth_stub(Some(&json!({"query_http_lifecycle": true}))),
        AuditApiContractsDepth::QueryHttpLifecycle
    );
    assert_eq!(audit_api_criteria_total(), 9);
}

#[tokio::test]
async fn audit_store_wire_http_ph_s1371() {
    std::env::remove_var("POOLAI_AUDIT_STORE");
    std::env::remove_var("POOLAI_AUDIT_DATA_DIR");

    let app = enterprise_app().await;
    let (status, wire) = request_json(&app, "GET", "/api/enterprise/audit/store", None).await;
    assert_eq!(status, StatusCode::OK, "store wire: {wire}");
    let obj = wire.as_object().expect("wire object");
    for key in ["mode", "durable_path", "configured"] {
        assert!(obj.contains_key(key), "wire missing `{key}`: {obj:?}");
    }
    assert_eq!(obj.get("mode").and_then(|m| m.as_str()), Some("file"));
    assert_eq!(obj.get("configured").and_then(|c| c.as_bool()), Some(false));

    std::env::set_var("POOLAI_AUDIT_STORE", "sqlite");
    std::env::set_var("POOLAI_AUDIT_DATA_DIR", "data/dev/audit");
    let (status2, wire2) = request_json(&app, "GET", "/api/enterprise/audit/store", None).await;
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
    assert!(path.contains("audit"), "path={path}");

    std::env::remove_var("POOLAI_AUDIT_STORE");
    std::env::remove_var("POOLAI_AUDIT_DATA_DIR");

    assert_eq!(
        audit_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
        AuditApiContractsDepth::StoreWireHttp
    );
}

#[tokio::test]
async fn audit_event_field_fixtures_http_ph_s1373() {
    let app = enterprise_app().await;

    let (ok_status, ok_body) = request_json(
        &app,
        "POST",
        "/api/enterprise/audit/events/validate",
        Some(json!({
            "action": "create_instance",
            "resource_type": "vm_instance",
            "result": "success",
            "level": "INFO"
        })),
    )
    .await;
    assert_eq!(ok_status, StatusCode::OK, "valid: {ok_body}");
    assert_eq!(ok_body.get("valid").and_then(|v| v.as_bool()), Some(true));

    let (missing_action_status, missing_action) = request_json(
        &app,
        "POST",
        "/api/enterprise/audit/events/validate",
        Some(json!({
            "action": "",
            "resource_type": "vm_instance",
            "result": "success"
        })),
    )
    .await;
    assert_eq!(
        missing_action_status,
        StatusCode::BAD_REQUEST,
        "missing action: {missing_action}"
    );
    let action_code = missing_action
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| missing_action.get("code").and_then(|c| c.as_str()));
    assert!(
        action_code == Some("AUDIT_MISSING_ACTION")
            || missing_action.to_string().contains("AUDIT_MISSING_ACTION")
            || missing_action.to_string().contains("missing action"),
        "unexpected missing-action body: {missing_action}"
    );

    let (missing_resource_status, missing_resource) = request_json(
        &app,
        "POST",
        "/api/enterprise/audit/events/validate",
        Some(json!({
            "action": "create_instance",
            "resource_type": "  ",
            "result": "success"
        })),
    )
    .await;
    assert_eq!(
        missing_resource_status,
        StatusCode::BAD_REQUEST,
        "missing resource: {missing_resource}"
    );
    let resource_code = missing_resource
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| missing_resource.get("code").and_then(|c| c.as_str()));
    assert!(
        resource_code == Some("AUDIT_MISSING_RESOURCE")
            || missing_resource
                .to_string()
                .contains("AUDIT_MISSING_RESOURCE")
            || missing_resource
                .to_string()
                .contains("missing resource_type"),
        "unexpected missing-resource body: {missing_resource}"
    );

    assert_eq!(
        audit_api_contracts_depth_stub(Some(&json!({"event_field_fixtures": true}))),
        AuditApiContractsDepth::EventFieldFixtures
    );
}
