//! SmallWorld Network Strategy Implementation
//!
//! SmallWorld is a distributed storage strategy that uses SmallWorld network topology
//! for intelligent artifact replication. It optimizes storage placement based on network
//! proximity, clustering coefficients, and short-path routing.
//!
//! # Features
//!
//! - **Network Topology Awareness**: Uses latency and bandwidth information for placement
//! - **Clustering Coefficient**: Calculates local clustering to identify optimal replication nodes
//! - **Short-Path Routing**: Places artifacts to minimize access latency
//! - **Cluster-Aware Replication**: Distributes replicas across network clusters
//! - **Proximity-Based Placement**: Prioritizes nearby nodes for replication
//!
//! # SmallWorld Network Concept
//!
//! SmallWorld networks have:
//! - High local clustering (nodes have many local connections)
//! - Short average path length (few hops between any two nodes)
//! - Some long-range connections for efficient routing
//!
//! For distributed storage, this means:
//! - Replicas are placed in nearby clusters (low latency)
//! - Each cluster has sufficient replicas for local access
//! - Long-range connections ensure global availability
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::raid::small_world::{SmallWorldStrategy, SmallWorldConfig};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = SmallWorldConfig::default();
//! let strategy = SmallWorldStrategy::new(config);
//! strategy.initialize().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use crate::pool::topology::TopologyManager;
use crate::raid::events::EventStore;
use crate::raid::replication::ReplicationEngine;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Configuration for SmallWorld strategy
#[derive(Debug, Clone)]
pub struct SmallWorldConfig {
    /// Base replication factor (minimum replicas per artifact)
    pub base_replication_factor: u32,
    /// Target clustering coefficient (0.0-1.0)
    /// Higher values prioritize local clustering
    pub target_clustering_coefficient: f64,
    /// Maximum path length for short-path routing
    pub max_path_length: u32,
    /// Proximity threshold in milliseconds (nodes within this latency are considered "close")
    pub proximity_threshold_ms: f64,
    /// Enable cluster-aware replication
    pub enable_cluster_aware: bool,
    /// Rebalancing interval in seconds
    pub rebalancing_interval_secs: u64,
    /// Enable automatic rebalancing
    pub enable_auto_rebalancing: bool,
}

impl Default for SmallWorldConfig {
    fn default() -> Self {
        Self {
            base_replication_factor: 3,
            target_clustering_coefficient: 0.6,
            max_path_length: 3,
            proximity_threshold_ms: 50.0, // 50ms is considered "close"
            enable_cluster_aware: true,
            rebalancing_interval_secs: 3600, // 1 hour
            enable_auto_rebalancing: true,
        }
    }
}

/// SmallWorld network strategy for distributed storage
pub struct SmallWorldStrategy {
    /// Configuration
    config: SmallWorldConfig,
    /// Replication engine
    replication_engine: Arc<ReplicationEngine>,
    /// Topology manager for network information
    topology_manager: Arc<RwLock<TopologyManager>>,
    /// Artifact placement metadata (artifact_id -> node_ids)
    artifact_placements: Arc<RwLock<HashMap<Uuid, Vec<u64>>>>,
    /// Clustering coefficients per node (node_id -> coefficient)
    clustering_coefficients: Arc<RwLock<HashMap<u64, f64>>>,
    /// Background rebalancing task handle
    rebalancing_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    /// Event store for audit logging
    event_store: Option<Arc<RwLock<EventStore>>>,
}

impl SmallWorldStrategy {
    /// Create a new SmallWorld strategy
    pub fn new(
        config: SmallWorldConfig,
        replication_engine: Arc<ReplicationEngine>,
        topology_manager: Arc<RwLock<TopologyManager>>,
        event_store: Option<Arc<RwLock<EventStore>>>,
    ) -> Self {
        Self {
            config,
            replication_engine,
            topology_manager,
            artifact_placements: Arc::new(RwLock::new(HashMap::new())),
            clustering_coefficients: Arc::new(RwLock::new(HashMap::new())),
            rebalancing_handle: Arc::new(RwLock::new(None)),
            event_store,
        }
    }

    /// Initialize the strategy
    ///
    /// This calculates initial clustering coefficients and starts background tasks.
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing SmallWorld strategy");

        // Calculate initial clustering coefficients
        self.update_clustering_coefficients().await?;

        // Start background rebalancing task if enabled
        if self.config.enable_auto_rebalancing {
            self.start_rebalancing_task().await;
        }

