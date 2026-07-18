//! PH-S1000: Multi-module wire smoke harness — top 5 grid metrics APIs in one test.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai_ui_core::multi_module_depth::MULTI_MODULE_BAND35_TOP5_GRID_APIS;
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

#[tokio::test]
async fn multi_module_wire_smoke_top5_grid_apis_ph_s1000() {
    let app = api_app();
    for path in MULTI_MODULE_BAND35_TOP5_GRID_APIS {
        let (status, v) = get_json(&app, path).await;
        assert_eq!(status, StatusCode::OK, "path {path}");
        let o = v.as_object().expect("metrics response object");
        assert_eq!(o.get("ok"), Some(&Value::Bool(true)), "path {path}");
        assert!(
            o.get("metrics").and_then(|m| m.as_object()).is_some(),
            "path {path} missing metrics object: {v:?}"
        );
    }
    assert_eq!(MULTI_MODULE_BAND35_TOP5_GRID_APIS.len(), 5);
}
