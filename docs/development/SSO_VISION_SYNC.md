# SSO Vision Sync — Enterprise Phase B (Band 68)

Canonical doc: [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md) (band 68, PH-S1325).

## Overview

Band 68 consolidates SSO phase-B canon with **`GSV/docs/vision/*`** under one vision-sync
gate (`--sso-vision-sync` / `VERIFY_SSO_VISION_SYNC`). Prior docs-canon remains
[`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md); this band proves vision artifacts stay
aligned with the enterprise SSO journal.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `sso_vision_sync_depth.rs` | enum + criteria + `SSO_VISION_SYNC_SLICES` |
| Slice artifacts | `GSV/docs/vision/` + `SSO_DOCS_CANON.md` | `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · prior canon |
| Aggregate | `--sso-vision-sync` | `sso_vision_sync_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_SSO_VISION_SYNC` / `--sso-vision-sync` | vision-sync gate only |
| Contracts | `sso_vision_sync_integration` | slice presence + criteria totals |

**Boundary:** band 68 = vision-sync gate for SSO phase B; prior docs-canon remains
[`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md) (band 67). Mirror: [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md).
Next: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md) (band 69).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --sso-vision-sync
cargo run --bin poolai-loc-audit -- --sso-vision-sync --advisory --min-ratio 0.95
VERIFY_SSO_VISION_SYNC=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-vision-sync
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_vision_sync_mode` | `true` when `--sso-vision-sync` (PH-S1324) |
| `sso_vision_sync_criteria_total` | Registry size (10) |
| `sso_vision_sync_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/sso_vision_sync_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `sso_vision_sync_integration.rs`, `galaxy_horizon_s1319_integration.rs`
- stand-smoke export shape: `sso_vision_sync_band68_export_shape_ph_s1323`
- prior: [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md) · [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.49 band 68 journal
- Slices: `manifest.json` · `extensions.json` · `README.md` · `vision.svg` · `index.html` · `SSO_DOCS_CANON.md`
- PH-S1319 · SSO_VISION_SYNC_SLICES · sso_vision_sync_integration · VERIFY_SSO_VISION_SYNC · PH-S1324 · --sso-vision-sync · PH-S1328
