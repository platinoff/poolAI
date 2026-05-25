//! PH-S38: core → sidecar NDJSON domain events (`POOLAI_ONCHAIN_EVENTS_DIR`).

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::job::{events_dir_from_env, JobStore};
use poolai::network::api::create_api_routes;
use serde_json::json;
use std::sync::{Mutex, OnceLock};
use tempfile::TempDir;
use tower::ServiceExt;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn jobs_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn patch_to_rewarded_appends_job_completed_ndjson() {
    let _guard = env_lock();
    let tmp = TempDir::new().expect("tempdir");
    std::env::set_var(
        "POOLAI_ONCHAIN_EVENTS_DIR",
        tmp.path().to_string_lossy().as_ref(),
    );
    assert!(events_dir_from_env().is_some());

    let app = jobs_app();
    let create = Request::builder()
        .method("POST")
        .uri("/api/v1/jobs")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({ "kind": "inference" })).unwrap(),
        ))
        .unwrap();
    let create_res = app.clone().oneshot(create).await.unwrap();
    assert_eq!(create_res.status(), StatusCode::CREATED);
    let bytes = to_bytes(create_res.into_body(), usize::MAX).await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let id = created
        .get("id")
        .and_then(|x| x.as_str())
        .expect("id")
        .to_string();

    for status in ["executing", "verifying", "rewarded"] {
        let patch = Request::builder()
            .method("PATCH")
            .uri(format!("/api/v1/jobs/{id}"))
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({ "status": status })).unwrap(),
            ))
            .unwrap();
        let res = app.clone().oneshot(patch).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "patch to {status}");
    }

    let path = tmp.path().join("events.ndjson");
    let text = std::fs::read_to_string(&path).expect("events file");
    let line = text.lines().next().expect("one line");
    let v: serde_json::Value = serde_json::from_str(line).expect("json");
    assert_eq!(v.get("schema_version").and_then(|x| x.as_u64()), Some(1));
    assert_eq!(
        v.get("type").and_then(|t| t.as_str()),
        Some("job_completed")
    );
    assert_eq!(v.get("job_id").and_then(|j| j.as_str()), Some(id.as_str()));

    std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    let _ = JobStore::global();
}
