# Monitoring Admin/Ops Glue — Enterprise Phase E (Band 94)

Canonical doc: [`MONITORING_ADMIN_OPS.md`](./MONITORING_ADMIN_OPS.md) (band 94, PH-S1586).
Prev: [`MONITORING_API.md`](./MONITORING_API.md) (band 93 ✅). Next: Monitoring stand smoke (band 95). Mirror: [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md).

## Overview

Band 94 wires **admin UI + ops hooks** for monitoring store/query surfaces already
verified as HTTP contracts in band 93 ([`MONITORING_API.md`](./MONITORING_API.md)).

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin/monitoring` `#monitoring-store-badge` | `GET /api/enterprise/monitoring/store` |
| Query refresh | Monitoring header **Refresh** (`refreshMonitoring`) | alerts / dashboards reload |
| Verify / quick | `VERIFY_MONITORING_ADMIN_OPS` / `--monitoring-admin-ops` | loc-audit gate |

**Boundary:** band 94 = admin/ops glue + verify/loc-audit; live stand smoke is a
later phase-E band. Master backlog template rows for band 94 are overridden here
(mirror Policies band 84). Warnings-first: clippy `required-features` on
mis-gated integration tests folded into PH-S1579.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --monitoring-admin-ops
cargo run --bin poolai-loc-audit -- --monitoring-admin-ops --advisory --min-ratio 0.95
VERIFY_MONITORING_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_admin_ops_mode` | `true` when `--monitoring-admin-ops` (PH-S1585) |
| `monitoring_admin_ops_criteria_total` | Registry size (10) |
| `monitoring_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/monitoring_admin_ops_depth.rs)
- admin UI: [`monitoring.rs`](../../src/ui/admin/monitoring.rs)
- tests: `monitoring_admin_ops_integration.rs`, `galaxy_horizon_s1579_integration.rs`
- API contracts: [`MONITORING_API.md`](./MONITORING_API.md)
- store wire: [`MONITORING_STORE.md`](./MONITORING_STORE.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.75 band 94 journal
- OpenAPI tag `EnterpriseMonitoring` + schema `MonitoringStoreWire`
- PH-S1579 · monitoring_admin_ops_depth · PH-S1580 · monitoring-store-badge · PH-S1581 · refreshMonitoring · PH-S1584 · VERIFY_MONITORING_ADMIN_OPS · PH-S1585 · --monitoring-admin-ops · PH-S1588
