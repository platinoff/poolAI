//! PH-S185: Galaxy cross-region egress MB gauge on rank/prefetch path → `/metrics`.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    plan_prefetch, PrefetchPolicyConfig, PrefetchTrigger, SeedInventoryEntry,
};
use poolai::grid::galaxy_locality::{
    rank_workers_by_locality, reset_locality_metrics_for_test, LocalityHotTier,
    LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
    DEFAULT_PREFETCH_CROSS_REGION_EGRESS_MB_PER_SHARD, METRIC_CROSS_REGION_EGRESS_MB,
};
use poolai::grid::galaxy_prefetch_metrics::reset_prefetch_metrics_for_test;
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static EGRESS_METRICS_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn egress_metrics_lock() -> std::sync::MutexGuard<'static, ()> {
    EGRESS_METRICS_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy cross region egress mb integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn rank_and_prefetch_paths_observe_cross_region_egress_mb_on_scrape() {
    let _lock = egress_metrics_lock();
    reset_locality_metrics_for_test();
    reset_prefetch_metrics_for_test();

    let task = LocalityTask {
        required_shard_ids: vec!["w:gpu-weights".into()],
        task_profile: "inference:text".into(),
        estimated_cross_region_egress_mb: 75.0,
        source_region: Some("eu-west".into()),
    };
    let worker = LocalityWorker {
        worker_id: "us-remote".into(),
        queue_depth: 0,
        pricing_usd_micro: None,
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec![],
            hot_tier: LocalityHotTier::default(),
            local_replica_regions: vec!["us-east".into()],
        },
        network_profile: LocalityNetworkProfile {
            region: "us-east".into(),
            latency_ms_p50: 90,
            latency_ms_p95: None,
            profile_age_secs: Some(0),
        },
    };
    let _ = rank_workers_by_locality(&[worker], &task);

    let inventory = SeedInventoryEntry::default();
    let _ = plan_prefetch(
        &inventory,
        &["w:cold-a".into(), "w:cold-b".into()],
        PrefetchTrigger::JobAdmitted,
        false,
        &PrefetchPolicyConfig::default(),
    );

    let app = metrics_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert!(body.contains(METRIC_CROSS_REGION_EGRESS_MB));
    assert!(body.contains(&format!(
        "{METRIC_CROSS_REGION_EGRESS_MB} {}",
        (2.0 * DEFAULT_PREFETCH_CROSS_REGION_EGRESS_MB_PER_SHARD) as u64
    )));

    reset_locality_metrics_for_test();
    reset_prefetch_metrics_for_test();
}
