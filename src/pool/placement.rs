//! Topology-aware placement strategies
//!
//! This module provides placement calculators that consider network topology,
//! resource availability, and latency when placing model instances.

use crate::core::error::AppError;
use crate::core::model_interface::ModelInfo;
use crate::pool::topology::get_global_topology_manager;
use crate::runtime::instance::{InstancePlacement, PlacementCalculator, PlacementStrategy};
use crate::runtime::sharding::tensor_placement_from_nodes;
use std::collections::HashMap;

/// Topology-aware placement calculator
///
/// Considers network topology, latency, bandwidth, and resource availability
/// when calculating placement options for model instances.
pub struct TopologyAwarePlacementCalculator;

#[async_trait::async_trait]
impl PlacementCalculator for TopologyAwarePlacementCalculator {
    async fn calculate_placements(
        &self,
        _model_id: &str,
        model_info: &ModelInfo,
    ) -> Result<Vec<InstancePlacement>, AppError> {
        let required_memory = model_info.gpu_requirements.recommended_memory_mb;

        // Get topology manager
        let topology_manager = get_global_topology_manager()
            .ok_or_else(|| AppError::ConfigError("Topology manager not initialized".to_string()))?;

        let topology = topology_manager.read().await;

        // Calculate placements
        let mut placements = Vec::new();

        // Strategy 1: Single node placement (best resource match)
        if let Some(single_placement) =
            calculate_single_placement(&topology, model_info, required_memory).await
        {
            placements.push(single_placement);
        }

        // Strategy 2: Pipeline parallelism (if model is large enough)
        if required_memory > 4000 {
            if let Some(pipeline_placement) =
                calculate_pipeline_placement(&topology, model_info, required_memory).await
            {
                placements.push(pipeline_placement);
            }
        }

        // Strategy 3: Tensor parallelism (if supported by nodes)
        if let Some(tensor_placement) =
            calculate_tensor_placement(&topology, model_info, required_memory).await
        {
            placements.push(tensor_placement);
        }

        // Sort placements by score (lower latency, better resource utilization)
        placements.sort_by(|a, b| {
            let score_a = placement_score(a);
            let score_b = placement_score(b);
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(placements)
    }
}

/// Calculate single node placement
async fn calculate_single_placement(
    topology: &crate::pool::topology::TopologyManager,
    _model_info: &ModelInfo,
    required_memory: u64,
) -> Option<InstancePlacement> {
    // Find best single node
    let candidates = topology
        .find_best_nodes(
            required_memory,
            1, // Minimum 1 CPU core
            1, // Single node
        )
        .await;

    if candidates.is_empty() {
        return None;
    }

    let node_id = candidates[0].clone();
    let _resources = topology.get_node_resources(&node_id).await?;

    Some(InstancePlacement {
        strategy: PlacementStrategy::Single,
        node_ids: vec![node_id.clone()],
        memory_by_node: {
            let mut map = HashMap::new();
            map.insert(node_id, required_memory);
            map
        },
        memory_delta: required_memory as i64,
        error: None,
    })
}

/// Calculate pipeline parallelism placement
async fn calculate_pipeline_placement(
    topology: &crate::pool::topology::TopologyManager,
    _model_info: &ModelInfo,
    required_memory: u64,
) -> Option<InstancePlacement> {
    // Split model across multiple nodes in pipeline
    // Each node handles a stage of the pipeline
    let memory_per_node = required_memory / 2; // Split between 2 nodes

    let candidates = topology.find_best_nodes(memory_per_node, 1, 2).await;

    if candidates.len() < 2 {
        return None;
    }

    // Check latency between nodes (pipeline requires low latency)
    let avg_latency = topology.get_average_latency(&candidates).await;
    if let Some(latency) = avg_latency {
        if latency > 50.0 {
            // Latency too high for pipeline parallelism
            return None;
        }
    }

    let mut memory_by_node = HashMap::new();
    for node_id in &candidates {
        memory_by_node.insert(node_id.clone(), memory_per_node);
    }

    Some(InstancePlacement {
        strategy: PlacementStrategy::Pipeline,
        node_ids: candidates,
        memory_by_node,
        memory_delta: required_memory as i64,
        error: None,
    })
}

/// Calculate tensor parallelism placement (FM-036 sharding runtime).
async fn calculate_tensor_placement(
    topology: &crate::pool::topology::TopologyManager,
    _model_info: &ModelInfo,
    required_memory: u64,
) -> Option<InstancePlacement> {
    let memory_per_node = required_memory / 2;
    let candidates = topology.find_best_nodes(memory_per_node, 1, 2).await;
    if candidates.len() < 2 {
        return None;
    }

    let node1 = &candidates[0];
    let node2 = &candidates[1];
    let bandwidth = topology.get_bandwidth(node1, node2).await;
    tensor_placement_from_nodes(candidates, required_memory, bandwidth)
}

/// Calculate placement score (lower is better)
fn placement_score(placement: &InstancePlacement) -> f64 {
    // Score based on number of nodes (fewer is better for simplicity)
    // and memory delta (lower is better if negative)
    let node_count_penalty = placement.node_ids.len() as f64 * 10.0;
    let memory_penalty = placement.memory_delta as f64 * 0.01;

    node_count_penalty + memory_penalty
}
