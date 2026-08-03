# Ratio96 Loc-Audit — Phase F (Band 106)

Canonical doc: [`RATIO96_LOC_AUDIT.md`](./RATIO96_LOC_AUDIT.md) (band 106, PH-S1704).
Prev: [`RATIO96_STAND_SMOKE.md`](./RATIO96_STAND_SMOKE.md) (band 105 ✅). Mirror: [`MONITORING_LOC_AUDIT.md`](./MONITORING_LOC_AUDIT.md).

## Overview

Band 106 performs **loc-audit for ratio96** store/wire/migration surface verified in band 105
(`ratio96_stand_smoke_depth`, `smoke_ratio96_store_wire`, `smoke_ratio96_query`). The loc-audit runs via `poolai-loc-audit --ratio96-loc-audit`
and validates the ratio96 slice against the running dev stand.

| Surface | Where | Notes |
|---------|-------|-------|
| Loc-audit smoke | `smoke_ratio96_loc_audit` | `GET /api/v1/ops/ratio96` live call |
| Migration advisory | `smoke_ratio96_migration_advisory` | ratio96 migration path validation |
| Export shape | `ratio96_loc_audit_band106_export_shape` | unit test gate |

**Boundary:** band 106 = loc-audit + migration advisory + export shape + docs canon; stand smoke was band 105.
Warnings-first: `ratio96_loc_audit_depth` criteria registry is the gate source for `--ratio96-loc-audit`.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --ratio96-loc-audit
cargo run --bin poolai-loc-audit -- --ratio96-loc-audit --advisory --min-ratio 0.95
VERIFY_RATIO96_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ratio96-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `ratio96_loc_audit_mode` | `true` when `--ratio96-loc-audit` (PH-S1703) |
| `ratio96_loc_audit_criteria_total` | Registry size (10) |
| `ratio96_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`ratio96_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/ratio96_loc_audit_depth.rs)
- stand smoke: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_ratio96_loc_audit`, `smoke_ratio96_migration_advisory`)
- export shape: `ratio96_loc_audit_band106_export_shape` (PH-S1702)
- ops API: [`ops.rs`](../../src/network/api/ops.rs) (`GET /api/v1/ops/ratio96`)
- tests: `galaxy_horizon_s1699_integration.rs`
- store docs: [`RATIO96_STAND_SMOKE.md`](./RATIO96_STAND_SMOKE.md) · advisory: [`RATIO96_RATIO_ADVISORY.md`](./RATIO96_RATIO_ADVISORY.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.12 journal (band 106) · §5.87
- Phase F stretch gate (`stretch_spirit_gate_met` / `meets_min_ratio`)
- PH-S1699 · ratio96_loc_audit_depth · PH-S1700 · smoke_ratio96_loc_audit · PH-S1701 · smoke_ratio96_migration_advisory · PH-S1702 · ratio96_loc_audit_band106_export_shape · PH-S1703 · --ratio96-loc-audit · PH-S1708
