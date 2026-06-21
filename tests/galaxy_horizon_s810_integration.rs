//! PH-S819: Galaxy horizon close band (PH-S810…S818).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::observability::{self, metrics_handler};
use poolai::security::secret_rotation::{init_default_rotation_hooks, rotation_status};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::security::render_secret_rotation_panel_html;
use poolai_ui_core::topology::render_topology_stats_strip_html;
use serde_json::json;
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
async fn horizon_s810_band_admin_wasm_slim_security_topology_ph_s819() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"security_rotation_panel": true}))),
        AdminWasmSlimDepth::SecurityRotationPanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"topology_stats_strip": true}))),
        AdminWasmSlimDepth::TopologyStatsStrip
    );

    let sec_html = render_secret_rotation_panel_html(
        r#"[{"kind":"jwt","configured":true,"hook_count":1,"rotation_count":0,"grace_active":false}]"#,
        r#"{"admin.sec.rot.heading":"Secret rotation"}"#,
    );
    assert!(sec_html.contains("secret-rotation-table"));
    assert!(sec_html.contains("rotateSecret"));

    let topo_html = render_topology_stats_strip_html(
        r#"{"node_count":2,"latency_measurements":1,"last_updated":"2026-06-21T12:00:00Z"}"#,
        r#"{"admin.topo.stat.nodes":"Nodes"}"#,
    );
    assert!(topo_html.contains("topology-node-count"));
    assert!(topo_html.contains("2026-06-21 12:00:00 UTC"));

    init_default_rotation_hooks();
    let status = rotation_status();
    assert!(!status.is_empty());

    let app = grid_app();
    let (status_code, _) = get_text(&app, "/metrics").await;
    assert_eq!(status_code, StatusCode::OK);
}
