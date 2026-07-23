# SSO Ratio Advisory — Enterprise Phase B (Band 69)

Canonical doc: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md) (band 69, PH-S1335).

## Overview

Band 69 aggregates prior SSO phase-B loc-audit slices under one
**ratio-advisory** gate (`--sso-ratio-advisory` / `VERIFY_SSO_RATIO_ADVISORY`).
Prior vision-sync remains [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md) (band 68).

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `sso_ratio_advisory_depth.rs` | enum + criteria + `SSO_RATIO_ADVISORY_SLICES` |
| Slice aggregate | prior `--sso*` + vision-sync | `--sso-store` · `--sso-api` · `--sso-admin-ops` · `--sso-docs-canon` · `--sso-vision-sync` · `SSO_VISION_SYNC.md` |
| Aggregate | `--sso-ratio-advisory` | `sso_ratio_advisory_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_SSO_RATIO_ADVISORY` / `--sso-ratio-advisory` | ratio-advisory gate |
| Contracts | `sso_ratio_advisory_integration` | slice presence + criteria totals |

**Boundary:** band 69 = ratio-advisory ops gate for SSO phase B (no new SSO durable store).
Prior vision-sync: [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md) (band 68).
Mirror: [`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md). Next: band 70 SSO horizon close.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --sso-ratio-advisory
cargo run --bin poolai-loc-audit -- --sso-ratio-advisory --advisory --min-ratio 0.95
VERIFY_SSO_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-ratio-advisory
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_ratio_advisory_mode` | `true` when `--sso-ratio-advisory` (PH-S1334) |
| `sso_ratio_advisory_criteria_total` | Registry size (10) |
| `sso_ratio_advisory_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/sso_ratio_advisory_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `sso_ratio_advisory_integration.rs`, `galaxy_horizon_s1329_integration.rs`
- stand-smoke export shape: `sso_ratio_advisory_band69_export_shape_ph_s1333`
- prior: [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md) · [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.50 band 69 journal
- Slices: `--sso-store` · `--sso-api` · `--sso-admin-ops` · `--sso-docs-canon` · `--sso-vision-sync` · `SSO_VISION_SYNC.md`
- PH-S1329 · SSO_RATIO_ADVISORY_SLICES · sso_ratio_advisory_integration · VERIFY_SSO_RATIO_ADVISORY · PH-S1334 · --sso-ratio-advisory · PH-S1338
