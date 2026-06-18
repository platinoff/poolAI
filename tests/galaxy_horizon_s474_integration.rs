//! PH-S481: Galaxy horizon wire integration band (PH-S474…S479 metrics).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    fetch_seed_shards_from_peer_hook, prefetch_egress_blocked_skip, PrefetchPlan, PrefetchPlanItem,
    PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger, ENV_PREFETCH_COORDINATOR_REGION,
    ENV_PREFETCH_PEER_EGRESS_POLICY, ENV_PREFETCH_PEER_REGION,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_egress_blocked_total, prefetch_peer_fetch_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_replay_metrics::{
    emit_verification_replay_record, reset_replay_pending_metrics_for_test,
    verification_replay_history,
};
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement_metrics::{
    payout_batch_history, record_payout_batch_ledger_entry, reset_payout_batch_history_for_test,
    reset_settlement_metrics_for_test,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::telegram_seat_service::{
    reset_telegram_seats_for_test, try_admit_telegram_edge, ENV_TELEGRAM_SEAT_LIMIT,
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
    let text = String::from_utf8(bytes.to_vec()).unwrap_or_default();
    (status, text)
}

#[tokio::test]
async fn metrics_export_horizon_s474_band_ph_s481() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_replay_pending_metrics_for_test();
    reset_telegram_seats_for_test();
    std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
    std::env::remove_var(ENV_PREFETCH_PEER_REGION);
    std::env::remove_var(ENV_PREFETCH_PEER_EGRESS_POLICY);
    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);

    record_payout_batch_ledger_entry(PayoutBatchLedgerEntry::minimal(
        "job-hist-1",
        "2026-06-18T12:00:00Z",
    ));
    emit_verification_replay_record("job-replay-1", None);
    std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "2");
    assert!(try_admit_telegram_edge("tg-peer-1").is_ok());

    std::env::set_var(ENV_PREFETCH_COORDINATOR_REGION, "eu-west");
    std::env::set_var(ENV_PREFETCH_PEER_REGION, "us-east");
    std::env::set_var(ENV_PREFETCH_PEER_EGRESS_POLICY, "lan_only");
    assert!(prefetch_egress_blocked_skip());
    assert!(prefetch_egress_blocked_total() >= 1);

    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "w:emb-1".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 15_000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    assert_eq!(fetch_seed_shards_from_peer_hook(&plan), 1);
    assert_eq!(payout_batch_history(5).len(), 1);
    assert_eq!(verification_replay_history(5).len(), 1);

    assert_eq!(prefetch_peer_fetch_total(), 1);

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    for name in [
        "galaxy_prefetch_egress_blocked_total",
        "galaxy_prefetch_peer_fetch_total",
        "galaxy_prefetch_peer_fetch_miss_total",
    ] {
        assert!(body.contains(name), "missing {name} in metrics body");
        assert!(
            body.contains(&format!("# TYPE {name} gauge")),
            "missing TYPE gauge for {name}"
        );
    }

    let (hist_status, hist_body) =
        get_text(&app, "/api/v1/grid/payout-batch/history?limit=5").await;
    assert_eq!(hist_status, StatusCode::OK);
    assert!(hist_body.contains("job-hist-1"));

    let (replay_status, replay_body) =
        get_text(&app, "/api/v1/grid/verification-replay/history?limit=5").await;
    assert_eq!(replay_status, StatusCode::OK);
    assert!(replay_body.contains("job-replay-1"));

    std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
    std::env::remove_var(ENV_PREFETCH_PEER_REGION);
    std::env::remove_var(ENV_PREFETCH_PEER_EGRESS_POLICY);
    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
    reset_prefetch_metrics_for_test();
    reset_settlement_metrics_for_test();
    reset_payout_batch_history_for_test();
    reset_replay_pending_metrics_for_test();
    reset_telegram_seats_for_test();
}
