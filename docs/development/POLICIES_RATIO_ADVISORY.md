# Policies Ratio Advisory — Enterprise Phase D (Band 89)

Canonical doc: [`POLICIES_RATIO_ADVISORY.md`](./POLICIES_RATIO_ADVISORY.md) (band 89, PH-S1535).

## Overview

Band 89 aggregates prior Policies phase-D loc-audit slices under one
**ratio-advisory** gate (`--policy-ratio-advisory` / `VERIFY_POLICY_RATIO_ADVISORY`).
Prior vision-sync remains [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md) (band 88).

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `policy_ratio_advisory_depth.rs` | enum + criteria + `POLICY_RATIO_ADVISORY_SLICES` |
| Slice aggregate | prior `--policy*` + vision-sync | `--policy-store` · `--policy-api` · `--policy-admin-ops` · `--policy-docs-canon` · `--policy-vision-sync` · `POLICIES_VISION_SYNC.md` |
| Aggregate | `--policy-ratio-advisory` | `policy_ratio_advisory_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_POLICY_RATIO_ADVISORY` / `--policy-ratio-advisory` | ratio-advisory gate |
| Contracts | `policy_ratio_advisory_integration` | slice presence + criteria totals |

**Boundary:** band 89 = ratio-advisory ops gate for Policies phase D (no new Policies durable store).
Prior vision-sync: [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md) (band 88).
Mirror: [`AUDIT_RATIO_ADVISORY.md`](./AUDIT_RATIO_ADVISORY.md). Next: band 90 Policies horizon close (`PH-S1539…S1548`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --policy-ratio-advisory
cargo run --bin poolai-loc-audit -- --policy-ratio-advisory --advisory --min-ratio 0.95
VERIFY_POLICY_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-ratio-advisory
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_ratio_advisory_mode` | `true` when `--policy-ratio-advisory` (PH-S1534) |
| `policy_ratio_advisory_criteria_total` | Registry size (10) |
| `policy_ratio_advisory_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/policy_ratio_advisory_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `policy_ratio_advisory_integration.rs`, `galaxy_horizon_s1529_integration.rs`
- stand-smoke export shape: `policy_ratio_advisory_band89_export_shape_ph_s1533`
- prior: [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md) · [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.70 band 89 journal
- Slices: `--policy-store` · `--policy-api` · `--policy-admin-ops` · `--policy-docs-canon` · `--policy-vision-sync` · `POLICIES_VISION_SYNC.md`
- PH-S1529 · POLICY_RATIO_ADVISORY_SLICES · policy_ratio_advisory_integration · VERIFY_POLICY_RATIO_ADVISORY · PH-S1534 · --policy-ratio-advisory · PH-S1538
