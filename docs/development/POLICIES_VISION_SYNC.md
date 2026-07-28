# Policies Vision Sync — Enterprise Phase D (Band 88)

Canonical doc: [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md) (band 88, PH-S1525).

## Overview

Band 88 consolidates Policies phase-D docs-canon with **`docs/vision/*`** under one vision-sync
gate (`--policy-vision-sync` / `VERIFY_POLICY_VISION_SYNC`). Prior docs-canon remains
[`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md); this band proves vision artifacts stay
aligned with the enterprise Policies journal.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `policy_vision_sync_depth.rs` | enum + criteria + `POLICY_VISION_SYNC_SLICES` |
| Slice artifacts | `docs/vision/` + `POLICIES_DOCS_CANON.md` | `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · prior canon |
| Aggregate | `--policy-vision-sync` | `policy_vision_sync_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_POLICY_VISION_SYNC` / `--policy-vision-sync` | vision-sync gate only |
| Contracts | `policy_vision_sync_integration` | slice presence + criteria totals |

**Boundary:** band 88 = vision-sync gate for Policies phase D; prior docs-canon remains
[`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md) (band 87). Mirror: [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md).
Next: band 89 Policies ratio-advisory (`PH-S1529…S1538`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --policy-vision-sync
cargo run --bin poolai-loc-audit -- --policy-vision-sync --advisory --min-ratio 0.95
VERIFY_POLICY_VISION_SYNC=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-vision-sync
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_vision_sync_mode` | `true` when `--policy-vision-sync` (PH-S1524) |
| `policy_vision_sync_criteria_total` | Registry size (10) |
| `policy_vision_sync_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/policy_vision_sync_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `policy_vision_sync_integration.rs`, `galaxy_horizon_s1519_integration.rs`
- stand-smoke export shape: `policy_vision_sync_band88_export_shape_ph_s1523`
- prior: [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md) · [`POLICIES_LOC_AUDIT.md`](./POLICIES_LOC_AUDIT.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.69 band 88 journal
- Slices: `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · `POLICIES_DOCS_CANON.md`
- PH-S1519 · POLICY_VISION_SYNC_SLICES · policy_vision_sync_integration · VERIFY_POLICY_VISION_SYNC · PH-S1524 · --policy-vision-sync · PH-S1528
