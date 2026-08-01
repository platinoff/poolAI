# Monitoring Ratio Advisory — Enterprise Phase E (Band 99)

Canonical doc: [`MONITORING_RATIO_ADVISORY.md`](./MONITORING_RATIO_ADVISORY.md) (band 99, PH-S1635).

## Overview

Band 99 consolidates the Monitoring loc-audit ratio gate under one ratio-advisory
aggregate (`--monitoring-ratio-advisory` / `VERIFY_MONITORING_RATIO_ADVISORY`). Prior
docs-canon remains [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md) (band 98);
this band proves the [`rust_ratio.json`](./rust_ratio.json) hold floor stays aligned with
the enterprise Monitoring journal and the ratio strategy.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `monitoring_ratio_advisory_depth.rs` | enum + criteria + `MONITORING_RATIO_ADVISORY_SLICES` |
| Slice artifacts | `rust_ratio.json` · `RUST_RATIO_STRATEGY_2026-06-13.md` · `MONITORING_VISION_SYNC.md` · `poolai_loc_audit.rs` · `run-poolai.sh` · `verify-dev-stand.sh` | ratio hold floor + prior canon |
| Aggregate | `--monitoring-ratio-advisory` | `monitoring_ratio_advisory_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_MONITORING_RATIO_ADVISORY` / `--monitoring-ratio-advisory` | ratio-advisory gate only |
| Contracts | `monitoring_ratio_advisory_integration` | slice presence + criteria totals |

**Boundary:** band 99 = ratio-advisory gate for Monitoring phase E; prior docs-canon remains
[`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md) (band 98). Mirror: [`POLICIES_RATIO_ADVISORY.md`](./POLICIES_RATIO_ADVISORY.md).
Next: band 100 Monitoring horizon-close (`PH-S1639…S1648`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --monitoring-ratio-advisory
cargo run --bin poolai-loc-audit -- --monitoring-ratio-advisory --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-ratio-advisory
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_ratio_advisory_mode` | `true` when `--monitoring-ratio-advisory` (PH-S1634) |
| `monitoring_ratio_advisory_criteria_total` | Registry size (10) |
| `monitoring_ratio_advisory_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/monitoring_ratio_advisory_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `monitoring_ratio_advisory_integration.rs`, `galaxy_horizon_s1629_integration.rs`
- stand-smoke export shape: `monitoring_ratio_advisory_band99_export_shape_ph_s1633`
- prior: [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md) · [`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md)
- strategy: [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · [`rust_ratio.json`](./rust_ratio.json)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.80 band 99 journal
- Slices: `rust_ratio.json` · `RUST_RATIO_STRATEGY_2026-06-13.md` · `MONITORING_VISION_SYNC.md` · `poolai_loc_audit.rs` · `run-poolai.sh` · `verify-dev-stand.sh`
- PH-S1629 · MONITORING_RATIO_ADVISORY_SLICES · monitoring_ratio_advisory_integration · VERIFY_MONITORING_RATIO_ADVISORY · PH-S1634 · --monitoring-ratio-advisory · PH-S1638
