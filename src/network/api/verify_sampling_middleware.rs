//! PH-S164: expose configured Galaxy verification sample rate on grid HTTP routes.

use axum::body::Body;
use axum::http::header::{HeaderName, HeaderValue};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::grid::galaxy_verify_sampling::{
    format_verify_base_sample_rate_header, VerifySamplingConfig, HEADER_VERIFY_BASE_SAMPLE_RATE,
};

fn is_verify_sampling_grid_path(path: &str) -> bool {
    path == "/grid/envelope" || path == "/grid/pricing"
}

fn apply_verify_sampling_header(resp: &mut Response, config: VerifySamplingConfig) {
    let value = format_verify_base_sample_rate_header(config.base_sample_rate);
    if let Ok(header_value) = HeaderValue::from_str(&value) {
        let _ = resp.headers_mut().insert(
            HeaderName::from_static(HEADER_VERIFY_BASE_SAMPLE_RATE),
            header_value,
        );
    }
}

/// Attach env-backed verification sample rate header on grid wire routes.
pub async fn verify_sampling_middleware(req: Request<Body>, next: Next) -> Response {
    if !is_verify_sampling_grid_path(req.uri().path()) {
        return next.run(req).await;
    }
    let config = VerifySamplingConfig::from_env();
    let mut resp = next.run(req).await;
    apply_verify_sampling_header(&mut resp, config);
    resp
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    fn test_router() -> Router {
        Router::new()
            .route("/grid/envelope", get(|| async { "ok" }))
            .route("/status", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(verify_sampling_middleware))
    }

    #[tokio::test]
    async fn grid_route_includes_verify_sample_rate_header() {
        std::env::set_var(
            crate::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE,
            "0.25",
        );
        let app = test_router();
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/grid/envelope")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(
            res.headers()
                .get(HEADER_VERIFY_BASE_SAMPLE_RATE)
                .and_then(|v| v.to_str().ok()),
            Some("0.250000")
        );
        std::env::remove_var(crate::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE);
    }

    #[tokio::test]
    async fn non_grid_route_skips_verify_sampling_header() {
        let app = test_router();
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/status")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert!(res.headers().get(HEADER_VERIFY_BASE_SAMPLE_RATE).is_none());
    }
}
