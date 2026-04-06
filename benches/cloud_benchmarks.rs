//! Cloud configuration and manager micro-benchmarks (feature `cloud`).
//!
//! Run: `cargo bench -j 1 --bench cloud_benchmarks --features cloud`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poolai::cloud::{CloudConfig, CloudManager};
use tokio::runtime::Runtime;

fn bench_cloud_config_validate(c: &mut Criterion) {
    let config = CloudConfig::default();
    let mut group = c.benchmark_group("cloud_config");
    group.bench_function("validate_default", |b| {
        b.iter(|| black_box(&config).validate().unwrap());
    });
    group.finish();
}

fn bench_cloud_manager_lifecycle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("cloud_manager");
    group.bench_function("init_shutdown_default_config", |b| {
        b.iter(|| {
            rt.block_on(async {
                let m = CloudManager::new(CloudConfig::default());
                m.initialize().await.unwrap();
                m.shutdown().await.unwrap();
            });
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_cloud_config_validate,
    bench_cloud_manager_lifecycle
);
criterion_main!(benches);
