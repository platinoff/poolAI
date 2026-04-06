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
| `runtime_benchmarks` | `cargo bench -j 1 --bench runtime_benchmarks` | Memory pool, LRU, model structs, cache keys, local RAID put, **replication engine** (node selection + quorum), **VM lifecycle**, **RAID protocol JSON**, **health-shaped JSON** |
| `turboquant_benchmarks` | `cargo bench -j 1 --bench turboquant_benchmarks --features ml` | TurboQuant pack/unpack and `dot_f32` (requires `ml`). Workspace **`bench`** profile uses `opt-level = 0` so `cargo bench` can complete on MSVC hosts where `rustc` otherwise AVs on the full crate (see baseline note below). |
| `cloud_benchmarks` | `cargo bench -j 1 --bench cloud_benchmarks --features cloud` | `CloudConfig::validate`, manager `initialize`/`shutdown` (default config) |
| `service_layer_benchmarks` | `cargo bench -j 1 --bench service_layer_benchmarks --features test-utils` | `RaidService` list/quota/cluster_status over temp `RaidManager` |

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

Record the Criterion summary lines (or archive `target/criterion/`) when publishing regression comparisons.

### Reference machine baselines (manual)

After a full run on a **named** host (CPU model, RAM, OS, disk type), paste Criterion means (or a link to saved HTML) into the table below. Replace placeholders—**not** CI-enforced.

The **dev sample** rows used a shortened Criterion profile (`--sample-size 20`, `--warm-up-time 0.3`, `--measurement-time 0.5`) on **Windows 10**, `cargo bench` **release** profile — for regression *trends* on the same machine, not absolute SLA.

The **win11-criterion-full** row set is a default Criterion profile (100 samples × ~5 s measurement each, 3 s warmup) from `cargo bench -j 1 --bench runtime_benchmarks -- --noplot` on **Windows** (release), 2026-04-06 — use as a **P4** snapshot until a named Linux ref host is recorded.

**TurboQuant / MSVC note:** On some Windows **MSVC** hosts, `rustc` exits with **STATUS_ACCESS_VIOLATION** when compiling the full `poolai` library at typical release-style optimization for `cargo bench`. The workspace sets **`[profile.bench]`** with **`opt-level = 0`** so Criterion targets (e.g. `turboquant_benchmarks`) can link and run. The **win-msvc-turboquant-bench-opt0** rows below used the same **short Criterion profile** as dev-sample (`--sample-size 20`, `--warm-up-time 0.3`, `--measurement-time 0.5`) and are useful for **before/after trends on one machine**, not for comparing absolute speed to a Linux `-O3` server build. Prefer **GNU** (`1.92.0-x86_64-pc-windows-gnu` + toolchain in PATH per repo docs) when you need release-grade bench binaries on Windows.

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
| `turboquant/pack_uniform_rows_64x256` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~491 µs | 2026-04-06 |
| `turboquant/unpack_to_rows_64x256` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~80.5 µs | 2026-04-06 |
| `turboquant/dot_f32_4096` | win-msvc-turboquant-bench-opt0-2026-04-06 | ~10.2 µs | 2026-04-06 |
| `raid_service/list_artifacts` | dev-win-sample | ~218 ns | 2026-04-06 |
| `raid_service/quota` | dev-win-sample | ~74.5 µs | 2026-04-06 |
| `raid_service/cluster_status` | dev-win-sample | ~55.2 µs | 2026-04-06 |
| `wrk` `/api/v1/health` | *manual* | RPS / p50 / p95 | — |
| *your ref host* / … | *e.g. ref-linux-01* | *from Criterion* | *YYYY-MM-DD* |

### Target metrics (P4 roadmap, non-binding)

Use these as **internal guardrails** when changing hot paths; replace with numbers from your ref host. They are **not** enforced in CI.

| Area | Criterion group / function | Soft target (same machine, trend) |
|------|---------------------------|-----------------------------------|
| In-memory | `memory_pool/*`, `lru_cache/get_hit` | No large regression vs last saved Criterion baseline |
| Local RAID | `raid_local_put/put_artifact_4096` | Stable median order-of-magnitude on fixed disk temp dir |
| Replication (CP) | `raid_replication_engine/*` | Sub-ms median for `select_*` with ≤100 registered nodes; `calculate_quorum` nanosecond-scale |
| Service layer | `raid_service/list_*` | Sub-µs median for list/queries on tiny temp stores |
| JSON | `http_health_json`, `raid_protocol_put_payload` | Track median; investigate if >2× prior baseline |
| TurboQuant | `turboquant/*` (`--features ml`) | Stable pack/unpack; `dot_f32` scales linearly with dimension |

### Changelog (bench docs)

| Date | Note |
|------|------|
| 2026-04-06 | Filled `raid_service/quota`/`cluster_status` dev-sample medians; P4 target table; `std::hint::black_box` in benches; `ui.rs` gates `State` on `enterprise`; Criterion group `raid_replication_engine` in `runtime_benchmarks` (P2b proxy vs full multi-node I/O). |
| 2026-04-06 | P4: `runtime_benchmarks` full-profile snapshot (`win11-criterion-full-2026-04-06`); P2b: `tests/distributed_raid_wire_integration.rs` (`test-utils`, optional `ml` for TQ01 wire JSON size). |
| 2026-04-06 | P2b: TurboQuant **decode** inner loop — 4-wide unroll + tail (`push_dequantized_row`), симетрично до pack; прогін `turboquant_benchmarks` після зміни — за бажанням на реф-хості. |
| 2026-04-06 | P4: `turboquant_benchmarks` — фактичні медіани (short Criterion) у таблиці baseline під міткою **win-msvc-turboquant-bench-opt0-2026-04-06**; у кореневому `Cargo.toml` додано **`[profile.bench] opt-level = 0`** як обхід **MSVC rustc AV** на збірці `poolai` для `cargo bench`. |

---

## HTTP and system load tests (manual)

With the server running (default or configured bind address):

```bash
# wrk
wrk -t12 -c400 -d30s http://127.0.0.1:8080/api/v1/health

# Apache Bench
ab -n 100000 -c 100 http://127.0.0.1:8080/api/v1/health

# hey
hey -n 100000 -c 100 http://127.0.0.1:8080/api/v1/health
```

Interpretation depends on CPU count, TLS, auth middleware, and disk; compare against your own baseline after deploy.

On Windows, **`wrk` is often absent** from PATH; use WSL, a Linux ref host, or **`hey`** / **`ab`** for the same URL pattern.

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
