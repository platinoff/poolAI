//! Topology API endpoints
//!
//! Provides endpoints for querying network topology information:
//! - Network topology overview
//! - Latency matrix between nodes
//! - Node resource information

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::network::api::common::HttpAppError;
use crate::pool::topology_graph::{compute_topology_graph_layout, TopologyGraphLayout};
use crate::services::topology_service::{TopologyNotReady, TopologyService};

#[derive(Debug, Deserialize)]
struct TopologyGraphQuery {
    width: Option<u32>,
    height: Option<u32>,
    iterations: Option<u32>,
}

/// Topology response
#[derive(Serialize)]
struct TopologyResponse {
    /// Total number of nodes
    node_count: usize,
    /// Number of latency measurements
    latency_measurements: usize,
    /// Last update timestamp
    last_updated: String,
    /// Node IDs
    node_ids: Vec<String>,
}

/// Latency matrix response
#[derive(Serialize)]
struct LatencyMatrixResponse {
    /// Latency matrix (key format: "node_id1:node_id2", value: latency in ms)
    latency_matrix: HashMap<String, f64>,
}

/// Node resources response
#[derive(Serialize)]
struct NodeResourcesResponse {
    /// Node ID
    node_id: String,
    /// Available GPU memory (MB)
    available_gpu_memory_mb: u64,
    /// Total GPU memory (MB)
    total_gpu_memory_mb: u64,
    /// Available CPU cores
    available_cpu_cores: usize,
    /// Total CPU cores
    total_cpu_cores: usize,
    /// Available system memory (MB)
    available_memory_mb: u64,
    /// Total system memory (MB)
    total_memory_mb: u64,
    /// Current load (0.0-1.0)
    current_load: f32,
}

/// All nodes resources response
#[derive(Serialize)]
struct AllNodesResourcesResponse {
    /// Resources for each node
    nodes: HashMap<String, NodeResourcesResponse>,
}

/// Create topology routes
pub fn create_topology_routes() -> Router<ApiContext> {
    Router::new()
        .route("/topology/graph", get(topology_graph_handler))
        .route("/topology", get(topology_handler))
        .route("/topology/latency", get(latency_matrix_handler))
        .route("/topology/nodes", get(all_nodes_resources_handler))
        .route("/topology/nodes/{node_id}", get(node_resources_handler))
}

/// Handler for GET /api/v1/topology/graph (PH-S157)
async fn topology_graph_handler(
    State(ctx): State<ApiContext>,
    Query(query): Query<TopologyGraphQuery>,
) -> Result<Json<TopologyGraphLayout>, HttpAppError> {
    match TopologyService::get_snapshot(&ctx).await {
        Ok(topology) => Ok(Json(compute_topology_graph_layout(
            &topology.node_resources,
            &topology.latency_matrix,
            query.width,
            query.height,
            query.iterations,
        ))),
        Err(TopologyNotReady) => Err(HttpAppError::new(AppError::SubsystemUnavailable(
            "Topology manager not initialized".to_string(),
        ))
        .with_context(
            ErrorContext::new("topology_graph").with_resource("topology_manager", "default"),
        )),
    }
}

/// Handler for GET /api/v1/topology
/// Returns overview of network topology
async fn topology_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<TopologyResponse>, HttpAppError> {
    match TopologyService::get_snapshot(&ctx).await {
        Ok(topology) => {
            let node_ids: Vec<String> = topology.node_resources.keys().cloned().collect();
            Ok(Json(TopologyResponse {
                node_count: topology.node_resources.len(),
                latency_measurements: topology.latency_matrix.len(),
                last_updated: topology.last_updated.to_rfc3339(),
                node_ids,
            }))
        }
        Err(TopologyNotReady) => Err(HttpAppError::new(AppError::SubsystemUnavailable(
            "Topology manager not initialized".to_string(),
        ))
        .with_context(ErrorContext::new("topology").with_resource("topology_manager", "default"))),
    }
}

/// Handler for GET /api/v1/topology/latency
/// Returns latency matrix between all nodes
async fn latency_matrix_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<LatencyMatrixResponse>, HttpAppError> {
    match TopologyService::get_snapshot(&ctx).await {
        Ok(topology) => Ok(Json(LatencyMatrixResponse {
            latency_matrix: topology.latency_matrix.clone(),
        })),
        Err(TopologyNotReady) => Err(HttpAppError::new(AppError::SubsystemUnavailable(
            "Topology manager not initialized".to_string(),
        ))
        .with_context(
            ErrorContext::new("topology_latency").with_resource("topology_manager", "default"),
        )),
    }
}

/// Handler for GET /api/v1/topology/nodes
/// Returns resource information for all nodes
async fn all_nodes_resources_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<AllNodesResourcesResponse>, HttpAppError> {
    match TopologyService::get_snapshot(&ctx).await {
        Ok(topology) => {
            let mut nodes = HashMap::new();
            for (node_id, resources) in &topology.node_resources {
                nodes.insert(
                    node_id.clone(),
                    NodeResourcesResponse {
                        node_id: node_id.clone(),
                        available_gpu_memory_mb: resources.available_gpu_memory_mb,
                        total_gpu_memory_mb: resources.total_gpu_memory_mb,
                        available_cpu_cores: resources.available_cpu_cores,
                        total_cpu_cores: resources.total_cpu_cores,
                        available_memory_mb: resources.available_memory_mb,
                        total_memory_mb: resources.total_memory_mb,
                        current_load: resources.current_load,
                    },
                );
            }
            Ok(Json(AllNodesResourcesResponse { nodes }))
        }
        Err(TopologyNotReady) => Err(HttpAppError::new(AppError::SubsystemUnavailable(
            "Topology manager not initialized".to_string(),
        ))
        .with_context(
            ErrorContext::new("topology_nodes").with_resource("topology_manager", "default"),
        )),
    }
}

/// Handler for GET /api/v1/topology/nodes/{node_id}
/// Returns resource information for a specific node
async fn node_resources_handler(
    State(ctx): State<ApiContext>,
    Path(node_id): Path<String>,
) -> Result<Json<NodeResourcesResponse>, HttpAppError> {
    match TopologyService::get_node_resources(&ctx, &node_id).await {
        Ok(Some(resources)) => Ok(Json(NodeResourcesResponse {
            node_id: resources.node_id.clone(),
            available_gpu_memory_mb: resources.available_gpu_memory_mb,
            total_gpu_memory_mb: resources.total_gpu_memory_mb,
            available_cpu_cores: resources.available_cpu_cores,
            total_cpu_cores: resources.total_cpu_cores,
            available_memory_mb: resources.available_memory_mb,
            total_memory_mb: resources.total_memory_mb,
            current_load: resources.current_load,
        })),
        Ok(None) => Err(HttpAppError::new(AppError::ApiNotFound(format!(
            "Node '{}' not found",
            node_id
        )))
        .with_context(ErrorContext::new("node_resources").with_resource("node_id", &node_id))),
        Err(TopologyNotReady) => Err(HttpAppError::new(AppError::SubsystemUnavailable(
            "Topology manager not initialized".to_string(),
        ))
        .with_context(
            ErrorContext::new("node_resources").with_resource("topology_manager", "default"),
        )),
    }
}
