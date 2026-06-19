//! PH-S567: Galaxy horizon wire integration band (PH-S558…S566).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_capability_admission::{
    check_telegram_edge_capability_admission, record_peer_capabilities,
    record_raid_artifact_probe_success, reset_peer_capabilities_for_test,
    reset_probe_success_for_test,
};
use poolai::grid::galaxy_locality::{
    rank_workers_by_locality, reset_network_profile_stale_metrics_for_test, LocalityHotTier,
    LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
};
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    record_payout_batch_ledger_entry, reset_last_payout_batch_ledger_entry_for_test,
    reset_settlement_metrics_for_test,
};
use poolai::grid::{GridEnvelope, GridJobBody, GridMessage};
use poolai::job::JobStore;
use poolai::memory::MemoryShardStore;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::virtual_node_telegram_wallet_service::{
    VirtualNodeTelegramWalletService, ENV_WALLET_VERIFY_DEVNET,
};
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

#[tokio::test]
async fn horizon_s558_band_routing_wallet_capability_metrics_ph_s567() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_probe_success_for_test();
    reset_peer_capabilities_for_test();
    reset_network_profile_stale_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();

    std::env::set_var(ENV_WALLET_VERIFY_DEVNET, "1");
    let uid = format!("ph-s567-{}", uuid::Uuid::new_v4());
    let wallet = VirtualNodeTelegramWalletService::bind(
        &uid,
        "chat-1",
        "7EqQdE8uK9V3mN2pL4qR5sT6uV7wX8yZ9aB1cD2eF3",
        None,
    )
    .expect("wallet bind");
    assert!(wallet.verified);

    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry {
        job_id: "horizon-s558-job".into(),
        cleared_at: "2026-06-19T12:00:00Z".into(),
        primary_dev_lamports: Some(100),
        secondary_admin_lamports: Some(200),
        payout_pubkey: Some(wallet.payout_pubkey),
        ..PayoutBatchLedgerEntry::minimal("", "")
    });

    record_raid_artifact_probe_success("tg-edge");
    record_peer_capabilities("tg-edge", &["gpu_passthrough".into()]);
    check_telegram_edge_capability_admission(Some("tg-edge"), "inference:gpu").expect("gpu admit");

    let workers = vec![LocalityWorker {
        worker_id: "w-stale".into(),
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec!["s1".into()],
            hot_tier: LocalityHotTier::default(),
            local_replica_regions: vec![],
        },
        network_profile: LocalityNetworkProfile {
            region: "eu".into(),
            latency_ms_p50: 10,
            profile_age_secs: None,
        },
        queue_depth: 0,
        pricing_usd_micro: None,
    }];
    let task = LocalityTask {
        required_shard_ids: vec!["s1".into()],
        task_profile: "inference".into(),
        estimated_cross_region_egress_mb: 1.0,
        source_region: Some("us".into()),
    };
    rank_workers_by_locality(&workers, &task);

    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);
    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "horizon-s567-job".into(),
            task_kind: "inference".into(),
            verification_policy: None,
            input_artifact_ids: vec![],
            required_shard_ids: vec![],
            deadline: None,
        }),
        Some("tg-edge".into()),
    );
    poolai::grid::ingest_envelope(job_env, &jobs, &memory).expect("job ingest");

    let app = grid_app();
    let (_, metrics) = get_text(&app, "/metrics").await;
    assert!(metrics.contains("galaxy_network_profile_stale_total"));

    std::env::remove_var(ENV_WALLET_VERIFY_DEVNET);
    reset_probe_success_for_test();
    reset_peer_capabilities_for_test();
    reset_network_profile_stale_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_last_payout_batch_ledger_entry_for_test();
}
