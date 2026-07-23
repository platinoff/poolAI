# Audit Store Wire — Enterprise Phase C (Band 72)

Canonical doc: [`AUDIT_STORE.md`](./AUDIT_STORE.md) (band 72, PH-S1365).

## Overview

Band 72 wires the durable-path stub for the audit event store (FM-horizon v2,
enterprise §5.17 criterion — queryable audit). **Restart-safe SQLite CRUD**
remains a later phase-C band (API contracts / persist), mirroring SSO
(wire band 62 → CRUD later) and tenants (wire band 52 → CRUD band 59).

| Mode | Env | Status |
|------|-----|--------|
| `file` | `POOLAI_AUDIT_STORE=file` (default) | File-based events under `./data/audit` (band 71+) |
| `sqlite` unconfigured | `POOLAI_AUDIT_STORE=sqlite` without data dir | Wire label `sqlite_unconfigured` |
| `sqlite` configured | `POOLAI_AUDIT_STORE=sqlite` + `POOLAI_AUDIT_DATA_DIR=…` | Durable path → `…/audit.sqlite` |

**Boundary:** band 72 resolves the wire (`audit_store_wire()`);
later bands persist audit event CRUD / SIEM export via sqlite.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --audit-store
cargo run --bin poolai-loc-audit -- --audit-store --advisory --min-ratio 0.95
VERIFY_AUDIT_STORE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-store
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_store_mode` | `true` when `--audit-store` (PH-S1364) |
| `audit_store_criteria_total` | Registry size (7) |
| `audit_store_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_store_depth.rs`](../../crates/poolai-ui-core/src/audit_store_depth.rs)
- domain: [`audit.rs`](../../src/enterprise/audit.rs) — `audit_store_wire()`
- scaffold (band 71): [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.53 band 72 journal
- `POOLAI_AUDIT_DATA_DIR` — durable directory for future sqlite file
- Mirror: [`SSO_STORE.md`](./SSO_STORE.md)
- PH-S1359 · audit_store_depth · PH-S1360 · audit_store_wire · PH-S1362 · VERIFY_AUDIT_STORE · PH-S1364 · --audit-store · PH-S1368
