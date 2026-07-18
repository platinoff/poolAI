//! PH-S1061: Memory shard API OpenAPI contract tests.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
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

async fn post_json(app: &Router, uri: &str, payload: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[tokio::test]
async fn memory_shards_list_openapi_shape_ph_s1061() {
    let app = api_app();
    let (status, v) = get_json(&app, "/api/v1/memory/shards").await;
    assert_eq!(status, StatusCode::OK);
    let o = v.as_object().expect("memory shards list object");
    assert!(o.contains_key("shards"), "missing shards: {o:?}");
    assert!(o["shards"].is_array());
}

#[tokio::test]
async fn memory_shard_register_and_get_openapi_shape_ph_s1061() {
    let app = api_app();
    let shard_id = format!("contract-s1061-{}", uuid::Uuid::new_v4());
    let payload = format!(
        r#"{{"shard_id":"{shard_id}","artifact_id":"art-contract-1061","version":"1.0.0","raid_logical_name":"weights","seed_hints":["emb"]}}"#
    );
    let (post_status, created) = post_json(&app, "/api/v1/memory/shards", &payload).await;
    assert_eq!(post_status, StatusCode::CREATED, "body={created}");
    let shard = created["shard"].as_object().expect("created shard object");
    for key in ["shard_id", "artifact_id", "version"] {
        assert!(
            shard.contains_key(key),
            "created shard missing `{key}`: {shard:?}"
        );
    }
    assert_eq!(shard["shard_id"], shard_id);

    let (get_status, fetched) = get_json(&app, &format!("/api/v1/memory/shards/{shard_id}")).await;
    assert_eq!(get_status, StatusCode::OK);
    let got = fetched["shard"].as_object().expect("fetched shard object");
    assert_eq!(got["shard_id"], shard_id);
    assert_eq!(got["artifact_id"], "art-contract-1061");
}
