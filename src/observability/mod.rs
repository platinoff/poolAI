//! FM-038: HTTP request tracing and optional OpenTelemetry export.
//! FM-043 / PH-S07: Prometheus pull metrics (`feature = "prometheus"`).
//!
//! - Always: `tower-http` [`TraceLayer`] spans (`http.request`) bridged to `tracing`.
//! - Feature `otel`: W3C `traceparent` extraction + OTLP export when
//!   `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
//! - Feature `prometheus`: `GET /metrics` text exposition (complements OTLP, not a duplicate).

mod http_trace;
pub mod lease_trace;
mod tracing_init;

#[cfg(feature = "prometheus")]
pub mod prometheus_export;

pub use http_trace::{apply_http_trace, make_http_span};
pub use lease_trace::{
    trace_acquire_success, trace_lease_reject, trace_renew_success, LeaseOperation, LeaseOutcome,
    LeaseSource,
};
pub use tracing_init::{init_tracing, OtelGuard};

#[cfg(feature = "prometheus")]
pub use prometheus_export::{
    apply_prometheus_http_layer, encode_metrics_text, init_prometheus, metrics_handler,
    record_http_request, record_secret_rotation,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tracing_without_otel_does_not_panic() {
        let _guard = init_tracing();
    }

    #[cfg(feature = "otel")]
    #[test]
    fn otel_export_skipped_without_endpoint() {
        std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
        let guard = init_tracing();
        assert!(!guard.export_enabled());
    }
}
