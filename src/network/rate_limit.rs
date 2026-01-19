//! Rate Limiting Middleware
//!
//! Provides rate limiting for API endpoints to prevent abuse and DoS attacks.
//! Supports per-IP rate limiting with configurable limits and time windows.
//!
//! # Features
//!
//! - **Per-IP Rate Limiting**: Track requests per client IP address
//! - **Configurable Limits**: Different limits for different endpoint types
//! - **Sliding Window**: Time-based rate limiting with configurable windows
//! - **Burst Allowance**: Allow short bursts of requests
//! - **Custom Error Responses**: Rate limit exceeded responses with retry-after headers
//!
//! # Example
//!
//! ```no_run
//! use poolai::network::rate_limit::{RateLimitLayer, RateLimitConfig};
//!
//! # async fn example() {
//! let config = RateLimitConfig {
//!     requests_per_minute: 100,
//!     burst_size: 10,
//!     window_seconds: Some(60),
//! };
//!
//! let rate_limit = RateLimitLayer::new(config);
//!
//! // Use rate_limit.middleware() in your Axum middleware chain
//! // Example: router.layer(middleware::from_fn(|req, next| {
//! //     rate_limit.clone().middleware(req, next)
//! // }))
//! # }
//! ```

use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::warn;

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed per time window
    pub requests_per_minute: usize,
    /// Burst size (additional requests allowed in a short time)
    pub burst_size: usize,
    /// Time window in seconds (default: 60 seconds)
    pub window_seconds: Option<u64>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 10,
            window_seconds: Some(60),
        }
    }
}

/// Request entry for rate limiting
#[derive(Debug, Clone)]
struct RequestEntry {
    count: usize,
    window_start: Instant,
}

/// Rate limit state per IP address
#[derive(Debug, Clone)]
struct RateLimitState {
    entries: HashMap<IpAddr, RequestEntry>,
    last_cleanup: Instant,
}

impl RateLimitState {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            last_cleanup: Instant::now(),
        }
    }

    /// Check if request is allowed for the given IP
    fn is_allowed(
        &mut self,
        ip: IpAddr,
        config: &RateLimitConfig,
    ) -> Result<(), (usize, Duration)> {
        let now = Instant::now();
        let window = Duration::from_secs(config.window_seconds.unwrap_or(60));

        // Cleanup old entries periodically (every 5 minutes)
        if now.duration_since(self.last_cleanup) > Duration::from_secs(300) {
            self.cleanup(now, window);
            self.last_cleanup = now;
        }

        let entry = self.entries.entry(ip).or_insert_with(|| RequestEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) > window {
            entry.count = 0;
            entry.window_start = now;
        }

        // Check rate limit
        let max_requests = config.requests_per_minute + config.burst_size;
        if entry.count >= max_requests {
            let retry_after = window - now.duration_since(entry.window_start);
            return Err((entry.count, retry_after));
        }

        // Increment request count
        entry.count += 1;
        Ok(())
    }

    /// Cleanup old entries outside the time window
    fn cleanup(&mut self, now: Instant, window: Duration) {
        self.entries
            .retain(|_, entry| now.duration_since(entry.window_start) <= window);
    }
}

/// Rate limit store (shared across requests)
type RateLimitStore = Arc<RwLock<RateLimitState>>;

/// Rate limit layer for Axum
#[derive(Clone)]
pub struct RateLimitLayer {
    config: RateLimitConfig,
    store: RateLimitStore,
}

impl RateLimitLayer {
    /// Create rate limit layer with configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            store: Arc::new(RwLock::new(RateLimitState::new())),
        }
    }

    /// Create rate limit middleware function
    pub async fn middleware(&self, req: Request, next: Next) -> Response {
        // Extract client IP from request
        let ip = extract_client_ip(&req);

        // Check rate limit
        let mut store = self.store.write().await;
        match store.is_allowed(ip, &self.config) {
            Ok(()) => {
                // Request allowed, continue
                drop(store);
                let mut response = next.run(req).await;
                add_rate_limit_headers(&mut response, &self.config);
                response
            }
            Err((count, retry_after)) => {
                // Rate limit exceeded
                drop(store);
                warn!(
                    "Rate limit exceeded for IP {}: {} requests (limit: {})",
                    ip, count, self.config.requests_per_minute
                );
                create_rate_limit_response(retry_after)
            }
        }
    }
}

/// Extract client IP address from request
fn extract_client_ip(req: &Request) -> IpAddr {
    // Try to get IP from X-Forwarded-For header (for proxied requests)
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(header_str) = forwarded_for.to_str() {
            // Take the first IP in the chain
            if let Some(first_ip) = header_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(header_str) = real_ip.to_str() {
            if let Ok(ip) = header_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    // Fallback: try to get from ConnectInfo extension (if available)
    // Note: This requires the handler to set ConnectInfo
    // For now, use a default IP if not available
    std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)
}

/// Create rate limit exceeded response
fn create_rate_limit_response(retry_after: Duration) -> Response {
    let body = serde_json::json!({
        "error": "Rate limit exceeded",
        "message": "Too many requests. Please try again later.",
        "retry_after": retry_after.as_secs()
    });

    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )
        .header(
            axum::http::header::RETRY_AFTER,
            HeaderValue::from_str(&retry_after.as_secs().to_string())
                .unwrap_or_else(|_| HeaderValue::from_static("60")),
        )
        .body(axum::body::Body::from(
            serde_json::to_string(&body).unwrap(),
        ))
        .unwrap()
}

/// Add rate limit headers to successful responses
fn add_rate_limit_headers(response: &mut Response, config: &RateLimitConfig) {
    let headers = response.headers_mut();
    headers.insert(
        axum::http::header::HeaderName::from_static("x-ratelimit-limit"),
        HeaderValue::from_str(&config.requests_per_minute.to_string())
            .unwrap_or_else(|_| HeaderValue::from_static("100")),
    );
    headers.insert(
        axum::http::header::HeaderName::from_static("x-ratelimit-window"),
        HeaderValue::from_str(&config.window_seconds.unwrap_or(60).to_string())
            .unwrap_or_else(|_| HeaderValue::from_static("60")),
    );
}
