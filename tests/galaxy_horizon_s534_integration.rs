//! PH-S543: Galaxy horizon wire integration band (PH-S534…S542).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_capability_admission::{
    record_peer_capabilities, record_raid_artifact_probe_success, reset_peer_capabilities_for_test,
    reset_probe_success_for_test,
};
use poolai::grid::galaxy_replay_jobs::reset_replay_job_submit_for_test;
use poolai::grid::galaxy_settlement::{resolve_payout_pubkey, PayoutBatchLedgerEntry};
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
};
use poolai::grid::galaxy_verification_checker_jobs::{
    reset_verification_checker_job_submit_for_test, verification_checker_job_submit_total,
};
use poolai::grid::{
    coordinator_seed_inventory_snapshot, ingest_envelope, GridEnvelope, GridIngestKind,
    GridJobBody, GridMessage, GridResultBody, GridResultStatus,
};
use poolai::job::JobStore;
use poolai::memory::MemoryShardStore;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::virtual_node_telegram_wallet_service::VirtualNodeTelegramWalletService;
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn get_text(app: &Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        String::from_utf8(bytes.to_vec()).unwrap_or_default(),
    )
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({})),
    )
}

const VALID_PUBKEY: &str = "11111111111111111111111111111112";

#[tokio::test]
async fn horizon_s534_band_verification_settlement_and_prefetch_ph_s543() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_probe_success_for_test();
    reset_peer_capabilities_for_test();
    reset_replay_job_submit_for_test();
    reset_verification_checker_job_submit_for_test();
    reset_last_payout_batch_ledger_entry_for_test();

    std::env::set_var("POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE", "1.0");
    VirtualNodeTelegramWalletService::bind("user-s534", "-100534", VALID_PUBKEY, None)
        .expect("bind wallet");

    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);

    record_raid_artifact_probe_success("tg-edge");
    record_peer_capabilities("tg-edge", &["gpu_passthrough".into()]);

    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "horizon-s534-job".into(),
            task_kind: "inference:gpu".into(),
            verification_policy: Some("replication_strict".into()),
            input_artifact_ids: vec![],
            required_shard_ids: vec!["w:emb-1".into()],
            deadline: None,
        }),
        Some("tg-edge".into()),
    );
    let out = ingest_envelope(job_env, &jobs, &memory).expect("job ingest");
    assert!(matches!(out.kind, GridIngestKind::Job { .. }));
    assert!(jobs.get("horizon-s534-job-rep-0").expect("get").is_some());
    let lease_epoch = jobs
        .get("horizon-s534-job")
        .expect("get")
        .expect("row")
        .lease_epoch;

    let result_env = GridEnvelope::new(
        GridMessage::Result(GridResultBody {
            job_id: "horizon-s534-job".into(),
            status: GridResultStatus::Completed,
            output_artifact_ids: vec![],
            proof: None,
            metrics: Some(serde_json::json!({
                "trust_score": 90,
                "telegram_user_id": "user-s534",
                "gross_lamports": 5000,
                "secondary_admin_bps": 100
            })),
            lease_epoch,
        }),
        Some("tg-edge".into()),
    );
    ingest_envelope(result_env, &jobs, &memory).expect("result ingest");
    assert_eq!(verification_checker_job_submit_total(), 1);
    assert_eq!(
        resolve_payout_pubkey(Some("user-s534")).as_deref(),
        Some(VALID_PUBKEY)
    );

    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "horizon-s534".into(),
        cleared_at: "2026-06-18T22:00:00Z".into(),
        gross_lamports: Some(5000),
        payout_pubkey: Some(VALID_PUBKEY.into()),
        telegram_user_id: Some("user-s534".into()),
        ..PayoutBatchLedgerEntry::minimal("", "")
    });

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "payout-batch: {body}");
    assert_eq!(body["entry"]["payout_pubkey"].as_str(), Some(VALID_PUBKEY));

    let (status, metrics) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(metrics.contains("galaxy_verification_checker_job_submit_total"));
    assert!(metrics.contains("galaxy_prefetch_peer_fetch_total"));
    assert!(!coordinator_seed_inventory_snapshot().is_empty());

    std::env::remove_var("POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE");
    reset_probe_success_for_test();
    reset_peer_capabilities_for_test();
    reset_replay_job_submit_for_test();
    reset_verification_checker_job_submit_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
}
