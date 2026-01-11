//! Security Headers Middleware
//!
//! Provides middleware for adding security headers to HTTP/HTTPS responses.
//! Supports CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy, and more.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

/// Security headers configuration
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Content-Security-Policy header value
    pub content_security_policy: Option<String>,
    /// X-Frame-Options header value
    pub x_frame_options: Option<String>,
    /// X-Content-Type-Options header value
    pub x_content_type_options: Option<String>,
    /// Referrer-Policy header value
    pub referrer_policy: Option<String>,
    /// Permissions-Policy header value
    pub permissions_policy: Option<String>,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: None,
        }
    }
}

impl SecurityHeadersConfig {
    /// Create security headers config with custom settings
    pub fn new(
        content_security_policy: Option<String>,
        x_frame_options: Option<String>,
        x_content_type_options: Option<String>,
        referrer_policy: Option<String>,
    ) -> Self {
        Self {
            content_security_policy,
            x_frame_options: x_frame_options.or(Some("DENY".to_string())),
            x_content_type_options: x_content_type_options.or(Some("nosniff".to_string())),
            referrer_policy: referrer_policy
                .or(Some("strict-origin-when-cross-origin".to_string())),
            permissions_policy: None,
        }
    }

    /// Create minimal security headers config (only essential headers)
    pub fn minimal() -> Self {
        Self {
            content_security_policy: None,
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: None,
        }
    }
}

/// Middleware function to add security headers to responses
pub async fn security_headers_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let config = SecurityHeadersConfig::default();

    // Add Content-Security-Policy header
    if let Some(ref csp) = config.content_security_policy {
        if let Ok(header_value) = HeaderValue::from_str(csp) {
            response
                .headers_mut()
                .insert(axum::http::header::CONTENT_SECURITY_POLICY, header_value);
        }
    }

    // Add X-Frame-Options header
    if let Some(ref xfo) = config.x_frame_options {
        if let Ok(header_value) = HeaderValue::from_str(xfo) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("x-frame-options"),
                header_value,
            );
        }
    }

    // Add X-Content-Type-Options header
    if let Some(ref xcto) = config.x_content_type_options {
        if let Ok(header_value) = HeaderValue::from_str(xcto) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("x-content-type-options"),
                header_value,
            );
        }
    }

    // Add Referrer-Policy header
    if let Some(ref rp) = config.referrer_policy {
        if let Ok(header_value) = HeaderValue::from_str(rp) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("referrer-policy"),
                header_value,
            );
        }
    }

    // Add Permissions-Policy header (if configured)
    if let Some(ref pp) = config.permissions_policy {
        if let Ok(header_value) = HeaderValue::from_str(pp) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("permissions-policy"),
                header_value,
            );
        }
    }

    response
}
