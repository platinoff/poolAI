//! PH-S729: Galaxy horizon close band (PH-S720…S728).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_re_migrate_policy::{re_migrate_policy_depth_stub, ReMigratePolicyDepth};
use poolai::grid::galaxy_routing_policy::{
    routing_policy_locality_gate, RoutingPolicyLocalityVerdict,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_settlement_trust_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use serde_json::{json, Value};
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn request_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(json!({ "raw": String::from_utf8_lossy(&bytes) })),
    )
}

async fn request_metrics_text(app: &Router) -> (StatusCode, String) {
    let req = Request::builder()
        .method("GET")
        .uri("/metrics")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn horizon_s720_band_routing_settlement_trust_ph_s729() {
    let app = grid_app();

    let (prom_status, prom_text) = request_metrics_text(&app).await;
    assert_eq!(prom_status, StatusCode::OK);

    let (s_status, settlement) = request_json(&app, "/api/v1/grid/settlement-metrics").await;
    assert_eq!(s_status, StatusCode::OK);
    let (t_status, trust) = request_json(&app, "/api/v1/grid/trust-metrics").await;
    assert_eq!(t_status, StatusCode::OK);

    validate_settlement_trust_metrics_parity(&prom_text, &settlement, &trust)
        .expect("settlement/trust parity");

    assert_eq!(
        re_migrate_policy_depth_stub(Some(&json!({"re_migrate_delta_fetch": true}))),
        ReMigratePolicyDepth::DeltaFetch
    );
    assert_eq!(
        routing_policy_locality_gate(&[]),
        RoutingPolicyLocalityVerdict::Allowed
    );
    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"routing_locality_gate": true}))),
        StandSmokeMetricsParityDepth::RoutingLocality
    );
}
