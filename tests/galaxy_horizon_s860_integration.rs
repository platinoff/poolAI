//! PH-S869: Galaxy horizon close band (PH-S860…S868) — Memory shard persist + seed inventory.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
};
use poolai::memory::{
    memory_layer_depth_stub, memory_store_depth_stub, MemoryLayerDepth, MemoryStoreDepth,
};
use poolai::network::api::create_api_routes;
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::memory::render_memory_seed_meta_strip_html;
use serde_json::json;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
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
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!(null));
    (status, body)
}

#[tokio::test]
async fn horizon_s860_band_memory_shard_persist_ph_s869() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"memory_seed_meta_strip": true}))),
        AdminWasmSlimDepth::MemorySeedMetaStrip
    );
    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"memory_shard_persist": true}))),
        StandSmokeMetricsParityDepth::MemoryShardPersist
    );
    assert_eq!(
        memory_store_depth_stub(false, 0),
        MemoryStoreDepth::Ephemeral
    );
    assert_eq!(
        memory_store_depth_stub(true, 2),
        MemoryStoreDepth::JsonRestartPersist
    );
    assert_eq!(
        memory_layer_depth_stub(true, 2, 2),
        MemoryLayerDepth::FullDepth
    );

    let strip = render_memory_seed_meta_strip_html(
        false,
        1,
        "ephemeral",
        "registry_ephemeral",
        "Memory:",
        "Registered:",
    );
    assert!(strip.contains("seed-inventory-meta"));

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/seed-inventory").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["memory_store_depth"].is_string());
    assert!(body["memory_layer_depth"].is_string());
    assert_eq!(body["memory_layer_depth"], "seed_inventory_wire");
    assert!(body["registered_shard_count"].is_number());
    let entries = body["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 2);
}
