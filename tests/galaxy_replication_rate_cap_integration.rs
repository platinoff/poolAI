//! PH-S891: replication rate cap HTTP wire integration.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replication_metrics::{
    replication_rate_limited_total, reset_replication_strict_metrics_for_test,
    ENV_REPLICATION_MAX_PER_HOUR,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
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
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

#[tokio::test]
async fn replication_rate_cap_http_wire_ph_s891() {
    let _guard = env_lock();
    let prior = std::env::var(ENV_REPLICATION_MAX_PER_HOUR).ok();
    std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, "1");
    reset_replication_strict_metrics_for_test();

    let app = grid_app();
    let strict_job = |suffix: &str| {
        json!({
            "v": 1,
            "sent_at": "2026-06-21T12:00:00Z",
            "type": "job",
            "job_id": format!("ph-s891-{suffix}-{}", uuid::Uuid::new_v4()),
            "task_kind": "inference:text",
            "verification_policy": "replication_strict",
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })
    };

    let (first_status, _) =
        request_json(&app, "POST", "/api/v1/grid/envelope", Some(strict_job("a"))).await;
    assert_eq!(first_status, StatusCode::OK);
    let (second_status, _) =
        request_json(&app, "POST", "/api/v1/grid/envelope", Some(strict_job("b"))).await;
    assert_eq!(second_status, StatusCode::OK);
    assert!(replication_rate_limited_total() >= 1);

    let (metrics_status, metrics_body) =
        request_json(&app, "GET", "/api/v1/grid/replication-metrics", None).await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert_eq!(metrics_body["ok"], true);
    assert_eq!(metrics_body["rate_cap_per_hour"], 1);
    assert!(
        metrics_body["metrics"]["rate_limited_total"]
            .as_u64()
            .unwrap_or(0)
            >= 1
    );
    assert!(metrics_body["replication_depth"].is_string());

    match prior {
        Some(v) => std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, v),
        None => std::env::remove_var(ENV_REPLICATION_MAX_PER_HOUR),
    }
    reset_replication_strict_metrics_for_test();
}