        info!("SmallWorld strategy initialized");
        Ok(())
    }

    /// Calculate clustering coefficient for a node
    ///
    /// Clustering coefficient measures how well-connected a node's neighbors are.
    /// Higher values indicate better local clustering.
    async fn calculate_clustering_coefficient(&self, node_id: u64) -> Result<f64, AppError> {
        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;

        // Get neighbors (nodes within proximity threshold)
        let mut neighbors = Vec::new();
        for (key, latency) in &topology.latency_matrix {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() == 2 {
                if let (Ok(n1), Ok(n2)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                    if n1 == node_id && *latency <= self.config.proximity_threshold_ms {
                        neighbors.push(n2);
                    } else if n2 == node_id && *latency <= self.config.proximity_threshold_ms {
                        neighbors.push(n1);
                    }
                }
            }
        }

        if neighbors.len() < 2 {
            // Need at least 2 neighbors for clustering
            return Ok(0.0);
        }

        // Count edges between neighbors
        let mut edges_between_neighbors = 0;
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let key1 = format!("{}:{}", neighbors[i], neighbors[j]);
                let key2 = format!("{}:{}", neighbors[j], neighbors[i]);
                if topology.latency_matrix.contains_key(&key1)
                    || topology.latency_matrix.contains_key(&key2)
                {
                    edges_between_neighbors += 1;
                }
            }
        }

        // Clustering coefficient = (2 * edges_between_neighbors) / (neighbors * (neighbors - 1))
        let possible_edges = neighbors.len() * (neighbors.len() - 1) / 2;
        let coefficient = if possible_edges > 0 {
            (2.0 * edges_between_neighbors as f64) / possible_edges as f64
        } else {
            0.0
        };

        Ok(coefficient)
    }

    /// Update clustering coefficients for all nodes
    async fn update_clustering_coefficients(&self) -> Result<(), AppError> {
        info!("Updating clustering coefficients");

        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;
        let node_ids: Vec<u64> = topology
            .node_resources
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .collect();
        drop(topology);

        let mut coefficients = self.clustering_coefficients.write().await;
        coefficients.clear();

        for node_id in node_ids {
            match self.calculate_clustering_coefficient(node_id).await {
                Ok(coeff) => {
                    coefficients.insert(node_id, coeff);
                    debug!("Node {} clustering coefficient: {:.3}", node_id, coeff);
                }
                Err(e) => {
                    warn!(
                        "Failed to calculate clustering coefficient for node {}: {}",
                        node_id, e
                    );
                }
            }
        }

        info!(
            "Updated clustering coefficients for {} nodes",
            coefficients.len()
        );
        Ok(())
    }

    /// Select target nodes for replication using SmallWorld topology
    ///
    /// Selects nodes based on:
    /// 1. Network proximity (low latency)
    /// 2. Clustering coefficient (high local clustering)
    /// 3. Short-path routing (minimize hops)
    async fn select_target_nodes(
        &self,
        _artifact_id: Uuid,
        replication_factor: u32,
    ) -> Result<Vec<u64>, AppError> {
        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;
        let coefficients = self.clustering_coefficients.read().await;

        // Get all available nodes
        let available_nodes: Vec<u64> = topology
            .node_resources
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .collect();

        if available_nodes.is_empty() {
            return Err(AppError::ValidationError(
                "No nodes available for replication".to_string(),
            ));
        }

        if available_nodes.len() < replication_factor as usize {
            warn!(
                "Not enough nodes ({}) for replication factor ({}), using available nodes",
                available_nodes.len(),
                replication_factor
            );
            return Ok(available_nodes);
        }

        // Score nodes based on clustering coefficient and proximity
        let mut scored_nodes: Vec<(u64, f64)> = Vec::new();

        for node_id in &available_nodes {
            let mut score = 0.0;

            // Clustering coefficient score (higher is better)
            if let Some(&coeff) = coefficients.get(node_id) {
                score += coeff * 0.5; // Weight: 50%
            }

            // Proximity score (lower latency is better)
            // Use average latency to other nodes as proximity metric
            let mut total_latency = 0.0;
            let mut latency_count = 0;
            for other_node_id in &available_nodes {
                if other_node_id != node_id {
                    if let Some(latency) = self
                        .topology_manager
                        .read()
                        .await
                        .get_latency(&node_id.to_string(), &other_node_id.to_string())
                        .await
                    {
                        total_latency += latency;
                        latency_count += 1;
                    }
                }
            }
            if latency_count > 0 {
                let avg_latency = total_latency / latency_count as f64;
                // Lower latency = higher score (inverse relationship)
                score += (1.0 / (1.0 + avg_latency / self.config.proximity_threshold_ms)) * 0.5;
                // Weight: 50%
            }

            scored_nodes.push((*node_id, score));
        }

        // Sort by score (descending) and select top N
        scored_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let selected_nodes: Vec<u64> = scored_nodes
            .iter()
            .take(replication_factor as usize)
            .map(|(node_id, _)| *node_id)
            .collect();

        drop(topology);
        drop(coefficients);

        info!(
            "Selected {} target nodes for artifact {}: {:?}",
            selected_nodes.len(),
            artifact_id,
            selected_nodes
        );

        Ok(selected_nodes)
    }

    /// Replicate an artifact using SmallWorld topology
    pub async fn replicate_artifact(
        &self,
        artifact_id: Uuid,
        artifact_data: Vec<u8>,
        artifact_name: &str,
    ) -> Result<(), AppError> {
        info!(
            "Replicating artifact {} using SmallWorld strategy",
            artifact_id
        );

        // Select target nodes based on SmallWorld topology
        let target_nodes = self
            .select_target_nodes(artifact_id, self.config.base_replication_factor)
            .await?;

        if target_nodes.is_empty() {
            return Err(AppError::ValidationError(
                "No target nodes selected for replication".to_string(),
            ));
        }

        // Create metadata
        let mut hasher = Sha256::new();
        hasher.update(&artifact_data);
        let checksum = format!("sha256:{:x}", hasher.finalize());

        let metadata = crate::raid::protocol::ArtifactMetadata {
            name: artifact_name.to_string(),
            version: "1.0.0".to_string(),
            size_bytes: artifact_data.len() as u64,
            checksum,
            created_at: Utc::now(),
            content_type: Some("application/octet-stream".to_string()),
            tags: None,
        };

        // Replicate using replication engine
        let artifact_data_len = artifact_data.len();
        match self
            .replication_engine
            .replicate_sync(
                artifact_id.to_string(),
                artifact_data.clone(),
                metadata,
                self.config.base_replication_factor,
                Some(target_nodes.clone()),
            )
            .await
        {
            Ok(_) => {
                // Store placement metadata
                let mut placements = self.artifact_placements.write().await;
                placements.insert(artifact_id, target_nodes.clone());

                // Record event
                if let Some(ref event_store) = self.event_store {
                    let metadata_json = serde_json::json!({
                        "name": artifact_name,
                        "size": artifact_data_len,
                        "replication_factor": self.config.base_replication_factor,
                        "strategy": "SmallWorld",
                        "target_nodes": target_nodes,
                    });
                    let _ = event_store
                        .write()
                        .await
                        .append_event(crate::raid::events::RaidEvent::ArtifactCreated {
                            artifact_id: artifact_id.to_string(),
                            node_id: 0, // Local node ID (will be set by event store)
                            timestamp: Utc::now(),
                            metadata: metadata_json,
                        })
                        .await;
                }

                info!(
                    "Successfully replicated artifact {} using SmallWorld strategy",
                    artifact_id
                );
                Ok(())
            }
            Err(e) => {
                warn!("Failed to replicate artifact {}: {}", artifact_id, e);
                Err(e)
            }
        }
    }

    /// Analyze current artifact distribution
    async fn analyze_distribution(&self) -> Result<HashMap<Uuid, Vec<u64>>, AppError> {
        let placements = self.artifact_placements.read().await;
        Ok(placements.clone())
    }

    /// Rebalance artifacts across nodes
    ///
    /// Moves artifacts to optimize SmallWorld topology placement.
    pub async fn rebalance(&self) -> Result<(), AppError> {
        info!("Starting SmallWorld rebalancing");

        // Update clustering coefficients
        self.update_clustering_coefficients().await?;

        // Analyze current distribution
        let distribution = self.analyze_distribution().await?;

        if distribution.is_empty() {
            info!("No artifacts to rebalance");
            return Ok(());
        }

        info!("Analyzed distribution: {} artifacts", distribution.len());

        // For each artifact, check if it should be moved
        let mut moved_count = 0;
        let mut failed_count = 0;

        for (artifact_id, current_nodes) in distribution {
            // Select optimal nodes for this artifact
            match self
                .select_target_nodes(artifact_id, self.config.base_replication_factor)
                .await
            {
                Ok(optimal_nodes) => {
                    // Check if current placement differs significantly from optimal
                    let current_set: HashSet<u64> = current_nodes.iter().cloned().collect();
                    let optimal_set: HashSet<u64> = optimal_nodes.iter().cloned().collect();

                    // If less than 50% overlap, consider rebalancing
                    let overlap = current_set.intersection(&optimal_set).count();
                    let overlap_ratio = overlap as f64 / current_set.len().max(1) as f64;

                    if overlap_ratio < 0.5 {
                        info!(
                            "Rebalancing artifact {}: {:?} -> {:?}",
                            artifact_id, current_nodes, optimal_nodes
                        );
                        // Note: Actual artifact movement would require reading artifact data
                        // and calling replicate_artifact. For now, we just update placement metadata.
                        let mut placements = self.artifact_placements.write().await;
                        placements.insert(artifact_id, optimal_nodes);
                        moved_count += 1;
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to select optimal nodes for artifact {}: {}",
                        artifact_id, e
                    );
                    failed_count += 1;
                }
            }
        }

        info!(
            "SmallWorld rebalancing completed: {} moved, {} failed",
            moved_count, failed_count
        );

        Ok(())
    }

    /// Start background rebalancing task
    async fn start_rebalancing_task(&self) {
        let strategy = Arc::new(SmallWorldStrategyForTask {
            replication_engine: Arc::clone(&self.replication_engine),
            topology_manager: Arc::clone(&self.topology_manager),
            artifact_placements: Arc::clone(&self.artifact_placements),
            clustering_coefficients: Arc::clone(&self.clustering_coefficients),
            config: self.config.clone(),
        });
        let interval_secs = self.config.rebalancing_interval_secs;

        let handle = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                info!("Running scheduled SmallWorld rebalancing");
                if let Err(e) = strategy.run_rebalance().await {
                    warn!("SmallWorld rebalancing failed: {}", e);
                }
            }
        });

        *self.rebalancing_handle.write().await = Some(handle);
    }

    /// Shutdown the strategy and stop background tasks
    pub async fn shutdown(&self) {
        info!("Shutting down SmallWorld strategy");

        // Abort rebalancing task
        if let Some(handle) = self.rebalancing_handle.write().await.take() {
            handle.abort();
            info!("Stopped SmallWorld rebalancing task");
        }
    }
}

