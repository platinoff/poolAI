//! Benchmark tests for critical runtime paths
//!
//! This module contains benchmarks for:
//! - Memory pool operations (acquire/release)
//! - LRU cache operations (get/put)
//! - ModelRequest/Response processing
//! - VM lifecycle (in-process manager)
//! - RAID protocol JSON (distributed put payload)
//! - API-shaped health JSON serialization

use chrono::{TimeZone, Utc};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use poolai::core::model_interface::{ModelParameters, ModelRequest};
use poolai::raid::events::EventStore;
use poolai::raid::protocol::{ArtifactMetadata, PutArtifactPayload, SyncMode};
use poolai::raid::replication::ReplicationEngine;
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use poolai::runtime::{CacheManager, MemoryPool};
use poolai::vm::{VmIsolation, VmManager, VmResources};
use serde_json::json;
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

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

/// Local RAID: small artifact write (async disk path + manifest update).
fn bench_raid_local_put(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = RaidManager::new(config);
    rt.block_on(async {
        manager.initialize().await.unwrap();
    });
    let payload = vec![0u8; 4096];
    let mut group = c.benchmark_group("raid_local_put");
    group.bench_function("put_artifact_4096", |b| {
        let counter = AtomicU64::new(0);
        b.to_async(&rt).iter(|| async {
            let i = counter.fetch_add(1, Ordering::Relaxed);
            let name = format!("bench_{}", i);
            let _ = manager
                .put_artifact(black_box(name.as_str()), black_box(payload.as_slice()))
                .await;
        });
    });
    group.finish();
    let _ = rt.block_on(manager.shutdown());
}

/// VM: create → start → stop → delete (in-memory / health-monitor path).
fn bench_vm_lifecycle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("vm_lifecycle");
    group.bench_function("create_start_stop_delete", |b| {
        let counter = AtomicU64::new(0);
        b.to_async(&rt).iter(|| async {
            let manager = VmManager::new();
            manager.initialize().await.unwrap();
            let i = counter.fetch_add(1, Ordering::Relaxed);
            let inst = manager
                .create_instance(
                    format!("bench_vm_{}", i),
                    VmResources::default(),
                    VmIsolation::ProcessSandbox,
                )
                .await
                .unwrap();
            let id = inst.id;
            manager.start_instance(id).await.unwrap();
            manager.stop_instance(id).await.unwrap();
            manager.delete_instance(id).await.unwrap();
            manager.shutdown().await.unwrap();
        });
    });
    group.finish();
}

/// Replication control-plane hot paths (in-process; no real peer I/O).
fn bench_raid_replication_engine(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: dir.path().join("raid"),
        quota_bytes: Some(1024 * 1024 * 1024),
        retention_days: Some(30),
        gc_on_startup: false,
    };
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    rt.block_on(async {
        raid_manager.write().await.initialize().await.unwrap();
    });
    let event_store = Arc::new(RwLock::new(EventStore::new(dir.path().join("events"))));
    rt.block_on(async {
        event_store.write().await.initialize().await.unwrap();
    });
    let engine = ReplicationEngine::with_defaults(raid_manager, Some(event_store));
    rt.block_on(async {
        for i in 1..=64_u64 {
            engine
                .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
                .await;
        }
    });

    let mut group = c.benchmark_group("raid_replication_engine");
    group.bench_function("select_replication_nodes_factor_3", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = engine
                .select_replication_nodes(black_box(3), black_box(None))
                .await;
        });
    });
    group.bench_function("calculate_quorum_rf_7", |b| {
        b.iter(|| {
            black_box(engine.calculate_quorum(black_box(7)));
        });
    });
    group.finish();
}

/// Distributed RAID wire payload: serde round-trip (no sockets).
fn bench_raid_protocol_put_payload_serde(c: &mut Criterion) {
    let created_at = Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap();
    let metadata = ArtifactMetadata {
        name: "bench-artifact".to_string(),
        version: "1.0.0".to_string(),
        size_bytes: 4096,
        checksum: "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
            .to_string(),
        created_at,
        content_type: Some("application/octet-stream".to_string()),
        tags: None,
    };
    let payload = PutArtifactPayload {
        artifact_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        source_node: "node-bench-1".to_string(),
        data: Some("a".repeat(1024)),
        metadata,
        replication_factor: 3,
        sync_mode: SyncMode::Sync,
    };
    let mut group = c.benchmark_group("raid_protocol_put_payload");
    group.bench_function("serde_json_roundtrip", |b| {
        b.iter(|| {
            let v = serde_json::to_value(black_box(&payload)).unwrap();
            let s = serde_json::to_string(black_box(&v)).unwrap();
            let p: PutArtifactPayload = serde_json::from_str(black_box(&s)).unwrap();
            black_box(p);
        });
    });
    group.finish();
}

/// Shape similar to `GET /api/v1/health` JSON body (serialization only).
fn bench_api_health_json_serialize(c: &mut Criterion) {
    let health = json!({
        "status": "healthy",
        "timestamp": "2026-04-01T12:00:00Z",
        "version": "0.2.2",
        "uptime": 3600_u64,
        "checks": {
            "database": { "status": "healthy", "message": "OK", "response_time_ms": 5 },
            "memory": { "status": "healthy", "message": "45%", "response_time_ms": 2 },
            "workers": { "status": "healthy", "message": "8/8", "response_time_ms": 3 },
            "gpu": { "status": "healthy", "message": "65C", "response_time_ms": 8 }
        }
    });
    let mut group = c.benchmark_group("http_health_json");
    group.bench_function("serde_json_to_vec", |b| {
        b.iter(|| {
            let v = serde_json::to_vec(black_box(&health)).unwrap();
            black_box(v);
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_memory_pool_acquire_release,
    bench_lru_cache_operations,
    bench_model_request_response,
    bench_cache_key_generation,
    bench_raid_local_put,
    bench_raid_replication_engine,
    bench_vm_lifecycle,
    bench_raid_protocol_put_payload_serde,
    bench_api_health_json_serialize
);
criterion_main!(benches);
