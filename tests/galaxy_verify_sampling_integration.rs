//! PH-S164: Galaxy verification sampling — HTTP grid middleware + result ingest wire.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_verify_sampling::{
    reset_verify_sampling_metrics_for_test, verify_sample_scheduled_total,
    ENV_VERIFY_BASE_SAMPLE_RATE, HEADER_VERIFY_BASE_SAMPLE_RATE,
};
use poolai::network::api::create_api_routes;
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static VERIFY_SAMPLING_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn verify_sampling_lock() -> std::sync::MutexGuard<'static, ()> {
    VERIFY_SAMPLING_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy verify sampling integration lock")
}

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, axum::http::HeaderMap, Value) {
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
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, headers, v)
}

fn job_envelope(job_id: &str, peer: &str) -> Value {
    json!({
        "v": 1,
        "sent_at": "2026-06-14T12:00:00Z",
        "type": "job",
        "job_id": job_id,
        "task_kind": "inference",
        "input_artifact_ids": [format!("artifact-{job_id}")],
        "source_peer_id": peer
    })
}

fn result_envelope(job_id: &str, peer: &str, lease_epoch: u64) -> Value {
    json!({
        "v": 1,
        "sent_at": "2026-06-14T12:00:01Z",
        "type": "result",
        "job_id": job_id,
        "status": "completed",
        "output_artifact_ids": [format!("out-{job_id}")],
        "lease_epoch": lease_epoch,
        "source_peer_id": peer
    })
}

#[tokio::test]
async fn grid_envelope_response_includes_verify_sample_rate_header() {
    let _lock = verify_sampling_lock();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0.15");
    let app = grid_app();
    let job_id = format!(
        "ph-s164-header-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );

    let (status, headers, body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, "tg-header-probe")),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(
        headers
            .get(HEADER_VERIFY_BASE_SAMPLE_RATE)
            .and_then(|v| v.to_str().ok()),
        Some("0.150000")
    );
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
}

#[tokio::test]
async fn grid_result_telegram_edge_increments_verify_sample_counter() {
    let _lock = verify_sampling_lock();
    reset_verify_sampling_metrics_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "1");
    let app = grid_app();
    let job_id = format!(
        "ph-s164-verify-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );
    let peer = "tg-edge-ph-s164";

    let (job_status, _, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, peer)),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let (_, _, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"]
        .as_u64()
        .expect("lease_epoch");

    let (result_status, headers, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(result_envelope(&job_id, peer, epoch)),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert_eq!(
        headers
            .get(HEADER_VERIFY_BASE_SAMPLE_RATE)
            .and_then(|v| v.to_str().ok()),
        Some("1.000000")
    );
    assert_eq!(verify_sample_scheduled_total(), 1);

    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    reset_verify_sampling_metrics_for_test();
}
