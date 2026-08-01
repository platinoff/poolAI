# Monitoring Docs Canon — Enterprise Phase E (Band 97)

Canonical doc: [`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md) (band 97, PH-S1615).

## Overview

Band 97 consolidates band 91–95 **`MONITORING_*.md` canon docs** under one docs-canon
gate (`--monitoring-docs-canon` / `VERIFY_MONITORING_DOCS_CANON`). Individual slice docs
remain authoritative; the aggregate registry proves that all slice docs exist plus
verify/loc-audit/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `monitoring_docs_canon_depth.rs` | enum + criteria + `MONITORING_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `MONITORING_DEPTH.md` … `MONITORING_LOC_AUDIT.md` |
| Aggregate | `--monitoring-docs-canon` | `monitoring_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_MONITORING_DOCS_CANON` / `--monitoring-docs-canon` | docs-canon gate only |
| Contracts | `monitoring_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 97 = docs-canon matrix gate for Monitoring. Prior loc-audit remains
[`MONITORING_LOC_AUDIT.md`](./MONITORING_LOC_AUDIT.md) (band 96).
Next: band 98 Monitoring vision-sync — [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md). Band 99 ratio-advisory: [`MONITORING_RATIO_ADVISORY.md`](./MONITORING_RATIO_ADVISORY.md).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --monitoring-docs-canon
cargo run --bin poolai-loc-audit -- --monitoring-docs-canon --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_docs_canon_mode` | `true` when `--monitoring-docs-canon` (PH-S1614) |
| `monitoring_docs_canon_criteria_total` | Registry size (10) |
| `monitoring_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/monitoring_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `monitoring_docs_canon_integration.rs`, `galaxy_horizon_s1609_integration.rs`
- stand-smoke export shape: `monitoring_docs_canon_band97_export_shape_ph_s1613`
- slices: `MONITORING_DEPTH.md` · `MONITORING_STORE.md` · `MONITORING_API.md` · `MONITORING_ADMIN_OPS.md` · `MONITORING_STAND_SMOKE.md` · `MONITORING_LOC_AUDIT.md`
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.78 band 97 journal
- Markers: PH-S1609 · monitoring_docs_canon_depth · PH-S1610 · `MONITORING_DOCS_CANON_SLICES`
  · PH-S1611 · monitoring_docs_canon_integration · PH-S1612 · `VERIFY_MONITORING_DOCS_CANON`
  · PH-S1614 · `--monitoring-docs-canon` · PH-S1618

