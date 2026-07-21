# Tenant Docs Canon — Enterprise Phase A (Band 57)

Canonical doc: [`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md) (band 57, PH-S1215).

## Overview

Band 57 consolidates band 51–56 **`TENANT_*.md` canon docs** under one docs-canon
gate (`--tenant-docs-canon` / `VERIFY_TENANT_DOCS_CANON`). Individual slice docs remain
authoritative; the aggregate registry proves all six exist plus verify/loc-audit/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `tenant_docs_canon_depth.rs` | enum + criteria + `TENANT_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `TENANT_PERSIST.md` … `TENANT_LOC_AUDIT.md` |
| Aggregate | `--tenant-docs-canon` | `tenant_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_TENANT_DOCS_CANON` / `--tenant-docs-canon` | docs-canon gate only |
| Contracts | `tenant_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 57 = docs-canon matrix gate; prior loc-audit aggregate remains
[`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md) (band 56). Next: band 58 vision-sync.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --tenant-docs-canon
cargo run --bin poolai-loc-audit -- --tenant-docs-canon --advisory --min-ratio 0.95
VERIFY_TENANT_DOCS_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_docs_canon_mode` | `true` when `--tenant-docs-canon` (PH-S1214) |
| `tenant_docs_canon_criteria_total` | Registry size (10) |
| `tenant_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/tenant_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `tenant_docs_canon_integration.rs`, `galaxy_horizon_s1209_integration.rs`
- stand-smoke export shape: `tenant_docs_canon_band57_export_shape_ph_s1213`
- prior: [`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md) · [`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.38 band 57 journal
- Slices: `TENANT_PERSIST.md` · `TENANT_STORE.md` · `TENANT_API.md` · `TENANT_ADMIN_OPS.md` · `TENANT_STAND_SMOKE.md` · `TENANT_LOC_AUDIT.md`
