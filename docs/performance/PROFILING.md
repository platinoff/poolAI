# Performance Profiling Guide

## Overview

This document provides guidance on profiling PoolAI to identify hot paths (CPU and memory bottlenecks) for performance optimization.

## Prerequisites

### Linux

- `perf` - Linux performance monitoring tool
- `flamegraph` - Flame graph generation tool
  ```bash
  cargo install flamegraph
  ```

### Windows

- `Windows Performance Toolkit` (WPT) - For system-level profiling
- `flamegraph` (via WSL or cross-compilation)

## Profiling Tools

### 1. Cargo Flamegraph (Recommended)

**Installation:**
```bash
cargo install flamegraph
```

**Usage:**

#### Profile the main application:
```bash
# Build with profiling symbols
cargo build --release

# Generate flamegraph
sudo flamegraph -- target/release/poolai

# Output: flamegraph.svg
```

#### Profile benchmarks:
```bash
# Profile specific benchmark
sudo flamegraph --bench --bench runtime_benchmarks

# Profile all benchmarks
sudo flamegraph --bench
```

**Output:** `flamegraph.svg` - Interactive SVG flame graph showing CPU usage by function

### 2. Perf (Linux only)

**CPU Profiling:**
```bash
# Record CPU samples
sudo perf record -F 99 --call-graph dwarf target/release/poolai

# View report
sudo perf report

# Generate text report
sudo perf report > perf_report.txt
```

**Memory Profiling:**
```bash
# Record memory allocation samples
sudo perf record -e cache-misses target/release/poolai

# View cache miss report
sudo perf report
```

### 3. Heaptrack (Memory Profiling)

**Installation (Ubuntu/Debian):**
```bash
sudo apt-get install heaptrack heaptrack-gui
```

**Usage:**
```bash
# Profile memory allocations
heaptrack target/release/poolai

# View results in GUI
heaptrack_gui heaptrack.poolai.*.gz
```

## Hot Paths to Profile

### Identified Hot Paths

Based on code analysis, the following paths are likely hot paths (frequently executed):

#### 1. Request Processing Pipeline
- **Location**: `src/pool/worker.rs::process_request()`
- **Frequency**: Called for every model request
- **Operations**:
  - Cache key generation (`generate_cache_key()`)
  - Cache lookup (`check_cache()`)
  - Request processing (`simulate_request_processing()`)
  - Metrics update (`update_metrics()`)

#### 2. Memory Pool Operations
- **Location**: `src/runtime/memory_pool.rs`
- **Frequency**: Called for every ModelRequest/Response allocation
- **Operations**:
  - `acquire_request()` / `release_request()`
  - `acquire_response()` / `release_response()`
  - `acquire_string()` / `release_string()`

#### 3. LRU Cache Operations
- **Location**: `src/runtime/cache.rs`
- **Frequency**: Called for every cache lookup/insert
- **Operations**:
  - `get()` - Cache lookup with TTL check
  - `put()` - Cache insertion with LRU eviction

#### 4. HTTP Request Handling
- **Location**: `src/network/api/`
- **Frequency**: Called for every HTTP request
- **Operations**:
  - Route matching
  - Request deserialization
  - Response serialization
  - Authentication/authorization checks

#### 5. Tokio Runtime Tasks
- **Location**: `src/main.rs` - Tokio runtime configuration
- **Frequency**: All async operations
- **Operations**:
  - Task scheduling
  - I/O polling
  - Task wake-up/context switching

## Profiling Workflow

### Step 1: Baseline Measurement

Run benchmarks to establish baseline:
```bash
cargo bench --bench runtime_benchmarks
```

Record:
- Execution time per operation
- Memory allocations
- Cache hit/miss rates

### Step 2: CPU Profiling

```bash
# Build release binary with debug symbols
RUSTFLAGS="-g" cargo build --release

# Generate flamegraph
sudo flamegraph -- target/release/poolai

# Or use perf
sudo perf record -F 99 --call-graph dwarf target/release/poolai
sudo perf report
```

**Analysis:**
- Identify functions with highest CPU usage
- Look for unexpected allocations (e.g., `Vec::new`, `String::new`)
- Check for excessive cloning (`clone()` calls)
- Identify lock contention (`RwLock`, `Mutex`)

### Step 3: Memory Profiling

```bash
# Profile memory allocations
heaptrack target/release/poolai

# Or use valgrind (Linux)
valgrind --tool=massif target/release/poolai
ms_print massif.out.* > massif_report.txt
```

**Analysis:**
- Identify memory allocation hotspots
- Check for memory leaks
- Analyze allocation frequency
- Identify large allocations

