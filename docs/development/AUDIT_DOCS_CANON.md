# Audit Docs Canon — Enterprise Phase C (Band 77)

Canonical doc: [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md) (band 77, PH-S1415).

## Overview

Band 77 consolidates band 71–76 **`AUDIT_*.md` canon docs** under one docs-canon
gate (`--audit-docs-canon` / `VERIFY_AUDIT_DOCS_CANON`). Individual slice docs remain
authoritative; the aggregate registry proves all six exist plus verify/loc-audit/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `audit_docs_canon_depth.rs` | enum + criteria + `AUDIT_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `AUDIT_DEPTH.md` … `AUDIT_LOC_AUDIT.md` |
| Aggregate | `--audit-docs-canon` | `audit_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_AUDIT_DOCS_CANON` / `--audit-docs-canon` | docs-canon gate only |
| Contracts | `audit_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 77 = docs-canon matrix gate; prior loc-audit aggregate remains
[`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md) (band 76). Mirror: [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md).
Next: band 78 Audit vision-sync.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --audit-docs-canon
cargo run --bin poolai-loc-audit -- --audit-docs-canon --advisory --min-ratio 0.95
VERIFY_AUDIT_DOCS_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_docs_canon_mode` | `true` when `--audit-docs-canon` (PH-S1414) |
| `audit_docs_canon_criteria_total` | Registry size (10) |
| `audit_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/audit_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `audit_docs_canon_integration.rs`, `galaxy_horizon_s1409_integration.rs`
- stand-smoke export shape: `audit_docs_canon_band77_export_shape_ph_s1413`
- prior: [`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md) · [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.58 band 77 journal
- Slices: `AUDIT_DEPTH.md` · `AUDIT_STORE.md` · `AUDIT_API.md` · `AUDIT_ADMIN_OPS.md` · `AUDIT_STAND_SMOKE.md` · `AUDIT_LOC_AUDIT.md`
- Markers: PH-S1409 · AUDIT_DOCS_CANON_SLICES · PH-S1410 · PH-S1411 · audit_docs_canon_integration · PH-S1412 · VERIFY_AUDIT_DOCS_CANON · PH-S1414 · `--audit-docs-canon` · PH-S1418
