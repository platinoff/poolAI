//! FM-036 tensor sharding benchmarks (plan build + sync bus).

use criterion::{criterion_group, criterion_main, Criterion};
use poolai::runtime::instance::{InstancePlacement, PlacementStrategy};
use poolai::runtime::sharding::{build_tensor_shard_plan, ShardSyncBus};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn sample_placement() -> InstancePlacement {
    InstancePlacement {
        strategy: PlacementStrategy::Tensor,
        node_ids: (0..4).map(|i| format!("node-{i}")).collect(),
        memory_by_node: (0..4).map(|i| (format!("node-{i}"), 2048_u64)).collect(),
        memory_delta: 8192,
        error: None,
    }
}

fn bench_build_shard_plan(c: &mut Criterion) {
    let placement = sample_placement();
    c.bench_function("tensor_shard_plan_build_4_nodes", |b| {
        b.iter(|| {
            let plan = build_tensor_shard_plan("bench-model", black_box(&placement), 4096).unwrap();
            black_box(plan.shards.len());
        });
    });
}

fn bench_sync_all_reduce(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let placement = sample_placement();
    let plan = build_tensor_shard_plan("bench-model", &placement, 4096).unwrap();
    let mut group = c.benchmark_group("shard_sync_bus");
    group.bench_function("all_reduce_step_4_nodes", |b| {
        b.to_async(&rt).iter(|| async {
            let bus = ShardSyncBus::new();
            let n = bus
                .simulate_all_reduce_step(black_box(&plan), 0, 65536)
                .await;
            black_box(n);
        });
    });
    group.finish();
}

criterion_group!(
    sharding_benches,
    bench_build_shard_plan,
    bench_sync_all_reduce
);
criterion_main!(sharding_benches);