### Step 4: Cache Profiling

Enable cache statistics in code:
```rust
// In CacheManager
let stats = cache.get_stats().await;
println!("Cache hits: {}, misses: {}, hit rate: {:.2}%",
    stats.hits,
    stats.misses,
    (stats.hits as f64 / (stats.hits + stats.misses) as f64) * 100.0
);
```

### Step 5: Optimization

Based on profiling results:
1. **High CPU usage functions**: Optimize algorithms, reduce allocations
2. **Memory hotspots**: Use memory pools, reduce cloning
3. **Lock contention**: Reduce lock scope, use lock-free structures
4. **Cache misses**: Adjust cache size, TTL, or eviction policy

## Example Profiling Session

### Profile Memory Pool Operations

```bash
# 1. Build with profiling
RUSTFLAGS="-g" cargo build --release

# 2. Profile memory pool benchmarks
sudo flamegraph --bench --bench runtime_benchmarks -- memory_pool

# 3. Analyze flamegraph.svg
# - Look for `MemoryPool::acquire_*` functions
# - Check allocation overhead (e.g., `Vec::new`, `String::with_capacity`)
# - Identify lock contention (`RwLock::read`, `RwLock::write`)
```

### Profile LRU Cache Operations

```bash
# Profile cache operations
sudo flamegraph --bench --bench runtime_benchmarks -- lru_cache

# Check for:
# - Hash computation overhead (`DefaultHasher`)
# - LRU list operations (`LruCache::put`, `LruCache::get`)
# - TTL expiration checks (`DateTime` comparisons)
```

### Profile HTTP Request Processing

```bash
# Run application with realistic load
# (Use load testing tool like wrk or Apache Bench)

# Profile under load
sudo perf record -F 99 --call-graph dwarf target/release/poolai

# Generate report
sudo perf report --sort comm,pid,time,symbol > http_profiling.txt
```

## Interpreting Results

### Flamegraph Analysis

1. **Wide functions**: High cumulative time (many calls)
2. **Tall stacks**: Deep call chains (may indicate unnecessary abstraction)
3. **Hot spots**: Functions taking significant time

### Perf Report Analysis

1. **Samples**: Number of times function was sampled
2. **Percentage**: CPU time spent in function
3. **Children**: Functions called by this function

### Heaptrack Analysis

1. **Allocation sites**: Where memory is allocated
2. **Peak memory**: Maximum memory usage
3. **Leaked memory**: Memory not freed

## Performance Targets

Based on benchmarks, target performance:

### Memory Pool
- **acquire/release**: < 100ns per operation
- **Pool hit rate**: > 90% (object reused from pool)

### LRU Cache
- **get() hit**: < 200ns
- **get() miss**: < 100ns
- **put()**: < 500ns
- **Cache hit rate**: > 80% (realistic workloads)

### Request Processing
- **Cache lookup**: < 500ns
- **Request processing**: < 1ms (excluding model inference)
- **Response serialization**: < 100μs

## Continuous Profiling

### CI/CD Integration

Add profiling to CI pipeline:
```yaml
# .github/workflows/profile.yml
- name: Profile benchmarks
  run: |
    cargo install flamegraph
    sudo flamegraph --bench --bench runtime_benchmarks
    # Upload flamegraph.svg as artifact
```

### Periodic Profiling

Schedule regular profiling sessions:
- After major feature additions
- Before production deployments
- When performance degrades

## Troubleshooting

### Flamegraph generation fails

**Error**: `perf_event_open failed: Permission denied`
**Solution**: Run with `sudo` or configure `perf_event_paranoid`:
```bash
sudo sysctl -w kernel.perf_event_paranoid=1
```

### High overhead from profiling

**Symptom**: Profiled application runs significantly slower
**Solution**: 
- Reduce sampling frequency (`-F 99` → `-F 10`)
- Use `perf record` with `--no-call-graph` for lower overhead

### Missing symbols in flamegraph

**Symptom**: Functions appear as addresses instead of names
**Solution**:
- Build with debug symbols: `RUSTFLAGS="-g" cargo build --release`
- Ensure `debug` profile includes symbols in `Cargo.toml`

## References

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Flamegraph Documentation](https://github.com/flamegraph-rs/flamegraph)
- [Perf Tutorial](https://perf.wiki.kernel.org/index.php/Tutorial)
- [Heaptrack Documentation](https://milianw.de/tag/heaptrack)

---

**Last Updated**: 2026-01-16  
**Version**: 1.0 - Initial profiling guide
