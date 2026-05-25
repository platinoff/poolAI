//! PH-S07 / FM-043: Prometheus text exposition (`GET /metrics`).
//!
//! Pull-model metrics complement FM-038 OTLP tracing (no duplicate export path).
//! Gauges are refreshed on each scrape from [`ApiContext`] / enterprise monitoring.

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Router,
};
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, IntGauge, Opts, Registry, TextEncoder,
};
use std::sync::OnceLock;

use crate::core::state::ApiContext;

/// Lazily initialized Prometheus registry and metric handles.
pub struct PoolAiPrometheus {
    registry: Registry,
    http_requests_total: IntCounterVec,
    http_request_duration_seconds: HistogramVec,
    secret_rotations_total: IntCounterVec,
    workers_active: IntGauge,
    system_total_requests: IntGauge,
    uptime_seconds: IntGauge,
    build_info: IntGauge,
    #[cfg(feature = "enterprise")]
    monitoring_alert_rules: IntGauge,
    #[cfg(feature = "enterprise")]
    monitoring_dashboards: IntGauge,
}

static PROMETHEUS: OnceLock<PoolAiPrometheus> = OnceLock::new();

/// Register metrics once; safe to call repeatedly.
pub fn init_prometheus() -> &'static PoolAiPrometheus {
    PROMETHEUS.get_or_init(build_prometheus)
}

fn build_prometheus() -> PoolAiPrometheus {
    let registry = Registry::new();

    let http_requests_total = IntCounterVec::new(
        Opts::new("poolai_http_requests_total", "Total HTTP requests served"),
        &["method", "status"],
    )
    .expect("poolai_http_requests_total");
    registry
        .register(Box::new(http_requests_total.clone()))
        .expect("register poolai_http_requests_total");

    let http_request_duration_seconds = HistogramVec::new(
        HistogramOpts::new(
            "poolai_http_request_duration_seconds",
            "HTTP request wall time in seconds",
        )
        .buckets(vec![
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ]),
        &["method"],
    )
    .expect("poolai_http_request_duration_seconds");
    registry
        .register(Box::new(http_request_duration_seconds.clone()))
        .expect("register poolai_http_request_duration_seconds");

    let secret_rotations_total = IntCounterVec::new(
        Opts::new(
            "poolai_secret_rotations_total",
            "Secret rotation hook runs (PH-S24 / PH-S29)",
        ),
        &["kind", "success"],
    )
    .expect("poolai_secret_rotations_total");
    registry
        .register(Box::new(secret_rotations_total.clone()))
        .expect("register poolai_secret_rotations_total");

    let workers_active = IntGauge::with_opts(Opts::new(
        "poolai_workers_active",
        "Active workers from application system state",
    ))
    .expect("poolai_workers_active");
    registry
        .register(Box::new(workers_active.clone()))
        .expect("register poolai_workers_active");

    let system_total_requests = IntGauge::with_opts(Opts::new(
        "poolai_system_total_requests",
        "Cumulative request counter from system metrics snapshot",
    ))
    .expect("poolai_system_total_requests");
    registry
        .register(Box::new(system_total_requests.clone()))
        .expect("register poolai_system_total_requests");

    let uptime_seconds = IntGauge::with_opts(Opts::new(
        "poolai_uptime_seconds",
        "Process uptime since coordinator start",
    ))
    .expect("poolai_uptime_seconds");
    registry
        .register(Box::new(uptime_seconds.clone()))
        .expect("register poolai_uptime_seconds");

    let build_info = IntGauge::with_opts(
        Opts::new(
            "poolai_build_info",
            "PoolAI build metadata (value is always 1)",
        )
        .const_label("version", env!("CARGO_PKG_VERSION")),
    )
    .expect("poolai_build_info");
    build_info.set(1);
    registry
        .register(Box::new(build_info.clone()))
        .expect("register poolai_build_info");

    #[cfg(feature = "enterprise")]
    let monitoring_alert_rules = {
        let g = IntGauge::with_opts(Opts::new(
            "poolai_monitoring_alert_rules",
            "Enterprise monitoring alert rules (enabled + disabled)",
        ))
        .expect("poolai_monitoring_alert_rules");
        registry
            .register(Box::new(g.clone()))
            .expect("register poolai_monitoring_alert_rules");
        g
    };

    #[cfg(feature = "enterprise")]
    let monitoring_dashboards = {
        let g = IntGauge::with_opts(Opts::new(
            "poolai_monitoring_dashboards",
            "Enterprise monitoring dashboards persisted",
        ))
        .expect("poolai_monitoring_dashboards");
        registry
            .register(Box::new(g.clone()))
            .expect("register poolai_monitoring_dashboards");
        g
    };

    #[cfg(target_os = "linux")]
    {
        let collector = prometheus::process_collector::ProcessCollector::for_self();
        let _ = registry.register(Box::new(collector));
    }

    PoolAiPrometheus {
        registry,
        http_requests_total,
        http_request_duration_seconds,
        secret_rotations_total,
        workers_active,
        system_total_requests,
        uptime_seconds,
        build_info,
        #[cfg(feature = "enterprise")]
        monitoring_alert_rules,
        #[cfg(feature = "enterprise")]
        monitoring_dashboards,
    }
}

