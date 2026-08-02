//! Tensor sharding runtime (FM-036, EXO plan §3.1–3.2).
//!
//! Builds shard plans from [`InstancePlacement`] (tensor strategy) and provides
//! an in-process inter-worker sync bus for forward-pass coordination steps.

use crate::core::error::AppError;
use crate::runtime::instance::{InstancePlacement, PlacementStrategy};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Minimum link bandwidth (Mbps) for tensor parallelism (aligned with placement).
pub const MIN_TENSOR_BANDWIDTH_MBPS: f64 = 100.0;

/// Per-node tensor slice within a sharded model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TensorShardSpec {
    pub node_id: String,
    pub shard_index: usize,
    pub shard_count: usize,
    pub memory_mb: u64,
    /// Inclusive start of the logical parameter dimension range for this shard.
    pub dim_start: usize,
    /// Exclusive end of the logical parameter dimension range.
    pub dim_end: usize,
}

/// Plan describing how a model instance is split across workers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TensorShardPlan {
    pub plan_id: String,
    pub model_id: String,
    pub shards: Vec<TensorShardSpec>,
    pub total_memory_mb: u64,
    pub min_bandwidth_mbps: f64,
    /// EXO-oriented estimate (1.8× @ 2 devices, 3.2× @ 4+).
    pub estimated_speedup: f64,
}

/// Sync primitive between tensor-parallel workers (§3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShardSyncOp {
    AllReduce,
    Broadcast,
    Gather,
}

/// Logical sync message (payload size only — no raw tensor bytes in MVP).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShardSyncMessage {
    pub plan_id: String,
    pub from_node: String,
    pub to_node: String,
    pub op: ShardSyncOp,
    pub step: u64,
    pub payload_bytes: usize,
}

/// In-process bus for shard-step coordination (tests / single-host simulation).
#[derive(Debug, Default)]
pub struct ShardSyncBus {
    inbox: Arc<RwLock<VecDeque<ShardSyncMessage>>>,
}

impl ShardSyncBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn publish(&self, msg: ShardSyncMessage) {
        self.inbox.write().await.push_back(msg);
    }

    pub async fn drain_for_node(&self, node_id: &str) -> Vec<ShardSyncMessage> {
        let mut guard = self.inbox.write().await;
        let mut kept = VecDeque::new();
        let mut out = Vec::new();
        while let Some(msg) = guard.pop_front() {
            if msg.to_node == node_id || msg.to_node == "*" {
                out.push(msg);
            } else {
                kept.push_back(msg);
            }
        }
        *guard = kept;
        out
    }

    pub async fn len(&self) -> usize {
        self.inbox.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inbox.read().await.is_empty()
    }

    /// Simulate one forward step: all-reduce between every shard pair (both directions).
    pub async fn simulate_all_reduce_step(
        &self,
        plan: &TensorShardPlan,
        step: u64,
        payload_bytes: usize,
    ) -> usize {
        let mut count = 0usize;
        let nodes: Vec<_> = plan.shards.iter().map(|s| s.node_id.clone()).collect();
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let msg = ShardSyncMessage {
                    plan_id: plan.plan_id.clone(),
                    from_node: nodes[i].clone(),
                    to_node: nodes[j].clone(),
                    op: ShardSyncOp::AllReduce,
                    step,
                    payload_bytes,
                };
                self.publish(msg).await;
                count += 1;
            }
        }
        count
    }
}

/// Whether topology bandwidth supports tensor parallelism.
pub fn meets_tensor_bandwidth(bandwidth_mbps: Option<f64>) -> bool {
    match bandwidth_mbps {
        None => true,
        Some(bw) => bw >= MIN_TENSOR_BANDWIDTH_MBPS,
    }
}

/// Split total memory evenly across `node_ids`.
pub fn split_memory_across_nodes(
    required_memory_mb: u64,
    node_ids: &[String],
) -> HashMap<String, u64> {
    let n = node_ids.len().max(1) as u64;
    let per = required_memory_mb / n;
    let mut map = HashMap::new();
    for id in node_ids {
        map.insert(id.clone(), per);
    }
    map
}

/// EXO-oriented speedup estimate from shard count.
pub fn estimated_tensor_speedup(shard_count: usize) -> f64 {
    match shard_count {
        0 | 1 => 1.0,
        2 => 1.8,
        3 => 2.5,
        _ => 3.2,
    }
}

