# OpenTelemetry tracing (FM-038)

**Status:** HTTP spans via `tower-http` TraceLayer; OTLP export with feature `otel`.

## HTTP middleware

Every request through `network::start_server` gets a `http.request` span (`http.method`, `http.route`, `otel.name`).

With feature **`otel`**, incoming W3C `traceparent` / `tracestate` headers are linked as parent context.

## Build

```bash
cargo build --release --features enterprise,ml,cloud,otel
```

Default CI / `cargo test-ci` builds **without** `otel` (no extra OTLP deps). OTel integration tests:

```bash
cargo test --test observability_otel --features otel
```

## Export (OTLP HTTP)

| Env | Default | Purpose |
|-----|---------|---------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | — | Collector base URL (e.g. `http://127.0.0.1:4318`); **required** to enable export |
| `OTEL_SERVICE_NAME` | `poolai` | `service.name` resource attribute |
| `RUST_LOG` / `RUST_LOG` via subscriber | `info` | `tracing` filter (unchanged) |

Example (Jaeger OTLP HTTP):

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://127.0.0.1:4318
export OTEL_SERVICE_NAME=poolai
cargo run --features enterprise,ml,cloud,otel
```

Code: `src/observability/` (`tracing_init.rs`, `http_trace.rs`).

**Metrics (pull model):** FM-043 adds Prometheus text at `GET /metrics` (`feature = prometheus`) — see [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md). OTLP and Prometheus are complementary.

**Last updated:** 2026-05-23 (FM-038, FM-043 cross-link).
