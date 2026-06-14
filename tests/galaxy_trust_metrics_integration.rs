//! PH-S163: Galaxy trust settlement metrics wire — grid HTTP result path → Prometheus scrape.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_trust_score::{
    reset_settlement_gate_metrics_for_test, METRIC_PAYOUT_ELIGIBLE_TOTAL, METRIC_PAYOUT_HELD_TOTAL,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static TRUST_METRICS_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn trust_metrics_lock() -> std::sync::MutexGuard<'static, ()> {
    TRUST_METRICS_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy trust metrics integration lock")
}

fn trust_app() -> Router {
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

fn job_envelope(job_id: &str, source_peer_id: &str) -> Value {
    json!({
        "v": 1,
        "sent_at": "2026-06-14T12:00:00Z",
        "type": "job",
        "job_id": job_id,
        "task_kind": "inference",
        "input_artifact_ids": [format!("artifact-{job_id}")],
        "source_peer_id": source_peer_id
    })
}

fn result_envelope(
    job_id: &str,
    source_peer_id: &str,
    trust_score: u64,
    lease_epoch: u64,
) -> Value {
    json!({
        "v": 1,
        "sent_at": "2026-06-14T12:00:01Z",
        "type": "result",
        "job_id": job_id,
        "status": "completed",
        "output_artifact_ids": [format!("out-{job_id}")],
        "lease_epoch": lease_epoch,
        "metrics": { "trust_score": trust_score },
        "source_peer_id": source_peer_id
    })
}

#[tokio::test]
async fn grid_result_path_increments_trust_metrics_on_scrape() {
    let _lock = trust_metrics_lock();
    reset_settlement_gate_metrics_for_test();
    let app = trust_app();
    let job_id = format!(
        "ph-s163-trust-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );
    let peer = "tg-edge-ph-s163";

    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, peer)),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"]
        .as_u64()
        .expect("lease_epoch");

    let (held_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(result_envelope(&job_id, peer, 15, epoch)),
    )
    .await;
    assert_eq!(held_status, StatusCode::OK);

    let metrics_after_held = get_metrics_text(&app).await;
    assert!(metrics_after_held.contains(METRIC_PAYOUT_HELD_TOTAL));
    assert!(metrics_after_held.contains(&format!("{METRIC_PAYOUT_HELD_TOTAL} 1")));

    let job_id2 = format!("{job_id}-b");
    let (job2_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id2, peer)),
    )
    .await;
    assert_eq!(job2_status, StatusCode::OK);
    let (_, get2) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id2}"), None).await;
    let epoch2 = get2["job"]["lease_epoch"].as_u64().expect("lease_epoch");

    let (eligible_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(result_envelope(&job_id2, peer, 90, epoch2)),
    )
    .await;
    assert_eq!(eligible_status, StatusCode::OK);

    let metrics_final = get_metrics_text(&app).await;
    assert!(metrics_final.contains(&format!("{METRIC_PAYOUT_HELD_TOTAL} 1")));
    assert!(metrics_final.contains(&format!("{METRIC_PAYOUT_ELIGIBLE_TOTAL} 1")));

    reset_settlement_gate_metrics_for_test();
}
