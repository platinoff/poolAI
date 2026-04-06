//! Service-layer benchmarks over attached test handles (feature `test-utils`).
//!
//! Run: `cargo bench -j 1 --bench service_layer_benchmarks --features test-utils`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poolai::core::state::{ApiContext, AppState};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use poolai::services::raid_service::RaidService;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn bench_raid_service_paths(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = Arc::new(RaidManager::new(config));
    rt.block_on(async {
        manager.initialize().await.unwrap();
    });

    // `AppState::new` builds `WebSocketManager`, which `tokio::spawn`s background tasks.
    let state: ApiContext = {
        let _guard = rt.enter();
        Arc::new(AppState::new())
    };
    state
        .attach_raid_manager_for_test(manager.clone())
        .expect("attach raid once");

    rt.block_on(async {
        RaidService::put_artifact(&state, "warmup", b"warmup")
            .await
            .unwrap();
    });

    let mut group = c.benchmark_group("raid_service");
    group.bench_function("list_artifacts", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = RaidService::list_artifacts(black_box(&state)).await;
        });
    });
    group.bench_function("quota", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = RaidService::quota(black_box(&state)).await;
        });
    });
    group.bench_function("cluster_status", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = RaidService::cluster_status(black_box(&state)).await;
        });
    });
    group.finish();

    let _ = rt.block_on(manager.shutdown());
}

criterion_group!(benches, bench_raid_service_paths);
criterion_main!(benches);
