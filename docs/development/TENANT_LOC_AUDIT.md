# Tenant Loc-Audit Aggregate — Enterprise Phase A (Band 56)

Canonical doc: [`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md) (band 56, PH-S1205).

## Overview

Band 56 consolidates band 51–55 **`--tenant-*` loc-audit slices** under one aggregate
gate (`--tenant-loc-audit` / `VERIFY_TENANT_LOC_AUDIT`). Slice flags remain available
individually; the aggregate registry proves all five exist plus verify/docs/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `tenant_loc_audit_depth.rs` | enum + criteria + `TENANT_LOC_AUDIT_SLICES` |
| Slice flags | `poolai-loc-audit` | `--tenant-persist` … `--tenant-stand-smoke` |
| Aggregate | `--tenant-loc-audit` | `tenant_loc_audit_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_TENANT_LOC_AUDIT` / `--tenant-loc-audit` | loc-audit gate only |
| Contracts | `tenant_loc_audit_integration` | slice presence + criteria totals |

**Boundary:** band 56 = aggregate loc-audit ops gate; prior live smoke remains
[`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md) (band 55). Next: band 57 docs canon — [`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --tenant-loc-audit
cargo run --bin poolai-loc-audit -- --tenant-loc-audit --advisory --min-ratio 0.95
VERIFY_TENANT_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_loc_audit_mode` | `true` when `--tenant-loc-audit` (PH-S1204) |
| `tenant_loc_audit_criteria_total` | Registry size (10) |
| `tenant_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/tenant_loc_audit_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `tenant_loc_audit_integration.rs`, `galaxy_horizon_s1199_integration.rs`
- stand-smoke export shape: `tenant_loc_audit_band56_export_shape_ph_s1203`
- prior: [`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md) · [`TENANT_ADMIN_OPS.md`](./TENANT_ADMIN_OPS.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.37 band 56 journal
- Slices: `--tenant-persist` · `--tenant-store` · `--tenant-api` · `--tenant-admin-ops` · `--tenant-stand-smoke`
