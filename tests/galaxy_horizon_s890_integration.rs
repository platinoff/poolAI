//! PH-S899: Galaxy horizon close band (PH-S890…S898) — replication quorum production gates.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_replication_depth::{
    current_replication_depth, replication_depth_stub, replication_depth_wire_label,
    ReplicationDepth,
};
use poolai::grid::galaxy_replication_metrics::{
    replication_max_per_hour_from_env, replication_metrics_snapshot,
    replication_rate_limited_total, reset_replication_strict_metrics_for_test,
    ReplicationMetricsSnapshot, ENV_REPLICATION_MAX_PER_HOUR,
};
use poolai::grid::galaxy_replication_quorum_gate::{
    record_result_executor_digest, replication_quorum_allows_cleared,
    reset_replication_quorum_gate_for_test,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
};
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

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let req_body = if let Some(v) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&v).unwrap())
    } else {
        Body::empty()
    };
    let response = app
        .clone()
        .oneshot(builder.body(req_body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(json!(null)),
    )
}

#[tokio::test]
async fn horizon_s890_band_replication_quorum_production_ph_s899() {
    let _guard = env_lock();
    reset_replication_quorum_gate_for_test();
    reset_replication_strict_metrics_for_test();

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(
            &json!({"replication_quorum_production": true})
        )),
        StandSmokeMetricsParityDepth::ReplicationQuorumProduction
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(
            &json!({"grid_replication_pricing_rate_cap_strip": true})
        )),
        AdminWasmSlimDepth::GridReplicationPricingRateCapStrip
    );

    let empty = ReplicationMetricsSnapshot {
        strict_total: 0,
        enqueue_total: 0,
        executor_enqueue_total: 0,
        rate_limited_total: 0,
    };
    assert_eq!(
        replication_depth_stub(Some(&empty), 1000),
        ReplicationDepth::None
    );
    assert_eq!(replication_depth_wire_label(ReplicationDepth::None), "none");

    // PH-S890: strict tier quorum gate allows unanimous digests.
    record_result_executor_digest("job-quorum-s890", Some(&json!({"executor_digest": "d1"})));
    record_result_executor_digest("job-quorum-s890", Some(&json!({"executor_digest": "d1"})));
    record_result_executor_digest("job-quorum-s890", Some(&json!({"executor_digest": "d1"})));
    assert!(replication_quorum_allows_cleared(
        "job-quorum-s890",
        Some("replication_strict")
    ));

    // PH-S891: rate cap HTTP wire + metrics API depth fields.
    let prior = std::env::var(ENV_REPLICATION_MAX_PER_HOUR).ok();
    std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, "1");
    reset_replication_strict_metrics_for_test();
    let app = grid_app();
    let mut strict_job = json!({
        "v": 1,
        "sent_at": "2026-06-21T12:00:00Z",
        "type": "job",
        "job_id": format!("ph-s899-cap-{}", uuid::Uuid::new_v4()),
        "task_kind": "inference:text",
        "verification_policy": "replication_strict",
        "input_artifact_ids": [],
        "source_peer_id": "srv1-worker-a"
    });
    let (first_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(strict_job.clone()),
    )
    .await;
    assert_eq!(first_status, StatusCode::OK);
    strict_job["job_id"] = json!(format!("ph-s899-cap-b-{}", uuid::Uuid::new_v4()));
    let (second_status, _) =
        request_json(&app, "POST", "/api/v1/grid/envelope", Some(strict_job)).await;
    assert_eq!(second_status, StatusCode::OK);
    assert!(replication_rate_limited_total() >= 1);

    let (metrics_status, metrics_body) =
        request_json(&app, "GET", "/api/v1/grid/replication-metrics", None).await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert_eq!(metrics_body["ok"], true);
    assert!(metrics_body["replication_depth"].is_string());
    assert_eq!(
        metrics_body["replication_depth"],
        replication_depth_wire_label(current_replication_depth())
    );
    assert_eq!(
        metrics_body["rate_cap_per_hour"],
        replication_max_per_hour_from_env()
    );
    assert_eq!(
        metrics_body["metrics"]["rate_limited_total"],
        replication_metrics_snapshot().rate_limited_total
    );

    match prior {
        Some(v) => std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, v),
        None => std::env::remove_var(ENV_REPLICATION_MAX_PER_HOUR),
    }
    reset_replication_quorum_gate_for_test();
    reset_replication_strict_metrics_for_test();
}
