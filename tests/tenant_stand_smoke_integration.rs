//! PH-S1190…S1192: Tenant live stand-smoke contracts (band 55).
//! Marker: tenant_stand_smoke_integration
//!
//! CI canon uses in-process axum (no live stand). Live HTTP runners live in
//! `poolai-http-stand-smoke --tenant-stand-smoke`.

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::enterprise::multi_tenancy::TenantConfig;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::tenant_stand_smoke_depth::{
    tenant_stand_smoke_criteria_total, tenant_stand_smoke_depth_stub, TenantStandSmokeDepth,
    TENANT_STAND_SMOKE_CASES, TENANT_STAND_SMOKE_CRITERIA,
};
use serde_json::{json, Value};
use tower::ServiceExt;

fn admin_bearer() -> String {
    let token = generate_token("admin", UserRole::Admin).expect("admin token");
    format!("Bearer {token}")
}

async fn enterprise_app() -> Router {
    let ctx = ApiContext::default();
    ctx.tenant_manager.initialize().await.expect("tenant init");
    Router::new()
        .nest("/api/enterprise", create_enterprise_api_routes())
        .with_state(ctx)
}

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    auth: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(bearer) = auth {
        builder = builder.header("authorization", bearer);
    }
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

fn default_create_body(name: &str) -> Value {
    json!({
        "name": name,
        "config": TenantConfig::default()
    })
}

#[test]
fn tenant_stand_smoke_depth_registry_ph_s1189() {
    assert_eq!(TENANT_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(tenant_stand_smoke_criteria_total(), 10);
    assert!(TENANT_STAND_SMOKE_CASES.contains(&"live_store"));
    assert!(TENANT_STAND_SMOKE_CASES.contains(&"live_crud"));
    assert_eq!(
        tenant_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
        TenantStandSmokeDepth::LiveStore
    );
}

#[tokio::test]
async fn tenant_stand_smoke_store_wire_ph_s1190() {
    std::env::remove_var("POOLAI_TENANT_STORE");
    std::env::remove_var("POOLAI_TENANT_DATA_DIR");

    let app = enterprise_app().await;
    let (status, wire) =
        request_json(&app, "GET", "/api/enterprise/tenants/store", None, None).await;
    assert_eq!(status, StatusCode::OK, "store wire: {wire}");
    let obj = wire.as_object().expect("wire object");
    for key in ["mode", "durable_path", "configured"] {
        assert!(obj.contains_key(key), "wire missing `{key}`: {obj:?}");
    }
    assert_eq!(obj.get("mode").and_then(|m| m.as_str()), Some("memory"));
    assert_eq!(obj.get("configured").and_then(|c| c.as_bool()), Some(false));
}

#[tokio::test]
async fn tenant_stand_smoke_crud_lifecycle_ph_s1191() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("stand-smoke-{}", uuid::Uuid::new_v4());

    let (list0_status, list0) =
        request_json(&app, "GET", "/api/enterprise/tenants", None, None).await;
    assert_eq!(list0_status, StatusCode::OK, "list: {list0}");
    assert!(list0.as_array().is_some(), "list array: {list0}");

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/tenants",
        Some(default_create_body(&name)),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK, "create: {created}");
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id")
        .to_string();

    let (get_status, got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK, "get: {got}");
    assert_eq!(
        got.get("name").and_then(|n| n.as_str()),
        Some(name.as_str())
    );

    let (del_status, del) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK, "delete: {del}");

    let (gone_status, _) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(gone_status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn tenant_stand_smoke_usage_quota_isolation_ph_s1192() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("stand-quota-{}", uuid::Uuid::new_v4());

    let (_, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/tenants",
        Some(default_create_body(&name)),
        Some(&auth),
    )
    .await;
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id")
        .to_string();

    let (usage_status, usage) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id}/usage"),
        None,
        None,
    )
    .await;
    assert_eq!(usage_status, StatusCode::OK, "usage: {usage}");
    for key in [
        "workers",
        "memory_mb",
        "cpu_cores",
        "storage_mb",
        "vm_instances",
    ] {
        assert!(usage.get(key).is_some(), "usage missing `{key}`: {usage:?}");
    }

    let (allow_status, allow) = request_json(
        &app,
        "POST",
        &format!("/api/enterprise/tenants/{id}/quota"),
        Some(json!({
            "workers": 1,
            "memory_mb": 64,
            "cpu_cores": 1
        })),
        None,
    )
    .await;
    assert_eq!(allow_status, StatusCode::OK, "quota allow: {allow}");
    assert_eq!(allow.get("allowed").and_then(|a| a.as_bool()), Some(true));

    let (deny_status, deny) = request_json(
        &app,
        "POST",
        &format!("/api/enterprise/tenants/{id}/quota"),
        Some(json!({
            "workers": 10_000,
            "memory_mb": 64,
            "cpu_cores": 1
        })),
        None,
    )
    .await;
    assert_eq!(deny_status, StatusCode::OK, "quota deny: {deny}");
    assert_eq!(deny.get("allowed").and_then(|a| a.as_bool()), Some(false));

    let foreign = uuid::Uuid::new_v4();
    let (foreign_status, _) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{foreign}"),
        None,
        None,
    )
    .await;
    assert_eq!(foreign_status, StatusCode::NOT_FOUND);

    let _ = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        Some(&auth),
    )
    .await;
}
