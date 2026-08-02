//! PH-S178: Galaxy settlement pending verification total — grid result path → Prometheus scrape.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_settlement_metrics::{
    reset_settlement_pending_verification_metrics_for_test,
    METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static SETTLEMENT_PENDING_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn settlement_pending_lock() -> std::sync::MutexGuard<'static, ()> {
    SETTLEMENT_PENDING_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy settlement pending verification integration lock")
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
async fn grid_result_trust_hold_increments_settlement_pending_on_scrape() {
    let _lock = settlement_pending_lock();
    reset_settlement_pending_verification_metrics_for_test();
    let app = grid_app();
    let job_id = format!(
        "ph-s178-pending-{}",
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
            "input_artifact_ids": [format!("artifact-{job_id}")],
            "source_peer_id": "tg-edge"
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"]
        .as_u64()
        .expect("lease_epoch");

    let (result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-14T12:00:01Z",
            "type": "result",
            "job_id": job_id,
            "status": "completed",
            "output_artifact_ids": [format!("out-{job_id}")],
            "lease_epoch": epoch,
            "source_peer_id": "tg-edge",
            "metrics": { "trust_score": 10 }
        })),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);

    let metrics = get_metrics_text(&app).await;
    assert!(metrics.contains(METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL));
    assert!(metrics.contains(&format!("{METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL} 1")));

    reset_settlement_pending_verification_metrics_for_test();
}
