//! PH-S769: Galaxy horizon close band (PH-S760…S768).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    complete_prefetch_hook, fetch_seed_shards_hook, PrefetchPlan, PrefetchPlanItem,
    PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
};
use poolai::grid::galaxy_locality::{
    observe_last_hot_tier_hit_ratio, observe_last_shard_local_hit_ratio, rank_workers_by_locality,
    LocalityHotTier, LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
};
use poolai::grid::galaxy_locality_hot_tier_depth::{
    locality_hot_tier_depth_stub, LocalityHotTierDepth,
};
use poolai::grid::galaxy_locality_metrics::locality_metrics_snapshot;
use poolai::grid::galaxy_prefetch_metrics::{
    hot_evict_total, hot_promote_total, reset_prefetch_metrics_for_test, ENV_HOT_PROMOTE_THRESHOLD,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_locality_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::json;
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
async fn horizon_s760_band_locality_hot_tier_depth_ph_s769() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    std::env::set_var(ENV_HOT_PROMOTE_THRESHOLD, "1");

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"locality_hot_tier": true}))),
        StandSmokeMetricsParityDepth::LocalityHotTier
    );

    let workers = vec![LocalityWorker {
        worker_id: "w-s760".into(),
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec!["w:emb-s760".into()],
            hot_tier: LocalityHotTier {
                ram_bytes_used: 1_073_741_824,
                vram_bytes_used: 0,
                profiles: vec!["p1".into()],
            },
            local_replica_regions: vec!["eu-west".into()],
        },
        network_profile: LocalityNetworkProfile {
            region: "eu-west".into(),
            latency_ms_p50: 8,
            latency_ms_p95: None,
            profile_age_secs: Some(60),
        },
        queue_depth: 0,
        pricing_usd_micro: None,
    }];
    let task = LocalityTask {
        required_shard_ids: vec!["w:emb-s760".into()],
        task_profile: "infer".into(),
        estimated_cross_region_egress_mb: 0.0,
        source_region: Some("eu-west".into()),
    };
    let ranked = rank_workers_by_locality(&workers, &task);
    assert!(!ranked.is_empty());
    observe_last_shard_local_hit_ratio(1.0);
    observe_last_hot_tier_hit_ratio(0.75);

    let memory = MemoryShardStore::open_for_test(None);
    memory
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-s760"),
            artifact_id: "art-s760".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("upsert");

    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "w:emb-s760".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 15_000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    fetch_seed_shards_hook(&plan, &memory);
    complete_prefetch_hook(&plan, Some(&memory));
    assert!(hot_promote_total() >= 1 || hot_evict_total() >= 1);

    let snapshot = locality_metrics_snapshot();
    assert_eq!(
        locality_hot_tier_depth_stub(Some(&snapshot)),
        LocalityHotTierDepth::FullDepth
    );

    let app = grid_app();
    let (status, locality_json) = get_text(&app, "/api/v1/grid/locality-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&locality_json).expect("json");
    assert_eq!(body["ok"], true);

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_locality_metrics_parity(&prom, &body).expect("parity");
    assert!(prom.contains("galaxy_hot_promote_total") || prom.contains("galaxy_hot_evict_total"));
    assert!(prom.contains("galaxy_shard_local_hit_ratio"));

    std::env::remove_var(ENV_HOT_PROMOTE_THRESHOLD);
    reset_prefetch_metrics_for_test();
}
