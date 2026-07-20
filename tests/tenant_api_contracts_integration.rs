//! PH-S1170…S1173: Tenant HTTP API contracts (band 53).
//! Marker: tenant_api_contracts_integration

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::enterprise::multi_tenancy::TenantConfig;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::tenant_api_contracts_depth::{
    tenant_api_contracts_depth_stub, tenant_api_criteria_total, TenantApiContractsDepth,
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

fn assert_tenant_shape(o: &serde_json::Map<String, Value>) {
    for key in ["id", "name", "config", "usage", "created_at", "updated_at"] {
        assert!(o.contains_key(key), "tenant missing `{key}`: {o:?}");
    }
    let config = o
        .get("config")
        .and_then(|c| c.as_object())
        .expect("config object");
    for key in ["active", "max_workers", "max_memory_mb"] {
        assert!(
            config.contains_key(key),
            "config missing `{key}`: {config:?}"
        );
    }
    let usage = o
        .get("usage")
        .and_then(|u| u.as_object())
        .expect("usage object");
    for key in ["workers", "memory_mb", "cpu_cores"] {
        assert!(usage.contains_key(key), "usage missing `{key}`: {usage:?}");
    }
}

fn default_create_body(name: &str) -> Value {
    json!({
        "name": name,
        "config": TenantConfig::default()
    })
}

#[tokio::test]
async fn tenant_http_crud_lifecycle_ph_s1170() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("api-contract-{}", uuid::Uuid::new_v4());

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/tenants",
        Some(default_create_body(&name)),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK, "create: {created}");
    let created_obj = created.as_object().expect("created object");
    assert_tenant_shape(created_obj);
    let id = created_obj
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id")
        .to_string();
    assert_eq!(
        created_obj.get("name").and_then(|v| v.as_str()),
        Some(name.as_str())
    );

    let (get_status, got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK, "get: {got}");
    assert_tenant_shape(got.as_object().expect("get object"));

    let (upd_status, updated) = request_json(
        &app,
        "POST",
        &format!("/api/enterprise/tenants/{id}"),
        Some(json!({ "active": false })),
        Some(&auth),
    )
    .await;
    assert_eq!(upd_status, StatusCode::OK, "update: {updated}");
    let upd_obj = updated.as_object().expect("updated object");
    assert_tenant_shape(upd_obj);
    assert_eq!(
        upd_obj
            .get("config")
            .and_then(|c| c.get("active"))
            .and_then(|a| a.as_bool()),
        Some(false)
    );

    let (del_status, deleted) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK, "delete: {deleted}");
    assert_eq!(
        deleted.get("message").and_then(|m| m.as_str()),
        Some("Tenant deleted successfully")
    );

    let (missing_status, missing) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(missing_status, StatusCode::NOT_FOUND, "missing: {missing}");

    assert_eq!(
        tenant_api_contracts_depth_stub(Some(&json!({"http_crud": true}))),
        TenantApiContractsDepth::HttpCrud
    );
    assert_eq!(tenant_api_criteria_total(), 10);
}

#[tokio::test]
async fn tenant_quota_usage_http_ph_s1171() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("quota-{}", uuid::Uuid::new_v4());

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
    let usage_obj = usage.as_object().expect("usage object");
    for key in [
        "workers",
        "memory_mb",
        "cpu_cores",
        "storage_mb",
        "vm_instances",
    ] {
        assert!(
            usage_obj.contains_key(key),
            "usage missing `{key}`: {usage_obj:?}"
        );
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
    let allow_obj = allow.as_object().expect("quota object");
    for key in ["allowed", "reason", "projected_usage"] {
        assert!(
            allow_obj.contains_key(key),
            "quota missing `{key}`: {allow_obj:?}"
        );
    }
    assert_eq!(
        allow_obj.get("allowed").and_then(|a| a.as_bool()),
        Some(true)
    );

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
    assert_eq!(
        deny.get("allowed").and_then(|a| a.as_bool()),
        Some(false),
        "expected deny: {deny}"
    );
    assert!(deny.get("reason").and_then(|r| r.as_str()).is_some());

    let _ = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id}"),
        None,
        Some(&auth),
    )
    .await;
}

#[tokio::test]
async fn tenant_cross_tenant_isolation_http_ph_s1172() {
    let app = enterprise_app().await;
    let auth = admin_bearer();

    let (_, a) = request_json(
        &app,
        "POST",
        "/api/enterprise/tenants",
        Some(default_create_body(&format!(
            "iso-a-{}",
            uuid::Uuid::new_v4()
        ))),
        Some(&auth),
    )
    .await;
    let (_, b) = request_json(
        &app,
        "POST",
        "/api/enterprise/tenants",
        Some(default_create_body(&format!(
            "iso-b-{}",
            uuid::Uuid::new_v4()
        ))),
        Some(&auth),
    )
    .await;
    let id_a = a
        .get("id")
        .and_then(|v| v.as_str())
        .expect("a id")
        .to_string();
    let id_b = b
        .get("id")
        .and_then(|v| v.as_str())
        .expect("b id")
        .to_string();
    let name_b = b
        .get("name")
        .and_then(|v| v.as_str())
        .expect("b name")
        .to_string();

    let (upd_status, _) = request_json(
        &app,
        "POST",
        &format!("/api/enterprise/tenants/{id_a}"),
        Some(json!({ "active": false })),
        Some(&auth),
    )
    .await;
    assert_eq!(upd_status, StatusCode::OK);

    let (get_b_status, got_b) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{id_b}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_b_status, StatusCode::OK);
    let b_obj = got_b.as_object().expect("b object");
    assert_eq!(
        b_obj.get("name").and_then(|v| v.as_str()),
        Some(name_b.as_str())
    );
    assert_eq!(
        b_obj
            .get("config")
            .and_then(|c| c.get("active"))
            .and_then(|a| a.as_bool()),
        Some(true),
        "tenant B must remain active after A mutate"
    );

    let foreign = uuid::Uuid::new_v4();
    let (foreign_status, foreign_body) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/tenants/{foreign}"),
        None,
        None,
    )
    .await;
    assert_eq!(
        foreign_status,
        StatusCode::NOT_FOUND,
        "foreign: {foreign_body}"
    );

    let _ = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id_a}"),
        None,
        Some(&auth),
    )
    .await;
    let _ = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/tenants/{id_b}"),
        None,
        Some(&auth),
    )
    .await;
}

#[tokio::test]
async fn tenant_store_wire_http_ph_s1173() {
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

    std::env::set_var("POOLAI_TENANT_STORE", "sqlite");
    std::env::set_var("POOLAI_TENANT_DATA_DIR", "data/dev/tenants");
    let (status2, wire2) =
        request_json(&app, "GET", "/api/enterprise/tenants/store", None, None).await;
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
    assert!(path.contains("tenants.sqlite"), "path={path}");

    std::env::remove_var("POOLAI_TENANT_STORE");
    std::env::remove_var("POOLAI_TENANT_DATA_DIR");

    assert_eq!(
        tenant_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
        TenantApiContractsDepth::StoreWireHttp
    );
}
