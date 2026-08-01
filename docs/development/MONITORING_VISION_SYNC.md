# Monitoring Vision Sync — Enterprise Phase E (Band 98)

Canonical doc: [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md) (band 98, PH-S1625).

## Overview

Band 98 consolidates Monitoring phase-E docs-canon with **`docs/vision/*`** under one vision-sync
gate (`--monitoring-vision-sync` / `VERIFY_MONITORING_VISION_SYNC`). Prior docs-canon remains
[`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md); this band proves vision artifacts stay
aligned with the enterprise Monitoring journal.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `monitoring_vision_sync_depth.rs` | enum + criteria + `MONITORING_VISION_SYNC_SLICES` |
| Slice artifacts | `docs/vision/` + `MONITORING_DOCS_CANON.md` | `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · prior canon |
| Aggregate | `--monitoring-vision-sync` | `monitoring_vision_sync_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_MONITORING_VISION_SYNC` / `--monitoring-vision-sync` | vision-sync gate only |
| Contracts | `monitoring_vision_sync_integration` | slice presence + criteria totals |

**Boundary:** band 98 = vision-sync gate for Monitoring phase E; prior docs-canon remains
[`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md) (band 97). Mirror: [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md).
Next: band 99 Monitoring ratio-advisory (`PH-S1629…S1638`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --monitoring-vision-sync
cargo run --bin poolai-loc-audit -- --monitoring-vision-sync --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_VISION_SYNC=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-vision-sync
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_vision_sync_mode` | `true` when `--monitoring-vision-sync` (PH-S1624) |
| `monitoring_vision_sync_criteria_total` | Registry size (10) |
| `monitoring_vision_sync_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/monitoring_vision_sync_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `monitoring_vision_sync_integration.rs`, `galaxy_horizon_s1619_integration.rs`
- stand-smoke export shape: `monitoring_vision_sync_band98_export_shape_ph_s1623`
- prior: [`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md) · [`MONITORING_LOC_AUDIT.md`](./MONITORING_LOC_AUDIT.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.79 band 98 journal
- Slices: `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · `MONITORING_DOCS_CANON.md`
- PH-S1619 · MONITORING_VISION_SYNC_SLICES · monitoring_vision_sync_integration · VERIFY_MONITORING_VISION_SYNC · PH-S1624 · --monitoring-vision-sync · PH-S1628
