//! PH-S619: Galaxy horizon close band (PH-S610…S618).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{order_shards_by_access_weight, ENV_SHARD_ACCESS_WEIGHTS};
use poolai::grid::galaxy_locality::{
    hot_tier_gate_applied_total, pick_best_worker_by_locality_with_hot_tier_gate,
    reset_hot_tier_gate_metrics_for_test, LocalityHotTier, LocalityNetworkProfile,
    LocalitySeedInventory, LocalityTask, LocalityWorker, METRIC_HOT_TIER_GATE_APPLIED_TOTAL,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_re_migrate_total, reset_prefetch_metrics_for_test, METRIC_PREFETCH_RE_MIGRATE_TOTAL,
    METRIC_SHARD_ACCESS_TOTAL,
};
use poolai::grid::galaxy_replication_metrics::{
    replication_rate_limited_total, reset_replication_strict_metrics_for_test,
    ENV_REPLICATION_MAX_PER_HOUR, METRIC_REPLICATION_RATE_LIMITED_TOTAL,
};
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
    reset_settlement_metrics_for_test,
};
use poolai::grid::galaxy_trust_score::{trust_score_delta_total, METRIC_TRUST_SCORE_DELTA_TOTAL};
use poolai::grid::galaxy_trust_score_store::{
    lookup_peer_trust_score, reset_trust_score_store_for_test,
};
use poolai::grid::galaxy_verify_sampling::{
    checker_timeout_inconclusive_total, checker_timeout_retry_total, ENV_VERIFY_BASE_SAMPLE_RATE,
    METRIC_VERIFY_CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL, METRIC_VERIFY_CHECKER_TIMEOUT_RETRY_TOTAL,
};
use poolai::grid::galaxy_worker_health::{
    on_heartbeat_miss, reset_worker_health_for_test, ENV_HEARTBEAT_UNHEALTHY_THRESHOLD,
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
async fn horizon_s610_band_trust_hot_tier_prefetch_payout_ph_s619() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_trust_score_store_for_test();
    reset_worker_health_for_test();
    reset_hot_tier_gate_metrics_for_test();
    reset_prefetch_metrics_for_test();
    reset_replication_strict_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();

    let app = grid_app();
    let peer = "tg-edge-s610";

    // PH-S610: stale-epoch grid result → trust delta −50.
    reset_trust_score_store_for_test();
    poolai::grid::galaxy_trust_score_store::persist_peer_trust_score(peer, 80);
    let job_stale = format!("ph-s610-stale-{}", uuid::Uuid::new_v4());
    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:00Z",
            "type": "job",
            "job_id": job_stale,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-stale"],
            "source_peer_id": peer
        })),
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_stale}"), None).await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_delta = trust_score_delta_total();
    let (reject_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:01Z",
            "type": "result",
            "job_id": job_stale,
            "status": "completed",
            "output_artifact_ids": ["out-stale"],
            "lease_epoch": epoch + 99,
            "source_peer_id": peer,
            "metrics": { "trust_score": 80 }
        })),
    )
    .await;
    assert_eq!(reject_status, StatusCode::CONFLICT);
    assert!(trust_score_delta_total() > prior_delta);
    let adjusted = lookup_peer_trust_score(peer).expect("stored");
    assert_eq!(adjusted, 30);

    // PH-S611: unhealthy streak → trust delta −30.
    reset_trust_score_store_for_test();
    poolai::grid::galaxy_trust_score_store::persist_peer_trust_score("peer-unhealthy", 70);
    std::env::set_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD, "2");
    assert!(!on_heartbeat_miss("peer-unhealthy"));
    assert!(on_heartbeat_miss("peer-unhealthy"));
    assert_eq!(lookup_peer_trust_score("peer-unhealthy"), Some(40));

    // PH-S612: hot-tier gate prefers high-hit worker.
    let hot_worker = LocalityWorker {
        worker_id: "hot-worker".into(),
        queue_depth: 0,
        pricing_usd_micro: None,
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec!["w:emb-1".into()],
            hot_tier: LocalityHotTier {
                ram_bytes_used: 1_000_000,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            local_replica_regions: vec!["eu-west".into()],
        },
        network_profile: LocalityNetworkProfile {
            region: "eu-west".into(),
            latency_ms_p50: 20,
            latency_ms_p95: None,
            profile_age_secs: Some(0),
        },
    };
    let cold_worker = LocalityWorker {
        worker_id: "cold-worker".into(),
        queue_depth: 0,
        pricing_usd_micro: None,
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec![],
            hot_tier: LocalityHotTier {
                ram_bytes_used: 0,
                vram_bytes_used: 0,
                profiles: vec!["other-profile".into()],
            },
            local_replica_regions: vec![],
        },
        network_profile: LocalityNetworkProfile {
            region: "eu-west".into(),
            latency_ms_p50: 5,
            latency_ms_p95: None,
            profile_age_secs: Some(0),
        },
    };
    let task = LocalityTask {
        required_shard_ids: vec!["w:emb-1".into()],
        task_profile: "inference:text".into(),
        estimated_cross_region_egress_mb: 0.0,
        source_region: None,
    };
    let workers = [cold_worker, hot_worker];
    let picked = pick_best_worker_by_locality_with_hot_tier_gate(&workers, &task).expect("pick");
    assert_eq!(picked.worker_id, "hot-worker");
    assert!(hot_tier_gate_applied_total() >= 1);

    // PH-S614: access-weight ordering.
    std::env::set_var(ENV_SHARD_ACCESS_WEIGHTS, r#"{"w:low":1,"w:high":99}"#);
    let ordered = order_shards_by_access_weight(&["w:low".into(), "w:high".into(), "w:mid".into()]);
    assert_eq!(ordered[0], "w:high");
    std::env::remove_var(ENV_SHARD_ACCESS_WEIGHTS);

    // PH-S613 + PH-S605-style: re-migrate with memory delta-fetch + shard access on ingest.
    let migrate_shard = format!("w:migrate-s613-{}", uuid::Uuid::new_v4());
    MemoryShardStore::global()
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new(&migrate_shard),
            artifact_id: "artifact-migrate".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("seed shard in memory");
    reset_prefetch_metrics_for_test();
    let job_migrate = format!("ph-s613-{}", uuid::Uuid::new_v4());
    let (m_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:02Z",
            "type": "job",
            "job_id": job_migrate,
            "task_kind": "inference:text",
            "required_shard_ids": [migrate_shard.clone(), format!("w:missing-{}", uuid::Uuid::new_v4())],
            "input_artifact_ids": [],
            "source_peer_id": "srv1-worker-a"
        })),
    )
    .await;
    assert_eq!(m_job_status, StatusCode::OK);
    let (_, m_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_migrate}"), None).await;
    let m_epoch = m_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let (to_migrating, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{job_migrate}"),
        Some(json!({ "status": "migrating", "lease_epoch": m_epoch })),
    )
    .await;
    assert_eq!(to_migrating, StatusCode::OK);
    reset_prefetch_metrics_for_test();
    let (to_leased, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{job_migrate}"),
        Some(json!({ "status": "leased", "lease_epoch": m_epoch })),
    )
    .await;
    assert_eq!(to_leased, StatusCode::OK);
    assert!(prefetch_re_migrate_total() >= 1);

    // PH-S615: replication hourly cap via HTTP ingest.
    std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, "1");
    reset_replication_strict_metrics_for_test();
    let strict_job = |suffix: &str| {
        json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:03Z",
            "type": "job",
            "job_id": format!("ph-s615-{suffix}-{}", uuid::Uuid::new_v4()),
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

    // PH-S616: payout-batch primary/secondary/worker lamports split.
    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "ph-s616-job".into(),
        cleared_at: "2026-06-20T12:00:04Z".into(),
        gross_lamports: Some(1_000_000),
        primary_dev_lamports: Some(1_000),
        secondary_admin_lamports: Some(10_000),
        worker_lamports: Some(989_000),
        ..PayoutBatchLedgerEntry::minimal("", "")
    });
    let (payout_status, payout_body) =
        request_json(&app, "GET", "/api/v1/grid/payout-batch", None).await;
    assert_eq!(payout_status, StatusCode::OK);
    assert_eq!(
        payout_body["routing"]["primary_dev_lamports"].as_u64(),
        Some(1_000)
    );
    assert_eq!(
        payout_body["routing"]["secondary_admin_lamports"].as_u64(),
        Some(10_000)
    );
    assert_eq!(
        payout_body["routing"]["worker_lamports"].as_u64(),
        Some(989_000)
    );

    // PH-S617: checker-timeout grid result path.
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
    let job_timeout = format!("ph-s617-{}", uuid::Uuid::new_v4());
    let (t_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:05Z",
            "type": "job",
            "job_id": job_timeout,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-timeout"],
            "source_peer_id": "tg-edge-timeout"
        })),
    )
    .await;
    assert_eq!(t_job_status, StatusCode::OK);
    let (_, t_get) = request_json(&app, "GET", &format!("/api/v1/jobs/{job_timeout}"), None).await;
    let t_epoch = t_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let prior_retry = checker_timeout_retry_total();
    let (retry_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:06Z",
            "type": "result",
            "job_id": job_timeout,
            "status": "completed",
            "output_artifact_ids": ["out-timeout"],
            "lease_epoch": t_epoch,
            "source_peer_id": "tg-edge-timeout",
            "metrics": {
                "trust_score": 75,
                "checker_timeout": true,
                "checker_retry_count": 0
            }
        })),
    )
    .await;
    assert_eq!(retry_status, StatusCode::OK);
    assert!(checker_timeout_retry_total() > prior_retry);
    let (incon_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-20T12:00:07Z",
            "type": "result",
            "job_id": job_timeout,
            "status": "completed",
            "output_artifact_ids": ["out-timeout-2"],
            "lease_epoch": t_epoch,
            "source_peer_id": "tg-edge-timeout",
            "metrics": {
                "trust_score": 75,
                "checker_timeout": true,
                "checker_retry_count": 1
            }
        })),
    )
    .await;
    assert_eq!(incon_status, StatusCode::OK);
    assert!(checker_timeout_inconclusive_total() >= 1);

    let metrics = metrics_text(&app).await;
    for name in [
        METRIC_TRUST_SCORE_DELTA_TOTAL,
        METRIC_HOT_TIER_GATE_APPLIED_TOTAL,
        METRIC_PREFETCH_RE_MIGRATE_TOTAL,
        METRIC_REPLICATION_RATE_LIMITED_TOTAL,
        METRIC_SHARD_ACCESS_TOTAL,
        METRIC_VERIFY_CHECKER_TIMEOUT_RETRY_TOTAL,
        METRIC_VERIFY_CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL,
    ] {
        assert!(metrics.contains(name), "missing {name}");
    }

    std::env::remove_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD);
    std::env::remove_var(ENV_REPLICATION_MAX_PER_HOUR);
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    reset_trust_score_store_for_test();
    reset_worker_health_for_test();
    reset_hot_tier_gate_metrics_for_test();
    reset_prefetch_metrics_for_test();
    reset_replication_strict_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
}
