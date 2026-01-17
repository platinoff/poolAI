//! Benchmark tests for critical runtime paths
//!
//! This module contains benchmarks for:
//! - Memory pool operations (acquire/release)
//! - LRU cache operations (get/put)
//! - ModelRequest/Response processing

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use poolai::core::model_interface::{ModelParameters, ModelRequest};
use poolai::runtime::{CacheManager, MemoryPool};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::runtime::Runtime;

/// Benchmark memory pool acquire/release operations
fn bench_memory_pool_acquire_release(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let pool = MemoryPool::new();
    rt.block_on(async {
        pool.initialize().await.unwrap();
    });

    let mut group = c.benchmark_group("memory_pool");
    group.bench_function("acquire_release_request", |b| {
        b.to_async(&rt).iter(|| async {
            let request = pool.acquire_request().await;
            pool.release_request(request).await;
        });
    });
    group.bench_function("acquire_release_response", |b| {
        b.to_async(&rt).iter(|| async {
            let response = pool.acquire_response().await;
            pool.release_response(response).await;
        });
    });
    group.bench_function("acquire_release_string", |b| {
        b.to_async(&rt).iter(|| async {
            let s = pool.acquire_string().await;
            pool.release_string(s).await;
        });
    });
    group.finish();

    rt.block_on(async {
        pool.shutdown().await.unwrap();
    });
}

/// Benchmark LRU cache get/put operations
fn bench_lru_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut cache = CacheManager::new(1000);
    rt.block_on(async {
        cache.initialize().await.unwrap();
    });

    let mut group = c.benchmark_group("lru_cache");

    // Warm up cache
    rt.block_on(async {
        for i in 0..100 {
            cache
                .put(format!("key{}", i), format!("value{}", i), None)
                .await;
        }
    });

    group.bench_function("get_hit", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = cache.get(black_box("key50")).await;
        });
    });

    group.bench_function("get_miss", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = cache.get(black_box("nonexistent_key")).await;
        });
    });

    group.bench_function("put_new", |b| {
        let counter = AtomicU64::new(0);
        b.to_async(&rt).iter(|| async {
            let val = counter.fetch_add(1, Ordering::Relaxed);
            cache
                .put(
                    black_box(format!("new_key_{}", val)),
                    black_box("new_value"),
                    None,
                )
                .await;
        });
    });

    group.bench_function("put_existing", |b| {
        b.to_async(&rt).iter(|| async {
            cache
                .put(black_box("key50"), black_box("updated_value"), None)
                .await;
        });
    });

    // Benchmark cache with different sizes
    for size in [100, 500, 1000, 5000].iter() {
        let mut cache_size = CacheManager::new(*size);
        let key_to_test = format!("key{}", size / 20);
        rt.block_on(async {
            cache_size.initialize().await.unwrap();
            // Pre-fill to 80% capacity
            for i in 0..(*size * 8 / 10) {
                cache_size
                    .put(format!("key{}", i), format!("value{}", i), None)
                    .await;
            }
        });

        group.bench_with_input(
            BenchmarkId::new("get_hit_variable_size", size),
            size,
            |b, _| {
                let cache = &cache_size;
                b.to_async(&rt).iter(|| async {
                    let _ = cache.get(black_box(&key_to_test)).await;
                });
            },
        );

        rt.block_on(async {
            cache_size.shutdown().await.unwrap();
        });
    }

    group.finish();

    rt.block_on(async {
        cache.shutdown().await.unwrap();
    });
}

/// Benchmark ModelRequest/Response creation and cloning
fn bench_model_request_response(c: &mut Criterion) {
    let mut group = c.benchmark_group("model_request_response");

    group.bench_function("create_request", |b| {
        b.iter(|| {
            black_box(ModelRequest {
                input: "Test input text for benchmarking".to_string(),
                parameters: ModelParameters::default(),
                session_id: Some("test-session-id".to_string()),
                priority: 5,
                timeout: Some(30),
            });
        });
    });

    let request = ModelRequest {
        input: "Test input text for benchmarking".to_string(),
        parameters: ModelParameters::default(),
        session_id: Some("test-session-id".to_string()),
        priority: 5,
        timeout: Some(30),
    };

    group.bench_function("clone_request", |b| {
        b.iter(|| {
            black_box(request.clone());
        });
    });

    group.finish();
}

/// Benchmark cache key generation (hash-based)
fn bench_cache_key_generation(c: &mut Criterion) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut group = c.benchmark_group("cache_key_generation");

    let request = ModelRequest {
        input: "Test input text for benchmarking performance of cache key generation".to_string(),
        parameters: ModelParameters {
            temperature: 0.7,
            max_tokens: 100,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: vec![],
            seed: Some(42),
        },
        session_id: Some("test-session-id".to_string()),
        priority: 5,
        timeout: Some(30),
    };

    group.bench_function("generate_cache_key", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            request.input.hash(&mut hasher);
            request.parameters.temperature.to_bits().hash(&mut hasher);
            request.parameters.max_tokens.hash(&mut hasher);
            request.parameters.top_p.to_bits().hash(&mut hasher);
            black_box(format!("{:x}", hasher.finish()));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_memory_pool_acquire_release,
    bench_lru_cache_operations,
    bench_model_request_response,
    bench_cache_key_generation
);
criterion_main!(benches);
