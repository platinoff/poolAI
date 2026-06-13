# Prometheus metrics export (PH-S07 / FM-043)

**Status:** Pull-model exposition at **`GET /metrics`** (root path). Complements FM-038 OTLP tracing — does not replace it.

## Build

```bash
cargo build --release --features enterprise,ml,cloud,prometheus
```

`cargo test-ci` includes `prometheus` in the feature set.

## Scrape

| Item | Value |
|------|--------|
| Path | `GET /metrics` |
| Format | Prometheus text `0.0.4` (`Content-Type: text/plain; version=0.0.4; charset=utf-8`) |
| Auth | None by default (place scraper on trusted network or front with mTLS / reverse-proxy ACL) |

Example:

```bash
curl -sS http://127.0.0.1:8080/metrics | head -20
```

Prometheus `scrape_configs`:

```yaml
scrape_configs:
  - job_name: poolai
    metrics_path: /metrics
    static_configs:
      - targets: ["127.0.0.1:8080"]
```

## Metric families

| Name | Type | Source |
|------|------|--------|
| `poolai_build_info` | gauge | Crate version label |
| `poolai_uptime_seconds` | gauge | `version::get_uptime_seconds()` |
| `poolai_workers_active` | gauge | `AppState` system snapshot |
| `poolai_system_total_requests` | gauge | `SystemMetrics.total_requests` |
| `poolai_http_requests_total` | counter | HTTP middleware (`method`, `status`) |
| `poolai_http_request_duration_seconds` | histogram | HTTP middleware (`method`) |
| `poolai_monitoring_alert_rules` | gauge | Enterprise `MonitoringManager` (feature `enterprise`) |
| `poolai_monitoring_dashboards` | gauge | Enterprise dashboards count |
| `galaxy_pricing_fresh_served` | gauge | Galaxy pricing oracle L1 fresh serves (PH-S127) |
| `galaxy_pricing_stale_served` | gauge | Galaxy pricing oracle L1 stale serves (PH-S127) |
| `galaxy_pricing_forced_fallback_total` | gauge | Galaxy pricing oracle forced L2 quotes (PH-S127) |
| `galaxy_trust_payout_eligible_total` | gauge | Galaxy trust gate edge payout-eligible results (PH-S137) |
| `galaxy_trust_payout_held_total` | gauge | Galaxy trust gate edge payout-held results (PH-S137) |
| `process_*` | various | `prometheus` process collector when available |

JSON metrics for the admin UI remain at **`GET /api/v1/metrics`** — different contract.

## Alerting export

Prometheus alert rules live in your monitoring stack (Prometheus / Alertmanager / Grafana). PoolAI enterprise **alert rules** (`/api/enterprise/monitoring/alert-rules`) are reflected as the `poolai_monitoring_alert_rules` gauge on scrape; firing logic stays in Prometheus or enterprise monitoring APIs.

## Related docs

- [`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md) — traces (OTLP)
- [`../performance/BENCHMARKS.md`](../performance/BENCHMARKS.md) — load baselines
- [`../performance/PROFILING.md`](../performance/PROFILING.md) — hot-path profiling

**Last updated:** 2026-05-29 (PH-S127 galaxy pricing oracle gauges).