/// Record a secret rotation attempt (called from `security::secret_rotation`).
pub fn record_secret_rotation(kind: &str, success: bool) {
    let prom = init_prometheus();
    let success_label = if success { "true" } else { "false" };
    prom.secret_rotations_total
        .with_label_values(&[kind, success_label])
        .inc();
}

/// Record one completed HTTP request (called from middleware).
pub fn record_http_request(method: &str, status: u16, duration_secs: f64) {
    let prom = init_prometheus();
    let status_label = status.to_string();
    prom.http_requests_total
        .with_label_values(&[method, &status_label])
        .inc();
    prom.http_request_duration_seconds
        .with_label_values(&[method])
        .observe(duration_secs);
}

/// Refresh gauges from live application state before encoding the registry.
pub async fn refresh_scrape_gauges(ctx: &ApiContext) {
    let prom = init_prometheus();
    prom.uptime_seconds
        .set(crate::version::get_uptime_seconds() as i64);
    prom.build_info.set(1);

    let state = ctx.get_system_state();
    prom.workers_active.set(state.active_workers as i64);
    prom.system_total_requests
        .set(state.system_metrics.total_requests as i64);

    #[cfg(feature = "enterprise")]
    {
        if let Ok(rules) = ctx.enterprise_monitoring_manager.list_alert_rules().await {
            prom.monitoring_alert_rules.set(rules.len() as i64);
        }
        if let Ok(dashboards) = ctx
            .enterprise_monitoring_manager
            .list_dashboards(None)
            .await
        {
            prom.monitoring_dashboards.set(dashboards.len() as i64);
        }
    }
}

/// Encode all registered metrics as Prometheus text exposition format 0.0.4.
pub fn encode_metrics_text() -> Result<String, prometheus::Error> {
    let prom = init_prometheus();
    let families = prom.registry.gather();
    let mut buffer = Vec::new();
    TextEncoder::new().encode(&families, &mut buffer)?;
    String::from_utf8(buffer).map_err(|e| prometheus::Error::Msg(format!("utf8: {e}")))
}

/// `GET /metrics` — Prometheus scrape endpoint (not JSON `/api/v1/metrics`).
pub async fn metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    refresh_scrape_gauges(&ctx).await;
    match encode_metrics_text() {
        Ok(body) => (
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                "text/plain; version=0.0.4; charset=utf-8",
            )],
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("prometheus encode failed: {e}"),
        )
            .into_response(),
    }
}

/// Tower middleware: count requests and observe latency (skips scrape path to avoid noise).
pub async fn prometheus_http_metrics(request: Request<Body>, next: Next) -> Response<Body> {
    if request.uri().path() == "/metrics" {
        return next.run(request).await;
    }
    let method = request.method().as_str().to_string();
    let started = std::time::Instant::now();
    let response = next.run(request).await;
    let status = response.status().as_u16();
    record_http_request(&method, status, started.elapsed().as_secs_f64());
    response
}

/// Attach HTTP metrics middleware (inner relative to later layers).
pub fn apply_prometheus_http_layer<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(axum::middleware::from_fn(prometheus_http_metrics))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_encode_contains_poolai_metrics() {
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains("poolai_build_info"));
        assert!(body.contains("poolai_uptime_seconds"));
        assert!(body.contains("poolai_workers_active"));
    }

    #[test]
    fn record_http_request_increments_counter() {
        init_prometheus();
        record_http_request("GET", 200, 0.01);
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(r#"method="GET""#) || body.contains("method=\"GET\""));
        assert!(body.contains("poolai_http_requests_total"));
    }
}
