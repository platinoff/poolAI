//! PH-S629: Galaxy horizon close band (PH-S620…S628).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_prefetch_metrics::{
    hot_evict_total, hot_promote_total, prefetch_complete_total, prefetch_ingest_total,
    prefetch_lease_acquired_total, prefetch_wait_ms_total, reset_prefetch_metrics_for_test,
    shard_fetch_latency_ms_p50, ENV_HOT_PROMOTE_THRESHOLD,
};
use poolai::grid::galaxy_trust_score::{
    payout_held_total, reset_settlement_gate_metrics_for_test, trust_score_delta_total,
    ENV_MIN_TRUST_PAYOUT,
};
use poolai::grid::galaxy_trust_score_store::{
    lookup_peer_trust_score, reset_trust_score_store_for_test,
};
use poolai::grid::galaxy_verify_sampling::{
    reset_verify_sampling_metrics_for_test, verify_elevated_applied_total, ENV_VERIFY_ELEVATED_RATE,
};
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use tower::ServiceExt;

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
    let builder = Request::builder().method(method).uri(uri);
    let req = if let Some(json_body) = body {
        builder
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&json_body).unwrap()))
            .unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(json!({ "raw": String::from_utf8_lossy(&bytes) })),
    )
}

async fn metrics_text(app: &Router) -> String {
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
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap_or_default()
}

#[tokio::test]
async fn horizon_s620_band_trust_prefetch_telemetry_ph_s629() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_trust_score_store_for_test();
    reset_settlement_gate_metrics_for_test();
    reset_verify_sampling_metrics_for_test();
    reset_prefetch_metrics_for_test();

    let app = grid_app();
    let peer = "tg-edge-s620";

    // PH-S620: verification match trust delta persists to store.
    reset_trust_score_store_for_test();
    poolai::grid::galaxy_trust_score_store::persist_peer_trust_score(peer, 50);
    let job_match = format!("ph-s620-match-{}", uuid::Uuid::new_v4());
    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:00Z",
            "type": "job",
            "job_id": job_match,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-match"],
            "source_peer_id": peer
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_match}"), None).await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_delta = trust_score_delta_total();
    let (result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:01Z",
            "type": "result",
            "job_id": job_match,
            "status": "completed",
            "output_artifact_ids": ["out-match"],
            "lease_epoch": epoch,
            "source_peer_id": peer,
            "metrics": { "trust_score": 50, "verification_verdict": "match" }
        })),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert!(trust_score_delta_total() > prior_delta);
    assert_eq!(lookup_peer_trust_score(peer), Some(60));

    // PH-S621: telegram_edge low trust → payout held on /metrics.
    reset_settlement_gate_metrics_for_test();
    let job_held = format!("ph-s621-held-{}", uuid::Uuid::new_v4());
    let (held_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:02Z",
            "type": "job",
            "job_id": job_held,
            "task_kind": "inference:text",
            "input_artifact_ids": [],
            "source_peer_id": "tg-edge-held"
        })),
    )
    .await;
    assert_eq!(held_job_status, StatusCode::OK);
    let (_, held_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_held}"), None).await;
    let held_epoch = held_get["job"]["lease_epoch"].as_u64().expect("epoch");
    std::env::set_var(ENV_MIN_TRUST_PAYOUT, "40");
    let prior_held = payout_held_total();
    let (held_result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:03Z",
            "type": "result",
            "job_id": job_held,
            "status": "completed",
            "output_artifact_ids": [],
            "lease_epoch": held_epoch,
            "source_peer_id": "tg-edge-held",
            "metrics": { "trust_score": 30 }
        })),
    )
    .await;
    assert_eq!(held_result_status, StatusCode::OK);
    assert!(payout_held_total() > prior_held);
    let metrics = metrics_text(&app).await;
    assert!(metrics.contains("galaxy_trust_payout_held_total"));
    std::env::remove_var(ENV_MIN_TRUST_PAYOUT);

    // PH-S622: mismatch → elevated sampling via HTTP.
    reset_verify_sampling_metrics_for_test();
    std::env::set_var(ENV_VERIFY_ELEVATED_RATE, "1");
    let job_mismatch = format!("ph-s622-mismatch-{}", uuid::Uuid::new_v4());
    let (m_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:04Z",
            "type": "job",
            "job_id": job_mismatch,
            "task_kind": "inference:text",
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(m_job_status, StatusCode::OK);
    let (_, m_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_mismatch}"), None).await;
    let m_epoch = m_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_elevated = verify_elevated_applied_total();
    let (m_result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:05Z",
            "type": "result",
            "job_id": job_mismatch,
            "status": "completed",
            "output_artifact_ids": [],
            "lease_epoch": m_epoch,
            "source_peer_id": "srv1-worker-a",
            "metrics": { "verification_verdict": "mismatch" }
        })),
    )
    .await;
    assert_eq!(m_result_status, StatusCode::OK);
    assert!(verify_elevated_applied_total() > prior_elevated);
    std::env::remove_var(ENV_VERIFY_ELEVATED_RATE);

    // PH-S623: lease acquire → prefetch lease-acquired metric.
    reset_prefetch_metrics_for_test();
    let (lease_create_status, created) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({ "kind": "inference", "worker_id": "worker-lease-s623" })),
    )
    .await;
    assert_eq!(lease_create_status, StatusCode::CREATED);
    let lease_target = created
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();
    let prior_lease_metric = prefetch_lease_acquired_total();
    let (acquire_status, _) = request_json(
        &app,
        "POST",
        &format!("/api/v1/jobs/{lease_target}/lease"),
        Some(json!({ "lease_owner": "worker-lease-s623" })),
    )
    .await;
    assert_eq!(acquire_status, StatusCode::OK);
    assert!(prefetch_lease_acquired_total() > prior_lease_metric);

    // PH-S624 + PH-S625 + PH-S626: job ingest prefetch metrics + hot tier + fetch latency.
    reset_prefetch_metrics_for_test();
    std::env::set_var(ENV_HOT_PROMOTE_THRESHOLD, "1");
    let hot_shard = format!("w:hot-s624-{}", uuid::Uuid::new_v4());
    MemoryShardStore::global()
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new(&hot_shard),
            artifact_id: "artifact-hot".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("seed shard");
    let job_prefetch = format!("ph-s625-prefetch-{}", uuid::Uuid::new_v4());
    let prior_ingest = prefetch_ingest_total();
    let (prefetch_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:07Z",
            "type": "job",
            "job_id": job_prefetch,
            "task_kind": "inference:text",
            "required_shard_ids": [hot_shard, format!("w:missing-{}", uuid::Uuid::new_v4())],
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(prefetch_job_status, StatusCode::OK);
    assert!(prefetch_ingest_total() > prior_ingest);
    assert!(prefetch_wait_ms_total() > 0);
    assert!(prefetch_complete_total() > 0);
    assert!(hot_promote_total() >= 1 || hot_evict_total() >= 1);
    assert!(shard_fetch_latency_ms_p50() > 0);
    let metrics_prefetch = metrics_text(&app).await;
    assert!(metrics_prefetch.contains("galaxy_prefetch_ingest_total"));
    assert!(metrics_prefetch.contains("galaxy_shard_fetch_latency_ms_p50"));
    std::env::remove_var(ENV_HOT_PROMOTE_THRESHOLD);
}
