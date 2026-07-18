//! JSON shape and authenticated lifecycle for VM write APIs (PH-S03 / UI_QUALITY §P2).
//! Covers `VmService` handlers: POST/PUT/DELETE `/api/v1/vm/instances` and start/stop/restart.

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::AppState;
use poolai::network::api::create_api_routes;
use poolai::network::auth::{generate_token, UserRole};
use poolai::vm::VmManager;
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

fn admin_bearer() -> String {
    let token = generate_token("admin", UserRole::Admin).expect("admin token");
    format!("Bearer {token}")
}

fn viewer_bearer() -> String {
    let token = generate_token("viewer", UserRole::Viewer).expect("viewer token");
    format!("Bearer {token}")
}

async fn vm_app_with_manager() -> Router {
    let manager = Arc::new(VmManager::new());
    manager.initialize().await.expect("vm init");
    let state = Arc::new(AppState::default());
    state
        .attach_vm_manager_for_test(manager)
        .expect("attach vm manager");
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(state)
}

fn assert_structured_error(v: &Value) {
    assert!(
        v.get("error").is_some(),
        "expected structured JSON error: {v:?}"
    );
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

fn assert_vm_instance_shape(o: &serde_json::Map<String, Value>) {
    for key in ["id", "name", "status", "resources"] {
        assert!(o.contains_key(key), "vm instance missing `{key}`: {o:?}");
    }
    let resources = o
        .get("resources")
        .and_then(|r| r.as_object())
        .expect("resources object");
    for key in ["cpu_cores", "memory_mb"] {
        assert!(
            resources.contains_key(key),
            "vm resources missing `{key}`: {resources:?}"
        );
    }
}

#[tokio::test]
async fn vm_write_lifecycle_with_admin_auth_ph_s994() {
    let app = vm_app_with_manager().await;
    let auth = admin_bearer();
    let name = format!("contract-vm-{}", uuid::Uuid::new_v4());

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/v1/vm/instances",
        Some(json!({
            "name": name,
            "resources": {
                "cpu_cores": 2,
                "memory_mb": 1024,
                "gpu_required": false
            },
            "isolation": "ProcessSandbox"
        })),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    let created_obj = created.as_object().expect("create response object");
    assert_vm_instance_shape(created_obj);
    assert_eq!(
        created_obj.get("name").and_then(|n| n.as_str()),
        Some(name.as_str())
    );
    let id = created_obj
        .get("id")
        .and_then(|x| x.as_str())
        .expect("vm id")
        .to_string();

    let (list_status, list) = request_json(&app, "GET", "/api/v1/vm/instances", None, None).await;
    assert_eq!(list_status, StatusCode::OK);
    let arr = list.as_array().expect("instances array");
    assert!(
        arr.iter()
            .any(|i| i.get("id").and_then(|x| x.as_str()) == Some(id.as_str())),
        "created instance not in list: {arr:?}"
    );

    let updated_name = format!("{name}-updated");
    let (update_status, updated) = request_json(
        &app,
        "PUT",
        &format!("/api/v1/vm/instances/{id}"),
        Some(json!({
            "name": updated_name,
            "resources": {
                "cpu_cores": 4,
                "memory_mb": 2048,
                "gpu_required": true
            }
        })),
        Some(&auth),
    )
    .await;
    assert_eq!(update_status, StatusCode::OK);
    let updated_obj = updated.as_object().expect("update response object");
    assert_eq!(
        updated_obj.get("name").and_then(|n| n.as_str()),
        Some(updated_name.as_str())
    );
    assert_eq!(
        updated_obj
            .get("resources")
            .and_then(|r| r.get("cpu_cores"))
            .and_then(|c| c.as_u64()),
        Some(4)
    );

    let (start_status, start_body) = request_json(
        &app,
        "POST",
        &format!("/api/v1/vm/instances/{id}/start"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(start_status, StatusCode::OK);
    assert!(
        start_body.get("message").and_then(|m| m.as_str()).is_some(),
        "start response missing message: {start_body:?}"
    );

    let (stop_status, _) = request_json(
        &app,
        "POST",
        &format!("/api/v1/vm/instances/{id}/stop"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(stop_status, StatusCode::OK);

    let (restart_status, _) = request_json(
        &app,
        "POST",
        &format!("/api/v1/vm/instances/{id}/restart"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(restart_status, StatusCode::OK);

    let (delete_status, delete_body) = request_json(
        &app,
        "DELETE",
        &format!("/api/v1/vm/instances/{id}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(delete_status, StatusCode::OK);
    assert!(
        delete_body
            .get("message")
            .and_then(|m| m.as_str())
            .is_some(),
        "delete response missing message: {delete_body:?}"
    );

    let (_, list_after) = request_json(&app, "GET", "/api/v1/vm/instances", None, None).await;
    let arr_after = list_after.as_array().expect("instances array after delete");
    assert!(
        !arr_after
            .iter()
            .any(|i| i.get("id").and_then(|x| x.as_str()) == Some(id.as_str())),
        "deleted instance still listed"
    );
}

#[tokio::test]
async fn vm_create_without_auth_returns_401() {
    let app = vm_app_with_manager().await;
    let (status, body) = request_json(
        &app,
        "POST",
        "/api/v1/vm/instances",
        Some(json!({
            "name": "no-auth-vm",
            "resources": { "cpu_cores": 1, "memory_mb": 512, "gpu_required": false }
        })),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_structured_error(&body);
}

#[tokio::test]
async fn vm_create_viewer_returns_403() {
    let app = vm_app_with_manager().await;
    let (status, body) = request_json(
        &app,
        "POST",
        "/api/v1/vm/instances",
        Some(json!({
            "name": "viewer-vm",
            "resources": { "cpu_cores": 1, "memory_mb": 512, "gpu_required": false }
        })),
        Some(&viewer_bearer()),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_structured_error(&body);
}

#[tokio::test]
async fn vm_create_without_manager_returns_service_unavailable() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(Arc::new(AppState::default()));

    let (status, body) = request_json(
        &app,
        "POST",
        "/api/v1/vm/instances",
        Some(json!({
            "name": "no-manager-vm",
            "resources": { "cpu_cores": 1, "memory_mb": 512, "gpu_required": false }
        })),
        Some(&admin_bearer()),
    )
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_structured_error(&body);
}
