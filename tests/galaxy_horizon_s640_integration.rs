//! PH-S649: Galaxy horizon close band (PH-S640…S648).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::ENV_LOCALITY_MODE;
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_strict_mode_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_replay_metrics::{
    replay_pending_resolved_total, reset_replay_pending_metrics_for_test,
    verification_replay_record_total,
};
use poolai::grid::galaxy_settlement_metrics::{
    reset_settlement_metrics_for_test, settlement_resolved_total,
};
use poolai::grid::galaxy_trust_score::{
    payout_eligible_total, reset_settlement_gate_metrics_for_test,
};
use poolai::grid::galaxy_verification_metrics::{
    reset_verification_metrics_for_test, verification_checker_enqueue_total,
};
use poolai::grid::galaxy_verify_sampling::{
    reset_verify_sampling_metrics_for_test, ENV_VERIFY_BASE_SAMPLE_RATE,
};
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
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

async fn grid_job_result(
    app: &Router,
    job_id: &str,
    peer: &str,
    metrics: Value,
) -> (StatusCode, Value) {
    let (job_status, _) = request_json(
        app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:00Z",
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-in"],
            "source_peer_id": peer
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) = request_json(app, "GET", &format!("/api/v1/jobs/{job_id}"), None).await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().expect("epoch");
    request_json(
        app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:01Z",
            "type": "result",
            "job_id": job_id,
            "status": "completed",
            "output_artifact_ids": ["out"],
            "lease_epoch": epoch,
            "source_peer_id": peer,
            "metrics": metrics
        })),
    )
    .await
}

#[tokio::test]
async fn horizon_s640_band_replay_verify_settlement_prefetch_ph_s649() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_replay_pending_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_settlement_gate_metrics_for_test();
    reset_verify_sampling_metrics_for_test();
    reset_verification_metrics_for_test();
    reset_prefetch_metrics_for_test();

    let app = grid_app().await;

    // PH-S640: replay_verdict accepted → resolved metric.
    let job_replay = format!("ph-s640-replay-{}", uuid::Uuid::new_v4());
    reset_replay_pending_metrics_for_test();
    let prior_resolved = replay_pending_resolved_total();
    let (replay_status, _) = grid_job_result(
        &app,
        &job_replay,
        "srv1-worker-a",
        json!({ "replay_verdict": "accepted" }),
    )
    .await;
    assert_eq!(replay_status, StatusCode::OK);
    assert!(replay_pending_resolved_total() > prior_resolved);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_replay_pending_resolved_total"));

    // PH-S641: mismatch → replay record + history API.
    reset_replay_pending_metrics_for_test();
    let job_record = format!("ph-s641-record-{}", uuid::Uuid::new_v4());
    let prior_records = verification_replay_record_total();
    let (record_status, _) = grid_job_result(
        &app,
        &job_record,
        "srv1-worker-b",
        json!({ "verification_verdict": "mismatch" }),
    )
    .await;
    assert_eq!(record_status, StatusCode::OK);
    assert!(verification_replay_record_total() > prior_records);
    let (hist_status, hist_body) = request_json(
        &app,
        "GET",
        "/api/v1/grid/verification-replay/history?limit=5",
        None,
    )
    .await;
    assert_eq!(hist_status, StatusCode::OK);
    assert!(hist_body["records"]
        .as_array()
        .is_some_and(|a| !a.is_empty()));

    // PH-S642: telegram_edge sampled → checker enqueue.
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "1");
    reset_verification_metrics_for_test();
    let job_checker = format!("ph-s642-checker-{}", uuid::Uuid::new_v4());
    let prior_checker = verification_checker_enqueue_total();
    let (checker_status, _) = grid_job_result(
        &app,
        &job_checker,
        "tg-edge-checker",
        json!({ "trust_score": 75 }),
    )
    .await;
    assert_eq!(checker_status, StatusCode::OK);
    assert!(verification_checker_enqueue_total() > prior_checker);
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);

    // PH-S643: high-trust telegram_edge → payout eligible.
    reset_settlement_gate_metrics_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
    let job_eligible = format!("ph-s643-eligible-{}", uuid::Uuid::new_v4());
    let prior_eligible = payout_eligible_total();
    let (eligible_status, _) = grid_job_result(
        &app,
        &job_eligible,
        "tg-edge-eligible",
        json!({ "trust_score": 80, "verification_verdict": "match" }),
    )
    .await;
    assert_eq!(eligible_status, StatusCode::OK);
    assert!(payout_eligible_total() > prior_eligible);
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);

    // PH-S644: any grid result → settlement resolved.
    reset_settlement_metrics_for_test();
    let job_resolved = format!("ph-s644-resolved-{}", uuid::Uuid::new_v4());
    let prior_resolved_settlement = settlement_resolved_total();
    let (resolved_status, _) = grid_job_result(
        &app,
        &job_resolved,
        "srv1-worker-a",
        json!({ "trust_score": 50 }),
    )
    .await;
    assert_eq!(resolved_status, StatusCode::OK);
    assert!(settlement_resolved_total() > prior_resolved_settlement);
    assert!(metrics_text(&app)
        .await
        .contains("galaxy_settlement_resolved_total"));

    // PH-S645: strict_locality job ingest → prefetch strict-mode metric.
    reset_prefetch_metrics_for_test();
    std::env::set_var(ENV_LOCALITY_MODE, "strict_locality");
    MemoryShardStore::global()
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-1"),
            artifact_id: "artifact-emb".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("seed w:emb-1");
    let job_strict = format!("ph-s645-strict-{}", uuid::Uuid::new_v4());
    let prior_strict = prefetch_strict_mode_total();
    let (strict_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:10Z",
            "type": "job",
            "job_id": job_strict,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:emb-1"],
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(strict_status, StatusCode::OK);
    assert!(prefetch_strict_mode_total() > prior_strict);
    std::env::remove_var(ENV_LOCALITY_MODE);
}
