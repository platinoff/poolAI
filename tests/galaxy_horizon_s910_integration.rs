//! PH-S919: Galaxy horizon close band (PH-S910…S918) — Trust score SQLite persist.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_trust_persist_depth::{
    current_trust_persist_depth, trust_persist_depth_stub, trust_persist_depth_wire_label,
    TrustPersistDepth,
};
use poolai::grid::galaxy_trust_score::{
    evaluate_result_settlement_gate, reset_settlement_gate_metrics_for_test, TrustScoreGateConfig,
};
use poolai::grid::galaxy_trust_score_store::{
    current_trust_store_backend, persist_peer_trust_score, reset_trust_score_store_for_test,
    TrustStoreBackend,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, validate_settlement_trust_metrics_parity,
    StandSmokeMetricsParityDepth,
};
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::trust::render_grid_trust_persist_strip_html;
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

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let (status, text) = get_text(app, uri).await;
    let body: Value = serde_json::from_str(&text).unwrap_or(json!(null));
    (status, body)
}

#[tokio::test]
async fn horizon_s910_band_trust_sqlite_persist_ph_s919() {
    let _guard = env_lock();
    reset_trust_score_store_for_test();
    reset_settlement_gate_metrics_for_test();
    std::env::remove_var(poolai::grid::galaxy_trust_score_store::ENV_TRUST_SCORE_STORE_PATH);

    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"grid_trust_persist_strip": true}))),
        AdminWasmSlimDepth::GridTrustPersistStrip
    );
    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"trust_persist": true}))),
        StandSmokeMetricsParityDepth::TrustPersist
    );
    assert_eq!(
        trust_persist_depth_stub(TrustStoreBackend::Sqlite, 1),
        TrustPersistDepth::SqliteRestartPersist
    );

    let cfg = TrustScoreGateConfig::default_stub();
    evaluate_result_settlement_gate(Some("tg-persist-low"), Some(15), &cfg);
    persist_peer_trust_score("tg-persist-low", 15);
    assert_eq!(current_trust_store_backend(), TrustStoreBackend::Ephemeral);
    assert_eq!(current_trust_persist_depth(), TrustPersistDepth::Ephemeral);

    let strip = render_grid_trust_persist_strip_html(
        r#"{"ok":true,"trust_persist_depth":"ephemeral","trust_store_backend":"ephemeral","persisted_peer_count":1,"metrics":{"payout_held_total":1,"payout_eligible_total":0,"last_trust_score":15}}"#,
        r#"{}"#,
    );
    assert!(strip.contains("grid-trust-persist-strip"));

    let app = grid_app();
    let (status, trust_body) = get_json(&app, "/api/v1/grid/trust-metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(trust_body["ok"], true);
    assert!(trust_body["trust_persist_depth"].is_string());
    assert!(trust_body["trust_store_backend"].is_string());
    assert_eq!(
        trust_persist_depth_wire_label(current_trust_persist_depth()),
        trust_body["trust_persist_depth"].as_str().unwrap()
    );

    let (_, settlement) = get_json(&app, "/api/v1/grid/settlement-metrics").await;
    let (_, prom) = get_text(&app, "/metrics").await;
    validate_settlement_trust_metrics_parity(&prom, &settlement, &trust_body).expect("parity");

    reset_trust_score_store_for_test();
    reset_settlement_gate_metrics_for_test();
}