/// Build a shard plan from a tensor [`InstancePlacement`].
pub fn build_tensor_shard_plan(
    model_id: &str,
    placement: &InstancePlacement,
    logical_dim: usize,
) -> Result<TensorShardPlan, AppError> {
    if placement.strategy != PlacementStrategy::Tensor {
        return Err(AppError::ValidationError(format!(
            "expected tensor placement, got {:?}",
            placement.strategy
        )));
    }
    if placement.node_ids.len() < 2 {
        return Err(AppError::ValidationError(
            "tensor parallelism requires at least 2 nodes".into(),
        ));
    }
    if placement.error.is_some() {
        return Err(AppError::ValidationError(
            placement
                .error
                .clone()
                .unwrap_or_else(|| "placement marked invalid".into()),
        ));
    }

    let shard_count = placement.node_ids.len();
    let total_memory_mb: u64 = placement.memory_by_node.values().sum();
    let dim_per = logical_dim / shard_count;
    let mut shards = Vec::with_capacity(shard_count);

    for (idx, node_id) in placement.node_ids.iter().enumerate() {
        let memory_mb = placement
            .memory_by_node
            .get(node_id)
            .copied()
            .unwrap_or(total_memory_mb / shard_count as u64);
        let dim_start = idx * dim_per;
        let dim_end = if idx + 1 == shard_count {
            logical_dim
        } else {
            dim_start + dim_per
        };
        shards.push(TensorShardSpec {
            node_id: node_id.clone(),
            shard_index: idx,
            shard_count,
            memory_mb,
            dim_start,
            dim_end,
        });
    }

    Ok(TensorShardPlan {
        plan_id: format!("shard-{}", &Uuid::new_v4().to_string()[..8]),
        model_id: model_id.to_string(),
        shards,
        total_memory_mb,
        min_bandwidth_mbps: MIN_TENSOR_BANDWIDTH_MBPS,
        estimated_speedup: estimated_tensor_speedup(shard_count),
    })
}

/// Construct a tensor [`InstancePlacement`] from topology-selected nodes.
pub fn tensor_placement_from_nodes(
    node_ids: Vec<String>,
    required_memory_mb: u64,
    bandwidth_mbps: Option<f64>,
) -> Option<InstancePlacement> {
    if node_ids.len() < 2 {
        return None;
    }
    if !meets_tensor_bandwidth(bandwidth_mbps) {
        return None;
    }
    let memory_by_node = split_memory_across_nodes(required_memory_mb, &node_ids);
    Some(InstancePlacement {
        strategy: PlacementStrategy::Tensor,
        node_ids,
        memory_by_node,
        memory_delta: required_memory_mb as i64,
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn sample_tensor_placement() -> InstancePlacement {
        InstancePlacement {
            strategy: PlacementStrategy::Tensor,
            node_ids: vec!["node-a".into(), "node-b".into()],
            memory_by_node: HashMap::from([("node-a".into(), 2048), ("node-b".into(), 2048)]),
            memory_delta: 4096,
            error: None,
        }
    }

    #[test]
    fn build_plan_splits_dims() {
        let plan = build_tensor_shard_plan("m1", &sample_tensor_placement(), 1024).unwrap();
        assert_eq!(plan.shards.len(), 2);
        assert_eq!(plan.shards[0].dim_start, 0);
        assert_eq!(plan.shards[0].dim_end, 512);
        assert_eq!(plan.shards[1].dim_start, 512);
        assert_eq!(plan.shards[1].dim_end, 1024);
        assert!((plan.estimated_speedup - 1.8).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn sync_bus_drains_by_node() {
        let bus = ShardSyncBus::new();
        bus.publish(ShardSyncMessage {
            plan_id: "p1".into(),
            from_node: "a".into(),
            to_node: "b".into(),
            op: ShardSyncOp::AllReduce,
            step: 0,
            payload_bytes: 64,
        })
        .await;
        let msgs = bus.drain_for_node("b").await;
        assert_eq!(msgs.len(), 1);
        assert!(bus.drain_for_node("c").await.is_empty());
    }

    #[tokio::test]
    async fn simulate_all_reduce_produces_pair_messages() {
        let placement = sample_tensor_placement();
        let plan = build_tensor_shard_plan("m1", &placement, 100).unwrap();
        let bus = ShardSyncBus::new();
        let n = bus.simulate_all_reduce_step(&plan, 1, 128).await;
        assert_eq!(n, 2);
    }
}
