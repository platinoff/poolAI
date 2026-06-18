use axum::body::Body;
use axum::http::header::{HeaderName, HeaderValue, WARNING};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::grid::galaxy_protocol_negotiation_metrics::record_protocol_negotiation_rejected;
use crate::grid::protocol_compat::{negotiate, CompatStatus, MIN_COORDINATOR_VERSION_DOCS_URL};
use crate::network::json_errors::api_json_error;

const HEADER_PROTOCOL: &str = "x-poolai-protocol";
const HEADER_PROTOCOL_COORDINATOR: &str = "x-poolai-protocol-coordinator";
const HEADER_PROTOCOL_COMPAT: &str = "x-poolai-protocol-compat";
const HEADER_PROTOCOL_DOCS: &str = "x-poolai-protocol-docs";

fn is_protocol_guarded_path(path: &str) -> bool {
    path == "/grid/envelope"
        || path == "/grid/pricing"
        || path == "/discovery/register-remote"
        || path == "/discovery/heartbeat-remote"
        || path.starts_with("/virtual-nodes/")
}

fn compat_status_label(status: CompatStatus) -> &'static str {
    match status {
        CompatStatus::Accepted => "accepted",
        CompatStatus::UpgradeRequired => "upgrade_required",
        CompatStatus::Unsupported => "unsupported",
    }
}

fn apply_protocol_headers(resp: &mut Response, coordinator: &str, status: CompatStatus) {
    let headers = resp.headers_mut();
    let _ = headers.insert(
        HeaderName::from_static(HEADER_PROTOCOL_COORDINATOR),
        HeaderValue::from_str(coordinator).unwrap_or_else(|_| HeaderValue::from_static("1.2")),
    );
    let _ = headers.insert(
        HeaderName::from_static(HEADER_PROTOCOL_COMPAT),
        HeaderValue::from_static(compat_status_label(status)),
    );
    if let Ok(v) = HeaderValue::from_str(MIN_COORDINATOR_VERSION_DOCS_URL) {
        let _ = headers.insert(HeaderName::from_static(HEADER_PROTOCOL_DOCS), v);
    }
    if status == CompatStatus::UpgradeRequired {
        let _ = headers.insert(
            WARNING,
            HeaderValue::from_static("299 - poolai protocol upgrade recommended"),
        );
    }
}

/// PH-S103: protocol header negotiation middleware for selected Galaxy wire routes.
pub async fn protocol_header_middleware(req: Request<Body>, next: Next) -> Response {
    if !is_protocol_guarded_path(req.uri().path()) {
        return next.run(req).await;
    }

    let worker_protocol = req
        .headers()
        .get(HEADER_PROTOCOL)
        .and_then(|v| v.to_str().ok());
    let negotiation = negotiate(worker_protocol);

    if negotiation.status == CompatStatus::Unsupported {
        record_protocol_negotiation_rejected();
        let (status, body) = api_json_error(
            "protocol_unsupported",
            format!(
                "worker protocol is unsupported; upgrade worker to {} (see compat matrix)",
                negotiation.min_coordinator_version
            ),
            None,
            StatusCode::FORBIDDEN,
        );
        let mut resp = (status, body).into_response();
        apply_protocol_headers(
            &mut resp,
            &negotiation.coordinator_protocol_version,
            negotiation.status,
        );
        return resp;
    }

    let mut resp = next.run(req).await;
    apply_protocol_headers(
        &mut resp,
        &negotiation.coordinator_protocol_version,
        negotiation.status,
    );
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
            .layer(axum::middleware::from_fn(protocol_header_middleware))
    }

    #[tokio::test]
    async fn guarded_route_rejects_unsupported_protocol() {
        let app = test_router();
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/grid/envelope")
                    .header(HEADER_PROTOCOL, "1.0")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn unguarded_route_skips_protocol_negotiation() {
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
        assert_eq!(res.status(), StatusCode::OK);
        assert!(res.headers().get(HEADER_PROTOCOL_COORDINATOR).is_none());
    }
}
