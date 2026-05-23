//! FM-036 tensor sharding runtime integration tests.

use poolai::runtime::instance::{InstancePlacement, PlacementStrategy};
use poolai::runtime::sharding::{
    build_tensor_shard_plan, estimated_tensor_speedup, meets_tensor_bandwidth,
    tensor_placement_from_nodes, ShardSyncBus, ShardSyncOp, MIN_TENSOR_BANDWIDTH_MBPS,
};
use std::collections::HashMap;

fn tensor_placement_two_nodes() -> InstancePlacement {
    InstancePlacement {
        strategy: PlacementStrategy::Tensor,
        node_ids: vec!["n1".into(), "n2".into()],
        memory_by_node: HashMap::from([("n1".into(), 3000), ("n2".into(), 3000)]),
        memory_delta: 6000,
        error: None,
    }
}

#[test]
fn tensor_speedup_exo_targets() {
    assert!((estimated_tensor_speedup(2) - 1.8).abs() < f64::EPSILON);
    assert!((estimated_tensor_speedup(4) - 3.2).abs() < f64::EPSILON);
}

#[test]
fn bandwidth_gate_matches_placement_constant() {
    assert!(meets_tensor_bandwidth(Some(MIN_TENSOR_BANDWIDTH_MBPS)));
    assert!(!meets_tensor_bandwidth(Some(50.0)));
    assert!(meets_tensor_bandwidth(None));
}

#[test]
fn build_plan_rejects_single_node() {
    let bad = InstancePlacement {
        strategy: PlacementStrategy::Tensor,
        node_ids: vec!["solo".into()],
        memory_by_node: HashMap::from([("solo".into(), 8000)]),
        memory_delta: 8000,
        error: None,
    };
    assert!(build_tensor_shard_plan("model-x", &bad, 512).is_err());
}

#[test]
fn placement_helper_requires_two_nodes_and_bandwidth() {
    assert!(tensor_placement_from_nodes(vec!["a".into()], 4000, Some(1000.0)).is_none());
    assert!(tensor_placement_from_nodes(vec!["a".into(), "b".into()], 4000, Some(10.0)).is_none());
    let ok = tensor_placement_from_nodes(vec!["a".into(), "b".into()], 4000, Some(200.0))
        .expect("placement");
    assert_eq!(ok.strategy, PlacementStrategy::Tensor);
    assert_eq!(ok.node_ids.len(), 2);
}

#[test]
fn shard_plan_covers_full_logical_dim() {
    let placement = tensor_placement_two_nodes();
    let plan = build_tensor_shard_plan("llm-7b", &placement, 1000).unwrap();
    assert_eq!(plan.shards.len(), 2);
    let covered: usize = plan.shards.iter().map(|s| s.dim_end - s.dim_start).sum();
    assert_eq!(covered, 1000);
    assert!(plan.estimated_speedup >= 1.5);
}

#[tokio::test]
async fn sync_bus_all_reduce_step_count() {
    let placement = tensor_placement_two_nodes();
    let plan = build_tensor_shard_plan("m", &placement, 256).unwrap();
    let bus = ShardSyncBus::new();
    let sent = bus.simulate_all_reduce_step(&plan, 0, 4096).await;
    assert_eq!(sent, 2);
    let for_n1 = bus.drain_for_node("n1").await;
    assert_eq!(for_n1.len(), 1);
    assert_eq!(for_n1[0].op, ShardSyncOp::AllReduce);
}
