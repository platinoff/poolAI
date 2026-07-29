# Monitoring Stand Smoke — Enterprise Phase E (Band 95)

Canonical doc: [`MONITORING_STAND_SMOKE.md`](./MONITORING_STAND_SMOKE.md) (band 95, PH-S1595).

## Overview

Band 95 adds **live HTTP stand smoke** for the monitoring store wire, alerts
query and alert-rule validation fixtures against a running coordinator
(`POOLAI_BASE_URL`), plus loc-audit / verify hooks. In-process CI canon remains
`tests/monitoring_stand_smoke_integration.rs` (no stand).

| Surface | Where | Notes |
|---------|-------|-------|
| Store wire | `GET /api/enterprise/monitoring/store` | `{mode,durable_path,configured}` |
| Alerts query | `GET /api/enterprise/monitoring/alerts?limit=5` | optional `severity=` filter; array body |
| Field fixtures | `POST /api/enterprise/monitoring/alert-rules/validate` | empty name / invalid operator → 400 `MONITORING_MISSING_NAME` / `MONITORING_INVALID_OPERATOR` |
| CLI | `--monitoring-stand-smoke` / `POOLAI_STAND_SMOKE_MONITORING=1` | live suite |
| Verify / quick | `VERIFY_MONITORING_STAND_SMOKE` / `--monitoring-stand-smoke` | live + loc-audit |

**Boundary:** band 95 = live stand smoke + ops gates. Prior: [`MONITORING_ADMIN_OPS.md`](./MONITORING_ADMIN_OPS.md)
(band 94), [`MONITORING_API.md`](./MONITORING_API.md) (band 93), [`MONITORING_STORE.md`](./MONITORING_STORE.md) (band 92).
Next: Monitoring loc-audit aggregate (band 96). Mirror: [`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md) (band 85).

## Live smoke / loc-audit / verify

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --monitoring-stand-smoke
# or: POOLAI_STAND_SMOKE_MONITORING=1

cargo run --bin poolai-loc-audit -- --monitoring-stand-smoke
cargo run --bin poolai-loc-audit -- --monitoring-stand-smoke --advisory --min-ratio 0.95
VERIFY_MONITORING_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_stand_smoke_mode` | `true` when `--monitoring-stand-smoke` (PH-S1594) |
| `monitoring_stand_smoke_criteria_total` | Registry size (10) |
| `monitoring_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/monitoring_stand_smoke_depth.rs)
- live runners: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_monitoring_*`)
- tests: `monitoring_stand_smoke_integration.rs`, `galaxy_horizon_s1589_integration.rs`
- admin/ops (band 94): [`MONITORING_ADMIN_OPS.md`](./MONITORING_ADMIN_OPS.md)
- API contracts (band 93): [`MONITORING_API.md`](./MONITORING_API.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.76 band 95 journal
- OpenAPI tag `EnterpriseMonitoring` + schema `MonitoringStoreWire`
