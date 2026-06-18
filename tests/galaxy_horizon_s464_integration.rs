//! PH-S471: Galaxy horizon wire integration band (PH-S464…S469 metrics).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_backpressure_total, prefetch_raid_fetch_total, record_prefetch_backpressure,
    record_prefetch_raid_fetch, record_prefetch_re_migrate, reset_prefetch_metrics_for_test,
};
use poolai::grid::galaxy_protocol_negotiation_metrics::{
    protocol_negotiation_accepted_total, record_protocol_negotiation_accepted,
    reset_protocol_negotiation_metrics_for_test,
};
use poolai::grid::galaxy_trust_score::{
    apply_verification_trust_delta, reset_settlement_gate_metrics_for_test, trust_score_delta_total,
};
use poolai::grid::galaxy_verify_sampling::{
    evaluate_post_mismatch_elevated_sampling, reset_verify_sampling_metrics_for_test,
    verify_elevated_applied_total, VerifySamplingConfig,
};
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

#[tokio::test]
async fn metrics_export_horizon_s464_band_ph_s471() {
    reset_prefetch_metrics_for_test();
    reset_protocol_negotiation_metrics_for_test();
    reset_verify_sampling_metrics_for_test();
    reset_settlement_gate_metrics_for_test();

    record_prefetch_backpressure();
    record_prefetch_raid_fetch(1);
    record_prefetch_re_migrate();
    record_protocol_negotiation_accepted();
    let elevated_cfg = VerifySamplingConfig {
        elevated_sample_rate: 1.0,
        ..VerifySamplingConfig::default_stub()
    };
    assert!(evaluate_post_mismatch_elevated_sampling(
        "job-elevated",
        &elevated_cfg
    ));
    apply_verification_trust_delta("match", 50);

    assert_eq!(prefetch_backpressure_total(), 1);
    assert_eq!(prefetch_raid_fetch_total(), 1);
    assert_eq!(protocol_negotiation_accepted_total(), 1);
    assert_eq!(verify_elevated_applied_total(), 1);
    assert_eq!(trust_score_delta_total(), 1);

    let app = grid_app();
    let (status, body) = get_text(&app, "/metrics").await;
    assert_eq!(status, StatusCode::OK);
    for name in [
        "galaxy_prefetch_backpressure_total",
        "galaxy_prefetch_raid_fetch_total",
        "galaxy_prefetch_re_migrate_total",
        "poolai_protocol_negotiation_accepted_total",
        "galaxy_verification_elevated_applied_total",
        "galaxy_trust_score_delta_total",
    ] {
        assert!(body.contains(name), "missing {name} in metrics body");
        assert!(
            body.contains(&format!("# TYPE {name} gauge")),
            "missing TYPE gauge for {name}"
        );
    }

    reset_prefetch_metrics_for_test();
    reset_protocol_negotiation_metrics_for_test();
    reset_verify_sampling_metrics_for_test();
    reset_settlement_gate_metrics_for_test();
}
