# Audit Vision Sync — Enterprise Phase C (Band 78)

Canonical doc: [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md) (band 78, PH-S1425).

## Overview

Band 78 consolidates Audit phase-C docs-canon with **`docs/vision/*`** under one vision-sync
gate (`--audit-vision-sync` / `VERIFY_AUDIT_VISION_SYNC`). Prior docs-canon remains
[`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md); this band proves vision artifacts stay
aligned with the enterprise Audit journal.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `audit_vision_sync_depth.rs` | enum + criteria + `AUDIT_VISION_SYNC_SLICES` |
| Slice artifacts | `docs/vision/` + `AUDIT_DOCS_CANON.md` | `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · prior canon |
| Aggregate | `--audit-vision-sync` | `audit_vision_sync_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_AUDIT_VISION_SYNC` / `--audit-vision-sync` | vision-sync gate only |
| Contracts | `audit_vision_sync_integration` | slice presence + criteria totals |

**Boundary:** band 78 = vision-sync gate for Audit phase C; prior docs-canon remains
[`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md) (band 77). Mirror: [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md).
Next: band 79 Audit ratio-advisory (`PH-S1429…S1438`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --audit-vision-sync
cargo run --bin poolai-loc-audit -- --audit-vision-sync --advisory --min-ratio 0.95
VERIFY_AUDIT_VISION_SYNC=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-vision-sync
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_vision_sync_mode` | `true` when `--audit-vision-sync` (PH-S1424) |
| `audit_vision_sync_criteria_total` | Registry size (10) |
| `audit_vision_sync_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/audit_vision_sync_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `audit_vision_sync_integration.rs`, `galaxy_horizon_s1419_integration.rs`
- stand-smoke export shape: `audit_vision_sync_band78_export_shape_ph_s1423`
- prior: [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md) · [`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.59 band 78 journal
- Slices: `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · `AUDIT_DOCS_CANON.md`
- PH-S1419 · AUDIT_VISION_SYNC_SLICES · audit_vision_sync_integration · VERIFY_AUDIT_VISION_SYNC · PH-S1424 · --audit-vision-sync · PH-S1428
