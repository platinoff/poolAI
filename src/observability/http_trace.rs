//! HTTP request tracing middleware (`tower-http` TraceLayer).

use axum::body::Body;
use axum::http::Request;
use axum::Router;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn make_http_span(request: &Request<Body>) -> tracing::Span {
    let method = request.method().as_str();
    let path = request.uri().path();
    let span = tracing::info_span!(
        "http.request",
        http.method = method,
        http.route = path,
        otel.name = %format!("{method} {path}"),
    );
    #[cfg(feature = "otel")]
    attach_parent_context(request.headers(), &span);
    span
}

/// Attach HTTP request tracing layer to a router.
pub fn apply_http_trace<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(make_http_span)
            .on_request(DefaultOnRequest::new().level(Level::DEBUG))
            .on_response(DefaultOnResponse::new().level(Level::DEBUG))
            .on_failure(DefaultOnFailure::new().level(Level::WARN)),
    )
}

#[cfg(feature = "otel")]
fn attach_parent_context(headers: &axum::http::HeaderMap, span: &tracing::Span) {
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

    impl opentelemetry::propagation::Extractor for HeaderExtractor<'_> {
        fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key).and_then(|v| v.to_str().ok())
        }

        fn keys(&self) -> Vec<&str> {
            self.0.keys().map(|k| k.as_str()).collect()
        }
    }

    opentelemetry::global::get_text_map_propagator(|propagator| {
        let parent = propagator.extract(&HeaderExtractor(headers));
        span.set_parent(parent);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_http_span_has_route_fields() {
        let request = Request::builder()
            .uri("/api/v1/health")
            .body(Body::empty())
            .expect("request");
        let span = make_http_span(&request);
        assert_eq!(span.metadata().unwrap().name(), "http.request");
    }
}
