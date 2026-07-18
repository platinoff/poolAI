//! PH-S1016: `POST /api/v1/ops/power` wire — dev-stand safe shutdown/reboot.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::ops::power::power_ops_invocation_count;
use serde_json::Value;
use tower::ServiceExt;

fn app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn post_power(app: &Router, action: &str) -> (StatusCode, Value) {
    let before = power_ops_invocation_count();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ops/power")
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"action":"{action}"}}"#)))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(
        power_ops_invocation_count() > before,
        "power op counter should increment"
    );
    (status, body)
}

#[tokio::test]
async fn ops_power_shutdown_accepted_ph_s1016() {
    let app = app();
    let (status, body) = post_power(&app, "shutdown").await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["accepted"], true);
    assert_eq!(body["action"], "shutdown");
    assert_eq!(body["dev_guard"], true);
}

#[tokio::test]
async fn ops_power_reboot_dev_guard_ph_s1016() {
    let app = app();
    let (status, body) = post_power(&app, "reboot").await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["action"], "reboot");
    assert!(body["note"].as_str().unwrap_or("").contains("dev"));
}

#[tokio::test]
async fn ops_power_invalid_action_400_ph_s1016() {
    let app = app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ops/power")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"action":"panic"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
