# Audit Ratio Advisory — Enterprise Phase C (Band 79)

Canonical doc: [`AUDIT_RATIO_ADVISORY.md`](./AUDIT_RATIO_ADVISORY.md) (band 79, PH-S1435).

## Overview

Band 79 aggregates prior Audit phase-C loc-audit slices under one
**ratio-advisory** gate (`--audit-ratio-advisory` / `VERIFY_AUDIT_RATIO_ADVISORY`).
Prior vision-sync remains [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md) (band 78).

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `audit_ratio_advisory_depth.rs` | enum + criteria + `AUDIT_RATIO_ADVISORY_SLICES` |
| Slice aggregate | prior `--audit*` + vision-sync | `--audit-store` · `--audit-api` · `--audit-admin-ops` · `--audit-docs-canon` · `--audit-vision-sync` · `AUDIT_VISION_SYNC.md` |
| Aggregate | `--audit-ratio-advisory` | `audit_ratio_advisory_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_AUDIT_RATIO_ADVISORY` / `--audit-ratio-advisory` | ratio-advisory gate |
| Contracts | `audit_ratio_advisory_integration` | slice presence + criteria totals |

**Boundary:** band 79 = ratio-advisory ops gate for Audit phase C (no new Audit durable store).
Prior vision-sync: [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md) (band 78).
Mirror: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md). Next: band 80 Audit horizon close.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --audit-ratio-advisory
cargo run --bin poolai-loc-audit -- --audit-ratio-advisory --advisory --min-ratio 0.95
VERIFY_AUDIT_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-ratio-advisory
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_ratio_advisory_mode` | `true` when `--audit-ratio-advisory` (PH-S1434) |
| `audit_ratio_advisory_criteria_total` | Registry size (10) |
| `audit_ratio_advisory_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/audit_ratio_advisory_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `audit_ratio_advisory_integration.rs`, `galaxy_horizon_s1429_integration.rs`
- stand-smoke export shape: `audit_ratio_advisory_band79_export_shape_ph_s1433`
- prior: [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md) · [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.60 band 79 journal
- Slices: `--audit-store` · `--audit-api` · `--audit-admin-ops` · `--audit-docs-canon` · `--audit-vision-sync` · `AUDIT_VISION_SYNC.md`
- PH-S1429 · AUDIT_RATIO_ADVISORY_SLICES · audit_ratio_advisory_integration · VERIFY_AUDIT_RATIO_ADVISORY · PH-S1434 · --audit-ratio-advisory · PH-S1438
