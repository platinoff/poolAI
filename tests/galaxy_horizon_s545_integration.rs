//! PH-S554: Galaxy horizon wire integration band (PH-S545…S553).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_governance_metrics::reset_governance_metrics_for_test;
use poolai::grid::galaxy_prefetch_metrics::reset_prefetch_metrics_for_test;
use poolai::grid::galaxy_replication_quorum_gate::reset_replication_quorum_gate_for_test;
use poolai::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN;
use poolai::grid::galaxy_trust_score_store::reset_trust_score_store_for_test;
use poolai::grid::galaxy_update_policy::{tick_update_notify_from_env, ENV_UPDATE_POLICY};
use poolai::grid::{
    evaluate_strict_prefetch_timeout, ingest_envelope, GridEnvelope, GridJobBody, GridMessage,
    ENV_LOCALITY_MODE,
};
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
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({})),
    )
}

#[tokio::test]
async fn horizon_s545_band_quorum_prefetch_settlement_ph_s554() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_replication_quorum_gate_for_test();
    reset_prefetch_metrics_for_test();
    reset_governance_metrics_for_test();
    reset_trust_score_store_for_test();

    std::env::set_var(ENV_UPDATE_POLICY, "notify");
    std::env::set_var(ENV_SETTLEMENT_ON_CHAIN, "1");
    std::env::set_var(ENV_LOCALITY_MODE, "strict_locality");

    tick_update_notify_from_env();

    let memory = MemoryShardStore::open_for_test(None);
    assert!(evaluate_strict_prefetch_timeout(&["w:missing".into()], &memory).is_err());

    let jobs = JobStore::open_for_test(None);
    std::env::remove_var(ENV_LOCALITY_MODE);
    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "horizon-s545-job".into(),
            task_kind: "inference".into(),
            verification_policy: Some("replication_strict".into()),
            input_artifact_ids: vec![],
            required_shard_ids: vec![],
            deadline: None,
        }),
        Some("tg-edge".into()),
    );
    ingest_envelope(job_env, &jobs, &memory).expect("job ingest");

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/payout-batch").await;
    assert_eq!(status, StatusCode::OK, "payout-batch: {body}");
    assert_eq!(body["settlement_mode"].as_str(), Some("on_chain"));
    assert_eq!(body["on_chain_pending"].as_bool(), Some(true));

    let (status, metrics) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert!(metrics.contains("poolai_update_notify_pending"));
    assert!(poolai::grid::galaxy_prefetch_metrics::prefetch_timeout_total() >= 1);

    std::env::remove_var(ENV_UPDATE_POLICY);
    std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
    std::env::remove_var(ENV_LOCALITY_MODE);
    reset_replication_quorum_gate_for_test();
    reset_prefetch_metrics_for_test();
    reset_governance_metrics_for_test();
    reset_trust_score_store_for_test();
}
