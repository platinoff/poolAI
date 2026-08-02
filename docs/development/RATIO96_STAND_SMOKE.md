# Ratio96 Stand Smoke — Phase F (Band 105)

Canonical doc: [`RATIO96_STAND_SMOKE.md`](./RATIO96_STAND_SMOKE.md) (band 105, PH-S1695).
Prev: [`RATIO96_ADMIN_OPS.md`](./RATIO96_ADMIN_OPS.md) (band 104 ✅). Mirror: [`MONITORING_STAND_SMOKE.md`](./MONITORING_STAND_SMOKE.md).

## Overview

Band 105 performs **live stand smoke** for the ratio96 store/wire/query/fixtures surface verified in band 104
(`ratio96_store_wire_json`, `GET /api/v1/ops/ratio96`). The live smoke runs via `poolai-http-stand-smoke --ratio96-stand-smoke`
and validates the ratio96 slice against the running dev stand.

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire smoke | `smoke_ratio96_store_wire` | `GET /api/v1/ops/ratio96` live call |
| Query smoke | `smoke_ratio96_query` | ratio96 query path validation |
| Field fixtures smoke | `smoke_ratio96_field_fixtures` | ratio96 field-level fixtures (400 codes) |
| Export shape | `ratio96_stand_smoke_band105_export_shape` | unit test gate |

**Boundary:** band 105 = live stand smoke + export shape + loc-audit + docs canon; admin/ops glue was band 104.
Warnings-first: `ratio96_stand_smoke_depth` criteria registry is the gate source for `--ratio96-stand-smoke`.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --ratio96-stand-smoke
cargo run --bin poolai-loc-audit -- --ratio96-stand-smoke --advisory --min-ratio 0.95
VERIFY_RATIO96_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ratio96-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `ratio96_stand_smoke_mode` | `true` when `--ratio96-stand-smoke` (PH-S1694) |
| `ratio96_stand_smoke_criteria_total` | Registry size (10) |
| `ratio96_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`ratio96_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/ratio96_stand_smoke_depth.rs)
- stand smoke: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_ratio96_store_wire`, `smoke_ratio96_query`, `smoke_ratio96_field_fixtures`)
- export shape: `ratio96_stand_smoke_band105_export_shape` (PH-S1693)
- ops API: [`ops.rs`](../../src/network/api/ops.rs) (`GET /api/v1/ops/ratio96`)
- tests: `galaxy_horizon_s1689_integration.rs`
- store docs: [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md) · advisory: [`RATIO96_RATIO_ADVISORY.md`](./RATIO96_RATIO_ADVISORY.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.12 journal (band 105) · §5.86
- Phase F stretch gate (`stretch_spirit_gate_met` / `meets_min_ratio`)
- PH-S1689 · ratio96_stand_smoke_depth · PH-S1690 · smoke_ratio96_store_wire · PH-S1691 · smoke_ratio96_query · PH-S1692 · smoke_ratio96_field_fixtures · PH-S1693 · ratio96_stand_smoke_band105_export_shape · PH-S1694 · --ratio96-stand-smoke · PH-S1698
