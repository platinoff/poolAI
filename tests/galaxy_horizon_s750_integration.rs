//! PH-S759: Galaxy horizon close band (PH-S750…S758).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    fetch_seed_shards_hook, prefetch_backpressure_skip, with_prefetch_peer, PrefetchPlan,
    PrefetchPlanItem, PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
    ENV_PREFETCH_MIN_BANDWIDTH_MBPS,
};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_depth::{prefetch_depth_stub, PrefetchDepth};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_metrics_snapshot, record_prefetch_backpressure, record_prefetch_pull_bytes,
    reset_prefetch_metrics_for_test, DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_prefetch_metrics_parity,
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
async fn horizon_s750_band_prefetch_live_pull_depth_ph_s759() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    reset_network_profile_store_for_test();
    std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"prefetch_live_pull": true}))),
        StandSmokeMetricsParityDepth::PrefetchLivePull
    );

    let memory = MemoryShardStore::open_for_test(None);
    memory
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-s750"),
            artifact_id: "art-s750".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("upsert");

    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "w:emb-s750".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 15_000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    fetch_seed_shards_hook(&plan, &memory);
    record_prefetch_backpressure();

    let snapshot = prefetch_metrics_snapshot();
    assert_eq!(
        prefetch_depth_stub(Some(&snapshot)),
        PrefetchDepth::FullDepth
    );

    persist_peer_network_profile(
        "peer-s751",
        r#"{"region":"eu-west","latency_ms_p50":12,"bandwidth_mbps":10,"egress_policy":"direct"}"#,
    )
    .expect("persist");
    std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");
    with_prefetch_peer(Some("peer-s751"), || {
        assert!(prefetch_backpressure_skip());
    });
    assert!(prefetch_metrics_snapshot().backpressure_total >= 1);

    record_prefetch_pull_bytes(DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM);

    let app = grid_app();
    let (status, prefetch_json) = get_text(&app, "/api/v1/grid/prefetch-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let body: serde_json::Value = serde_json::from_str(&prefetch_json).expect("json");
    assert_eq!(body["ok"], true);

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_prefetch_metrics_parity(&prom, &body).expect("parity");
    assert!(prom.contains("galaxy_prefetch_pull_bytes_total"));
    assert!(prom.contains("galaxy_prefetch_backpressure_total"));

    std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
}
