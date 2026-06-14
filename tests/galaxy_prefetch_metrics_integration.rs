//! PH-S167: Galaxy prefetch metrics — plan_prefetch counters → Prometheus scrape.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    plan_prefetch, PrefetchPolicyConfig, PrefetchTrigger, SeedInventoryEntry, SeedInventoryHotTier,
};
use poolai::grid::galaxy_prefetch_metrics::{
    reset_prefetch_metrics_for_test, METRIC_PREFETCH_HOT_SKIP_TOTAL,
    METRIC_PREFETCH_PLANNED_SHARDS_TOTAL, METRIC_PREFETCH_PLAN_TOTAL,
};
use poolai::observability::{self, metrics_handler};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static PREFETCH_METRICS_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn prefetch_metrics_lock() -> std::sync::MutexGuard<'static, ()> {
    PREFETCH_METRICS_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy prefetch metrics integration lock")
}

fn metrics_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn get_metrics_text(app: &Router) -> String {
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
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).expect("utf8")
}

#[tokio::test]
async fn plan_prefetch_counters_visible_on_metrics_scrape() {
    let _lock = prefetch_metrics_lock();
    reset_prefetch_metrics_for_test();
    let inventory = SeedInventoryEntry {
        shard_ids: vec!["w:hot".into()],
        hot_tier: SeedInventoryHotTier {
            ram_bytes_used: 512,
            vram_bytes_used: 0,
            profiles: vec![],
        },
        ..Default::default()
    };
    let _ = plan_prefetch(
        &inventory,
        &["w:hot".into(), "w:cold".into(), "w:cold-2".into()],
        PrefetchTrigger::JobAdmitted,
        true,
        &PrefetchPolicyConfig::default(),
    );
    let app = metrics_app();
    let body = get_metrics_text(&app).await;
    assert!(body.contains(METRIC_PREFETCH_PLAN_TOTAL));
    assert!(body.contains(&format!("{METRIC_PREFETCH_PLAN_TOTAL} 1")));
    assert!(body.contains(&format!("{METRIC_PREFETCH_PLANNED_SHARDS_TOTAL} 2")));
    assert!(body.contains(&format!("{METRIC_PREFETCH_HOT_SKIP_TOTAL} 1")));
    reset_prefetch_metrics_for_test();
}
