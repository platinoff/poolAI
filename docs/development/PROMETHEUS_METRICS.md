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
| `galaxy_pricing_cache_age_seconds` | gauge | Galaxy pricing L1 cache age seconds last observed (PH-S168) |
| `galaxy_pricing_provider_catalog_lookups_total` | gauge | Galaxy pricing provider catalog allow-list lookups (PH-S172) |
| `galaxy_pricing_provider_catalog_hits_total` | gauge | Galaxy pricing provider catalog allow-list hits (PH-S172) |
| `galaxy_pricing_provider_errors_total` | gauge | Galaxy pricing live provider HTTP fetch failures (PH-S173) |
| `galaxy_pricing_quote_usd_micro` | gauge | Galaxy pricing last served PoolAI quote micro-USD (PH-S174) |
| `galaxy_pricing_market_min_usd_micro` | gauge | Galaxy pricing last observed market min micro-USD (PH-S181) |
| `galaxy_trust_score` | gauge | Galaxy last observed grid result trust score 0..=100 (PH-S182) |
| `galaxy_shard_local_hit_ratio` | gauge | Galaxy last top-ranked shard local hit ratio basis points 0-10000 (PH-S183) |
| `galaxy_prefetch_bytes_total` | gauge | Galaxy estimated prefetch bytes scheduled in plans (PH-S184 stub) |
| `galaxy_cross_region_egress_mb` | gauge | Galaxy last observed cross-region egress whole MB (PH-S185) |
| `galaxy_verification_sample_scheduled_total` | gauge | Galaxy verification stub samples scheduled on grid result path (PH-S164; PH-S186 /metrics) |
| `galaxy_verification_mismatch_total` | gauge | Galaxy verification digest mismatches on grid result path (PH-S175) |
| `galaxy_verification_sample_total` | gauge | Galaxy verification samples scheduled on grid result path (PH-S177) |
| `galaxy_verification_match_total` | gauge | Galaxy verification digest matches on grid result path (PH-S180) |
| `galaxy_replay_pending` | gauge | Galaxy replay verifications pending coordinator verdict (PH-S176) |
| `galaxy_settlement_pending_verification_total` | gauge | Galaxy settlement holds pending verification on grid result path (PH-S178) |
| `galaxy_settlement_cleared_total` | gauge | Galaxy settlement cleared on grid result path (PH-S187) |
| `galaxy_settlement_not_applicable_total` | gauge | Galaxy settlement not applicable on grid result path (PH-S354) |
| `galaxy_verification_sample_not_applicable_total` | gauge | Galaxy verification samples not applicable on local origin path (PH-S356) |
| `galaxy_fee_split_applied_total` | gauge | Galaxy fee split applied on grid result path (PH-S194) |
| `galaxy_replication_strict_total` | gauge | Galaxy replication strict tier grid job ingests (PH-S179) |

**Queued (FM §5.12 PH-S191…S200):** vision + code-first band — див. [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.
| `galaxy_trust_payout_eligible_total` | gauge | Galaxy trust gate edge payout-eligible results (PH-S137 stub; PH-S163 grid wire) |
| `galaxy_trust_payout_held_total` | gauge | Galaxy trust gate edge payout-held results (PH-S137 stub; PH-S163 grid wire) |
| `process_*` | various | `prometheus` process collector when available |

JSON metrics for the admin UI remain at **`GET /api/v1/metrics`** — different contract.

## Alerting export

Prometheus alert rules live in your monitoring stack (Prometheus / Alertmanager / Grafana). PoolAI enterprise **alert rules** (`/api/enterprise/monitoring/alert-rules`) are reflected as the `poolai_monitoring_alert_rules` gauge on scrape; firing logic stays in Prometheus or enterprise monitoring APIs.

## Related docs

- [`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md) — traces (OTLP)
- [`../performance/BENCHMARKS.md`](../performance/BENCHMARKS.md) — load baselines
- [`../performance/PROFILING.md`](../performance/PROFILING.md) — hot-path profiling

- [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) — stand smoke band PH-S244…S256 (`poolai-http-stand-smoke`)

**Last updated:** 2026-06-17 (PH-S254…S256 stand smoke ✅; Galaxy metrics stand smoke band complete).
