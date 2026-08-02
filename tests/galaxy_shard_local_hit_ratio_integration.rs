//! PH-S183: Galaxy shard local hit ratio gauge on GET /metrics.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_locality::{
    rank_workers_by_locality, reset_last_shard_local_hit_ratio_for_test, LocalityHotTier,
    LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
    METRIC_SHARD_LOCAL_HIT_RATIO,
};
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static LOCALITY_HIT_RATIO_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn locality_hit_ratio_lock() -> std::sync::MutexGuard<'static, ()> {
    LOCALITY_HIT_RATIO_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy shard local hit ratio integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

fn worker(id: &str, shards: &[&str]) -> LocalityWorker {
    LocalityWorker {
        worker_id: id.into(),
        queue_depth: 0,
        pricing_usd_micro: None,
        seed_inventory: LocalitySeedInventory {
            shard_ids: shards.iter().map(|s| (*s).to_string()).collect(),
            hot_tier: LocalityHotTier {
                ram_bytes_used: 4096,
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
    }
}

#[tokio::test]
async fn rank_path_observes_shard_local_hit_ratio_on_metrics_scrape() {
    let _lock = locality_hit_ratio_lock();
    reset_last_shard_local_hit_ratio_for_test();

    let task = LocalityTask {
        required_shard_ids: vec!["w:emb-1".into(), "w:ckpt-7".into()],
        task_profile: "inference:text".into(),
        estimated_cross_region_egress_mb: 0.0,
        source_region: Some("eu-west".into()),
    };
    let workers = [
        worker("partial", &["w:emb-1"]),
        worker("full", &["w:emb-1", "w:ckpt-7"]),
    ];
    let ranked = rank_workers_by_locality(&workers, &task);
    assert_eq!(ranked[0].worker_id, "full");

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
    assert!(body.contains(METRIC_SHARD_LOCAL_HIT_RATIO));
    assert!(body.contains(&format!("{METRIC_SHARD_LOCAL_HIT_RATIO} 10000")));

    reset_last_shard_local_hit_ratio_for_test();
}
