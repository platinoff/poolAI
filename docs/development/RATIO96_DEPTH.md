# Ratio96 Depth Scaffold — Phase F (Band 101)

Canonical doc: [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md) (band 101, PH-S1655).

## Overview

Band 101 opens **phase F (Ratio96)** — the stretch path to `rust_ratio >= 0.96`
per [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).
Phase F moves the last non-Rust product surfaces (wasm wiring, slim JS/i18n/charts,
Rust stand/e2e bins) while keeping the CI 93% advisory floor and the 95% hold band.
Band 101 is the **depth scaffold**: registry, store wire, contracts, glue, and the
aggregate `--ratio96` gate.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `ratio96_depth.rs` | enum + criteria + `RATIO96_PHASE_F_SLICES` |
| Slice aggregate | `Ratio96Depth` · `ratio96_store_wire` · `ratio96_depth_contracts` · `VERIFY_RATIO96` · `ratio96_band101_export_shape` · `--ratio96` · `RATIO96_DEPTH.md` · `RATIO96_RATIO_ADVISORY.md` · `ratio96_store_depth` · `PH-S1658` | phase-F depth markers |
| Store wire | `ratio96_store_depth.rs` | reads durable `rust_ratio.json` (stretch/hold gate) |
| Aggregate | `--ratio96` | `ratio96_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_RATIO96` / `--ratio96` | phase-F depth gate |
| Contracts | `ratio96_depth_contracts` | slices + criteria totals + store wire |

**Boundary:** band 101 = phase F depth scaffold (first of the Ratio96 bands).
Prior: [`MONITORING_HORIZON.md`](./MONITORING_HORIZON.md) (band 100, phase E close).
Next: [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) band 102 (Ratio96 store wire).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --ratio96
cargo run --bin poolai-loc-audit -- --ratio96 --advisory --min-ratio 0.95
VERIFY_RATIO96=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ratio96
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `ratio96_mode` | `true` when `--ratio96` (PH-S1654) |
| `ratio96_criteria_total` | Registry size (10) |
| `ratio96_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`ratio96_depth.rs`](../../crates/poolai-ui-core/src/ratio96_depth.rs) · [`ratio96_store_depth.rs`](../../crates/poolai-ui-core/src/ratio96_store_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `ratio96_depth_contracts.rs`, `galaxy_horizon_s1649_integration.rs`
- stand-smoke export shape: `ratio96_band101_export_shape_ph_s1653`
- advisory: [`RATIO96_RATIO_ADVISORY.md`](./RATIO96_RATIO_ADVISORY.md)
- strategy: [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) (phase F / stretch 96%)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.18 project close · §5.82 band 101 journal
- Slices: `Ratio96Depth` · `ratio96_store_wire` · `ratio96_depth_contracts` · `VERIFY_RATIO96` · `ratio96_band101_export_shape` · `--ratio96` · `RATIO96_DEPTH.md` · `RATIO96_RATIO_ADVISORY.md` · `ratio96_store_depth` · `PH-S1658`
- PH-S1649 · `RATIO96_PHASE_F_SLICES` · `ratio96_depth_contracts` · `VERIFY_RATIO96` · PH-S1654 · `--ratio96` · PH-S1658
