//! PH-S889: Galaxy horizon close band (PH-S880…S888) — verification checker lifecycle.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_verification_checker_jobs::{
    reset_verification_checker_job_submit_for_test, verification_checker_job_submit_total,
};
use poolai::grid::galaxy_verification_lifecycle_depth::{
    current_verification_lifecycle_depth, verification_lifecycle_depth_stub,
    verification_lifecycle_depth_wire_label, VerificationLifecycleDepth,
};
use poolai::grid::galaxy_verification_metrics::{
    drain_verification_checker_task, enqueue_verification_checker_task,
    reset_verification_checker_tasks_for_test, reset_verification_metrics_for_test,
    verification_checker_pending_total, verification_checker_tasks, verification_metrics_snapshot,
    VerificationMetricsSnapshot,
};
use poolai::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE;
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
};
use poolai::grid::{
    ingest_envelope, GridEnvelope, GridJobBody, GridMessage, GridResultBody, GridResultStatus,
};
use poolai::job::JobStore;
use poolai::memory::MemoryShardStore;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use serde_json::{json, Value};
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
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
        serde_json::from_slice(&bytes).unwrap_or(json!(null)),
    )
}

async fn grid_job_result_with_verdict(
    jobs: &JobStore,
    memory: &MemoryShardStore,
    job_id: &str,
    metrics: Value,
) {
    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: job_id.into(),
            task_kind: "inference:text".into(),
            verification_policy: None,
            input_artifact_ids: vec![],
            required_shard_ids: vec![],
            deadline: None,
        }),
        Some("tg-edge-s880".into()),
    );
    ingest_envelope(job_env, jobs, memory).expect("job ingest");
    let lease_epoch = jobs.get(job_id).expect("get").expect("row").lease_epoch;
    let result_env = GridEnvelope::new(
        GridMessage::Result(GridResultBody {
            job_id: job_id.into(),
            status: GridResultStatus::Completed,
            output_artifact_ids: vec![],
            proof: None,
            metrics: Some(metrics),
            lease_epoch,
        }),
        Some("tg-edge-s880".into()),
    );
    ingest_envelope(result_env, jobs, memory).expect("result ingest");
}

#[tokio::test]
async fn horizon_s880_band_verification_checker_lifecycle_ph_s889() {
    let _guard = env_lock();
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_verification_checker_job_submit_for_test();

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(
            &json!({"verification_checker_lifecycle": true})
        )),
        StandSmokeMetricsParityDepth::VerificationCheckerLifecycle
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_panel": true}))),
        AdminWasmSlimDepth::GridVerificationPanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_metrics_strip": true}))),
        AdminWasmSlimDepth::GridVerificationMetricsStrip
    );

    let empty = VerificationMetricsSnapshot {
        sample_total: 0,
        mismatch_total: 0,
        match_total: 0,
        sample_completed_total: 0,
        checker_enqueue_total: 0,
        checker_pending_total: 0,
    };
    assert_eq!(
        verification_lifecycle_depth_stub(Some(&empty), 0),
        VerificationLifecycleDepth::None
    );
    assert_eq!(
        verification_lifecycle_depth_wire_label(VerificationLifecycleDepth::None),
        "none"
    );

    // PH-S880: enqueue → HTTP tasks → drain on verdict via grid result.
    enqueue_verification_checker_task("job-drain-s880");
    assert_eq!(verification_checker_pending_total(), 1);

    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
    grid_job_result_with_verdict(
        &jobs,
        &memory,
        "job-drain-s880",
        json!({ "verification_verdict": "match", "trust_score": 75 }),
    )
    .await;
    assert!(!drain_verification_checker_task("job-drain-s880"));
    assert_eq!(verification_checker_tasks().len(), 0);
    assert_eq!(verification_checker_pending_total(), 0);

    // PH-S881: shadow job submit on sampled result.
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_verification_checker_job_submit_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "1.0");
    let job_shadow = format!("job-shadow-s881-{}", uuid::Uuid::new_v4());
    grid_job_result_with_verdict(&jobs, &memory, &job_shadow, json!({ "trust_score": 80 })).await;
    assert_eq!(verification_checker_job_submit_total(), 1);
    assert!(verification_checker_pending_total() >= 1);

    let app = grid_app();
    let (tasks_status, tasks_body) =
        get_json(&app, "/api/v1/grid/verification-checker/tasks").await;
    assert_eq!(tasks_status, StatusCode::OK);
    assert_eq!(tasks_body["ok"], true);
    assert!(tasks_body["tasks"].is_array());

    let (metrics_status, metrics_body) = get_json(&app, "/api/v1/grid/verification-metrics").await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert_eq!(metrics_body["ok"], true);
    assert!(metrics_body["lifecycle_depth"].is_string());
    assert_eq!(
        metrics_body["lifecycle_depth"],
        verification_lifecycle_depth_wire_label(current_verification_lifecycle_depth())
    );
    assert_eq!(
        metrics_body["metrics"]["checker_pending_total"],
        verification_metrics_snapshot().checker_pending_total
    );

    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    reset_verification_metrics_for_test();
    reset_verification_checker_tasks_for_test();
    reset_verification_checker_job_submit_for_test();
}
