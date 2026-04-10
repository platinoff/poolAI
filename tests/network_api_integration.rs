//! Network API Integration Tests
//!
//! Tests for API endpoints, request/response handling, and error cases.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_routes_creation() {
    let _router = create_api_routes();
}

#[tokio::test]
async fn test_status_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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

/// UI (dashboard + admin) expects these keys on each worker object when pool is empty (mock list).
#[tokio::test]
async fn test_workers_json_includes_ui_fields() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = v.as_array().expect("workers response is a JSON array");
    assert!(!arr.is_empty(), "mock workers list should be non-empty");
    for w in arr {
        let o = w.as_object().expect("worker is an object");
        for key in [
            "id",
            "status",
            "current_task",
            "is_healthy",
            "total_requests_processed",
            "queue_size",
            "active_connections",
            "average_response_time_ms",
        ] {
            assert!(
                o.contains_key(key),
                "worker JSON missing key `{key}`: {o:?}"
            );
        }
    }
}

#[tokio::test]
async fn test_libraries_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/vm/instances")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Registered route; 503 when VmManager is not attached (default test context).
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_raid_artifacts_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/artifacts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_raid_nodes_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/nodes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_rewards_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["error"]["code"], "VALIDATION_ERROR");
    assert!(v["error"]["message"].as_str().unwrap().contains("model_id"));
}

#[tokio::test]
async fn test_instance_list_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
async fn test_config_get_endpoint_registered() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/config")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    assert_ne!(status, StatusCode::NOT_FOUND);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        assert_eq!(v["error"]["code"], "CONFIG_GET_FAILED");
    } else {
        assert_eq!(status, StatusCode::OK);
        assert!(v.get("system").is_some() || v.get("pool").is_some());
    }
}

#[tokio::test]
async fn test_refresh_without_auth_header_returns_401_shape() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/refresh")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["error"]["code"], "AUTH_MISSING_HEADER");
}

#[tokio::test]
async fn test_chat_completions_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
async fn test_topology_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/topology")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_topology_latency_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/topology/latency")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_topology_nodes_endpoint_exists() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/topology/nodes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Endpoint exists if it returns something other than 404
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_nonexistent_endpoint_returns_404() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());
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
