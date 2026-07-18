//! PH-S1074: Grid metrics parity contract tests — extended JSON↔Prom pairs + API shape.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::stand_smoke_metrics_parity::{
    validate_band6_metrics_parity_v3, GRID_METRICS_API_PATHS, PREFETCH_EXTENDED_PARITY,
    PRICING_EXTENDED_PARITY, REPLICATION_EXTENDED_PARITY, SETTLEMENT_EXTENDED_PARITY,
    TRUST_EXTENDED_PARITY, VERIFICATION_EXTENDED_PARITY,
};
use poolai::network::api::create_api_routes;
use poolai_ui_core::grid_metrics_parity_depth::GRID_METRICS_API_PATHS as UI_CORE_GRID_PATHS;
use serde_json::Value;
use tower::ServiceExt;

fn api_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[test]
fn grid_metrics_extended_parity_constants_ph_s1069() {
    assert_eq!(VERIFICATION_EXTENDED_PARITY.len(), 2);
    assert_eq!(REPLICATION_EXTENDED_PARITY.len(), 2);
    assert_eq!(PRICING_EXTENDED_PARITY.len(), 4);
    assert_eq!(PREFETCH_EXTENDED_PARITY.len(), 3);
    assert_eq!(SETTLEMENT_EXTENDED_PARITY.len(), 2);
    assert_eq!(TRUST_EXTENDED_PARITY.len(), 2);
    assert_eq!(GRID_METRICS_API_PATHS.len(), 11);
    assert_eq!(GRID_METRICS_API_PATHS, UI_CORE_GRID_PATHS);
}

#[tokio::test]
async fn grid_metrics_all_apis_openapi_shape_ph_s1074() {
    let app = api_app();
    for path in GRID_METRICS_API_PATHS {
        let (status, v) = get_json(&app, path).await;
        assert_eq!(status, StatusCode::OK, "path {path}");
        let o = v.as_object().expect("metrics response object");
        assert_eq!(o.get("ok"), Some(&Value::Bool(true)), "path {path}");
        assert!(
            o.get("metrics").and_then(|m| m.as_object()).is_some(),
            "path {path} missing metrics object: {v:?}"
        );
    }
}

#[test]
fn grid_metrics_parity_v3_synthetic_gate_ph_s1073() {
    let prom = concat!(
        "galaxy_verification_sample_total 0\n",
        "galaxy_verification_checker_pending_total 0\n",
        "galaxy_verification_mismatch_total 0\n",
        "galaxy_verification_match_total 0\n",
        "galaxy_replay_pending 0\n",
        "galaxy_verification_replay_record_total 0\n",
        "galaxy_settlement_cleared_total 0\n",
        "galaxy_settlement_payout_batch_total 0\n",
        "galaxy_settlement_pending_verification_total 0\n",
        "galaxy_settlement_resolved_total 0\n",
        "galaxy_trust_payout_eligible_total 0\n",
        "galaxy_trust_score 0\n",
        "galaxy_trust_payout_held_total 0\n",
        "galaxy_trust_gate_min_threshold 40\n",
        "galaxy_replication_strict_total 0\n",
        "galaxy_replication_enqueue_total 0\n",
        "galaxy_replication_executor_enqueue_total 0\n",
        "galaxy_replication_rate_limited_total 0\n",
        "galaxy_pricing_fresh_served 0\n",
        "galaxy_pricing_stale_served 0\n",
        "galaxy_pricing_forced_fallback_total 0\n",
        "galaxy_pricing_provider_catalog_hits_total 0\n",
        "galaxy_pricing_provider_errors_total 0\n",
        "galaxy_pricing_provider_timeouts_total 0\n",
        "galaxy_prefetch_pull_bytes_total 0\n",
        "galaxy_prefetch_backpressure_total 0\n",
        "galaxy_prefetch_plan_total 0\n",
        "galaxy_prefetch_enqueue_total 0\n",
        "galaxy_prefetch_peer_fetch_total 0\n",
        "galaxy_shard_local_hit_ratio 0\n",
        "galaxy_hot_tier_hit_ratio 0\n",
        "galaxy_cross_region_egress_mb 0\n",
        "galaxy_hot_promote_total 0\n",
        "galaxy_hot_evict_total 0\n",
        "galaxy_fee_split_applied_total 0\n",
        "poolai_release_verify_total 0\n",
        "poolai_release_verify_fail_total 0\n",
        "poolai_update_notify_pending 0\n",
        "poolai_advisory_acknowledged_total 0\n",
        "galaxy_settlement_payout_batch_queue_depth 0\n",
        "galaxy_settlement_onchain_submit_total 0\n",
    );
    let verification = serde_json::json!({"ok": true, "metrics": {"sample_total": 0, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
    let replay = serde_json::json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
    let settlement = serde_json::json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
    let trust = serde_json::json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
    let replication = serde_json::json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
    let pricing = serde_json::json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0, "provider_catalog_hits_total": 0, "provider_errors_total": 0, "provider_timeouts_total": 0}});
    let prefetch = serde_json::json!({"ok": true, "metrics": {"pull_bytes_total": 0, "backpressure_total": 0, "plan_total": 0, "enqueue_total": 0, "peer_fetch_total": 0}});
    let locality = serde_json::json!({"ok": true, "metrics": {"shard_local_hit_ratio_bps": 0, "hot_tier_hit_ratio_bps": 0, "cross_region_egress_mb": 0, "hot_promote_total": 0, "hot_evict_total": 0}});
    let fee_split = serde_json::json!({"ok": true, "metrics": {"fee_split_applied_total": 0, "primary_dev_fee_bps": 10, "secondary_admin_fee_min_bps": 100, "secondary_admin_fee_max_bps": 500}});
    let governance = serde_json::json!({"ok": true, "metrics": {"release_verify_total": 0, "release_verify_fail_total": 0, "update_notify_pending": 0, "advisory_acknowledged_total": 0}});
    let payout_batch = serde_json::json!({"ok": true, "metrics": {"payout_batch_total": 0, "payout_batch_queue_depth": 0, "onchain_submit_total": 0}});
    validate_band6_metrics_parity_v3(
        prom,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
        &prefetch,
        &locality,
        &fee_split,
        &governance,
        &payout_batch,
    )
    .expect("v3 synthetic parity");
}
