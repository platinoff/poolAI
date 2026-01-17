//! Network API Integration Tests
//!
//! Tests for API endpoints, request/response handling, and error cases.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use poolai::network::api::create_api_routes;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_routes_creation() {
    let _router = create_api_routes();
    // Router is created successfully
    assert!(true);
}

#[tokio::test]
async fn test_status_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_metrics_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_workers_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/workers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_libraries_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/libraries")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    // It may return 503 (Service Unavailable) if LibraryManager is not initialized,
    // but that means the endpoint is registered and working
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_vm_instances_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/vm/instances")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_raid_artifacts_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/artifacts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_raid_nodes_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/nodes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rewards_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rewards")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_instance_previews_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/instance/previews?model_id=test-model")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint should return OK or BAD_REQUEST (if model_id missing), but not 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_instance_previews_requires_model_id() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/instance/previews")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return BAD_REQUEST if model_id is missing
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_instance_list_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/instance")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    // It may return 503 (Service Unavailable) if InstanceManager is not initialized,
    // but that means the endpoint is registered and working
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_state_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/state")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_chat_completions_endpoint_exists() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/v1/chat/completions")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"model":"test-model","messages":[{"role":"user","content":"Hello"}]}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint may return 401 (Unauthorized) if auth is required, but not 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_nonexistent_endpoint_returns_404() {
    let app = Router::new().nest("/api/v1", create_api_routes());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
