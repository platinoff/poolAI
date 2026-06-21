//! PH-S799: Galaxy horizon close band (PH-S790…S798).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_governance_depth::{governance_depth_stub, GovernanceDepth};
use poolai::grid::galaxy_governance_metrics::{
    governance_metrics_snapshot, record_release_verify_success, reset_governance_metrics_for_test,
    set_update_notify_pending,
};
use poolai::grid::galaxy_security_advisory::{
    acknowledge_security_advisory, reset_security_advisory_for_test,
};
use poolai::grid::galaxy_update_policy::{tick_update_notify_from_env, ENV_UPDATE_POLICY};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_governance_metrics_parity,
    validate_update_policy_json_export, StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::json;
use std::sync::{Mutex, OnceLock};
use tower::ServiceExt;

static GOVERNANCE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn governance_lock() -> std::sync::MutexGuard<'static, ()> {
    GOVERNANCE_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("galaxy governance integration lock")
}

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
async fn horizon_s790_band_governance_ops_depth_ph_s799() {
    let _lock = governance_lock();
    reset_governance_metrics_for_test();
    reset_security_advisory_for_test();
    std::env::set_var(ENV_UPDATE_POLICY, "notify");

    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"governance_ops": true}))),
        StandSmokeMetricsParityDepth::GovernanceOps
    );

    record_release_verify_success();
    tick_update_notify_from_env();
    set_update_notify_pending(1);
    acknowledge_security_advisory("CVE-2026-0001");

    let snap = governance_metrics_snapshot();
    assert!(snap.release_verify_total >= 1);
    assert!(snap.update_notify_pending >= 1);
    assert_eq!(snap.advisory_acknowledged_total, 1);
    assert_eq!(
        governance_depth_stub(Some(&snap)),
        GovernanceDepth::FullDepth
    );

    let app = grid_app();

    let (status, policy_json) = get_text(&app, "/api/v1/grid/update-policy").await;
    assert_eq!(status, StatusCode::OK);
    let policy_body: serde_json::Value = serde_json::from_str(&policy_json).expect("json");
    validate_update_policy_json_export(&policy_body).expect("policy shape");
    assert_eq!(policy_body["policy"]["mode"], "notify");

    let (status, gov_json) = get_text(&app, "/api/v1/grid/governance-metrics").await;
    assert_eq!(status, StatusCode::OK);
    let gov_body: serde_json::Value = serde_json::from_str(&gov_json).expect("json");
    assert_eq!(gov_body["ok"], true);

    let (_, prom) = get_text(&app, "/metrics").await;
    validate_governance_metrics_parity(&prom, &gov_body).expect("parity");
    assert!(prom.contains("poolai_advisory_acknowledged_total"));

    assert_eq!(
        governance_metrics_snapshot().advisory_acknowledged_total,
        gov_body["metrics"]["advisory_acknowledged_total"]
            .as_u64()
            .unwrap_or(0)
    );

    std::env::remove_var(ENV_UPDATE_POLICY);
    reset_governance_metrics_for_test();
    reset_security_advisory_for_test();
}
