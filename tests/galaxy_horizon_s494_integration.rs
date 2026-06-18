//! PH-S500: Galaxy horizon wire integration band (PH-S494…S499 metrics + read APIs).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    complete_prefetch_hook, ingest_job_locality_rank_stub, PrefetchPlan, PrefetchPlanItem,
    PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
};
use poolai::grid::galaxy_network_profile_store::{
    load_peer_network_profile, persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_pull_bytes_total, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_verification_metrics::{
    drain_verification_checker_task, enqueue_verification_checker_task,
    reset_verification_checker_tasks_for_test, reset_verification_metrics_for_test,
    verification_checker_pending_total, verification_checker_tasks,
};
use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
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
    let text = String::from_utf8(bytes.to_vec()).unwrap_or_default();
    (status, text)
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, serde_json::Value) {
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
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({}));
    (status, json)
}

#[tokio::test]
async fn metrics_export_horizon_s494_band_ph_s500() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_prefetch_metrics_for_test();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_network_profile_store_for_test();

    let profile = r#"{"region":"eu-west","latency_ms_p50":12}"#;
    persist_peer_network_profile("peer-np-s500", profile).expect("persist");
    assert_eq!(
        load_peer_network_profile("peer-np-s500").as_deref(),
        Some(profile)
    );

    let memory = MemoryShardStore::open_for_test(None);
    memory
        .upsert(MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-s500"),
            artifact_id: "art-s500".into(),
            version: "v1".into(),
            raid_logical_name: None,
            seed_hints: None,
        })
        .expect("upsert shard");

    let plan = PrefetchPlan {
        items: vec![PrefetchPlanItem {
            shard_id: "w:emb-s500".into(),
            target_tier: PrefetchTargetTier::Ram,
        }],
        trigger: PrefetchTrigger::JobAdmitted,
        deadline_ms: 15_000,
        mode: PrefetchPolicyMode::BestEffort,
    };
    assert_eq!(complete_prefetch_hook(&plan, Some(&memory)), 1);
    assert!(prefetch_pull_bytes_total() > 0);

    let _ = ingest_job_locality_rank_stub(&["w:emb-s500".into()], "inference");

    enqueue_verification_checker_task("job-vc-s500");
    assert_eq!(verification_checker_pending_total(), 1);
    assert!(drain_verification_checker_task("job-vc-s500"));
    assert_eq!(verification_checker_tasks().len(), 0);

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("galaxy_verification_checker_pending_total"));
    assert!(body.contains("galaxy_prefetch_pull_bytes_total"));

    let (status, json) = get_json(&app, "/api/v1/grid/verification-checker/tasks").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.get("ok").and_then(|v| v.as_bool()), Some(true));
    assert!(json.get("tasks").and_then(|v| v.as_array()).is_some());

    let (status, json) = get_json(&app, "/api/v1/grid/network-profiles/peer-np-s500").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        json.get("network_profile")
            .and_then(|p| p.get("region"))
            .and_then(|v| v.as_str()),
        Some("eu-west")
    );

    reset_prefetch_metrics_for_test();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_network_profile_store_for_test();
}
