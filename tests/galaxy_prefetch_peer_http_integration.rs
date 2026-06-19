//! PH-S574: peer HTTP seed-pull prefetch integration (wiremock).

use poolai::grid::dispatch::{
    PrefetchPlan, PrefetchPlanItem, PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_peer_fetch_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_prefetch_peer_pull::{
    fetch_seed_shards_from_peer_http, ENV_PREFETCH_PEER_HTTP_URL,
};

#[test]
fn prefetch_peer_http_fetch_increments_metric_ph_s574() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_prefetch_metrics_for_test();

    let mut server = mockito::Server::new();
    let body = r#"{"entries":[{"seed_inventory":{"shard_ids":["shard-a"],"hot_tier":{"ram_bytes_used":1,"vram_bytes_used":0,"profiles":[]},"local_replica_regions":[]}}]}"#;
    let mock = server
        .mock("GET", "/api/v1/grid/seed-inventory")
        .with_status(200)
        .with_body(body)
        .create();

    std::env::set_var(
        ENV_PREFETCH_PEER_HTTP_URL,
        format!("{}/api/v1/grid/seed-inventory", server.url()),
    );
    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "shard-a".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 1000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    let hits = fetch_seed_shards_from_peer_http(&plan);
    mock.assert();
    assert!(hits >= 1);
    assert!(prefetch_peer_fetch_total() >= 1);

    std::env::remove_var(ENV_PREFETCH_PEER_HTTP_URL);
    reset_prefetch_metrics_for_test();
}
