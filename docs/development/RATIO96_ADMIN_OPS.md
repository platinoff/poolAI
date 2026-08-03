# Ratio96 Admin/Ops Glue — Phase F (Band 104)

Canonical doc: [`RATIO96_ADMIN_OPS.md`](./RATIO96_ADMIN_OPS.md) (band 104, PH-S1685).
Prev: [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md) (band 101 ✅). Mirror: [`MONITORING_ADMIN_OPS.md`](./MONITORING_ADMIN_OPS.md).

## Overview

Band 104 wires **admin UI + ops hooks** for the ratio96 store surface verified in band 101
(`ratio96_store_wire` / `ratio96_store_wire_json`, phase-F stretch gate). The durable store is
`docs/development/rust_ratio.json`, written by `poolai-loc-audit`; the admin dashboard strip reads
it through `GET /api/v1/ops/ratio96`.

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin` dashboard `#ratio96-store-badge` | `GET /api/v1/ops/ratio96` |
| Refresh ops glue | Dashboard **Refresh** (`refreshRatio96`) | re-reads the ratio96 store wire |
| Verify / quick | `VERIFY_RATIO96_ADMIN_OPS` / `--ratio96-admin-ops` | loc-audit gate |

**Boundary:** band 104 = admin/ops glue + verify/loc-audit + docs canon; live stand smoke for the
ratio96 slice is a later phase-F band. Warnings-first: `ratio96_admin_ops_depth` criteria registry
is the gate source for `--ratio96-admin-ops`.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --ratio96-admin-ops
cargo run --bin poolai-loc-audit -- --ratio96-admin-ops --advisory --min-ratio 0.95
VERIFY_RATIO96_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --ratio96-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `ratio96_admin_ops_mode` | `true` when `--ratio96-admin-ops` (PH-S1684) |
| `ratio96_admin_ops_criteria_total` | Registry size (10) |
| `ratio96_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`ratio96_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/ratio96_admin_ops_depth.rs)
- admin UI: [`dashboard.rs`](../../src/ui/admin/dashboard.rs) (`ratio96-store-badge` strip)
- store wire: [`ratio96_store_depth.rs`](../../crates/poolai-ui-core/src/ratio96_store_depth.rs) (`ratio96_store_wire_json`)
- ops API: [`ops.rs`](../../src/network/api/ops.rs) (`GET /api/v1/ops/ratio96`)
- tests: `ratio96_admin_ops_integration.rs`, `galaxy_horizon_s1679_integration.rs`
- store docs: [`RATIO96_DEPTH.md`](./RATIO96_DEPTH.md) · advisory: [`RATIO96_RATIO_ADVISORY.md`](./RATIO96_RATIO_ADVISORY.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.12 journal (band 104) · §5.85
- Phase F stretch gate (`stretch_spirit_gate_met` / `meets_min_ratio`)
- PH-S1679 · ratio96_admin_ops_depth · PH-S1680 · ratio96_store_wire_json · PH-S1681 · ratio96_admin_ops_integration · PH-S1682 · ratio96-store-badge · PH-S1684 · --ratio96-admin-ops · PH-S1688
