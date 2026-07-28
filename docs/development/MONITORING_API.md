# Monitoring HTTP API Contracts — Enterprise Phase E (Band 93)

Canonical doc: [`MONITORING_API.md`](./MONITORING_API.md) (band 93, PH-S1576). Prev: [`MONITORING_STORE.md`](./MONITORING_STORE.md) (band 92 ✅). Next: Monitoring admin/ops glue (band 94).

## Overview

Band 93 verifies **HTTP** contracts for `/api/enterprise/monitoring/*`
and `/api/enterprise/monitoring/store`
(query lifecycle, store-wire read, alert-rule field validation fixtures).
Complements band 92 [`MONITORING_STORE.md`](./MONITORING_STORE.md) (durable-path wire)
and band 91 [`MONITORING_DEPTH.md`](./MONITORING_DEPTH.md) (store env + field stub).

| Surface | Method | Auth | Notes |
|---------|--------|------|-------|
| `/monitoring/alerts` | GET | — | Query filters (`severity`, `acknowledged`) + `limit` pagination stub |
| `/monitoring/alert-rules` | GET | — | List alert rules |
| `/monitoring/metrics` | GET | — | Metric history query |
| `/monitoring/store` | GET | — | `MonitoringStoreWire` `{mode,durable_path,configured}` |
| `/monitoring/alert-rules/validate` | POST | — | Fixture: missing name / invalid operator → 400 |

**Boundary:** band 93 = HTTP contract matrix + OpenAPI `MonitoringStoreWire`; durable
sqlite CRUD and admin WASM remain later phase-E bands.
Master backlog template rows for band 93 are **overridden** here
(mirror Policies band 83 / [`POLICIES_API.md`](./POLICIES_API.md)).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --monitoring-api
VERIFY_MONITORING_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_api_mode` | `true` when `--monitoring-api` (PH-S1575) |
| `monitoring_api_criteria_total` | Registry size (9) |
| `monitoring_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/monitoring_api_contracts_depth.rs)
- handlers: [`monitoring.rs`](../../src/network/enterprise_api/monitoring.rs)
- tests: `monitoring_api_contracts_integration.rs`, `galaxy_horizon_s1569_integration.rs`
- store wire (band 92): [`MONITORING_STORE.md`](./MONITORING_STORE.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.74 band 93 journal
- OpenAPI tag `EnterpriseMonitoring` + schema `MonitoringStoreWire`
- PH-S1569 · monitoring_api_contracts_depth · PH-S1571 · GET /monitoring/store · PH-S1574 · VERIFY_MONITORING_API · PH-S1575 · --monitoring-api · PH-S1578
- rustc unused batch (vm/grid/job) folded into PH-S1569 from `rust_diagnostics` priority 0
