# GPULimits Admin/Ops Glue — Phase H (Band 124)

Canonical doc: [`GPU_LIMITS_ADMIN_OPS.md`](./GPU_LIMITS_ADMIN_OPS.md) (band 124, PH-S1885).
Prev: [`GPU_LIMITS.md`](./GPU_LIMITS.md) (bands 122 ✅ · 123 ✅). Mirror: [`RATIO96_ADMIN_OPS.md`](./RATIO96_ADMIN_OPS.md).

## Overview

Band 124 wires **admin UI + ops hooks** for the GPULimits store surface verified in band 122
(`gpu_limits_store` / `gpu_limits_store_wire_json`, enterprise phase H) and band 123
(`GET /api/v1/gpu-limits`). The durable store is `docs/development/gpu_limits.json`; the admin
dashboard strip reads it through `GET /api/v1/gpu-limits`.

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin` dashboard `#gpu-limits-store-badge` | `GET /api/v1/gpu-limits` |
| Refresh ops glue | Dashboard **Refresh** (`refreshGpuLimits`) | re-reads the GPULimits store wire |
| Verify / quick | `VERIFY_GPU_LIMITS_ADMIN_OPS` / `--gpu-limits-admin-ops` | loc-audit gate |

**Boundary:** band 124 = admin/ops glue + verify/loc-audit + docs canon; live stand smoke for the
GPULimits slice is a later phase-H band. Warnings-first: `gpu_limits_admin_ops_depth` criteria
registry is the gate source for `--gpu-limits-admin-ops`.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --gpu-limits-admin-ops
cargo run --bin poolai-loc-audit -- --gpu-limits-admin-ops --advisory --min-ratio 0.95
VERIFY_GPU_LIMITS_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --gpu-limits-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `gpu_limits_admin_ops_mode` | `true` when `--gpu-limits-admin-ops` (PH-S1884) |
| `gpu_limits_admin_ops_criteria_total` | Registry size (10) |
| `gpu_limits_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`gpu_limits_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/gpu_limits_admin_ops_depth.rs)
- admin UI: [`dashboard.rs`](../../src/ui/admin/dashboard.rs) (`gpu-limits-store-badge` strip)
- store wire: [`gpu_limits_store.rs`](../../crates/poolai-ui-core/src/gpu_limits_store.rs) (`gpu_limits_store_wire_json`)
- HTTP surface: [`system.rs`](../../src/network/api/system.rs) (`GET /api/v1/gpu-limits`)
- tests: `gpu_limits_admin_ops_integration.rs`, `galaxy_horizon_s1879_integration.rs`
- store docs: [`GPU_LIMITS.md`](./GPU_LIMITS.md) · canon: [`GPU_LIMITS_ADMIN_OPS.md`](./GPU_LIMITS_ADMIN_OPS.md)
- roadmap: [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md)

## Related

- FM §5.12 journal (band 124) · §5.105
- Phase H stretch gate (`stretch_spirit_gate_met` / `meets_min_ratio`)
- PH-S1879 · gpu_limits_admin_ops_depth · PH-S1880 · gpu_limits_store_wire_json · PH-S1881 · gpu_limits_admin_ops_contracts · PH-S1882 · gpu-limits-store-badge · PH-S1884 · --gpu-limits-admin-ops · PH-S1888
