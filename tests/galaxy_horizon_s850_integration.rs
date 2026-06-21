//! PH-S859: Galaxy horizon close band (PH-S850…S858) — Job store RAID production path.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::job::{job_store_depth_stub, JobStoreDepth};
use poolai::network::api::create_api_routes;
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::jobs::{normalize_store_backend_key, render_jobs_store_badge_html};
use serde_json::json;
use tower::ServiceExt;

fn jobs_app() -> Router {
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
async fn horizon_s850_band_job_store_raid_ph_s859() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"jobs_store_badge": true}))),
        AdminWasmSlimDepth::JobsStoreBadge
    );
    assert_eq!(normalize_store_backend_key("RAID"), "raid");
    assert_eq!(
        job_store_depth_stub(Some("raid"), 1),
        JobStoreDepth::RaidRestartPersist
    );

    let badge = render_jobs_store_badge_html("raid", "Store:", "Job persistence backend", "RAID");
    assert!(badge.contains("status-badge active"));
    assert!(badge.contains("RAID"));

    let app = jobs_app();
    let (status, list) = get_json(&app, "/api/v1/jobs").await;
    assert_eq!(status, StatusCode::OK);
    let backend = list
        .get("store_backend")
        .and_then(|v| v.as_str())
        .unwrap_or("json");
    assert!(matches!(backend, "json" | "sqlite" | "raid"));
    assert_eq!(
        job_store_depth_stub(Some(backend), 0),
        match backend {
            "raid" => JobStoreDepth::RaidSnapshot,
            "sqlite" => JobStoreDepth::SqliteDb,
            _ => JobStoreDepth::JsonFile,
        }
    );
}
