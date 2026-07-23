# SSO Loc-Audit Aggregate — Enterprise Phase B (Band 66)

Canonical doc: [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md) (band 66, PH-S1305).

## Overview

Band 66 consolidates band 61–65 **`--sso*` loc-audit slices** under one aggregate
gate (`--sso-loc-audit` / `VERIFY_SSO_LOC_AUDIT`). Slice flags remain available
individually; the aggregate registry proves all five exist plus verify/docs/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `sso_loc_audit_depth.rs` | enum + criteria + `SSO_LOC_AUDIT_SLICES` |
| Slice flags | `poolai-loc-audit` | `--sso` … `--sso-stand-smoke` |
| Aggregate | `--sso-loc-audit` | `sso_loc_audit_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_SSO_LOC_AUDIT` / `--sso-loc-audit` | loc-audit gate only |
| Contracts | `sso_loc_audit_integration` | slice presence + criteria totals |

**Boundary:** band 66 = aggregate loc-audit ops gate; prior live smoke remains
[`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md) (band 65). Next: band 67 docs canon.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --sso-loc-audit
cargo run --bin poolai-loc-audit -- --sso-loc-audit --advisory --min-ratio 0.95
VERIFY_SSO_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_loc_audit_mode` | `true` when `--sso-loc-audit` (PH-S1304) |
| `sso_loc_audit_criteria_total` | Registry size (10) |
| `sso_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/sso_loc_audit_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `sso_loc_audit_integration.rs`, `galaxy_horizon_s1299_integration.rs`
- stand-smoke export shape: `sso_loc_audit_band66_export_shape_ph_s1303`
- prior: [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md) · [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md)
- mirror: [`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md) (band 56)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.47 band 66 journal
- Slices: `--sso` · `--sso-store` · `--sso-api` · `--sso-admin-ops` · `--sso-stand-smoke`
