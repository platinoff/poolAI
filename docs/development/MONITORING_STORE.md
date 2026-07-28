# Monitoring Store Wire — Enterprise Phase E (Band 92)

Canonical doc: [`MONITORING_STORE.md`](./MONITORING_STORE.md) (band 92, PH-S1565).

## Overview

Band 92 wires the durable-path snapshot API for the monitoring store (FM-horizon v2,
enterprise §5.17 criterion — Monitoring durable alert_rules + dashboards). **SQLite CRUD**
for rules/dashboards already exists when `POOLAI_MONITORING_DATA_DIR` is set (band 91+ /
FM-030); this band adds an explicit wire label and ops hooks.

| Mode | Env | Status |
|------|-----|--------|
| `memory` | unset / `POOLAI_MONITORING_STORE=memory` | In-process maps (default) |
| `sqlite` unconfigured | `POOLAI_MONITORING_STORE=sqlite` without data dir | Wire label `sqlite_unconfigured` |
| `sqlite` configured | data dir set (`POOLAI_MONITORING_DATA_DIR`) and/or `STORE=sqlite` | Durable path → `…/monitoring.db` |

**Boundary:** band 92 resolves the wire (`monitoring_store_wire()`);
later phase-E bands deepen API contracts / admin ops / stand smoke.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --monitoring-store
cargo run --bin poolai-loc-audit -- --monitoring-store --advisory --min-ratio 0.95
VERIFY_MONITORING_STORE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-store
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_store_mode` | `true` when `--monitoring-store` (PH-S1564) |
| `monitoring_store_criteria_total` | Registry size (7) |
| `monitoring_store_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_store_depth.rs`](../../crates/poolai-ui-core/src/monitoring_store_depth.rs)
- domain: [`monitoring.rs`](../../src/enterprise/monitoring.rs) — `monitoring_store_wire()`
- scaffold (band 91): [`MONITORING_DEPTH.md`](./MONITORING_DEPTH.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.73 band 92 journal
- `POOLAI_MONITORING_DATA_DIR` — durable directory for `monitoring.db`
- Mirror: [`POLICIES_STORE.md`](./POLICIES_STORE.md) · [`AUDIT_STORE.md`](./AUDIT_STORE.md)
- PH-S1559 · monitoring_store_depth · PH-S1560 · monitoring_store_wire · PH-S1562 · VERIFY_MONITORING_STORE · PH-S1564 · --monitoring-store · PH-S1568
