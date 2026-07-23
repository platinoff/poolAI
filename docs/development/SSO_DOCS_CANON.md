# SSO Docs Canon — Enterprise Phase B (Band 67)

Canonical doc: [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md) (band 67, PH-S1315).

## Overview

Band 67 consolidates band 61–66 **`SSO_*.md` canon docs** under one docs-canon
gate (`--sso-docs-canon` / `VERIFY_SSO_DOCS_CANON`). Individual slice docs remain
authoritative; the aggregate registry proves all six exist plus verify/loc-audit/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `sso_docs_canon_depth.rs` | enum + criteria + `SSO_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `SSO_DEPTH.md` … `SSO_LOC_AUDIT.md` |
| Aggregate | `--sso-docs-canon` | `sso_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_SSO_DOCS_CANON` / `--sso-docs-canon` | docs-canon gate only |
| Contracts | `sso_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 67 = docs-canon matrix gate; prior loc-audit aggregate remains
[`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md) (band 66). Mirror: [`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md).
Next: band 68 vision-sync → band 69 ratio advisory.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --sso-docs-canon
cargo run --bin poolai-loc-audit -- --sso-docs-canon --advisory --min-ratio 0.95
VERIFY_SSO_DOCS_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_docs_canon_mode` | `true` when `--sso-docs-canon` (PH-S1314) |
| `sso_docs_canon_criteria_total` | Registry size (10) |
| `sso_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/sso_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `sso_docs_canon_integration.rs`, `galaxy_horizon_s1309_integration.rs`
- stand-smoke export shape: `sso_docs_canon_band67_export_shape_ph_s1313`
- prior: [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md) · [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.48 band 67 journal
- Slices: `SSO_DEPTH.md` · `SSO_STORE.md` · `SSO_API.md` · `SSO_ADMIN_OPS.md` · `SSO_STAND_SMOKE.md` · `SSO_LOC_AUDIT.md`
- Markers: PH-S1309 · SSO_DOCS_CANON_SLICES · PH-S1310 · PH-S1311 · sso_docs_canon_integration · PH-S1312 · VERIFY_SSO_DOCS_CANON · PH-S1314 · `--sso-docs-canon` · PH-S1318
