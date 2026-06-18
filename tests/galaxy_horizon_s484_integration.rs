//! PH-S491: Galaxy horizon wire integration band (PH-S484…S489 metrics).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    complete_prefetch_hook, fetch_seed_shards_hook, ingest_job_locality_rank_stub, PrefetchPlan,
    PrefetchPlanItem, PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
};
use poolai::grid::galaxy_network_profile_store::{
    load_peer_network_profile, persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_pull_bytes_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_verification_metrics::{
    enqueue_verification_checker_task, reset_verification_checker_tasks_for_test,
    reset_verification_metrics_for_test, verification_checker_tasks,
};
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::services::telegram_seat_service::{
    compute_seat_limit, reset_telegram_seats_for_test, ENV_TELEGRAM_SEAT_POLICY,
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
async fn metrics_export_horizon_s484_band_ph_s491() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_telegram_seats_for_test();
    reset_network_profile_store_for_test();
    std::env::remove_var(ENV_TELEGRAM_SEAT_POLICY);

    std::env::set_var(ENV_TELEGRAM_SEAT_POLICY, "bound_wallet_session");
    assert_eq!(compute_seat_limit(10, 2), 2);

    let profile = r#"{"region":"eu-west","latency_ms_p50":12}"#;
    persist_peer_network_profile("peer-np-1", profile).expect("persist");
    assert_eq!(
        load_peer_network_profile("peer-np-1").as_deref(),
        Some(profile)
    );

    let memory = MemoryShardStore::open_for_test(None);
    memory
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-1"),
            artifact_id: "art-1".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("upsert shard");

    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "w:emb-1".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 15_000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    assert_eq!(complete_prefetch_hook(&plan, Some(&memory)), 1);
    assert!(prefetch_pull_bytes_total() > 0);
    assert_eq!(fetch_seed_shards_hook(&plan, &memory), 1);

    let ranked = ingest_job_locality_rank_stub(&["w:emb-1".into(), "w:ckpt-7".into()], "inference");
    assert!(ranked.is_some());

    enqueue_verification_checker_task("job-vc-s491");
    assert_eq!(verification_checker_tasks().len(), 1);

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("galaxy_prefetch_pull_bytes_total"));

    std::env::remove_var(ENV_TELEGRAM_SEAT_POLICY);
    reset_prefetch_metrics_for_test();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_network_profile_store_for_test();
}
