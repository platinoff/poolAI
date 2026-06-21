//! PH-S829: Galaxy horizon close band (PH-S820…S828).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::admin_vm_workers::{
    admin_worker_galaxy_telemetry_subset, validate_vm_instances_admin_list_shape,
    validate_workers_admin_list_shape,
};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::libs::render_libs_panel_html;
use poolai_ui_core::vm::render_vm_panel_html;
use poolai_ui_core::workers::render_workers_panel_html;
use serde_json::json;
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn get_text(app: &Router, uri: &str) -> (StatusCode, String) {
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
    let text = String::from_utf8(bytes.to_vec()).unwrap_or_default();
    (status, text)
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, serde_json::Value) {
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
    let text = String::from_utf8(bytes.to_vec()).unwrap_or_default();
    let body: serde_json::Value = serde_json::from_str(&text).unwrap_or(json!(null));
    (status, body)
}

#[tokio::test]
async fn horizon_s820_band_admin_wasm_slim_vm_workers_libs_ph_s829() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true}))),
        AdminWasmSlimDepth::VmPanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"workers_panel": true}))),
        AdminWasmSlimDepth::WorkersPanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"libs_panel": true}))),
        AdminWasmSlimDepth::LibsPanel
    );

    let vm_html = render_vm_panel_html(
        r#"[{"id":"vm-1","name":"n","status":"running","resources":{"cpu_cores":1,"memory_mb":512}}]"#,
        "Name",
        "Status",
        "Resources",
        "Actions",
        "VM",
        "CPU",
        "MEM",
        "Start",
        "Stop",
        "Delete",
        "Empty",
    );
    assert!(vm_html.contains("data-vm-action=\"start\""));

    let wrk_html = render_workers_panel_html(
        r#"[{"id":"w1","is_healthy":true,"total_requests_processed":2}]"#,
        "ID",
        "Status",
        "Metrics",
        "Actions",
        "Workers",
        "Healthy",
        "Unhealthy",
        "Requests:",
        "Delete",
        "Empty",
    );
    assert!(wrk_html.contains("data-worker-id=\"w1\""));

    let libs_html = render_libs_panel_html(
        r#"[{"name":"lib-a","version":"1.0.0","installed":false}]"#,
        "Name",
        "Version",
        "Status",
        "Actions",
        "Libraries",
        "Installed",
        "Not Installed",
        "Uninstall",
        "Update",
        "Install",
        "Empty",
    );
    assert!(libs_html.contains("data-lib-action=\"install\""));

    let app = grid_app();
    let (workers_status, workers_body) = get_json(&app, "/api/v1/workers").await;
    assert_eq!(workers_status, StatusCode::OK);
    validate_workers_admin_list_shape(&workers_body).expect("workers shape");
    if let Some(first) = workers_body.as_array().and_then(|a| a.first()) {
        assert!(admin_worker_galaxy_telemetry_subset(first).is_some());
    }

    let (vm_status, _) = get_json(&app, "/api/v1/vm/instances").await;
    assert_eq!(vm_status, StatusCode::SERVICE_UNAVAILABLE);

    let mock_vms = json!([{
        "id": "vm-mock",
        "name": "mock",
        "status": "stopped",
        "resources": { "cpu_cores": 1, "memory_mb": 512 }
    }]);
    validate_vm_instances_admin_list_shape(&mock_vms).expect("vm mock shape");

    let (metrics_status, metrics_text) = get_text(&app, "/metrics").await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert!(metrics_text.contains("# TYPE"));
}
