//! PH-S609: Galaxy horizon close band (PH-S600…S608).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    prefetch_topology_admission_blocked_skip, with_prefetch_peer, ENV_LOCALITY_MODE,
    ENV_PREFETCH_COORDINATOR_REGION, ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING,
    ENV_PREFETCH_DEADLINE_MS,
};
use poolai::grid::galaxy_fraud_proof::{
    fraud_proof_pending_total, reset_fraud_proof_metrics_for_test, ENV_FRAUD_PROOF,
};
use poolai::grid::galaxy_locality::{
    reset_tail_latency_penalty_metrics_for_test, tail_latency_penalty_total,
    METRIC_TAIL_LATENCY_PENALTY_TOTAL,
};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_metrics::{
    locality_unsatisfied_total, prefetch_raid_fetch_total, prefetch_re_migrate_total,
    prefetch_timeout_total, prefetch_topology_blocked_total, reset_prefetch_metrics_for_test,
    METRIC_LOCALITY_UNSATISFIED_TOTAL, METRIC_PREFETCH_RAID_FETCH_TOTAL,
    METRIC_PREFETCH_RE_MIGRATE_TOTAL, METRIC_PREFETCH_TOPOLOGY_BLOCKED_TOTAL,
};
use poolai::grid::galaxy_settlement_metrics::{
    reset_settlement_metrics_for_test, settlement_human_review_total,
    METRIC_SETTLEMENT_HUMAN_REVIEW_TOTAL,
};
use poolai::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE;
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::virtual_node_telegram_wallet_service::{
    reset_wallet_rebind_override_for_test, VirtualNodeTelegramWalletService,
    ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS,
};
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
    auth: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = auth {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
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
async fn horizon_s600_band_strict_locality_wallet_topology_ph_s609() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_fraud_proof_metrics_for_test();
    reset_tail_latency_penalty_metrics_for_test();
    VirtualNodeTelegramWalletService::clear_all();
    reset_wallet_rebind_override_for_test();

    let app = grid_app();

    // PH-S600: strict_locality rejects cold shards over HTTP.
    std::env::set_var(ENV_LOCALITY_MODE, "strict_locality");
    let job_cold = format!("ph-s600-cold-{}", uuid::Uuid::new_v4());
    let (cold_status, cold_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:00Z",
            "type": "job",
            "job_id": job_cold,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:missing-ph-s600"],
            "input_artifact_ids": []
        })),
        None,
    )
    .await;
    assert_eq!(cold_status, StatusCode::CONFLICT, "cold job: {cold_body}");
    assert_eq!(
        cold_body["error"]["code"].as_str(),
        Some("locality_unsatisfied")
    );
    assert!(locality_unsatisfied_total() >= 1);

    // PH-S600: prefetch-timeout when hot in inventory but absent from memory.
    std::env::set_var(ENV_PREFETCH_DEADLINE_MS, "1");
    let job_timeout = format!("ph-s600-timeout-{}", uuid::Uuid::new_v4());
    let (timeout_status, timeout_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:01Z",
            "type": "job",
            "job_id": job_timeout,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:emb-1"],
            "input_artifact_ids": []
        })),
        None,
    )
    .await;
    assert_eq!(
        timeout_status,
        StatusCode::CONFLICT,
        "timeout job: {timeout_body}"
    );
    assert_eq!(
        timeout_body["error"]["code"].as_str(),
        Some("prefetch-timeout")
    );
    assert!(prefetch_timeout_total() >= 1);
    std::env::remove_var(ENV_PREFETCH_DEADLINE_MS);

    // PH-S601: semantic_hash human-review hold on grid result HTTP.
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
    let job_review = format!("ph-s601-{}", uuid::Uuid::new_v4());
    std::env::remove_var(ENV_LOCALITY_MODE);
    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:02Z",
            "type": "job",
            "job_id": job_review,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-review"],
            "source_peer_id": "tg-edge"
        })),
        None,
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) = request_json(
        &app,
        "GET",
        &format!("/api/v1/jobs/{job_review}"),
        None,
        None,
    )
    .await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().expect("epoch");
    let (result_status, result_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:03Z",
            "type": "result",
            "job_id": job_review,
            "status": "completed",
            "output_artifact_ids": ["out-review"],
            "lease_epoch": epoch,
            "source_peer_id": "tg-edge",
            "metrics": {
                "trust_score": 88,
                "task_profile": "non_deterministic",
                "expected_semantic_hash": "abc123",
                "semantic_hash": "other-hash"
            }
        })),
        None,
    )
    .await;
    assert_eq!(result_status, StatusCode::OK, "result: {result_body}");
    assert!(settlement_human_review_total() >= 1);
    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);

    // PH-S602: wallet rebind cooldown (non-admin path).
    std::env::set_var(ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS, "86400");
    let pk1 = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";
    let pk2 = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
    VirtualNodeTelegramWalletService::bind("900600", "-100600", pk1, None).expect("bind");
    let (rebind_status, rebind_body) = request_json(
        &app,
        "POST",
        "/api/v1/virtual-nodes/telegram/wallet",
        Some(json!({
            "telegram_user_id": "900600",
            "chat_id": "-100600",
            "payout_pubkey": pk2,
            "chain": "solana"
        })),
        None,
    )
    .await;
    assert_eq!(rebind_status, StatusCode::CONFLICT);
    assert_eq!(
        rebind_body["error"].as_str(),
        Some("wallet_rebind_cooldown")
    );

    // PH-S603: tail latency penalty via persisted profile + job ingest.
    persist_peer_network_profile(
        "srv1-worker-a",
        r#"{"region":"eu-west","latency_ms_p50":20,"latency_ms_p95":120}"#,
    )
    .expect("persist p95 profile");
    let job_p95 = format!("ph-s603-{}", uuid::Uuid::new_v4());
    let (p95_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:04Z",
            "type": "job",
            "job_id": job_p95,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:emb-1"],
            "source_peer_id": "srv1-worker-a",
            "input_artifact_ids": []
        })),
        None,
    )
    .await;
    assert_eq!(p95_status, StatusCode::OK);
    assert!(tail_latency_penalty_total() >= 1);

    // PH-S604: topology / white-IP admission guard via persisted profile.
    std::env::set_var(ENV_PREFETCH_COORDINATOR_REGION, "eu-west");
    std::env::set_var(ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING, "ring-a");
    let peer_topo = format!("peer-topo-{}", uuid::Uuid::new_v4());
    persist_peer_network_profile(
        &peer_topo,
        r#"{"region":"us-east","latency_ms_p50":40,"white_ip_only":true,"topology_ring":"ring-b"}"#,
    )
    .expect("persist topo profile");
    reset_prefetch_metrics_for_test();
    assert!(with_prefetch_peer(
        Some(&peer_topo),
        prefetch_topology_admission_blocked_skip
    ));
    let job_topo = format!("ph-s604-{}", uuid::Uuid::new_v4());
    let (topo_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:05Z",
            "type": "job",
            "job_id": job_topo,
            "task_kind": "inference:text",
            "required_shard_ids": ["w:topo-cold-s604"],
            "source_peer_id": peer_topo,
            "input_artifact_ids": []
        })),
        None,
    )
    .await;
    assert_eq!(topo_status, StatusCode::OK);
    assert!(prefetch_topology_blocked_total() >= 1);

    // PH-S605: RAID prefetch fetch on grid job ingest HTTP.
    let raid_shard = format!("w:raid-s605-{}", uuid::Uuid::new_v4());
    MemoryShardStore::global()
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new(&raid_shard),
            artifact_id: "artifact-raid".into(),
            version: "v1".into(),
            raid_logical_name: Some(raid_shard.clone()),
            seed_hints: None,
        })
        .expect("raid shard");
    reset_prefetch_metrics_for_test();
    let job_raid = format!("ph-s605-{}", uuid::Uuid::new_v4());
    let (raid_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:06Z",
            "type": "job",
            "job_id": job_raid,
            "task_kind": "inference:text",
            "required_shard_ids": [raid_shard],
            "source_peer_id": "srv1-worker-a",
            "input_artifact_ids": []
        })),
        None,
    )
    .await;
    assert_eq!(raid_status, StatusCode::OK);
    assert!(prefetch_raid_fetch_total() >= 1);

    // PH-S606: re-migrate prefetch on Migrating→Leased PATCH HTTP.
    let job_migrate = format!("ph-s606-{}", uuid::Uuid::new_v4());
    let (m_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:07Z",
            "type": "job",
            "job_id": job_migrate,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-migrate"],
            "source_peer_id": "srv1-worker-a"
        })),
        None,
    )
    .await;
    assert_eq!(m_job_status, StatusCode::OK);
    let (_, m_get) = request_json(
        &app,
        "GET",
        &format!("/api/v1/jobs/{job_migrate}"),
        None,
        None,
    )
    .await;
    let m_epoch = m_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let (to_migrating, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{job_migrate}"),
        Some(json!({ "status": "migrating", "lease_epoch": m_epoch })),
        None,
    )
    .await;
    assert_eq!(to_migrating, StatusCode::OK);
    reset_prefetch_metrics_for_test();
    let (to_leased, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{job_migrate}"),
        Some(json!({ "status": "leased", "lease_epoch": m_epoch })),
        None,
    )
    .await;
    assert_eq!(to_leased, StatusCode::OK);
    assert!(prefetch_re_migrate_total() >= 1);

    // PH-S607: fraud-proof hold via HTTP grid envelope.
    std::env::set_var(ENV_FRAUD_PROOF, "1");
    let job_fraud = format!("ph-s607-{}", uuid::Uuid::new_v4());
    let (f_job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:08Z",
            "type": "job",
            "job_id": job_fraud,
            "task_kind": "inference:text",
            "input_artifact_ids": ["artifact-fraud"],
            "source_peer_id": "srv1-worker-a"
        })),
        None,
    )
    .await;
    assert_eq!(f_job_status, StatusCode::OK);
    let (_, f_get) = request_json(
        &app,
        "GET",
        &format!("/api/v1/jobs/{job_fraud}"),
        None,
        None,
    )
    .await;
    let f_epoch = f_get["job"]["lease_epoch"].as_u64().expect("epoch");
    let (f_result_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:09Z",
            "type": "result",
            "job_id": job_fraud,
            "status": "completed",
            "output_artifact_ids": ["out-fraud"],
            "lease_epoch": f_epoch,
            "source_peer_id": "srv1-worker-a",
            "metrics": {
                "trust_score": 90,
                "verification_verdict": "mismatch"
            }
        })),
        None,
    )
    .await;
    assert_eq!(f_result_status, StatusCode::OK);
    assert!(fraud_proof_pending_total() >= 1);

    let metrics = metrics_text(&app).await;
    for name in [
        METRIC_LOCALITY_UNSATISFIED_TOTAL,
        METRIC_SETTLEMENT_HUMAN_REVIEW_TOTAL,
        METRIC_TAIL_LATENCY_PENALTY_TOTAL,
        METRIC_PREFETCH_TOPOLOGY_BLOCKED_TOTAL,
        METRIC_PREFETCH_RAID_FETCH_TOTAL,
        METRIC_PREFETCH_RE_MIGRATE_TOTAL,
        "galaxy_fraud_proof_pending_total",
    ] {
        assert!(metrics.contains(name), "missing {name}");
    }

    std::env::remove_var(ENV_LOCALITY_MODE);
    std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
    std::env::remove_var(ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING);
    std::env::remove_var(ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS);
    std::env::remove_var(ENV_FRAUD_PROOF);
    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_fraud_proof_metrics_for_test();
    reset_tail_latency_penalty_metrics_for_test();
    VirtualNodeTelegramWalletService::clear_all();
    reset_wallet_rebind_override_for_test();
}
