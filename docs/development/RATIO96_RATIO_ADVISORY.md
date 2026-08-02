# Ratio96 Ratio Hold Advisory (Band 101)

Canonical doc: [`RATIO96_RATIO_ADVISORY.md`](./RATIO96_RATIO_ADVISORY.md) (band 101, PH-S1657).

## Overview

Phase F (Ratio96) holds the 95% regression floor while chasing the 96% stretch spirit.
The CI gate stays **advisory** (`--min-ratio 0.95 --advisory`) so a dip below floor prints
a warning but does not fail the drain; the stretch gate (`stretch_spirit_gate_met`) reports
`false` until `rust_ratio >= 0.96`.

| Gate | Value | Behavior |
|------|-------|----------|
| Warn floor | `--warn-below 0.93` | below → warning (CI advisory) |
| Hold floor | `--min-ratio 0.95 --advisory` | below → warning, exit 0 |
| Target | `--target 0.95` | formal phase target |
| Stretch spirit | `--stretch 0.96` | `stretch_spirit_gate_met` gate |

## Run

```bash
cargo run --bin poolai-loc-audit -- --ratio96 --warn-below 0.93 --target 0.95 --stretch 0.96 --min-ratio 0.95 --advisory
```

## Store

`ratio96_store_wire()` reads `docs/development/rust_ratio.json` and classifies both gates:
`stretch_gate_met()` (0.96) and `hold_gate_met()` (0.95). Band 101 is a scaffold band —
hold stays advisory; stretch target remains open until phase-F migration lands.

Related: [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md) · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).
