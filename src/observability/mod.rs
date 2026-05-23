//! FM-038: HTTP request tracing and optional OpenTelemetry export.
//!
//! - Always: `tower-http` [`TraceLayer`] spans (`http.request`) bridged to `tracing`.
//! - Feature `otel`: W3C `traceparent` extraction + OTLP export when
//!   `OTEL_EXPORTER_OTLP_ENDPOINT` is set.

mod http_trace;
mod tracing_init;

pub use http_trace::apply_http_trace;
pub use tracing_init::{init_tracing, OtelGuard};

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
