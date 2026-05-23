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
    /// Strict-Transport-Security (HSTS) header value
    pub strict_transport_security: Option<String>,
    /// X-Frame-Options header value
    pub x_frame_options: Option<String>,
    /// X-Content-Type-Options header value
    pub x_content_type_options: Option<String>,
    /// Referrer-Policy header value
    pub referrer_policy: Option<String>,
    /// Permissions-Policy header value
    pub permissions_policy: Option<String>,
    /// X-XSS-Protection header value (legacy, but still useful)
    pub x_xss_protection: Option<String>,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'".to_string()),
            strict_transport_security: Some("max-age=31536000; includeSubDomains; preload".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: Some("geolocation=(), microphone=(), camera=()".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
        }
    }
}

impl SecurityHeadersConfig {
    /// Create security headers config with custom settings
    pub fn new(
        content_security_policy: Option<String>,
        strict_transport_security: Option<String>,
        x_frame_options: Option<String>,
        x_content_type_options: Option<String>,
        referrer_policy: Option<String>,
    ) -> Self {
        Self {
            content_security_policy,
            strict_transport_security,
            x_frame_options: x_frame_options.or(Some("DENY".to_string())),
            x_content_type_options: x_content_type_options.or(Some("nosniff".to_string())),
            referrer_policy: referrer_policy
                .or(Some("strict-origin-when-cross-origin".to_string())),
            permissions_policy: Some("geolocation=(), microphone=(), camera=()".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
        }
    }

    /// Create minimal security headers config (only essential headers)
    pub fn minimal() -> Self {
        Self {
            content_security_policy: None,
            strict_transport_security: None,
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: None,
            x_xss_protection: None,
        }
    }
}

/// Build security headers from application config (HSTS only when HTTPS is enabled).
pub fn security_headers_from_app_config() -> SecurityHeadersConfig {
    let mut config = SecurityHeadersConfig::default();
    if let Ok(pool_config) = crate::core::config::get_config() {
        if pool_config.https.enabled {
            if let Ok(tls) =
                crate::network::tls_config::TlsConfig::from_https_config(&pool_config.https)
            {
                config.strict_transport_security = tls.hsts_header();
            }
        } else {
            config.strict_transport_security = None;
        }
    }
    config
}

/// Middleware function to add security headers to responses
pub async fn security_headers_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let config = security_headers_from_app_config();

    // Add Content-Security-Policy header
    if let Some(ref csp) = config.content_security_policy {
        if let Ok(header_value) = HeaderValue::from_str(csp) {
            response
                .headers_mut()
                .insert(axum::http::header::CONTENT_SECURITY_POLICY, header_value);
        }
    }

    // Add Strict-Transport-Security (HSTS) header (only for HTTPS)
    if let Some(ref hsts) = config.strict_transport_security {
        // Only add HSTS when using HTTPS (check if request was secure)
        // In practice, HSTS should be configured at the reverse proxy level,
        // but we add it here for completeness
        if let Ok(header_value) = HeaderValue::from_str(hsts) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("strict-transport-security"),
                header_value,
            );
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

    // Add X-XSS-Protection header (legacy, but still useful for older browsers)
    if let Some(ref xxss) = config.x_xss_protection {
        if let Ok(header_value) = HeaderValue::from_str(xxss) {
            response.headers_mut().insert(
                axum::http::header::HeaderName::from_static("x-xss-protection"),
                header_value,
            );
        }
    }

    response
}