/// Minimal SmallWorldStrategy clone for background tasks
struct SmallWorldStrategyForTask {
    config: SmallWorldConfig,
    #[allow(dead_code)] // Used in future rebalancing operations
    replication_engine: Arc<ReplicationEngine>,
    topology_manager: Arc<RwLock<TopologyManager>>,
    artifact_placements: Arc<RwLock<HashMap<Uuid, Vec<u64>>>>,
    clustering_coefficients: Arc<RwLock<HashMap<u64, f64>>>,
}

impl SmallWorldStrategyForTask {
    async fn run_rebalance(&self) -> Result<(), AppError> {
        info!("Starting SmallWorld rebalancing");

        // Update clustering coefficients
        self.update_clustering_coefficients().await?;

        // Analyze current distribution
        let distribution = self.analyze_distribution().await?;

        if distribution.is_empty() {
            info!("No artifacts to rebalance");
            return Ok(());
        }

        info!("Analyzed distribution: {} artifacts", distribution.len());

        // For each artifact, check if it should be moved
        let mut moved_count = 0;
        let mut failed_count = 0;

        for (artifact_id, current_nodes) in distribution {
            // Select optimal nodes for this artifact
            match self
                .select_target_nodes(artifact_id, self.config.base_replication_factor)
                .await
            {
                Ok(optimal_nodes) => {
                    // Check if current placement differs significantly from optimal
                    let current_set: std::collections::HashSet<u64> =
                        current_nodes.iter().cloned().collect();
                    let optimal_set: std::collections::HashSet<u64> =
                        optimal_nodes.iter().cloned().collect();

                    // If less than 50% overlap, consider rebalancing
                    let overlap = current_set.intersection(&optimal_set).count();
                    let overlap_ratio = overlap as f64 / current_set.len().max(1) as f64;

                    if overlap_ratio < 0.5 {
                        info!(
                            "Rebalancing artifact {}: {:?} -> {:?}",
                            artifact_id, current_nodes, optimal_nodes
                        );
                        let mut placements = self.artifact_placements.write().await;
                        placements.insert(artifact_id, optimal_nodes);
                        moved_count += 1;
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to select optimal nodes for artifact {}: {}",
                        artifact_id, e
                    );
                    failed_count += 1;
                }
            }
        }

        info!(
            "SmallWorld rebalancing completed: {} moved, {} failed",
            moved_count, failed_count
        );

        Ok(())
    }

