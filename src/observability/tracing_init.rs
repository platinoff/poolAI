//! Tracing subscriber setup (`fmt` + optional OpenTelemetry OTLP layer).

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Keeps the OTel tracer provider alive until graceful shutdown.
pub struct OtelGuard {
    #[cfg(feature = "otel")]
    provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
    #[cfg(feature = "otel")]
    export_enabled: bool,
}

impl OtelGuard {
    #[cfg(feature = "otel")]
    pub fn export_enabled(&self) -> bool {
        self.export_enabled
    }
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        #[cfg(feature = "otel")]
        if let Some(provider) = self.provider.take() {
            if let Err(e) = provider.shutdown() {
                eprintln!("OpenTelemetry provider shutdown failed: {e}");
            }
        }
    }
}

/// Initialize global `tracing` subscriber (idempotent guard returned for OTel shutdown).
pub fn init_tracing() -> OtelGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    #[cfg(feature = "otel")]
    {
        let (provider, export_enabled) = init_otel_provider();
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        if export_enabled {
            if let Some(ref provider) = provider {
                use opentelemetry::trace::TracerProvider as _;
                let tracer = provider.tracer("poolai");
                let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
                let _ = Registry::default()
                    .with(env_filter)
                    .with(tracing_subscriber::fmt::layer())
                    .with(otel_layer)
                    .try_init();
                return OtelGuard {
                    provider: Some(provider.clone()),
                    export_enabled: true,
                };
            }
        }

        let _ = Registry::default()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .try_init();
        return OtelGuard {
            provider,
            export_enabled: false,
        };
    }

    #[cfg(not(feature = "otel"))]
    {
        let _ = Registry::default()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .try_init();
        OtelGuard {}
    }
}

#[cfg(feature = "otel")]
fn init_otel_provider() -> (Option<opentelemetry_sdk::trace::SdkTracerProvider>, bool) {
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::Resource;

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();
    let Some(endpoint) = endpoint.filter(|v| !v.trim().is_empty()) else {
        return (None, false);
    };

    let service_name = std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "poolai".into());

    let exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .build()
    {
        Ok(exporter) => exporter,
        Err(e) => {
            eprintln!("OpenTelemetry OTLP exporter init failed: {e}");
            return (None, false);
        }
    };

    let resource = Resource::builder_empty()
        .with_attribute(KeyValue::new("service.name", service_name))
        .build();

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    opentelemetry::global::set_tracer_provider(provider.clone());

    (Some(provider), true)
}
