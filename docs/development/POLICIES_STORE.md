# Policies Store Wire — Enterprise Phase D (Band 82)

Canonical doc: [`POLICIES_STORE.md`](./POLICIES_STORE.md) (band 82, PH-S1465).

## Overview

Band 82 wires the durable-path stub for the security policy store (FM-horizon v2,
enterprise §5.17 criterion — Policies / secrets). **Restart-safe SQLite CRUD**
remains a later phase-D band (API contracts / persist), mirroring Audit
(wire band 72 → CRUD later) and SSO (wire band 62 → CRUD later).

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_POLICY_STORE=memory` (default) | In-memory `SecurityPolicy` CRUD (band 81+) |
| `sqlite` unconfigured | `POOLAI_POLICY_STORE=sqlite` without data dir | Wire label `sqlite_unconfigured` |
| `sqlite` configured | `POOLAI_POLICY_STORE=sqlite` + `POOLAI_POLICY_DATA_DIR=…` | Durable path → `…/policy.sqlite` |

**Boundary:** band 82 resolves the wire (`policy_store_wire()`);
band 83 exposes it over HTTP — see [`POLICIES_API.md`](./POLICIES_API.md);
later bands persist policy CRUD / RBAC via sqlite.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --policy-store
cargo run --bin poolai-loc-audit -- --policy-store --advisory --min-ratio 0.95
VERIFY_POLICY_STORE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-store
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_store_mode` | `true` when `--policy-store` (PH-S1464) |
| `policy_store_criteria_total` | Registry size (7) |
| `policy_store_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_store_depth.rs`](../../crates/poolai-ui-core/src/policy_store_depth.rs)
- domain: [`security.rs`](../../src/enterprise/security.rs) — `policy_store_wire()`
- scaffold (band 81): [`POLICIES_DEPTH.md`](./POLICIES_DEPTH.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.63 band 82 journal
- `POOLAI_POLICY_DATA_DIR` — durable directory for future sqlite file
- Mirror: [`AUDIT_STORE.md`](./AUDIT_STORE.md) · [`SSO_STORE.md`](./SSO_STORE.md)
- PH-S1459 · policy_store_depth · PH-S1460 · policy_store_wire · PH-S1462 · VERIFY_POLICY_STORE · PH-S1464 · --policy-store · PH-S1468
