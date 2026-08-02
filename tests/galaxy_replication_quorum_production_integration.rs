//! PH-S890: replication quorum gate production — strict tier HTTP wire integration.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replication_quorum_gate::{
    record_result_executor_digest, reset_replication_quorum_gate_for_test,
};
use poolai::grid::galaxy_settlement_metrics::{
    reset_settlement_pending_verification_metrics_for_test, settlement_pending_verification_total,
};
use poolai::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE;
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
async fn replication_quorum_strict_tier_blocks_cleared_on_http_ph_s890() {
    let _guard = env_lock();
    reset_replication_quorum_gate_for_test();
    reset_settlement_pending_verification_metrics_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");

    let app = grid_app();
    let job_id = format!("ph-s890-quorum-{}", uuid::Uuid::new_v4());

    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-21T12:00:00Z",
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference:text",
            "verification_policy": "replication_strict",
            "input_artifact_ids": ["artifact-quorum"],
            "source_peer_id": "tg-edge-s890"
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    record_result_executor_digest(&job_id, Some(&json!({"executor_digest": "digest-a"})));
    record_result_executor_digest(&job_id, Some(&json!({"executor_digest": "digest-a"})));
    record_result_executor_digest(&job_id, Some(&json!({"executor_digest": "digest-b"})));

    let (_, job_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let lease_epoch = job_get["job"]["lease_epoch"].as_u64().expect("epoch");

    let (result_status, result_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-21T12:00:01Z",
            "type": "result",
            "job_id": job_id,
            "status": "completed",
            "output_artifact_ids": [],
            "lease_epoch": lease_epoch,
            "metrics": {
                "trust_score": 900,
                "executor_digest": "digest-a"
            },
            "source_peer_id": "tg-edge-s890"
        })),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert_eq!(result_body["ok"], true);
    assert_eq!(result_body["type"], "result");
    assert_eq!(result_body["status"], "completed");
    assert!(settlement_pending_verification_total() >= 1);

    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    reset_replication_quorum_gate_for_test();
    reset_settlement_pending_verification_metrics_for_test();
}
