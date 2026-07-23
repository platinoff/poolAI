# Audit Depth — Enterprise Phase C (Band 71+)

Canonical doc: [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md) (band 71, PH-S1355).

## Overview

Band 71 scaffolds the **audit production path** for FM-horizon v2 (enterprise §5.17 criterion — queryable audit + retention).

Today `AuditLogger` writes **file-based** events under `./data/audit`. Horizon path:

| Mode | Env | Status |
|------|-----|--------|
| `file` | `POOLAI_AUDIT_STORE=file` (default) | Current — rotation + query on disk |
| `sqlite` | `POOLAI_AUDIT_STORE=sqlite` | Band 72 store wire — see [`AUDIT_STORE.md`](./AUDIT_STORE.md) |

Production verify stub (PH-S1350): `validate_audit_event_fields` requires non-empty **action** + **resource_type** under `cargo test-ci`. Full SIEM export / retention remains later in phase C.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --audit
cargo run --bin poolai-loc-audit -- --audit --advisory --min-ratio 0.95
VERIFY_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_mode` | `true` when `--audit` (PH-S1354) |
| `audit_criteria_total` | Registry size (8) |
| `audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_depth.rs`](../../crates/poolai-ui-core/src/audit_depth.rs)
- domain: [`audit.rs`](../../src/enterprise/audit.rs) — `POOLAI_AUDIT_STORE`, `audit_store_mode()`, event field stub
- tests: `audit_depth_audit.rs`, `galaxy_horizon_s1349_integration.rs`
- stand-smoke export shape: `audit_band71_export_shape_ph_s1353`
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.52 band 71 journal
- Phase B SSO closed at band 70 — see [`SSO_HORIZON.md`](./SSO_HORIZON.md)
- Mirror: [`SSO_DEPTH.md`](./SSO_DEPTH.md) · store wire: [`AUDIT_STORE.md`](./AUDIT_STORE.md)
- PH-S1349 · audit_depth · POOLAI_AUDIT_STORE · PH-S1352 · VERIFY_AUDIT · PH-S1354 · --audit · PH-S1358
