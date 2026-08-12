# GPU Limits — Phase H (Band 122)

Canonical doc: [`GPU_LIMITS.md`](./GPU_LIMITS.md) (band 122, PH-S1865). Full path:
`docs/development/GPU_LIMITS.md`.

## Overview

Band 122 (enterprise phase H, **GPU admission + worker limits, single-host**)
implements the **store/wire slice** for GPU admission config. Pattern mirror:
band 107 [`RATIO96_DOCS_CANON.md`](./RATIO96_DOCS_CANON.md) (depth registry + store +
contracts + verify + band close). Slice markers covered by `GPU_LIMITS_SLICES`
(3): `GPU_LIMITS.md`, `docs/development/GPU_LIMITS.md`, `docs/development/gpu_limits.json`.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `gpu_limits_depth.rs` | `GpuLimitsDepth` enum + criteria registry (10) |
| Durable store | `docs/development/gpu_limits.json` | `GpuLimits` config (max_gpus / gpu_memory_mb / admission_enabled / utilization_threshold) |
| Store/wire | `gpu_limits_store.rs` | `gpu_limits_store_load` / `gpu_limits_store_save` / `gpu_limits_store_wire_json` |
| API contracts | `tests/gpu_limits_integration.rs` | store roundtrip + wire shape markers |
| Verify / quick | `VERIFY_GPU_LIMITS` / `--gpu-limits` | verify-dev-stand gate + run-poolai quick flag |
| Stand smoke | `gpu_limits_band122_export_shape_ph_s1859` | unit export shape |
| Loc-audit | `--gpu-limits` | `gpu_limits_*` fields in `rust_ratio.json` |

**Boundary:** band 122 = durable store/wire slice; API surface, admin/ops glue,
stand smoke, loc-audit, vision-sync, and ratio advisory complete the phase-H band
pipeline (PH-S1859…S1868). Mirror: [`RATIO96_DOCS_CANON.md`](./RATIO96_DOCS_CANON.md).

## Durable store

`docs/development/gpu_limits.json` — GPU admission + worker-limit config:

```json
{
  "max_gpus": 0,
  "gpu_memory_mb": null,
  "admission_enabled": false,
  "utilization_threshold": null
}
```

| Field | Meaning |
|-------|---------|
| `max_gpus` | Max concurrently admitted GPU devices (0 = no GPU admission) |
| `gpu_memory_mb` | Per-worker GPU memory cap in MB (`null` = unlimited) |
| `admission_enabled` | GPU admission enabled for new workers |
| `utilization_threshold` | Utilization alert threshold percent (`null` = unset) |

`GpuLimitsStoreState.admission_active()` = `admission_enabled && max_gpus > 0`.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --gpu-limits
cargo run --bin poolai-loc-audit -- --gpu-limits --advisory --min-ratio 0.95
VERIFY_GPU_LIMITS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --gpu-limits
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `gpu_limits_mode` | `true` when `--gpu-limits` (PH-S1864) |
| `gpu_limits_criteria_total` | Registry size (10) |
| `gpu_limits_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`gpu_limits_depth.rs`](../../crates/poolai-ui-core/src/gpu_limits_depth.rs) ·
  [`gpu_limits_store.rs`](../../crates/poolai-ui-core/src/gpu_limits_store.rs)
- durable store: [`gpu_limits.json`](./gpu_limits.json)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `gpu_limits_integration.rs`, `galaxy_horizon_s1859_integration.rs`
- stand-smoke export shape: `gpu_limits_band122_export_shape_ph_s1859`
- roadmap: [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) band 122 ·
  [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md) phase H

## Related

- FM §5.103 band 122 journal · §5.12 header
- Markers: PH-S1859 · gpu_limits_depth · PH-S1860 · gpu_limits_store · PH-S1861 ·
  gpu_limits_integration · PH-S1862 · VERIFY_GPU_LIMITS · PH-S1864 · `--gpu-limits` · PH-S1868

# Band 123 — GPULimits API contracts

Canonical doc: [`GPU_LIMITS.md`](./GPU_LIMITS.md) (band 123, PH-S1875). Full path:
`docs/development/GPU_LIMITS.md`.

## Overview

Band 123 (enterprise phase H, **GPU admission + worker limits, single-host**)
implements the **HTTP API-contracts slice** over the band-122 durable store.
Pattern mirror: band 122 `gpu_limits_depth`. Slice markers covered by
`GPU_LIMITS_API_SLICES` (3): `GPU_LIMITS.md`, `docs/development/GPU_LIMITS.md`,
`docs/development/gpu_limits.json`.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `gpu_limits_api_depth.rs` | `GpuLimitsApiDepth` enum + criteria registry (10) |
| HTTP route | `src/network/api/system.rs` | `GET /api/v1/gpu-limits` → `gpu_limits_store_wire_json` |
| API contracts | `tests/gpu_limits_api_contracts_integration.rs` | 200 + wire markers |
| Verify / quick | `VERIFY_GPU_LIMITS_API` / `--gpu-limits-api` | verify-dev-stand gate + run-poolai quick flag |
| Stand smoke | `gpu_limits_api_band123_export_shape_ph_s1869` | unit export shape |
| Loc-audit | `--gpu-limits-api` | `gpu_limits_api_*` fields in `rust_ratio.json` |
| Band close | `tests/galaxy_horizon_s1869_integration.rs` | PH-S1878 · FM §5.104 |

**Boundary:** band 123 = HTTP API-contracts slice; band 124 (admin/ops glue)
completes the phase-H band pipeline (PH-S1879…S1888).

## HTTP surface

`src/network/api/system.rs` exposes `GET /api/v1/gpu-limits`:

```bash
curl -s http://127.0.0.1:3000/api/v1/gpu-limits
```

Responds with the durable store wire shape (`gpu_limits_store_wire_json`):
`mode`, `available`, `max_gpus`, `gpu_memory_mb_cap`, `admission_enabled`,
`admission_active`.

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --gpu-limits-api
cargo run --bin poolai-loc-audit -- --gpu-limits-api --advisory --min-ratio 0.95
VERIFY_GPU_LIMITS_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --gpu-limits-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `gpu_limits_api_mode` | `true` when `--gpu-limits-api` (PH-S1872) |
| `gpu_limits_api_criteria_total` | Registry size (10) |
| `gpu_limits_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`gpu_limits_api_depth.rs`](../../crates/poolai-ui-core/src/gpu_limits_api_depth.rs)
- HTTP route: [`system.rs`](../../src/network/api/system.rs)
- tests: `gpu_limits_api_contracts_integration.rs`, `galaxy_horizon_s1869_integration.rs`
- stand-smoke export shape: `gpu_limits_api_band123_export_shape_ph_s1869`
- roadmap: [`PH_S_MASTER_BACKLOG_1000.md`](./PH_S_MASTER_BACKLOG_1000.md) band 123 ·
  [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md) phase H

## Related

- FM §5.104 band 123 journal · §5.12 header
- Markers: PH-S1869 · gpu_limits_api_depth · PH-S1870 · gpu-limits · PH-S1871 ·
  gpu_limits_api_contracts_integration · PH-S1872 · VERIFY_GPU_LIMITS_API ·
  PH-S1874 · `--gpu-limits-api` · PH-S1878
- Band 124 (admin/ops glue, PH-S1879…S1888): [`GPU_LIMITS_ADMIN_OPS.md`](./GPU_LIMITS_ADMIN_OPS.md) ·
  `gpu-limits-store-badge` · `refreshGpuLimits` · `VERIFY_GPU_LIMITS_ADMIN_OPS`
