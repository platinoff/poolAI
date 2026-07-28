# Monitoring Depth — Enterprise Phase E (Band 91+)

Canonical doc: [`MONITORING_DEPTH.md`](./MONITORING_DEPTH.md) (band 91, PH-S1555).

## Overview

Band 91 scaffolds the **enterprise monitoring production path** for FM-horizon v2 (enterprise §5.17 criterion — Monitoring durable alert_rules + dashboards).

Today `MonitoringManager` persists to SQLite when `POOLAI_MONITORING_DATA_DIR` is set (`monitoring.db`). Horizon path:

| Mode | Env | Status |
|------|-----|--------|
| `memory` | unset `POOLAI_MONITORING_DATA_DIR` | Current default — in-process maps |
| `sqlite` | `POOLAI_MONITORING_DATA_DIR=<dir>` | Durable wire (FM-030) — band 91+ |

Production verify stub (PH-S1550): `validate_monitoring_alert_fields` requires non-empty **name** / **metric** and a supported **operator** under `cargo test-ci`. Deeper store/API/admin bands follow 92–100.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --monitoring
cargo run --bin poolai-loc-audit -- --monitoring --advisory --min-ratio 0.95
VERIFY_MONITORING=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_mode` | `true` when `--monitoring` (PH-S1554) |
| `monitoring_criteria_total` | Registry size (8) |
| `monitoring_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_depth.rs`](../../crates/poolai-ui-core/src/monitoring_depth.rs)
- domain: [`monitoring.rs`](../../src/enterprise/monitoring.rs) — `POOLAI_MONITORING_DATA_DIR`, `monitoring_store_mode()`, field stub
- tests: `monitoring_depth_audit.rs`, `galaxy_horizon_s1549_integration.rs`
- stand-smoke export shape: `monitoring_band91_export_shape_ph_s1553`
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.72 band 91 journal
- Phase D Policies closed at band 90 — see [`POLICIES_HORIZON.md`](./POLICIES_HORIZON.md)
- Mirror: [`POLICIES_DEPTH.md`](./POLICIES_DEPTH.md) · [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md) · [`SSO_DEPTH.md`](./SSO_DEPTH.md)
- Store wire (band 92): master backlog Monitoring · store wire
- PH-S1549 · monitoring_depth · POOLAI_MONITORING_DATA_DIR · PH-S1552 · VERIFY_MONITORING · PH-S1554 · --monitoring · PH-S1558
