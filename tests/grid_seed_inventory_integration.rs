//! PH-S195: Galaxy seed inventory GET — coordinator stub snapshot wire.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use serde_json::Value;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
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
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

#[tokio::test]
async fn grid_seed_inventory_returns_coordinator_stub_entries() {
    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/seed-inventory").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["generated_at"].is_string());

    let entries = body["entries"].as_array().expect("entries array");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0]["peer_id"], "srv1-worker-a");
    assert_eq!(
        entries[0]["seed_inventory"]["shard_ids"],
        serde_json::json!(["w:emb-1", "w:ckpt-7"])
    );
    assert_eq!(
        entries[0]["seed_inventory"]["hot_tier"]["ram_bytes_used"],
        3_221_225_472u64
    );
    assert_eq!(
        entries[0]["seed_inventory"]["local_replica_regions"],
        serde_json::json!(["eu-west"])
    );
    assert_eq!(entries[1]["peer_id"], "srv2-worker-b");
    assert!(body["memory_persist"].is_boolean());
    assert!(body["memory_store_depth"].is_string());
    assert!(body["memory_layer_depth"].is_string());
    assert_eq!(body["memory_layer_depth"], "seed_inventory_wire");
}
