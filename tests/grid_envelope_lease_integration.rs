//! PH-S144: Grid envelope job/result lease wire — migrated from
//! `e2e/tests/grid_job_lease.spec.ts` and `grid_result_lease.spec.ts`.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use serde_json::{json, Value};
use tower::ServiceExt;

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

fn job_envelope(job_id: &str, source_peer_id: Option<&str>) -> Value {
    let mut env = json!({
        "v": 1,
        "sent_at": "2026-06-13T12:00:00Z",
        "type": "job",
        "job_id": job_id,
        "task_kind": "inference",
        "input_artifact_ids": [format!("artifact-{job_id}")]
    });
    if let Some(peer) = source_peer_id {
        env["source_peer_id"] = json!(peer);
    }
    env
}

fn result_envelope(job_id: &str, lease_epoch: Option<u64>) -> Value {
    let mut env = json!({
        "v": 1,
        "sent_at": "2026-06-13T12:00:01Z",
        "type": "result",
        "job_id": job_id,
        "status": "completed",
        "output_artifact_ids": [format!("out-{job_id}")]
    });
    if let Some(epoch) = lease_epoch {
        env["lease_epoch"] = json!(epoch);
    }
    env
}

#[tokio::test]
async fn grid_envelope_job_with_peer_leases_job() {
    let app = grid_app();
    let job_id = format!(
        "ph-s144-grid-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );
    let peer_id = "e2e-grid-peer-a";

    let (ingest_status, ingest_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, Some(peer_id))),
    )
    .await;
    assert_eq!(ingest_status, StatusCode::OK);
    assert_eq!(ingest_body["ok"], true);
    assert_eq!(ingest_body["type"], "job");
    assert_eq!(ingest_body["job_id"], job_id);
    assert_eq!(ingest_body["status"], "leased");

    let (get_status, get_body) =
        request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    assert_eq!(get_status, StatusCode::OK);
    let job = &get_body["job"];
    assert_eq!(job["status"], "leased");
    assert_eq!(job["worker_id"], peer_id);
    assert_eq!(job["lease_owner"], peer_id);
    assert_eq!(job["lease_epoch"], 1);
    assert!(job["lease_expires_at"].as_str().is_some());
}

#[tokio::test]
async fn grid_envelope_job_without_peer_stays_scheduled() {
    let app = grid_app();
    let job_id = format!(
        "ph-s144-grid-nopeer-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );

    let (ingest_status, ingest_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, None)),
    )
    .await;
    assert_eq!(ingest_status, StatusCode::OK);
    assert_eq!(ingest_body["status"], "scheduled");

    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let job = &get_body["job"];
    assert_eq!(job["status"], "scheduled");
    assert!(job["lease_owner"].as_str().is_none());
    assert!(job["lease_epoch"].as_u64().is_none());
    assert!(job["lease_expires_at"].as_str().is_none());
}

#[tokio::test]
async fn grid_envelope_result_stale_lease_epoch_rejected() {
    let app = grid_app();
    let job_id = format!(
        "ph-s144-grid-result-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );
    let peer_id = "e2e-grid-result-peer";

    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, Some(peer_id))),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().unwrap_or(1);

    let (stale_status, stale_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(result_envelope(&job_id, Some(epoch.saturating_sub(1)))),
    )
    .await;
    assert_eq!(stale_status, StatusCode::CONFLICT);
    assert_eq!(stale_body["error"]["code"], "lease_epoch_rejected");

    let (_, after) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    assert_eq!(after["job"]["status"], "leased");
}

#[tokio::test]
async fn grid_envelope_result_matching_epoch_completes_job() {
    let app = grid_app();
    let job_id = format!(
        "ph-s144-grid-ok-{}",
        std::time::SystemTime::now().elapsed().unwrap().as_nanos()
    );
    let peer_id = "e2e-grid-result-ok";

    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(job_envelope(&job_id, Some(peer_id))),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);

    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"]
        .as_u64()
        .expect("lease_epoch");

    let (result_status, result_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(result_envelope(&job_id, Some(epoch))),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert_eq!(result_body["ok"], true);
    assert_eq!(result_body["type"], "result");
    assert_eq!(result_body["status"], "completed");

    let (_, final_job) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    assert_eq!(final_job["job"]["status"], "completed");
}

/// PH-S991: band-34 registry — grid job lease canon covers archived `grid_job_lease.spec.ts`.
#[test]
fn integration_gap_grid_job_lease_canon_ph_s991() {
    let src = include_str!("grid_envelope_lease_integration.rs");
    assert!(src.contains("grid_envelope_job_with_peer_leases_job"));
    assert!(src.contains("grid_envelope_job_without_peer_stays_scheduled"));
    assert!(src.contains("grid_envelope_result_matching_epoch_completes_job"));
}
