//! PH-S639: Galaxy horizon close band (PH-S630…S638).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_seed_pull_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_replay_metrics::{
    replay_verification_enqueue_total, reset_replay_pending_metrics_for_test,
};
use poolai::grid::galaxy_replication_metrics::{
    replication_executor_enqueue_total, reset_replication_strict_metrics_for_test,
};
use poolai::grid::galaxy_settlement_metrics::{
    reset_settlement_metrics_for_test, settlement_payout_batch_total,
};
use poolai::grid::galaxy_trust_score::trust_score_delta_total;
use poolai::grid::galaxy_trust_score_store::{
    lookup_peer_trust_score, reset_trust_score_store_for_test,
};
use poolai::grid::galaxy_verify_sampling::{
    reset_verify_sampling_metrics_for_test, ENV_VERIFY_BASE_SAMPLE_RATE,
};
use poolai::grid::galaxy_worker_health::{
    galaxy_worker_unhealthy_total, reset_worker_health_for_test, ENV_HEARTBEAT_UNHEALTHY_THRESHOLD,
};
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

async fn grid_app() -> Router {
    observability::init_prometheus();
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18080));
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        addr,
        None,
    ));
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ctx)
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
async fn horizon_s630_band_trust_settlement_prefetch_health_ph_s639() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_trust_score_store_for_test();
    reset_settlement_metrics_for_test();
    reset_prefetch_metrics_for_test();
    reset_replication_strict_metrics_for_test();
    reset_replay_pending_metrics_for_test();
    reset_worker_health_for_test();
    reset_verify_sampling_metrics_for_test();

    let app = grid_app().await;
    let peer = "tg-edge-s630";

    // PH-S630: verification mismatch trust delta persists to store (−100 from stored 80 → 0).
    poolai::grid::galaxy_trust_score_store::persist_peer_trust_score(peer, 80);
    let job_mismatch = format!("ph-s630-mismatch-{}", uuid::Uuid::new_v4());
    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:00Z",
            "type": "job",
            "job_id": job_mismatch,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-mismatch"],
            "source_peer_id": peer
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) =
        request_json(&app, "GET", &format!("/api/v1/jobs/{job_mismatch}"), None).await;
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
            "job_id": job_mismatch,
            "status": "completed",
            "output_artifact_ids": ["out-mismatch"],
            "lease_epoch": epoch,
            "source_peer_id": peer,
            "metrics": { "verification_verdict": "mismatch" }
        })),
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert!(trust_score_delta_total() > prior_delta);
    assert_eq!(lookup_peer_trust_score(peer), Some(0));

    // PH-S631: cleared settlement → payout-batch metric on /metrics.
    reset_settlement_metrics_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
    let job_cleared = format!("ph-s631-cleared-{}", uuid::Uuid::new_v4());
    let (c_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:02Z",
            "type": "job",
            "job_id": job_cleared,
            "task_kind": "inference:text",
            "input_artifact_ids": [],
            "source_peer_id": "tg-edge-cleared"
        })),
    )
    .await;
    assert_eq!(c_job_status, StatusCode::OK);
    let (_, c_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_cleared}"), None).await;
    let c_epoch = c_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_payout = settlement_payout_batch_total();
    let (c_result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:03Z",
            "type": "result",
            "job_id": job_cleared,
            "status": "completed",
            "output_artifact_ids": [],
            "lease_epoch": c_epoch,
            "source_peer_id": "tg-edge-cleared",
            "metrics": { "trust_score": 80, "verification_verdict": "match" }
        })),
    )
    .await;
    assert_eq!(c_result_status, StatusCode::OK);
    assert!(settlement_payout_batch_total() > prior_payout);
    let metrics = metrics_text(&app).await;
    assert!(metrics.contains("galaxy_settlement_payout_batch_total"));
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);

    // PH-S632: prefetch seed-pull on job ingest (stub inventory shard w:emb-1).
    reset_prefetch_metrics_for_test();
    let job_seed = format!("ph-s632-seed-{}", uuid::Uuid::new_v4());
    let prior_seed_pull = prefetch_seed_pull_total();
    let (seed_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:04Z",
            "type": "job",
            "job_id": job_seed,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:emb-1"],
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(seed_job_status, StatusCode::OK);
    assert!(prefetch_seed_pull_total() > prior_seed_pull);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_prefetch_seed_pull_total"));

    // PH-S633: replication executor enqueue on grid job ingest.
    reset_replication_strict_metrics_for_test();
    let prior_rep = replication_executor_enqueue_total();
    let rep_job = format!("ph-s633-rep-{}", uuid::Uuid::new_v4());
    let (rep_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:05Z",
            "type": "job",
            "job_id": rep_job,
            "task_kind": "inference:text",
            "verification_policy": "replication_strict",
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(rep_status, StatusCode::OK);
    assert!(replication_executor_enqueue_total() > prior_rep);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_replication_executor_enqueue_total"));

    // PH-S634: replay verification enqueue on mismatch.
    reset_replay_pending_metrics_for_test();
    let job_replay = format!("ph-s634-replay-{}", uuid::Uuid::new_v4());
    let (r_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:06Z",
            "type": "job",
            "job_id": job_replay,
            "task_kind": "inference:text",
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-b"
        })),
    )
    .await;
    assert_eq!(r_job_status, StatusCode::OK);
    let (_, r_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_replay}"), None).await;
    let r_epoch = r_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_replay = replay_verification_enqueue_total();
    let (r_result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:07Z",
            "type": "result",
            "job_id": job_replay,
            "status": "completed",
            "output_artifact_ids": [],
            "lease_epoch": r_epoch,
            "source_peer_id": "srv1-worker-b",
            "metrics": { "verification_verdict": "mismatch" }
        })),
    )
    .await;
    assert_eq!(r_result_status, StatusCode::OK);
    assert!(replay_verification_enqueue_total() > prior_replay);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_replay_verification_enqueue_total"));

    // PH-S635: worker-unhealthy via consecutive heartbeat-remote misses.
    reset_worker_health_for_test();
    std::env::set_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD, "3");
    let prior_unhealthy = galaxy_worker_unhealthy_total();
    for _ in 0..3 {
        let (hb_status, _) = request_json(
            &app,
            "POST",
            "/api/v1/discovery/heartbeat-remote",
            Some(json!({ "peer_id": "missing-peer-s635" })),
        )
        .await;
        assert_eq!(hb_status, StatusCode::NOT_FOUND);
    }
    assert!(galaxy_worker_unhealthy_total() > prior_unhealthy);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_worker_unhealthy_total"));
    std::env::remove_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD);
}
