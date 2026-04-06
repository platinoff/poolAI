//! FM-001 / P1: verify the composed `/api/v1` router uses injected [`ApiContext`] only.
//!
//! No `raid::initialize`, `vm::initialize`, or other module globals — managers are attached via
//! [`AppState::attach_*_for_test`] (`feature = "test-utils"`).
//!
//! Run: `cargo test -j 1 --features test-utils --test appstate_http_injection_integration`

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use poolai::core::state::{ApiContext, AppState};
use poolai::network::api::create_api_routes;
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use poolai::vm::VmManager;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

async fn api_ctx_with_raid(temp: &TempDir) -> ApiContext {
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = Arc::new(RaidManager::new(config));
    manager.initialize().await.unwrap();

    let ctx: ApiContext = Arc::new(AppState::new());
    ctx.attach_raid_manager_for_test(manager)
        .expect("attach raid manager for test");
    ctx
}

fn api_ctx_with_vm_only() -> ApiContext {
    let ctx: ApiContext = Arc::new(AppState::new());
    ctx.attach_vm_manager_for_test(Arc::new(VmManager::new()))
        .expect("attach vm manager for test");
    ctx
}

#[tokio::test]
async fn full_api_routes_raid_nodes_from_injected_manager() {
    let temp = TempDir::new().unwrap();
    let ctx = api_ctx_with_raid(&temp).await;
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/nodes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        v.is_array(),
        "GET /raid/nodes should return a JSON array, got {v:?}"
    );
}

#[tokio::test]
async fn full_api_routes_vm_instances_from_injected_manager() {
    let ctx = api_ctx_with_vm_only();
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/vm/instances")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        v.is_array(),
        "GET /vm/instances should return a JSON array, got {v:?}"
    );
}
