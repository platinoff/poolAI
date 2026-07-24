//! PH-S1390…S1392: Audit live stand-smoke contracts (band 75).
//! Marker: audit_stand_smoke_integration
//!
//! CI canon uses in-process axum (no live stand). Live HTTP runners live in
//! `poolai-http-stand-smoke --audit-stand-smoke`.

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::audit_stand_smoke_depth::{
    audit_stand_smoke_criteria_total, audit_stand_smoke_depth_stub, AuditStandSmokeDepth,
    AUDIT_STAND_SMOKE_CASES, AUDIT_STAND_SMOKE_CRITERIA,
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

#[test]
fn audit_stand_smoke_depth_registry_ph_s1389() {
    assert_eq!(AUDIT_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(audit_stand_smoke_criteria_total(), 10);
    assert!(AUDIT_STAND_SMOKE_CASES.contains(&"live_store"));
    assert!(AUDIT_STAND_SMOKE_CASES.contains(&"live_events_query"));
    assert_eq!(
        audit_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
        AuditStandSmokeDepth::LiveStore
    );
}

#[tokio::test]
async fn audit_stand_smoke_store_wire_ph_s1390() {
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
}

#[tokio::test]
async fn audit_stand_smoke_events_query_ph_s1391() {
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
    assert!(filtered.is_array(), "filtered array: {filtered}");

    assert_eq!(
        audit_stand_smoke_depth_stub(Some(&json!({"live_events_query": true}))),
        AuditStandSmokeDepth::LiveEventsQuery
    );
}

#[tokio::test]
async fn audit_stand_smoke_event_field_fixtures_ph_s1392() {
    let app = enterprise_app().await;

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
        audit_stand_smoke_depth_stub(Some(&json!({"live_event_field_fixtures": true}))),
        AuditStandSmokeDepth::LiveEventFieldFixtures
    );
}
