# Performance benchmarks

## Overview

This document ties together **reproducible micro-benchmarks** (Criterion, in-tree) and **higher-level targets** (HTTP, RAID throughput) used for design and capacity planning.

- **How to profile hot paths**: [`PROFILING.md`](./PROFILING.md)
- **Runtime / Tokio tuning**: [`TUNING.md`](./TUNING.md)
- **Architecture-oriented measurement backlog**: [`../development/PERFORMANCE_OPTIMIZATION_PLAN_2026-03-17.md`](../development/PERFORMANCE_OPTIMIZATION_PLAN_2026-03-17.md)

**PoolAI version**: 0.2.x. **Rust**: stable (see workspace MSRV notes in `Cargo.toml` / `README.md`).

---

## Criterion benchmarks (measured locally)

Benchmarks live under `benches/` and use [Criterion](https://github.com/bheisler/criterion.rs). They report mean / median and confidence intervals in the terminal and write HTML under `target/criterion/`.

### Register targets in `Cargo.toml`

| Bench target | Command | Notes |
|--------------|---------|--------|
| `runtime_benchmarks` | `cargo bench -j 1 --bench runtime_benchmarks` | Memory pool, LRU, model structs, cache keys, local RAID put, **replication engine** (node selection + quorum), **VM lifecycle**, **RAID protocol JSON**, **health-shaped JSON**. Same workspace **`bench`** / MSVC workaround as `turboquant_benchmarks` (see baseline note). |
| `turboquant_benchmarks` | `cargo bench -j 1 --bench turboquant_benchmarks --features ml` | TurboQuant pack/unpack and `dot_f32` (requires `ml`). Workspace **`bench`** profile uses `opt-level = 0` so `cargo bench` can complete on MSVC hosts where `rustc` otherwise AVs on the full crate (see baseline note below). |
| `cloud_benchmarks` | `cargo bench -j 1 --bench cloud_benchmarks --features cloud` | `CloudConfig::validate`, manager `initialize`/`shutdown` (default config). For kube OpenAPI alignment with CI, set **`K8S_OPENAPI_ENABLED_VERSION=1.28`** (see [`.github/workflows/benchmarks.yml`](../../.github/workflows/benchmarks.yml)). |
| `service_layer_benchmarks` | `cargo bench -j 1 --bench service_layer_benchmarks --features test-utils` | `RaidService` list/quota/cluster_status over temp `RaidManager`. Same **`bench`** / MSVC workaround as other Criterion targets (see baseline note). |
| `http_hotpath_benchmarks` | `cargo bench -j 1 --bench http_hotpath_benchmarks` | **FM-042:** `http_json_errors` (`api_error_response`, status map), `http_trace` (`make_http_span`). |
| `sharding_benchmarks` | `cargo bench -j 1 --bench sharding_benchmarks` | FM-036: `tensor_shard_plan_build_4_nodes`, `shard_sync_bus/all_reduce_step_4_nodes`. |

Use **`-j 1`** on memory-constrained hosts (e.g. Windows linking many binaries) to reduce parallel link pressure.

### Groups in `runtime_benchmarks`

- **`memory_pool`**: `acquire_release_request`, `acquire_release_response`, `acquire_release_string`
- **`lru_cache`**: `get_hit`, `get_miss`, `put_new`, `put_existing`, `get_hit_variable_size` (cache capacities 100–5000)
- **`model_request_response`**: `create_request`, `clone_request`
- **`cache_key_generation`**: `generate_cache_key` (hash over request fields)
- **`raid_local_put`**: `put_artifact_4096` — temp-dir `RaidManager`, 4 KiB payload, unique logical names per iteration
- **`raid_replication_engine`**: `select_replication_nodes_factor_3` (64 registered nodes, factor 3), `calculate_quorum_rf_7` — in-process control plane only; wire replication remains environment-specific. For **artifact byte volume** before/after compression, compare `put_artifact` payload sizes with TurboQuant/TQ01 outputs (`turboquant_benchmarks`, ML pipeline).
- **`vm_lifecycle`**: `create_start_stop_delete` — fresh `VmManager` per iteration (initialize → create → start → stop → delete → shutdown)
- **`raid_protocol_put_payload`**: `serde_json_roundtrip` — `PutArtifactPayload` (+ ~1 KiB `data` string); proxy for node-to-node JSON cost, not real sockets
- **`http_health_json`**: `serde_json_to_vec` — static `serde_json::Value` shaped like `GET /api/v1/health` (serialization only; use `wrk` for end-to-end RPS)

### Groups in `cloud_benchmarks` (`--features cloud`)

- **`cloud_config`**: `validate_default`
- **`cloud_manager`**: `init_shutdown_default_config`

### Groups in `service_layer_benchmarks` (`--features test-utils`)

- **`raid_service`**: `list_artifacts`, `quota`, `cluster_status` (one warmup `put_artifact` on temp storage)

### Group in `turboquant_benchmarks` (`--features ml`)

- **`turboquant`**: `pack_uniform_rows_64x256`, `unpack_to_rows_64x256`, `dot_f32_4096` (single Criterion group)

### Groups in `http_hotpath_benchmarks` (FM-042)

- **`http_json_errors`**: `http_status_for_app_error_not_found`, `api_error_response_not_found`, `api_error_response_validation` (JSON body serialize to `Vec<u8>`)
- **`http_trace`**: `make_http_span_health` — per-request span from FM-038 `TraceLayer` (no OTLP)

### Groups in `sharding_benchmarks` (FM-036)

- **`tensor_shard_plan_build_4_nodes`**: `build_tensor_shard_plan` for 4 nodes
- **`shard_sync_bus`**: `all_reduce_step_4_nodes` — in-process sync bus step

Record the Criterion summary lines (or archive `target/criterion/`) when publishing regression comparisons.

### Reference machine baselines (manual)

After a full run on a **named** host (CPU model, RAM, OS, disk type), paste Criterion means (or a link to saved HTML) into the table below. Replace placeholders—**not** CI-enforced.

The **dev sample** rows used a shortened Criterion profile (`--sample-size 20`, `--warm-up-time 0.3`, `--measurement-time 0.5`) on **Windows 10**, `cargo bench` **release** profile — for regression *trends* on the same machine, not absolute SLA.

The **win11-criterion-full** row set is a default Criterion profile (100 samples × ~5 s measurement each, 3 s warmup) from `cargo bench -j 1 --bench runtime_benchmarks -- --noplot` on **Windows** (release), 2026-04-06 — use as a **P4** snapshot until a named Linux ref host is recorded.

**MSVC / `cargo bench` note:** On some Windows **MSVC** hosts, `rustc` exits with **STATUS_ACCESS_VIOLATION** when compiling the full `poolai` library at typical release-style optimization for `cargo bench`. The workspace sets **`[profile.bench]`** with **`opt-level = 0`** so Criterion targets (**`turboquant_benchmarks`**, **`runtime_benchmarks`**, …) can link and run. Rows labelled **win-msvc-*-bench-opt0-2026-04-06** used the **short Criterion profile** (`--sample-size 20`, `--warm-up-time 0.3`, `--measurement-time 0.5`) and are for **before/after trends on one machine** only — not for comparing absolute speed to **dev-win-sample** / **win11-criterion-full** (those used higher codegen). Prefer **GNU** (`1.92.0-x86_64-pc-windows-gnu` + toolchain in PATH per repo docs) when you need release-grade bench binaries on Windows.

| Bench / function | Host label | Median (typical Criterion centre of `[low … high]`) | Date |
|------------------|------------|-----------------------------------------------------|------|
| `memory_pool/acquire_release_request` | dev-win-sample | ~132 ns | 2026-04-06 |
| `memory_pool/acquire_release_request` | win11-criterion-full-2026-04-06 | ~126 ns | 2026-04-06 |
| `lru_cache/get_hit` | dev-win-sample | ~235 ns | 2026-04-06 |
| `lru_cache/get_hit` | win11-criterion-full-2026-04-06 | ~238 ns | 2026-04-06 |
| `raid_local_put/put_artifact_4096` | dev-win-sample | ~11.7 ms | 2026-04-06 |
| `raid_local_put/put_artifact_4096` | win11-criterion-full-2026-04-06 | ~8.68 ms | 2026-04-06 |
| `raid_replication_engine/select_replication_nodes_factor_3` | win11-criterion-full-2026-04-06 | ~229 ns | 2026-04-06 |
| `raid_replication_engine/calculate_quorum_rf_7` | win11-criterion-full-2026-04-06 | ~0.68 ns | 2026-04-06 |
| `vm_lifecycle/create_start_stop_delete` | dev-win-sample | ~5.2 µs | 2026-04-06 |
| `vm_lifecycle/create_start_stop_delete` | win11-criterion-full-2026-04-06 | ~5.26 µs | 2026-04-06 |
| `raid_protocol_put_payload/serde_json_roundtrip` | dev-win-sample | ~5.6 µs | 2026-04-06 |
| `raid_protocol_put_payload/serde_json_roundtrip` | win11-criterion-full-2026-04-06 | ~5.28 µs | 2026-04-06 |
| `http_health_json/serde_json_to_vec` | dev-win-sample | ~1.09 µs | 2026-04-06 |
| `http_health_json/serde_json_to_vec` | win11-criterion-full-2026-04-06 | ~1.02 µs | 2026-04-06 |
| `memory_pool/acquire_release_request` | win-msvc-runtime-bench-opt0-2026-04-06 | ~1.35 µs | 2026-04-06 |
| `lru_cache/get_hit` | win-msvc-runtime-bench-opt0-2026-04-06 | ~1.94 µs | 2026-04-06 |
| `raid_local_put/put_artifact_4096` | win-msvc-runtime-bench-opt0-2026-04-06 | ~8.64 ms | 2026-04-06 |
| `raid_replication_engine/select_replication_nodes_factor_3` | win-msvc-runtime-bench-opt0-2026-04-06 | ~2.68 µs | 2026-04-06 |
| `raid_replication_engine/calculate_quorum_rf_7` | win-msvc-runtime-bench-opt0-2026-04-06 | ~16.8 ns | 2026-04-06 |
| `vm_lifecycle/create_start_stop_delete` | win-msvc-runtime-bench-opt0-2026-04-06 | ~38.5 µs | 2026-04-06 |
| `raid_protocol_put_payload/serde_json_roundtrip` | win-msvc-runtime-bench-opt0-2026-04-06 | ~58 µs | 2026-04-06 |
| `http_health_json/serde_json_to_vec` | win-msvc-runtime-bench-opt0-2026-04-06 | ~12.9 µs | 2026-04-06 |
| `memory_pool/acquire_release_request` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~1.25 µs | 2026-04-10 |
| `lru_cache/get_hit` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~1.64 µs | 2026-04-10 |
| `raid_local_put/put_artifact_4096` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~14.9 ms | 2026-04-10 |
| `raid_replication_engine/select_replication_nodes_factor_3` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~2.20 µs | 2026-04-10 |
| `raid_replication_engine/calculate_quorum_rf_7` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~13.8 ns | 2026-04-10 |
| `vm_lifecycle/create_start_stop_delete` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~22.3 µs | 2026-04-10 |
| `raid_protocol_put_payload/serde_json_roundtrip` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~44.4 µs | 2026-04-10 |
| `http_health_json/serde_json_to_vec` | win10-local-26200-runtime-bench-opt0-2026-04-10 | ~10.6 µs | 2026-04-10 |
| `memory_pool/acquire_release_request` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~1.28 µs | 2026-04-12 |
| `lru_cache/get_hit` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~1.66 µs | 2026-04-12 |
| `raid_local_put/put_artifact_4096` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~7.89 ms | 2026-04-12 |
| `raid_replication_engine/select_replication_nodes_factor_3` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~2.18 µs | 2026-04-12 |
| `raid_replication_engine/calculate_quorum_rf_7` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~13.7 ns | 2026-04-12 |
| `vm_lifecycle/create_start_stop_delete` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~23.8 µs | 2026-04-12 |
| `raid_protocol_put_payload/serde_json_roundtrip` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~43.3 µs | 2026-04-12 |
| `http_health_json/serde_json_to_vec` | win10-local-26200-runtime-bench-opt0-2026-04-12 | ~10.5 µs | 2026-04-12 |
| `turboquant/pack_uniform_rows_64x256` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~491 µs | 2026-04-06 |
| `turboquant/unpack_to_rows_64x256` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~80.5 µs | 2026-04-06 |
| `turboquant/dot_f32_4096` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~10.2 µs | 2026-04-06 |
| `turboquant/pack_uniform_rows_64x256` | win10-local-26200-turboquant-bench-opt0-2026-04-12 | ~438 µs | 2026-04-12 |
| `turboquant/unpack_to_rows_64x256` | win10-local-26200-turboquant-bench-opt0-2026-04-12 | ~68.7 µs | 2026-04-12 |
| `turboquant/dot_f32_4096` | win10-local-26200-turboquant-bench-opt0-2026-04-12 | ~8.33 µs | 2026-04-12 |
| `raid_service/list_artifacts` | dev-win-sample | ~218 ns | 2026-04-06 |
| `raid_service/quota` | dev-win-sample | ~74.5 µs | 2026-04-06 |
| `raid_service/cluster_status` | dev-win-sample | ~55.2 µs | 2026-04-06 |
| `raid_service/list_artifacts` | win-msvc-service-layer-bench-opt0-2026-04-06 | ~1.40 µs | 2026-04-06 |
| `raid_service/quota` | win-msvc-service-layer-bench-opt0-2026-04-06 | ~61.6 µs | 2026-04-06 |
| `raid_service/cluster_status` | win-msvc-service-layer-bench-opt0-2026-04-06 | ~81.5 µs | 2026-04-06 |
| `raid_service/list_artifacts` | win10-local-26200-service-layer-bench-opt0-2026-04-12 | ~1.30 µs | 2026-04-12 |
| `raid_service/quota` | win10-local-26200-service-layer-bench-opt0-2026-04-12 | ~60.4 µs | 2026-04-12 |
| `raid_service/cluster_status` | win10-local-26200-service-layer-bench-opt0-2026-04-12 | ~71.2 µs | 2026-04-12 |
| `cloud_config/validate_default` | win-msvc-cloud-bench-opt0-2026-04-06 | ~15.6 ns | 2026-04-06 |
| `cloud_manager/init_shutdown_default_config` | win-msvc-cloud-bench-opt0-2026-04-06 | ~2.78 µs | 2026-04-06 |
| `wrk` `/api/v1/health` | *manual* | RPS / p50 / p95 | — |
| *your ref host* / … | *e.g. ref-linux-01* | *from Criterion* | *YYYY-MM-DD* |

### `poolai_health_load --json` (FM-003 / P4 baseline)

After **`cargo run --release --bin poolai_health_load -- --json <URL> <seconds> <concurrency>`** against a running server on a **named ref host**, append one row per run. Paste numeric fields from the JSON object on stdout (`rps_ok_only`, `latency_p50_ms`, `latency_p95_ms`, `latency_p99_ms`, `ok_requests`, `error_requests`, `wall_seconds`). LAN / P2b replication + TQ01 volume checks stay manual on the stand (open checkbox in `NEXT_STEPS_ARCHITECT`).

| Host label | URL | wall_s | workers | ok | errors | rps_ok | p50_ms | p95_ms | p99_ms | Date |
|------------|-----|--------|---------|-----|--------|--------|--------|--------|--------|------|
| *e.g. ref-linux-01* | `http://127.0.0.1:8080/api/v1/health` | *from JSON* | *from CLI* | *ok_requests* | *error_requests* | *rps_ok_only* | *latency_p50_ms* | *latency_p95_ms* | *latency_p99_ms* | *YYYY-MM-DD* |
| win10-local-26200 | `http://127.0.0.1:8080/api/v1/health` | 5.006 | 50 | 149858 | 0 | 29934.63 | 1.568 | 2.704 | 3.623 | 2026-04-10 |
| win10-local-26200 | `http://127.0.0.1:8080/api/v1/health` | 5.016 | 50 | 18221 | 0 | 3632.46 | 12.136 | 24.848 | 34.903 | 2026-05-18 |
| win10-local-26200-dual-stand / node-A | `http://127.0.0.1:8080/api/v1/health` | 10.017 | 50 | 35145 | 0 | 3508.67 | 12.310 | 27.391 | 41.334 | 2026-05-20 |
| win10-local-26200-dual-stand / node-B | `http://127.0.0.1:8081/api/v1/health` | 10.013 | 50 | 37458 | 0 | 3740.84 | 11.982 | 23.918 | 31.976 | 2026-05-20 |

### P2b single-host dual-port stand (FM-028)

Captured via **`bin/capture-p2b-single-host-metrics.sh`** after **`bin/run-lan-nodes.sh`** (`enterprise,ml,cloud,test-utils`, debug `poolai` on `:8080` + `:8081`). Raw JSON: `data/lan-stand/metrics-fm028-YYYYMMDD.json` (gitignored). **Not** a substitute for FM-003 §4 two-physical-host sign-off.

| Metric | Node A (`:8080`) | Node B (`:8081`) | Notes |
|--------|------------------|------------------|-------|
| `poolai_health_load` wall_s | 10.017 | 10.013 | concurrency 50 |
| `rps_ok_only` | 3508.67 | 3740.84 | release load gen → debug servers |
| `latency_p50_ms` | 12.31 | 11.98 | |
| `latency_p95_ms` | 27.39 | 23.92 | |
| TQ01 64×256 `bytes_in` → `bytes_out` | — | — | 65536 → 16653 (ratio **3.94×**) |
| Wire JSON PutArtifact (base64 payload) | — | — | TQ01 **22651** B vs raw f32 **87830** B (**74.2%** smaller) |
| Host label | `win10-local-26200-dual-stand` | same stand | 2026-05-20 |

TQ01 numbers from **`cargo run --bin poolai-p2b-tq01-snapshot --features ml`** (same matrix as `distributed_raid_wire_integration`).

### Target metrics (P4 roadmap, non-binding)

Use these as **internal guardrails** when changing hot paths; replace with numbers from your ref host. They are **not** enforced in CI.

| Area | Criterion group / function | Soft target (same machine, trend) |
|------|---------------------------|-----------------------------------|
| In-memory | `memory_pool/*`, `lru_cache/get_hit` | No large regression vs last saved Criterion baseline |
| Local RAID | `raid_local_put/put_artifact_4096` | Stable median order-of-magnitude on fixed disk temp dir |
| Replication (CP) | `raid_replication_engine/*` | Sub-ms median for `select_*` with ≤100 registered nodes; `calculate_quorum` nanosecond-scale |
| Service layer | `raid_service/list_*` | Sub-µs median for list/queries on tiny temp stores |
| JSON | `http_health_json`, `raid_protocol_put_payload`, **`http_json_errors/*`** | Track median; investigate if >2× prior baseline |
| HTTP trace | `http_trace/make_http_span_health` | FM-038 span overhead — track trend on same host |
| Sharding | `tensor_shard_plan_build_*`, `shard_sync_bus/*` | FM-036 — no large regression vs prior baseline |
| TurboQuant | `turboquant/*` (`--features ml`) | Stable pack/unpack; `dot_f32` scales linearly with dimension |
| Cloud (config + manager) | `cloud_config/validate_default`, `cloud_manager/init_shutdown_default_config` (`--features cloud`) | Track median; init/shutdown includes async runtime — expect low-µs order on default config |

### Changelog (bench docs)

| Date | Note |
|------|------|
| 2026-05-23 | **FM-042:** `http_hotpath_benchmarks` (JSON errors + `make_http_span`); `sharding_benchmarks` у [`benchmarks.yml`](../../.github/workflows/benchmarks.yml); PROFILING.md § FM-042. |
| 2026-05-18 | AUTO_RUN 2026-06-08 S1: **P4** `poolai_health_load --json` (release, MSYS2 UCRT64) на **win10-local-26200** — рядок у таблиці `poolai_health_load`; coordinator already on `:8080`; FM-003 §4 лишається **BLOCKED**. |
| 2026-05-20 | **FM-028:** single-host dual-port stand — `capture-p2b-single-host-metrics.sh`, `poolai-p2b-tq01-snapshot`; health_load rows **node-A/B** + TQ01 table above; artifact `data/lan-stand/metrics-fm028-*.json`. |
| 2026-05-20 | FM-027: LAN sign-off prep — [`LAN_SIGNOFF_CHECKLIST.md`](./LAN_SIGNOFF_CHECKLIST.md), `bin/verify-lan-prep.*`; §4 sign-off **BLOCKED** (2 physical hosts); без нового health_load row. |
| 2026-06-01 | AUTO_RUN 2026-06-01: FM-003 §4 **BLOCKED** (2 фізичні хости відсутні); [`LAN_BENCHMARK_RUNBOOK.md`](./LAN_BENCHMARK_RUNBOOK.md) §6 оновлено; dev stand §5.1 (`verify-dev-stand`) — канон для однієї машини; **без** нового `poolai_health_load` / LAN replication row (baseline **2026-04-10** чинний). |
| 2026-05-17 | AUTO_RUN 2026-05-17 S1: LAN-стенд недоступний (1 хост); runbook оновлено; FM-003 **Planned (ops)** — без нового рядка `poolai_health_load` (baseline **2026-04-10** чинний). |
| 2026-05-16 | FM-003 ops: [`LAN_BENCHMARK_RUNBOOK.md`](./LAN_BENCHMARK_RUNBOOK.md) (два вузли LAN, replication + TQ01); baseline `poolai_health_load` — рядок **2026-04-10** лишається чинним до нового прогону на стенді. |
| 2026-04-12 | FM-007 / P2b harness **`distributed_raid_wire_integration`**: додано wire-тести **`SyncArtifacts`** — **Pull** (`missing_artifacts`), **Bidirectional** (симетрична різниця, відсортована), відсутність conflict при однаковому `stored_at` у `remote_versions`; юніт-тест **`no_conflict_when_local_and_remote_timestamps_equal`** у `raid_distributed_protocol_service`. |
| 2026-04-12 | FM-003 / P4: short Criterion (`--sample-size 20`, `--warm-up-time 0.3`, `--measurement-time 0.5`) на **win10-local-26200** — `runtime_benchmarks`, `service_layer_benchmarks` (`--features test-utils`), `turboquant_benchmarks` (`--features ml`); рядки **`win10-local-26200-*-bench-opt0-2026-04-12`** у таблиці baseline. |
| 2026-04-10 | FM-003 / P4: `cargo bench -j 1 --bench runtime_benchmarks -- --noplot` на **win10-local-26200**; додано baseline-рядки `win10-local-26200-runtime-bench-opt0-2026-04-10` + `poolai_health_load --json` рядок у таблицю. |
| 2026-04-07 | P2b harness **`distributed_raid_wire_integration`**: wire-тести **`SyncArtifacts`** (Push / `missing_artifacts`) та **`LeaveCluster`** (перевірка членства кластера перед graceful replication). |
| 2026-04-06 | P4: **`poolai_health_load --json`** — структурований звіт на stdout для baseline / `jq`; юніт-тести парсера аргументів у `src/bin/poolai_health_load.rs`. |
| 2026-04-06 | Filled `raid_service/quota`/`cluster_status` dev-sample medians; P4 target table; `std::hint::black_box` in benches; `ui.rs` gates `State` on `enterprise`; Criterion group `raid_replication_engine` in `runtime_benchmarks` (P2b proxy vs full multi-node I/O). |
| 2026-04-06 | P4: `runtime_benchmarks` full-profile snapshot (`win11-criterion-full-2026-04-06`); P2b: `tests/distributed_raid_wire_integration.rs` (`test-utils`, optional `ml` for TQ01 wire JSON size). |
| 2026-04-06 | P2b: TurboQuant **decode** inner loop — 4-wide unroll + tail (`push_dequantized_row`), симетрично до pack; прогін `turboquant_benchmarks` після зміни — за бажанням на реф-хості. |
| 2026-04-06 | P4: `turboquant_benchmarks` — фактичні медіани (short Criterion) у таблиці baseline під міткою **win-msvc-turboquant-bench-opt0-2026-04-06**; у кореневому `Cargo.toml` додано **`[profile.bench] opt-level = 0`** як обхід **MSVC rustc AV** на збірці `poolai` для `cargo bench`. |
| 2026-04-06 | P4: `runtime_benchmarks` — той самий short Criterion + **win-msvc-runtime-bench-opt0-2026-04-06** у таблиці baseline; нотатка MSVC узагальнена для всіх `cargo bench` targets. |
| 2026-04-06 | P4: `service_layer_benchmarks` (`--features test-utils`) — медіани під **win-msvc-service-layer-bench-opt0-2026-04-06**; колонка *Notes* у таблиці реєстрації. |
| 2026-04-06 | P4: `cloud_benchmarks` — short Criterion + **`K8S_OPENAPI_ENABLED_VERSION=1.28`**, медіани під **win-msvc-cloud-bench-opt0-2026-04-06** (`cloud_config/*`, `cloud_manager/*`). |
| 2026-04-06 | P4: бінарник **`poolai_health_load`** (`src/bin/poolai_health_load.rs`) — HTTP навантаження **`GET /api/v1/health`** на Rust (`reqwest` + Tokio), без зовнішніх load-генераторів. |
| 2026-04-06 | FM-003: таблиця **`poolai_health_load --json`** для рядків baseline на реф-хості; LAN-заміри реплікації / TQ01 — окремо на стенді. |

---

## HTTP and system load tests (manual)

With the server running (default or configured bind address):

### In-tree Rust load tool (no wrk / hey / Python)

**`poolai_health_load`** — Tokio + `reqwest`, same process style as the rest of the repo. Arguments: optional **`--json`** (machine-readable report on **stdout**), then `URL` (optional), duration in **seconds** (default `30`), concurrent **workers** (default `400`). The `--json` flag may appear before or after the positional arguments.

```bash
# Terminal 1
cargo run --release

# Terminal 2 — same shape as wrk -c400 -d30s
cargo run --release --bin poolai_health_load -- http://127.0.0.1:8080/api/v1/health 30 400

# One-line baseline capture (pretty JSON)
cargo run --release --bin poolai_health_load -- --json http://127.0.0.1:8080/api/v1/health 30 400 > health_load_report.json
```

Human mode (default) prints wall time, OK count, errors, RPS (successful requests only), and latency mean / p50 / p95 / p99 to **stderr**. Above **200k** successful requests, latencies use a **reservoir sample** (see stderr note). **`--json`** writes a single JSON object to **stdout** (`rps_ok_only`, `latency_p50_ms`, `total_ok_exceeds_sample`, …) suitable for ref-host rows or `jq`.

### External tools (optional)

```bash
# wrk
wrk -t12 -c400 -d30s http://127.0.0.1:8080/api/v1/health

# Apache Bench
ab -n 100000 -c 100 http://127.0.0.1:8080/api/v1/health

# hey
hey -n 100000 -c 100 http://127.0.0.1:8080/api/v1/health
```

Interpretation depends on CPU count, TLS, auth middleware, and disk; compare against your own baseline after deploy.

On Windows, **`wrk` is often absent** from PATH; prefer **`poolai_health_load`**, or WSL / a Linux ref host, or **`hey`** / **`ab`**.

### P2b stand harness (in-tree)

Automated **HTTP wire** path for distributed `PutArtifact` (Axum `oneshot`, temp `RaidManager`), plus an optional **TQ01 vs raw f32** JSON size check when built with **`ml`**:

```bash
cargo test -j 1 --features test-utils --test distributed_raid_wire_integration
cargo test -j 1 --features test-utils,ml --test distributed_raid_wire_integration
```

Multi-node LAN timings remain **manual** on your stand; this test locks the handler + serde path.

---

## Illustrative design targets (not CI-guarded)

The tables below are **example staging numbers** for documentation and roadmap discussions. They are **not** continuously verified in CI—replace them with measurements from your environment (Criterion + `wrk`/tracing) when making performance claims.

### HTTP API (example)

| Endpoint | Method | Example RPS | Example p50 (ms) | Example p95 (ms) |
|----------|--------|-------------|------------------|-------------------|
| `/api/v1/health` | GET | high | low | low |
| `/api/v1/workers` | GET | medium | medium | higher with auth |
| `/api/v1/raid/artifacts` | GET/POST | workload-dependent | — | — |

### RAID / replication (example)

| Scenario | Direction | Example throughput | Example latency |
|----------|-----------|-------------------|-----------------|
| Local RAID | sequential read/write | hardware-bound | sub-ms to few ms |
| Replicated | write (quorum) | lower than local | network + fsync bound |

### VM / worker pool (example)

| Metric | Example target |
|--------|----------------|
| Instance lifecycle | sub-second on lightweight paths |
| Max workers / instances | deployment-specific |

---

## Continuous benchmarking (optional CI)

У репозиторії: **[`.github/workflows/benchmarks.yml`](../../.github/workflows/benchmarks.yml)** — `workflow_dispatch` і щотижневий cron (неділя 06:00 UTC), усі чотири bench-таргети з `-j 1`, `K8S_OPENAPI_ENABLED_VERSION=1.28` для `cloud_benchmarks`, артефакт **`criterion-report`** (`target/criterion/`).

`--noplot` у workflow прибирає зайве від plotters, якщо достатньо консольного виводу.

---

## Notes

- Micro-benchmarks isolate hot paths; end-to-end latency includes JSON, auth, and I/O.
- Distributed RAID on the wire: use `raid_protocol_put_payload` for JSON CPU cost; real replication still needs topology/peers and is environment-specific.
- Cloud SDK calls are not exercised by `cloud_benchmarks` (config + manager lifecycle only).
- `service_layer_benchmarks` constructs `AppState` (WebSocket manager spawns Tokio tasks); the bench uses `Runtime::enter()` around `AppState::new()` so `tokio::spawn` succeeds.
- Storage and network numbers vary strongly by hardware; treat illustrative tables as non-binding.
