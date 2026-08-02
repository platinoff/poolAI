//! PH-S589: Galaxy horizon close band (PH-S580…S588).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_locality::{
    last_hot_tier_hit_ratio_bps, rank_workers_by_locality, reset_last_hot_tier_hit_ratio_for_test,
    LocalityHotTier, LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
    METRIC_HOT_TIER_HIT_RATIO,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_co_access_total, reset_prefetch_metrics_for_test, METRIC_PREFETCH_CO_ACCESS_TOTAL,
};
use poolai::grid::galaxy_security_advisory::list_security_advisories;
use poolai::grid::{ingest_envelope, GridEnvelope, GridIngestKind, GridJobBody, GridMessage};
use poolai::job::JobStore;
use poolai::memory::MemoryShardStore;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
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
async fn horizon_s580_band_hot_tier_co_access_advisories_ph_s589() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    reset_last_hot_tier_hit_ratio_for_test();
    std::env::set_var(
        "POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON",
        r#"{"w:emb-1":["w:ckpt-7"]}"#,
    );

    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);
    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "horizon-s588-job".into(),
            task_kind: "inference:text".into(),
            verification_policy: None,
            input_artifact_ids: vec![],
            required_shard_ids: vec!["w:emb-1".into()],
            deadline: None,
        }),
        None,
    );
    let out = ingest_envelope(job_env, &jobs, &memory).expect("job ingest");
    assert!(matches!(out.kind, GridIngestKind::Job { .. }));
    assert!(
        prefetch_co_access_total() >= 1,
        "co-access prefetch counter"
    );

    let task = LocalityTask {
        required_shard_ids: vec!["w:emb-1".into()],
        task_profile: "inference:text".into(),
        estimated_cross_region_egress_mb: 0.0,
        source_region: None,
    };
    let worker = LocalityWorker {
        worker_id: "w1".into(),
        seed_inventory: LocalitySeedInventory {
            shard_ids: vec!["w:emb-1".into()],
            hot_tier: LocalityHotTier {
                ram_bytes_used: 1,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            local_replica_regions: vec![],
        },
        network_profile: LocalityNetworkProfile {
            region: "eu-west".into(),
            latency_ms_p50: 20,
            latency_ms_p95: None,
            profile_age_secs: Some(0),
        },
        queue_depth: 0,
        pricing_usd_micro: None,
    };
    let _ = rank_workers_by_locality(&[worker], &task);
    assert_eq!(last_hot_tier_hit_ratio_bps(), 10_000);

    let advisories = list_security_advisories();
    assert_eq!(advisories.len(), 3);

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    for name in [
        METRIC_HOT_TIER_HIT_RATIO,
        METRIC_PREFETCH_CO_ACCESS_TOTAL,
        "poolai_advisory_acknowledged_total",
    ] {
        assert!(body.contains(name), "missing {name} in metrics body");
        assert!(
            body.contains(&format!("# TYPE {name} gauge")),
            "missing TYPE gauge for {name}"
        );
    }

    let (adv_status, adv_body) = get_text(&app, "/api/v1/admin/security-advisories").await;
    assert_eq!(adv_status, StatusCode::OK);
    assert!(adv_body.contains("CVE-2026-0001"));

    std::env::remove_var("POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON");
    reset_prefetch_metrics_for_test();
    reset_last_hot_tier_hit_ratio_for_test();
}
