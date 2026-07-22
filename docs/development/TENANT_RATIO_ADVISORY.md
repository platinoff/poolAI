# Tenant Ratio Advisory — Enterprise Phase A (Band 59)

Canonical doc: [`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md) (band 59, PH-S1235).

## Overview

Band 59 aggregates prior tenant phase-A loc-audit slices under one
**ratio-advisory** gate (`--tenant-ratio-advisory` / `VERIFY_TENANT_RATIO_ADVISORY`)
and lands **restart-safe SQLite CRUD** for
`POOLAI_TENANT_STORE=sqlite` + `POOLAI_TENANT_DATA_DIR`.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `tenant_ratio_advisory_depth.rs` | enum + criteria + `TENANT_RATIO_ADVISORY_SLICES` |
| Slice aggregate | prior `--tenant-*` + `tenants.sqlite` | `--tenant-persist` · `--tenant-store` · `--tenant-api` · `--tenant-docs-canon` · `--tenant-vision-sync` · `tenants.sqlite` |
| SQLite CRUD | `multi_tenancy.rs` | `persist_tenant_to_sqlite` · load on `initialize` |
| Aggregate | `--tenant-ratio-advisory` | `tenant_ratio_advisory_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_TENANT_RATIO_ADVISORY` / `--tenant-ratio-advisory` | ratio-advisory gate |
| Contracts | `tenant_ratio_advisory_integration` · `tenant_sqlite_durable_integration` | slices + restart-safe create/get |

**Boundary:** band 59 = ratio-advisory ops gate + restart-safe tenant SQLite CRUD.
Prior vision-sync remains [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md) (band 58).
Store wire history: [`TENANT_STORE.md`](./TENANT_STORE.md). Next: [`TENANT_HORIZON.md`](./TENANT_HORIZON.md) (band 60 ✅).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --tenant-ratio-advisory
cargo run --bin poolai-loc-audit -- --tenant-ratio-advisory --advisory --min-ratio 0.95
VERIFY_TENANT_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-ratio-advisory
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_ratio_advisory_mode` | `true` when `--tenant-ratio-advisory` (PH-S1234) |
| `tenant_ratio_advisory_criteria_total` | Registry size (10) |
| `tenant_ratio_advisory_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/tenant_ratio_advisory_depth.rs)
- domain: [`multi_tenancy.rs`](../../src/enterprise/multi_tenancy.rs) — `persist_tenant_to_sqlite`
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `tenant_ratio_advisory_integration.rs`, `tenant_sqlite_durable_integration.rs`, `galaxy_horizon_s1229_integration.rs`
- stand-smoke export shape: `tenant_ratio_advisory_band59_export_shape_ph_s1233`
- prior: [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md) · [`TENANT_STORE.md`](./TENANT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.40 band 59 journal
- Slices: `--tenant-persist` · `--tenant-store` · `--tenant-api` · `--tenant-docs-canon` · `--tenant-vision-sync` · `tenants.sqlite`
- PH-S1229 · TENANT_RATIO_ADVISORY_SLICES · tenant_ratio_advisory_integration · VERIFY_TENANT_RATIO_ADVISORY · PH-S1234 · --tenant-ratio-advisory · PH-S1238