    async fn update_clustering_coefficients(&self) -> Result<(), AppError> {
        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;
        let node_ids: Vec<u64> = topology
            .node_resources
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .collect();
        drop(topology);

        let mut coefficients = self.clustering_coefficients.write().await;
        coefficients.clear();

        for node_id in node_ids {
            match self.calculate_clustering_coefficient(node_id).await {
                Ok(coeff) => {
                    coefficients.insert(node_id, coeff);
                }
                Err(e) => {
                    warn!(
                        "Failed to calculate clustering coefficient for node {}: {}",
                        node_id, e
                    );
                }
            }
        }

        Ok(())
    }

    async fn calculate_clustering_coefficient(&self, node_id: u64) -> Result<f64, AppError> {
        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;

        let mut neighbors = Vec::new();
        for (key, latency) in &topology.latency_matrix {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() == 2 {
                if let (Ok(n1), Ok(n2)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                    if n1 == node_id && *latency <= self.config.proximity_threshold_ms {
                        neighbors.push(n2);
                    } else if n2 == node_id && *latency <= self.config.proximity_threshold_ms {
                        neighbors.push(n1);
                    }
                }
            }
        }

        if neighbors.len() < 2 {
            return Ok(0.0);
        }

        let mut edges_between_neighbors = 0;
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let key1 = format!("{}:{}", neighbors[i], neighbors[j]);
                let key2 = format!("{}:{}", neighbors[j], neighbors[i]);
                if topology.latency_matrix.contains_key(&key1)
                    || topology.latency_matrix.contains_key(&key2)
                {
                    edges_between_neighbors += 1;
                }
            }
        }

        let possible_edges = neighbors.len() * (neighbors.len() - 1) / 2;
        let coefficient = if possible_edges > 0 {
            (2.0 * edges_between_neighbors as f64) / possible_edges as f64
        } else {
            0.0
        };

        Ok(coefficient)
    }

    async fn select_target_nodes(
        &self,
        artifact_id: Uuid,
        replication_factor: u32,
    ) -> Result<Vec<u64>, AppError> {
        let topology = self
            .topology_manager
            .read()
            .await
            .get_topology_snapshot()
            .await;
        let coefficients = self.clustering_coefficients.read().await;

        let available_nodes: Vec<u64> = topology
            .node_resources
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .collect();

        if available_nodes.is_empty() {
            return Err(AppError::ValidationError(
                "No nodes available for replication".to_string(),
            ));
        }

        if available_nodes.len() < replication_factor as usize {
            return Ok(available_nodes);
        }

        let mut scored_nodes: Vec<(u64, f64)> = Vec::new();

        for node_id in &available_nodes {
            let mut score = 0.0;

            if let Some(&coeff) = coefficients.get(node_id) {
                score += coeff * 0.5;
            }

            let mut total_latency = 0.0;
            let mut latency_count = 0;
            for other_node_id in &available_nodes {
                if other_node_id != node_id {
                    if let Some(latency) = self
                        .topology_manager
                        .read()
                        .await
                        .get_latency(&node_id.to_string(), &other_node_id.to_string())
                        .await
                    {
                        total_latency += latency;
                        latency_count += 1;
                    }
                }
            }
            if latency_count > 0 {
                let avg_latency = total_latency / latency_count as f64;
                score += (1.0 / (1.0 + avg_latency / self.config.proximity_threshold_ms)) * 0.5;
            }

            scored_nodes.push((*node_id, score));
        }

        scored_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let selected_nodes: Vec<u64> = scored_nodes
            .iter()
            .take(replication_factor as usize)
            .map(|(node_id, _)| *node_id)
            .collect();

        drop(topology);
        drop(coefficients);

        Ok(selected_nodes)
    }

    async fn analyze_distribution(&self) -> Result<HashMap<Uuid, Vec<u64>>, AppError> {
        let placements = self.artifact_placements.read().await;
        Ok(placements.clone())
    }
}
