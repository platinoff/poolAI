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
| `runtime_benchmarks` | `cargo bench -j 1 --bench runtime_benchmarks` | Memory pool, LRU cache, model request structs, cache-key hashing, **local RAID `put_artifact`** |
| `turboquant_benchmarks` | `cargo bench -j 1 --bench turboquant_benchmarks --features ml` | TurboQuant pack/unpack and `dot_f32` (requires `ml`) |

Use **`-j 1`** on memory-constrained hosts (e.g. Windows linking many binaries) to reduce parallel link pressure.

### Groups in `runtime_benchmarks`

- **`memory_pool`**: `acquire_release_request`, `acquire_release_response`, `acquire_release_string`
- **`lru_cache`**: `get_hit`, `get_miss`, `put_new`, `put_existing`, `get_hit_variable_size` (cache capacities 100–5000)
- **`model_request_response`**: `create_request`, `clone_request`
- **`cache_key_generation`**: `generate_cache_key` (hash over request fields)
- **`raid_local_put`**: `put_artifact_4096` — temp-dir `RaidManager`, 4 KiB payload, unique logical names per iteration

### Group in `turboquant_benchmarks` (`--features ml`)

- **`turboquant`**: `pack_uniform_rows_64x256`, `unpack_to_rows_64x256`, `dot_f32_4096` (single Criterion group)

Record the Criterion summary lines (or archive `target/criterion/`) when publishing regression comparisons.

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

Example weekly job sketch (adapt checkout/rust-toolchain to your org):

```yaml
name: Criterion benchmarks
on:
  schedule:
    - cron: '0 6 * * 0'
  workflow_dispatch: {}

jobs:
  bench:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo bench -j 2 --bench runtime_benchmarks -- --noplot
      - run: cargo bench -j 2 --bench turboquant_benchmarks --features ml -- --noplot
```

`--noplot` avoids plotters-related work if you only need console output; upload `target/criterion/` as an artifact if you want HTML diffs.

---

## Notes

- Micro-benchmarks isolate hot paths; end-to-end latency includes JSON, auth, and I/O.
- Distributed RAID and cloud paths need environment-specific harnesses not covered by default Criterion targets.
- Storage and network numbers vary strongly by hardware; treat illustrative tables as non-binding.
