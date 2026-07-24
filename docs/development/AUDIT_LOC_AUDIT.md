# Audit Loc-Audit Aggregate — Enterprise Phase C (Band 76)

Canonical doc: [`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md) (band 76, PH-S1405).

## Overview

Band 76 consolidates band 71–75 **`--audit*` loc-audit slices** under one aggregate
gate (`--audit-loc-audit` / `VERIFY_AUDIT_LOC_AUDIT`). Slice flags remain available
individually; the aggregate registry proves all five exist plus verify/docs/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `audit_loc_audit_depth.rs` | enum + criteria + `AUDIT_LOC_AUDIT_SLICES` |
| Slice flags | `poolai-loc-audit` | `--audit` … `--audit-stand-smoke` |
| Aggregate | `--audit-loc-audit` | `audit_loc_audit_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_AUDIT_LOC_AUDIT` / `--audit-loc-audit` | loc-audit gate only |
| Contracts | `audit_loc_audit_integration` | slice presence + criteria totals |

**Boundary:** band 76 = aggregate loc-audit ops gate; prior live smoke remains
[`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md) (band 75). Next: [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md) (band 77).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --audit-loc-audit
cargo run --bin poolai-loc-audit -- --audit-loc-audit --advisory --min-ratio 0.95
VERIFY_AUDIT_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_loc_audit_mode` | `true` when `--audit-loc-audit` (PH-S1404) |
| `audit_loc_audit_criteria_total` | Registry size (10) |
| `audit_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/audit_loc_audit_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `audit_loc_audit_integration.rs`, `galaxy_horizon_s1399_integration.rs`
- stand-smoke export shape: `audit_loc_audit_band76_export_shape_ph_s1403`
- prior: [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md) · [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md)
- mirror: [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md) (band 66)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.57 band 76 journal
- Slices: `--audit` · `--audit-store` · `--audit-api` · `--audit-admin-ops` · `--audit-stand-smoke`
