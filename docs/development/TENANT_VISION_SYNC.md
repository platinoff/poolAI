# Tenant Vision Sync — Enterprise Phase A (Band 58)

Canonical doc: [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md) (band 58, PH-S1225).

## Overview

Band 58 consolidates tenant phase-A canon with **`GSV/docs/vision/*`** under one vision-sync
gate (`--tenant-vision-sync` / `VERIFY_TENANT_VISION_SYNC`). Prior docs-canon remains
[`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md); this band proves vision artifacts stay
aligned with the enterprise tenant journal.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `tenant_vision_sync_depth.rs` | enum + criteria + `TENANT_VISION_SYNC_SLICES` |
| Slice artifacts | `GSV/docs/vision/` + `TENANT_DOCS_CANON.md` | `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · prior canon |
| Aggregate | `--tenant-vision-sync` | `tenant_vision_sync_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_TENANT_VISION_SYNC` / `--tenant-vision-sync` | vision-sync gate only |
| Contracts | `tenant_vision_sync_integration` | slice presence + criteria totals |

**Boundary:** band 58 = vision-sync gate for tenant phase A; prior docs-canon remains
[`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md) (band 57). Next: band 59
[`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md) ✅.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --tenant-vision-sync
cargo run --bin poolai-loc-audit -- --tenant-vision-sync --advisory --min-ratio 0.95
VERIFY_TENANT_VISION_SYNC=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-vision-sync
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_vision_sync_mode` | `true` when `--tenant-vision-sync` (PH-S1224) |
| `tenant_vision_sync_criteria_total` | Registry size (10) |
| `tenant_vision_sync_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/tenant_vision_sync_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `tenant_vision_sync_integration.rs`, `galaxy_horizon_s1219_integration.rs`
- stand-smoke export shape: `tenant_vision_sync_band58_export_shape_ph_s1223`
- prior: [`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md) · [`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.39 band 58 journal
- Slices: `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · `TENANT_DOCS_CANON.md`
- PH-S1219 · TENANT_VISION_SYNC_SLICES · tenant_vision_sync_integration · VERIFY_TENANT_VISION_SYNC · PH-S1224 · --tenant-vision-sync · PH-S1228
