# Policies Docs Canon — Enterprise Phase D (Band 87)

Canonical doc: [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md) (band 87, PH-S1515).

## Overview

Band 87 consolidates band 81–86 **`POLICIES_*.md` canon docs** under one docs-canon
gate (`--policy-docs-canon` / `VERIFY_POLICY_DOCS_CANON`). Individual slice docs remain
authoritative; the aggregate registry proves all six exist plus verify/loc-audit/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `policy_docs_canon_depth.rs` | enum + criteria + `POLICY_DOCS_CANON_SLICES` |
| Slice docs | `docs/development/` | `POLICIES_DEPTH.md` … `POLICIES_LOC_AUDIT.md` |
| Aggregate | `--policy-docs-canon` | `policy_docs_canon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_POLICY_DOCS_CANON` / `--policy-docs-canon` | docs-canon gate only |
| Contracts | `policy_docs_canon_integration` | slice presence + criteria totals |

**Boundary:** band 87 = docs-canon matrix gate; prior loc-audit aggregate remains
[`POLICIES_LOC_AUDIT.md`](./POLICIES_LOC_AUDIT.md) (band 86). Mirror: [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md).
Next: band 88 Policies vision-sync.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --policy-docs-canon
cargo run --bin poolai-loc-audit -- --policy-docs-canon --advisory --min-ratio 0.95
VERIFY_POLICY_DOCS_CANON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-docs-canon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_docs_canon_mode` | `true` when `--policy-docs-canon` (PH-S1514) |
| `policy_docs_canon_criteria_total` | Registry size (10) |
| `policy_docs_canon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/policy_docs_canon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `policy_docs_canon_integration.rs`, `galaxy_horizon_s1509_integration.rs`
- stand-smoke export shape: `policy_docs_canon_band87_export_shape_ph_s1513`
- prior: [`POLICIES_LOC_AUDIT.md`](./POLICIES_LOC_AUDIT.md) · [`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.68 band 87 journal
- Slices: `POLICIES_DEPTH.md` · `POLICIES_STORE.md` · `POLICIES_API.md` · `POLICIES_ADMIN_OPS.md` · `POLICIES_STAND_SMOKE.md` · `POLICIES_LOC_AUDIT.md`
- Markers: PH-S1509 · POLICY_DOCS_CANON_SLICES · PH-S1510 · PH-S1511 · policy_docs_canon_integration · PH-S1512 · VERIFY_POLICY_DOCS_CANON · PH-S1514 · `--policy-docs-canon` · PH-S1518
