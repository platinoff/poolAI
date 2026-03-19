//! Topology API endpoints
//!
//! Provides endpoints for querying network topology information:
//! - Network topology overview
//! - Latency matrix between nodes
//! - Node resource information

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashMap;

use crate::core::state::ApiContext;
use crate::pool::topology::get_global_topology_manager;

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
        .route("/topology", get(topology_handler))
        .route("/topology/latency", get(latency_matrix_handler))
        .route("/topology/nodes", get(all_nodes_resources_handler))
        .route("/topology/nodes/{node_id}", get(node_resources_handler))
}

/// Handler for GET /api/v1/topology
/// Returns overview of network topology
async fn topology_handler() -> impl IntoResponse {
    if let Some(manager_arc) = get_global_topology_manager() {
        let manager = manager_arc.read().await;
        let topology = manager.get_topology_snapshot().await;

        let node_ids: Vec<String> = topology.node_resources.keys().cloned().collect();

        let response = TopologyResponse {
            node_count: topology.node_resources.len(),
            latency_measurements: topology.latency_matrix.len(),
            last_updated: topology.last_updated.to_rfc3339(),
            node_ids,
        };

        (StatusCode::OK, Json(response)).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Topology manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/topology/latency
/// Returns latency matrix between all nodes
async fn latency_matrix_handler() -> impl IntoResponse {
    if let Some(manager_arc) = get_global_topology_manager() {
        let manager = manager_arc.read().await;
        let topology = manager.get_topology_snapshot().await;

        let response = LatencyMatrixResponse {
            latency_matrix: topology.latency_matrix.clone(),
        };

        (StatusCode::OK, Json(response)).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Topology manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/topology/nodes
/// Returns resource information for all nodes
async fn all_nodes_resources_handler() -> impl IntoResponse {
    if let Some(manager_arc) = get_global_topology_manager() {
        let manager = manager_arc.read().await;
        let topology = manager.get_topology_snapshot().await;

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

        let response = AllNodesResourcesResponse { nodes };
        (StatusCode::OK, Json(response)).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Topology manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/topology/nodes/{node_id}
/// Returns resource information for a specific node
async fn node_resources_handler(Path(node_id): Path<String>) -> impl IntoResponse {
    if let Some(manager_arc) = get_global_topology_manager() {
        let manager = manager_arc.read().await;

        if let Some(resources) = manager.get_node_resources(&node_id).await {
            let response = NodeResourcesResponse {
                node_id: resources.node_id.clone(),
                available_gpu_memory_mb: resources.available_gpu_memory_mb,
                total_gpu_memory_mb: resources.total_gpu_memory_mb,
                available_cpu_cores: resources.available_cpu_cores,
                total_cpu_cores: resources.total_cpu_cores,
                available_memory_mb: resources.available_memory_mb,
                total_memory_mb: resources.total_memory_mb,
                current_load: resources.current_load,
            };

            (StatusCode::OK, Json(response)).into_response()
        } else {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": format!("Node '{}' not found", node_id)})),
            )
                .into_response()
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Topology manager not initialized"})),
        )
            .into_response()
    }
}
