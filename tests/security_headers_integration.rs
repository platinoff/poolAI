//! Integration tests for Security Headers middleware
//!
//! Tests:
//! - Security headers are added to all responses
//! - Content-Security-Policy header
//! - X-Frame-Options header
//! - X-Content-Type-Options header
//! - Referrer-Policy header

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
use poolai::network::api::create_api_routes;
use poolai::network::security_headers::{SecurityHeadersConfig, security_headers_middleware};
use axum::Router;
use axum::routing::get;

async fn setup_app() -> Router {
    let router = Router::new()
        .route("/test", get(|| async { "test response" }))
        .layer(axum::middleware::from_fn(security_headers_middleware));
    
    router
}

#[tokio::test]
async fn test_security_headers_present() {
    let app = setup_app().await;
    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    
    // Check X-Frame-Options header
    assert!(headers.contains_key("x-frame-options"));
    assert_eq!(
        headers.get("x-frame-options").unwrap().to_str().unwrap(),
        "DENY"
    );
    
    // Check X-Content-Type-Options header
    assert!(headers.contains_key("x-content-type-options"));
    assert_eq!(
        headers.get("x-content-type-options").unwrap().to_str().unwrap(),
        "nosniff"
    );
    
    // Check Referrer-Policy header
    assert!(headers.contains_key("referrer-policy"));
    assert_eq!(
        headers.get("referrer-policy").unwrap().to_str().unwrap(),
        "strict-origin-when-cross-origin"
    );
}

#[tokio::test]
async fn test_content_security_policy_header() {
    let app = setup_app().await;
    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    
    // Check Content-Security-Policy header
    assert!(headers.contains_key("content-security-policy"));
    let csp = headers.get("content-security-policy").unwrap().to_str().unwrap();
    assert!(csp.contains("default-src 'self'"));
    assert!(csp.contains("script-src 'self' 'unsafe-inline'"));
    assert!(csp.contains("style-src 'self' 'unsafe-inline'"));
}

#[tokio::test]
async fn test_security_headers_config_default() {
    let config = SecurityHeadersConfig::default();
    
    assert!(config.x_frame_options.is_some());
    assert_eq!(config.x_frame_options.as_ref().unwrap(), "DENY");
    
    assert!(config.x_content_type_options.is_some());
    assert_eq!(config.x_content_type_options.as_ref().unwrap(), "nosniff");
    
    assert!(config.referrer_policy.is_some());
    assert_eq!(
        config.referrer_policy.as_ref().unwrap(),
        "strict-origin-when-cross-origin"
    );
    
    assert!(config.content_security_policy.is_some());
    let csp = config.content_security_policy.as_ref().unwrap();
    assert!(csp.contains("default-src 'self'"));
}

#[tokio::test]
async fn test_security_headers_config_minimal() {
    let config = SecurityHeadersConfig::minimal();
    
    assert!(config.x_frame_options.is_some());
    assert_eq!(config.x_frame_options.as_ref().unwrap(), "DENY");
    
    assert!(config.x_content_type_options.is_some());
    assert_eq!(config.x_content_type_options.as_ref().unwrap(), "nosniff");
    
    assert!(config.referrer_policy.is_some());
    assert_eq!(
        config.referrer_policy.as_ref().unwrap(),
        "strict-origin-when-cross-origin"
    );
    
    // Minimal config should not have CSP
    assert!(config.content_security_policy.is_none());
}

#[tokio::test]
async fn test_security_headers_config_custom() {
    let config = SecurityHeadersConfig::new(
        Some("default-src 'self'".to_string()),
        Some("SAMEORIGIN".to_string()),
        Some("nosniff".to_string()),
        Some("no-referrer".to_string()),
    );
    
    assert_eq!(config.content_security_policy.as_ref().unwrap(), "default-src 'self'");
    assert_eq!(config.x_frame_options.as_ref().unwrap(), "SAMEORIGIN");
    assert_eq!(config.x_content_type_options.as_ref().unwrap(), "nosniff");
    assert_eq!(config.referrer_policy.as_ref().unwrap(), "no-referrer");
}

#[tokio::test]
async fn test_security_headers_on_api_endpoints() {
    let app = create_api_routes()
        .layer(axum::middleware::from_fn(security_headers_middleware));
    
    let response = app
        .oneshot(Request::builder().uri("/status").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let headers = response.headers();
    
    // Check that security headers are present
    assert!(headers.contains_key("x-frame-options"));
    assert!(headers.contains_key("x-content-type-options"));
    assert!(headers.contains_key("referrer-policy"));
}
