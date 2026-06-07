//! FM-038: OpenTelemetry tracing (feature `otel`).

use axum::http::HeaderValue;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::propagation::TraceContextPropagator;

struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

impl opentelemetry::propagation::Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

#[test]
fn traceparent_header_is_extracted() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "traceparent",
        HeaderValue::from_static("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01"),
    );

    let propagator = TraceContextPropagator::new();
    let ctx = propagator.extract(&HeaderExtractor(&headers));
    assert!(!ctx
        .span()
        .span_context()
        .trace_id()
        .to_bytes()
        .iter()
        .all(|&b| b == 0));
}

#[test]
fn init_tracing_without_otlp_endpoint_skips_export() {
    std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
    let guard = poolai::observability::init_tracing();
    assert!(!guard.export_enabled());
}

#[test]
fn lease_reject_span_contract_name() {
    use poolai::observability::lease_trace::{
        trace_lease_reject, LeaseOperation, LeaseOutcome, LeaseSource,
    };
    let span = tracing::info_span!("otel-lease-reject-test");
    let _guard = span.enter();
    trace_lease_reject(
        "job-otel-1",
        LeaseOperation::Renew,
        LeaseSource::Api,
        LeaseOutcome::Rejected,
        "lease_epoch_rejected",
        Some(1),
        Some(0),
        Some(409),
    );
}
