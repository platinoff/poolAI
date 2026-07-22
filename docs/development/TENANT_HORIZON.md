# Tenant Horizon Close — Enterprise Phase A (Band 60)

Canonical doc: [`TENANT_HORIZON.md`](./TENANT_HORIZON.md) (band 60, PH-S1245).

## Overview

Band 60 closes **enterprise phase A (Tenants)** by aggregating bands 51–59
under one **horizon** gate (`--tenant-horizon` / `VERIFY_TENANT_HORIZON`).
Phase A delivered persistence, store wire, API contracts, admin/ops, stand smoke,
loc-audit, docs canon, vision-sync, and ratio-advisory (restart-safe SQLite CRUD).

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `tenant_horizon_depth.rs` | enum + criteria + `TENANT_HORIZON_SLICES` |
| Slice aggregate | all phase-A `--tenant-*` + `tenants.sqlite` | `--tenant-persist` · `--tenant-store` · `--tenant-api` · `--tenant-admin-ops` · `--tenant-stand-smoke` · `--tenant-loc-audit` · `--tenant-docs-canon` · `--tenant-vision-sync` · `--tenant-ratio-advisory` · `tenants.sqlite` |
| SQLite CRUD hold | `multi_tenancy.rs` | `persist_tenant_to_sqlite` (band 59) |
| Aggregate | `--tenant-horizon` | `tenant_horizon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_TENANT_HORIZON` / `--tenant-horizon` | phase-A horizon gate |
| Contracts | `tenant_horizon_integration` | slices + criteria totals |

**Boundary:** band 60 = phase A horizon close (no new tenant domain API).
Prior ratio-advisory: [`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md) (band 59).
Next: band 61 **B SSO** depth scaffold (PH-S1249…).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --tenant-horizon
cargo run --bin poolai-loc-audit -- --tenant-horizon --advisory --min-ratio 0.95
VERIFY_TENANT_HORIZON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-horizon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_horizon_mode` | `true` when `--tenant-horizon` (PH-S1244) |
| `tenant_horizon_criteria_total` | Registry size (10) |
| `tenant_horizon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_horizon_depth.rs`](../../crates/poolai-ui-core/src/tenant_horizon_depth.rs)
- domain hold: [`multi_tenancy.rs`](../../src/enterprise/multi_tenancy.rs) — `persist_tenant_to_sqlite`
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `tenant_horizon_integration.rs`, `galaxy_horizon_s1239_integration.rs`
- stand-smoke export shape: `tenant_horizon_band60_export_shape_ph_s1243`
- prior: [`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md) · [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md) · [`TENANT_STORE.md`](./TENANT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.41 band 60 journal
- Slices: `--tenant-persist` · `--tenant-store` · `--tenant-api` · `--tenant-admin-ops` · `--tenant-stand-smoke` · `--tenant-loc-audit` · `--tenant-docs-canon` · `--tenant-vision-sync` · `--tenant-ratio-advisory` · `tenants.sqlite`
- PH-S1239 · TENANT_HORIZON_SLICES · tenant_horizon_integration · VERIFY_TENANT_HORIZON · PH-S1244 · --tenant-horizon · PH-S1248
