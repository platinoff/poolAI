//! PH-S179: Galaxy replication strict total — grid job ingest path → Prometheus scrape.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replication_metrics::{
    reset_replication_strict_metrics_for_test, METRIC_REPLICATION_STRICT_TOTAL,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static REPLICATION_STRICT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn replication_strict_lock() -> std::sync::MutexGuard<'static, ()> {
    REPLICATION_STRICT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy replication strict integration lock")
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

async fn get_metrics_text(app: &Router) -> String {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).expect("utf8")
}

#[tokio::test]
async fn grid_job_strict_policy_increments_replication_strict_on_scrape() {
    let _lock = replication_strict_lock();
    reset_replication_strict_metrics_for_test();
    let app = grid_app();
    let job_id = format!(
        "ph-s179-strict-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );

    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-14T12:00:00Z",
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference",
            "verification_policy": "replication_strict",
            "input_artifact_ids": [format!("artifact-{job_id}")]
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let metrics = get_metrics_text(&app).await;
    assert!(metrics.contains(METRIC_REPLICATION_STRICT_TOTAL));
    assert!(metrics.contains(&format!("{METRIC_REPLICATION_STRICT_TOTAL} 1")));

    reset_replication_strict_metrics_for_test();
}
