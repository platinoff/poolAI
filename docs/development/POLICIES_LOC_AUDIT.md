# Policies Loc-Audit Aggregate — Enterprise Phase D (Band 86)

Canonical doc: [`POLICIES_LOC_AUDIT.md`](./POLICIES_LOC_AUDIT.md) (band 86, PH-S1505).

## Overview

Band 86 consolidates band 81–85 **`--policy*` loc-audit slices** under one aggregate
gate (`--policy-loc-audit` / `VERIFY_POLICY_LOC_AUDIT`). Slice flags remain available
individually; the aggregate registry proves all five exist plus verify/docs/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `policy_loc_audit_depth.rs` | enum + criteria + `POLICY_LOC_AUDIT_SLICES` |
| Slice flags | `poolai-loc-audit` | `--policy` … `--policy-stand-smoke` |
| Aggregate | `--policy-loc-audit` | `policy_loc_audit_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_POLICY_LOC_AUDIT` / `--policy-loc-audit` | loc-audit gate only |
| Contracts | `policy_loc_audit_integration` | slice presence + criteria totals |

**Boundary:** band 86 = aggregate loc-audit ops gate; prior live smoke remains
[`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md) (band 85). Next: [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md) (band 87).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --policy-loc-audit
cargo run --bin poolai-loc-audit -- --policy-loc-audit --migration-advisory --advisory --min-ratio 0.95
VERIFY_POLICY_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_loc_audit_mode` | `true` when `--policy-loc-audit` (PH-S1504) |
| `policy_loc_audit_criteria_total` | Registry size (10) |
| `policy_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/policy_loc_audit_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `policy_loc_audit_integration.rs`, `galaxy_horizon_s1499_integration.rs`
- stand-smoke export shape: `policy_loc_audit_band86_export_shape_ph_s1503`
- prior: [`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md) · [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md)
- mirror: [`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md) (band 76)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.67 band 86 journal
- Slices: `--policy` · `--policy-store` · `--policy-api` · `--policy-admin-ops` · `--policy-stand-smoke`
