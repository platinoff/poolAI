# Policies Depth — Enterprise Phase D (Band 81+)

Canonical doc: [`POLICIES_DEPTH.md`](./POLICIES_DEPTH.md) (band 81, PH-S1455).

## Overview

Band 81 scaffolds the **security policies production path** for FM-horizon v2 (enterprise §5.17 criterion — Policies / secrets → persist + rotation wire).

Today `SecurityManager` keeps **in-memory** `SecurityPolicy` CRUD. Horizon path:

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_POLICY_STORE=memory` (default) | Current — API/admin CRUD |
| `sqlite` | `POOLAI_POLICY_STORE=sqlite` | Band 82 store wire — see [`POLICIES_STORE.md`](./POLICIES_STORE.md) |

Production verify stub (PH-S1450): `validate_security_policy_fields` requires non-empty **name** and **session_timeout** in `1..=86400` under `cargo test-ci`. Full persisted CRUD / RBAC cross-tenant remains later in phase D.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --policy
cargo run --bin poolai-loc-audit -- --policy --advisory --min-ratio 0.95
VERIFY_POLICY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_mode` | `true` when `--policy` (PH-S1454) |
| `policy_criteria_total` | Registry size (8) |
| `policy_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_depth.rs`](../../crates/poolai-ui-core/src/policy_depth.rs)
- domain: [`security.rs`](../../src/enterprise/security.rs) — `POOLAI_POLICY_STORE`, `policy_store_mode()`, field stub
- tests: `policy_depth_audit.rs`, `galaxy_horizon_s1449_integration.rs`
- stand-smoke export shape: `policy_band81_export_shape_ph_s1453`
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.62 band 81 journal
- Phase C Audit closed at band 80 — see [`AUDIT_HORIZON.md`](./AUDIT_HORIZON.md)
- Mirror: [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md) · [`SSO_DEPTH.md`](./SSO_DEPTH.md)
- Store wire (band 82): [`POLICIES_STORE.md`](./POLICIES_STORE.md)
- PH-S1449 · policy_depth · POOLAI_POLICY_STORE · PH-S1452 · VERIFY_POLICY · PH-S1454 · --policy · PH-S1458
