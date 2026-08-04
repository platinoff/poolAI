# Ratio96 Docs Canon — Phase F (Band 107)

Canonical doc: [`RATIO96_DOCS_CANON.md`](./RATIO96_DOCS_CANON.md) (band 107, PH-S1715).

## Overview

Band 107 consolidates band 101–106 **`RATIO96_*.md` canon docs** under one docs-canon
gate (`--ratio96-docs-canon` / `VERIFY_RATIO96_DOCS_CANON`). Individual slice docs remain
authoritative; the aggregate registry proves all four exist plus verify/loc-audit/stand-smoke/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `ratio96_docs_canon_depth.rs` | enum + criteria + `RATIO96_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `RATIO96_DEPTH.md` … `RATIO96_LOC_AUDIT.md` |
| Aggregate | `--ratio96-docs-canon` | `ratio96_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_RATIO96_DOCS_CANON` / `--ratio96-docs-canon` | docs-canon gate only |
| Contracts | `ratio96_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 107 = docs-canon matrix gate; prior loc-audit aggregate remains
[`RATIO96_LOC_AUDIT.md`](./RATIO96_LOC_AUDIT.md) (band 106). Mirror: [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md).
Next: band 108 vision-sync → band 109 ratio advisory.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --ratio96-docs-canon
cargo run --bin poolai-loc-audit -- --ratio96-docs-canon --advisory --min-ratio 0.95
VERIFY_RATIO96_DOCS_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ratio96-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `ratio96_docs_canon_mode` | `true` when `--ratio96-docs-canon` (PH-S1714) |
| `ratio96_docs_canon_criteria_total` | Registry size (10) |
| `ratio96_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`ratio96_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/ratio96_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `ratio96_docs_canon_integration.rs`, `galaxy_horizon_s1709_integration.rs`
- stand-smoke export shape: `ratio96_docs_canon_band107_export_shape_ph_s1713`
- prior: [`RATIO96_LOC_AUDIT.md`](./RATIO96_LOC_AUDIT.md) · [`RATIO96_STAND_SMOKE.md`](./RATIO96_STAND_SMOKE.md) · [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.18 project close · §5.88 band 107 journal
- Slices: `RATIO96_DEPTH.md` · `RATIO96_ADMIN_OPS.md` · `RATIO96_STAND_SMOKE.md` · `RATIO96_LOC_AUDIT.md`
- Markers: PH-S1709 · RATIO96_DOCS_CANON_SLICES · PH-S1710 · PH-S1711 · ratio96_docs_canon_integration · PH-S1712 · VERIFY_RATIO96_DOCS_CANON · PH-S1714 · `--ratio96-docs-canon` · PH-S1718
