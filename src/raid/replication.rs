//! Replication Engine for Distributed RAID
//!
//! This module provides replication functionality for distributing artifacts
//! across multiple nodes in the distributed RAID system.
//!
//! Features:
//! - Synchronous replication (quorum-based)
//! - Asynchronous replication (background queue)
//! - Read replica support
//! - Conflict resolution
//! - Node selection and health-aware routing

use crate::core::error::AppError;
use crate::raid::client::ProtocolClient;
use crate::raid::events::{EventStore, RaidEvent};
use crate::raid::protocol::{ArtifactMetadata, SyncMode};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{timeout, Duration as TokioDuration};
use tracing::{debug, info, warn};

/// Async replication task
#[derive(Debug, Clone)]
pub struct AsyncReplicationTask {
    /// Artifact ID
    pub artifact_id: String,
    /// Artifact data
    pub artifact_data: Vec<u8>,
    /// Artifact metadata
    pub metadata: ArtifactMetadata,
    /// Target replication factor
    pub replication_factor: u32,
    /// Target nodes (if None, nodes will be selected automatically)
    pub target_nodes: Option<Vec<u64>>,
    /// Retry attempt count
    pub retry_count: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
}

/// Replication configuration
#[derive(Debug, Clone)]
pub struct ReplicationConfig {
    /// Default replication factor (number of copies)
    pub default_replication_factor: u32,
    /// Timeout for synchronous replication (seconds)
    pub sync_timeout_seconds: u64,
    /// Number of retry attempts for async replication
    pub async_retry_attempts: u32,
    /// Delay between async retry attempts (seconds)
    pub async_retry_delay_seconds: u64,
    /// Maximum queue size for async replication
    pub async_queue_size: usize,
    /// Number of background workers for async replication
    pub async_worker_count: usize,
}

impl Default for ReplicationConfig {
    fn default() -> Self {
        Self {
            default_replication_factor: 3,
            sync_timeout_seconds: 30,
            async_retry_attempts: 3,
            async_retry_delay_seconds: 5,
            async_queue_size: 1000,
            async_worker_count: 2,
        }
    }
}

/// Replication status for an artifact
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicationStatus {
    /// Not replicated yet
    Pending,
    /// Replication in progress
    InProgress,
    /// Replicated to target nodes
    Completed,
    /// Replication failed
    Failed { reason: String },
    /// Partially replicated (some nodes failed)
    Partial { successful: u32, failed: u32 },
    /// Queued for asynchronous replication
    Queued,
}

/// Replication metadata for an artifact
#[derive(Debug, Clone)]
pub struct ReplicationMetadata {
    /// Artifact ID
    pub artifact_id: String,
    /// Current replication status
    pub status: ReplicationStatus,
    /// Target replication factor
    pub target_factor: u32,
    /// Current replication count
    pub current_count: u32,
    /// List of nodes where artifact is replicated
    pub replica_nodes: Vec<u64>,
    /// Timestamp of last replication attempt
    pub last_replication_attempt: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of successful replication
    pub replicated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Replication Engine
///
/// Coordinates artifact replication across multiple nodes in the distributed RAID system.
pub struct ReplicationEngine {
    /// RAID manager reference (will be used for local artifact operations)
    #[allow(dead_code)]
    raid_manager: Arc<RwLock<crate::raid::RaidManager>>,
    /// Event store for auditability
    event_store: Option<Arc<RwLock<EventStore>>>,
    /// Configuration
    config: ReplicationConfig,
    /// Replication metadata by artifact ID
    replication_metadata: Arc<RwLock<HashMap<String, ReplicationMetadata>>>,
    /// Available nodes for replication (node_id -> address)
    available_nodes: Arc<RwLock<HashMap<u64, String>>>,
    /// Protocol clients by node ID (lazy initialization)
    protocol_clients: Arc<RwLock<HashMap<u64, Arc<ProtocolClient>>>>,
    /// Async replication queue (sender side)
    async_queue_tx: Option<mpsc::Sender<AsyncReplicationTask>>,
    /// Async replication queue (receiver side, for worker)
    async_queue_rx: Option<Arc<RwLock<Option<mpsc::Receiver<AsyncReplicationTask>>>>>,
    /// Background worker handle
    background_worker: Option<tokio::task::JoinHandle<()>>,
}

impl ReplicationEngine {
    /// Create a new replication engine
    pub fn new(
        raid_manager: Arc<RwLock<crate::raid::RaidManager>>,
        event_store: Option<Arc<RwLock<EventStore>>>,
        config: ReplicationConfig,
    ) -> Self {
        Self {
            raid_manager,
            event_store,
            config,
            replication_metadata: Arc::new(RwLock::new(HashMap::new())),
            available_nodes: Arc::new(RwLock::new(HashMap::new())),
            protocol_clients: Arc::new(RwLock::new(HashMap::new())),
            async_queue_tx: None,
            async_queue_rx: None,
            background_worker: None,
        }
    }

    /// Create with default configuration
    pub fn with_defaults(
        raid_manager: Arc<RwLock<crate::raid::RaidManager>>,
        event_store: Option<Arc<RwLock<EventStore>>>,
    ) -> Self {
        Self::new(raid_manager, event_store, ReplicationConfig::default())
    }

    /// Register an available node for replication
    pub async fn register_node(&self, node_id: u64, address: String) {
        let address_clone = address.clone();
        let mut nodes = self.available_nodes.write().await;
        nodes.insert(node_id, address);
        info!("Registered node {} for replication at {}", node_id, address_clone);
    }

    /// Unregister a node (e.g., when it becomes unavailable)
    pub async fn unregister_node(&self, node_id: u64) {
        let mut nodes = self.available_nodes.write().await;
        nodes.remove(&node_id);
        info!("Unregistered node {} from replication", node_id);
    }

    /// Get list of available nodes
    pub async fn get_available_nodes(&self) -> Vec<(u64, String)> {
        let nodes = self.available_nodes.read().await;
        nodes.iter().map(|(id, address)| (*id, address.clone())).collect()
    }

    /// Select nodes for replication based on replication factor
    ///
    /// This is a simple round-robin selection. In production, this could be
    /// enhanced with health checks, load balancing, and geographic awareness.
    pub async fn select_replication_nodes(
        &self,
        replication_factor: u32,
        exclude_nodes: Option<Vec<u64>>,
    ) -> Result<Vec<u64>, AppError> {
        let nodes = self.available_nodes.read().await;
        
        if nodes.is_empty() {
            return Err(AppError::ConfigError(
                "No available nodes for replication".to_string(),
            ));
        }

        let mut candidate_nodes: Vec<u64> = nodes.keys().cloned().collect();

        // Exclude specified nodes
        if let Some(exclude) = exclude_nodes {
            candidate_nodes.retain(|id| !exclude.contains(id));
        }

        if candidate_nodes.is_empty() {
            return Err(AppError::ConfigError(
                "No available nodes after exclusions".to_string(),
            ));
        }

        // Simple selection: take first N nodes
        // In production, this should consider:
        // - Node health (via circuit breaker)
        // - Current load
        // - Geographic distribution
        // - Network latency
        let selected: Vec<u64> = candidate_nodes
            .into_iter()
            .take(replication_factor as usize)
            .collect();

        if selected.len() < replication_factor as usize {
            warn!(
                "Requested replication factor {} but only {} nodes available",
                replication_factor,
                selected.len()
            );
        }

        Ok(selected)
    }

    /// Get replication metadata for an artifact
    pub async fn get_replication_metadata(
        &self,
        artifact_id: &str,
    ) -> Option<ReplicationMetadata> {
        let metadata = self.replication_metadata.read().await;
        metadata.get(artifact_id).cloned()
    }

    /// Get or create protocol client for a node
    async fn get_protocol_client(&self, node_id: u64, address: &str) -> Arc<ProtocolClient> {
        let mut clients = self.protocol_clients.write().await;
        
        if let Some(client) = clients.get(&node_id) {
            return client.clone();
        }

        let client = Arc::new(ProtocolClient::new(
            address.to_string(),
            format!("node-{}", node_id),
        ));
        clients.insert(node_id, client.clone());
        client
    }

    /// Update replication metadata
    async fn update_metadata(
        &self,
        artifact_id: String,
        status: ReplicationStatus,
        replica_nodes: Vec<u64>,
    ) {
        let mut metadata_map = self.replication_metadata.write().await;
        
        let metadata = metadata_map.entry(artifact_id.clone()).or_insert_with(|| {
            ReplicationMetadata {
                artifact_id: artifact_id.clone(),
                status: ReplicationStatus::Pending,
                target_factor: self.config.default_replication_factor,
                current_count: 0,
                replica_nodes: Vec::new(),
                last_replication_attempt: None,
                replicated_at: None,
            }
        });

        metadata.status = status.clone();
        metadata.replica_nodes = replica_nodes.clone();
        metadata.current_count = replica_nodes.len() as u32;
        metadata.last_replication_attempt = Some(Utc::now());

        if matches!(status, ReplicationStatus::Completed) {
            metadata.replicated_at = Some(Utc::now());
        }

        debug!(
            "Updated replication metadata for artifact {}: status={:?}, nodes={:?}",
            artifact_id, status, replica_nodes
        );
    }

    /// Initialize replication metadata for a new artifact
    pub async fn initialize_replication(
        &self,
        artifact_id: String,
        target_factor: u32,
    ) -> Result<(), AppError> {
        let mut metadata_map = self.replication_metadata.write().await;
        
        if metadata_map.contains_key(&artifact_id) {
            return Err(AppError::ValidationError(format!(
                "Replication already initialized for artifact {}",
                artifact_id
            )));
        }

        let metadata = ReplicationMetadata {
            artifact_id: artifact_id.clone(),
            status: ReplicationStatus::Pending,
            target_factor,
            current_count: 0,
            replica_nodes: Vec::new(),
            last_replication_attempt: None,
            replicated_at: None,
        };

        metadata_map.insert(artifact_id, metadata);
        Ok(())
    }

    /// Get replication status for an artifact
    pub async fn get_replication_status(
        &self,
        artifact_id: &str,
    ) -> Result<ReplicationStatus, AppError> {
        let metadata = self.replication_metadata.read().await;
        metadata
            .get(artifact_id)
            .map(|m| m.status.clone())
            .ok_or_else(|| {
                AppError::ValidationError(format!(
                    "No replication metadata found for artifact {}",
                    artifact_id
                ))
            })
    }

    /// Check if artifact is fully replicated
    pub async fn is_fully_replicated(&self, artifact_id: &str) -> Result<bool, AppError> {
        let metadata = self.replication_metadata.read().await;
        let meta = metadata.get(artifact_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "No replication metadata found for artifact {}",
                artifact_id
            ))
        })?;

        Ok(matches!(meta.status, ReplicationStatus::Completed)
            && meta.current_count >= meta.target_factor)
    }

    /// Get configuration
    pub fn config(&self) -> &ReplicationConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: ReplicationConfig) {
        self.config = config;
    }

    /// Replicate an artifact synchronously to multiple nodes
    ///
    /// This method replicates an artifact to the specified nodes and waits for
    /// quorum confirmation before returning. Quorum is calculated as (N/2) + 1
    /// where N is the replication factor.
    ///
    /// # Arguments
    /// * `artifact_id` - ID of the artifact to replicate
    /// * `artifact_data` - The artifact data bytes
    /// * `metadata` - Artifact metadata
    /// * `replication_factor` - Target number of replicas
    /// * `target_nodes` - Optional list of specific nodes to replicate to (if None, nodes are selected automatically)
    ///
    /// # Returns
    /// * `Ok(Vec<u64>)` - List of node IDs where replication succeeded
    /// * `Err(AppError)` - Error if quorum is not reached or replication fails
    pub async fn replicate_sync(
        &self,
        artifact_id: String,
        artifact_data: Vec<u8>,
        metadata: ArtifactMetadata,
        replication_factor: u32,
        target_nodes: Option<Vec<u64>>,
    ) -> Result<Vec<u64>, AppError> {
        // Initialize replication metadata
        self.initialize_replication(artifact_id.clone(), replication_factor)
            .await?;

        // Update status to InProgress
        let selected_nodes = if let Some(nodes) = target_nodes {
            nodes
        } else {
            self.select_replication_nodes(replication_factor, None).await?
        };

        self.update_metadata(
            artifact_id.clone(),
            ReplicationStatus::InProgress,
            selected_nodes.clone(),
        )
        .await;

        info!(
            "Starting synchronous replication of artifact {} to {} nodes",
            artifact_id, replication_factor
        );

        // Emit ReplicationStarted events for each target node
        if let Some(ref event_store) = self.event_store {
            let source_node = 0; // TODO: Get actual source node ID from Raft
            for target_node in &selected_nodes {
                let _ = event_store.write().await.append_event(RaidEvent::ReplicationStarted {
                    artifact_id: artifact_id.clone(),
                    source_node,
                    target_node: *target_node,
                    timestamp: Utc::now(),
                }).await;
            }
        }

        // Calculate quorum: (N/2) + 1
        let quorum = (replication_factor / 2) + 1;
        let mut successful_nodes = Vec::new();
        let mut failed_nodes = Vec::new();

        // Create replication tasks for all target nodes
        let mut replication_tasks = Vec::new();

        for node_id in &selected_nodes {
            let node_id = *node_id;
            let nodes = self.available_nodes.read().await;
            let address = nodes.get(&node_id).ok_or_else(|| {
                AppError::ConfigError(format!("Node {} not found in available nodes", node_id))
            })?;

            let artifact_id_clone = artifact_id.clone();
            let artifact_data_clone = artifact_data.clone();
            let metadata_clone = metadata.clone();
            let client = self.get_protocol_client(node_id, address).await;

            let task = tokio::spawn(async move {
                let result = client
                    .put_artifact(
                        artifact_id_clone,
                        Some(artifact_data_clone),
                        metadata_clone,
                        replication_factor,
                        SyncMode::Sync,
                    )
                    .await;

                (node_id, result)
            });

            replication_tasks.push(task);
        }

        // Wait for all replication tasks with timeout
        let timeout_duration = TokioDuration::from_secs(self.config.sync_timeout_seconds);
        let timeout_result = timeout(timeout_duration, async {
            let mut results = Vec::new();
            for task in replication_tasks {
                if let Ok((node_id, result)) = task.await {
                    results.push((node_id, result));
                }
            }
            results
        })
        .await;

        let replication_results = match timeout_result {
            Ok(results) => results,
            Err(_) => {
                warn!(
                    "Synchronous replication timeout for artifact {} after {} seconds",
                    artifact_id, self.config.sync_timeout_seconds
                );
                return Err(AppError::NetworkError(format!(
                    "Replication timeout after {} seconds",
                    self.config.sync_timeout_seconds
                )));
            }
        };

        // Process results and emit events
        let source_node = 0; // TODO: Get actual source node ID from Raft
        for (node_id, result) in replication_results {
            match result {
                Ok(_) => {
                    successful_nodes.push(node_id);
                    debug!("Successfully replicated artifact {} to node {}", artifact_id, node_id);
                    
                    // Emit ReplicationCompleted event
                    if let Some(ref event_store) = self.event_store {
                        let _ = event_store.write().await.append_event(RaidEvent::ReplicationCompleted {
                            artifact_id: artifact_id.clone(),
                            source_node,
                            target_node: node_id,
                            timestamp: Utc::now(),
                        }).await;
                    }
                }
                Err(e) => {
                    failed_nodes.push(node_id);
                    warn!(
                        "Failed to replicate artifact {} to node {}: {}",
                        artifact_id, node_id, e
                    );
                    // Note: We don't emit ReplicationFailed events as they're not defined in RaidEvent
                    // This could be added in the future if needed
                }
            }
        }

        // Check if quorum is reached
        if successful_nodes.len() >= quorum as usize {
            // Quorum reached - replication successful
            self.update_metadata(
                artifact_id.clone(),
                ReplicationStatus::Completed,
                successful_nodes.clone(),
            )
            .await;

            info!(
                "Synchronous replication completed for artifact {}: {}/{} nodes (quorum: {})",
                artifact_id,
                successful_nodes.len(),
                replication_factor,
                quorum
            );

            Ok(successful_nodes)
        } else if successful_nodes.len() > 0 {
            // Partial success - some nodes succeeded but quorum not reached
            let status = ReplicationStatus::Partial {
                successful: successful_nodes.len() as u32,
                failed: failed_nodes.len() as u32,
            };
            self.update_metadata(artifact_id.clone(), status, successful_nodes.clone())
                .await;

            warn!(
                "Partial replication for artifact {}: {}/{} nodes (quorum: {} not reached)",
                artifact_id,
                successful_nodes.len(),
                replication_factor,
                quorum
            );

            Err(AppError::NetworkError(format!(
                "Quorum not reached: {}/{} successful (quorum: {})",
                successful_nodes.len(),
                replication_factor,
                quorum
            )))
        } else {
            // Complete failure
            self.update_metadata(
                artifact_id.clone(),
                ReplicationStatus::Failed {
                    reason: "All replication attempts failed".to_string(),
                },
                Vec::new(),
            )
            .await;

            Err(AppError::NetworkError(format!(
                "Replication failed: all {} nodes failed",
                replication_factor
            )))
        }
    }

    /// Wait for quorum confirmation
    ///
    /// This is a helper method that can be used to wait for quorum
    /// in custom replication scenarios.
    pub fn calculate_quorum(&self, replication_factor: u32) -> u32 {
        (replication_factor / 2) + 1
    }

    /// Initialize async replication queue and background workers
    ///
    /// This must be called before using `replicate_async()`.
    pub async fn initialize_async_replication(&mut self) -> Result<(), AppError> {
        if self.async_queue_tx.is_some() {
            return Err(AppError::ConfigError(
                "Async replication already initialized".to_string(),
            ));
        }

        let (tx, rx) = mpsc::channel(self.config.async_queue_size);
        self.async_queue_tx = Some(tx);
        self.async_queue_rx = Some(Arc::new(RwLock::new(Some(rx))));

        // Start background workers
        let worker_count = self.config.async_worker_count;
        let mut worker_handles = Vec::new();

        for worker_id in 0..worker_count {
            let rx_clone = self.async_queue_rx.as_ref().unwrap().clone();
            let engine_clone = self.clone_for_worker();

            let handle = tokio::spawn(async move {
                Self::async_replication_worker(worker_id, rx_clone, engine_clone).await;
            });

            worker_handles.push(handle);
        }

        // Store first worker handle (we'll need to manage all of them in production)
        if let Some(handle) = worker_handles.into_iter().next() {
            self.background_worker = Some(handle);
        }

        info!(
            "Initialized async replication with {} workers and queue size {}",
            worker_count, self.config.async_queue_size
        );

        Ok(())
    }

    /// Clone engine for background worker (without async queue to avoid circular reference)
    fn clone_for_worker(&self) -> Arc<ReplicationEngine> {
        Arc::new(ReplicationEngine {
            raid_manager: self.raid_manager.clone(),
            event_store: self.event_store.clone(),
            config: self.config.clone(),
            replication_metadata: self.replication_metadata.clone(),
            available_nodes: self.available_nodes.clone(),
            protocol_clients: self.protocol_clients.clone(),
            async_queue_tx: None,
            async_queue_rx: None,
            background_worker: None,
        })
    }

    /// Background worker for async replication
    async fn async_replication_worker(
        worker_id: usize,
        rx: Arc<RwLock<Option<mpsc::Receiver<AsyncReplicationTask>>>>,
        engine: Arc<ReplicationEngine>,
    ) {
        info!("Async replication worker {} started", worker_id);

        loop {
            let task = {
                let mut rx_guard = rx.write().await;
                if let Some(ref mut receiver) = *rx_guard {
                    match receiver.recv().await {
                        Some(task) => task,
                        None => {
                            debug!("Async replication worker {}: channel closed", worker_id);
                            break;
                        }
                    }
                } else {
                    debug!("Async replication worker {}: receiver not available", worker_id);
                    break;
                }
            };

            info!(
                "Worker {} processing async replication for artifact {} (attempt {}/{})",
                worker_id, task.artifact_id, task.retry_count + 1, task.max_retries
            );

            // Update status to InProgress
            engine
                .update_metadata(
                    task.artifact_id.clone(),
                    ReplicationStatus::InProgress,
                    task.target_nodes.clone().unwrap_or_default(),
                )
                .await;

            // Attempt replication
            let result = engine
                .replicate_sync(
                    task.artifact_id.clone(),
                    task.artifact_data.clone(),
                    task.metadata.clone(),
                    task.replication_factor,
                    task.target_nodes.clone(),
                )
                .await;

            match result {
                Ok(_) => {
                    info!(
                        "Worker {}: Async replication completed for artifact {}",
                        worker_id, task.artifact_id
                    );
                }
                Err(e) => {
                    warn!(
                        "Worker {}: Async replication failed for artifact {}: {}",
                        worker_id, task.artifact_id, e
                    );

                    // Max retries reached - mark as failed
                    if task.retry_count >= task.max_retries {
                        engine
                            .update_metadata(
                                task.artifact_id.clone(),
                                ReplicationStatus::Failed {
                                    reason: format!(
                                        "Max retries ({}) exceeded: {}",
                                        task.max_retries, e
                                    ),
                                },
                                Vec::new(),
                            )
                            .await;
                    }
                }
            }
        }

        info!("Async replication worker {} stopped", worker_id);
    }

    /// Replicate an artifact asynchronously
    ///
    /// This method queues the replication task and returns immediately.
    /// The actual replication is performed by background workers.
    pub async fn replicate_async(
        &self,
        artifact_id: String,
        artifact_data: Vec<u8>,
        metadata: ArtifactMetadata,
        replication_factor: u32,
        target_nodes: Option<Vec<u64>>,
    ) -> Result<(), AppError> {
        let tx = self.async_queue_tx.as_ref().ok_or_else(|| {
            AppError::ConfigError("Async replication not initialized. Call initialize_async_replication() first.".to_string())
        })?;

        // Initialize replication metadata
        self.initialize_replication(artifact_id.clone(), replication_factor)
            .await?;

        // Update status to Queued
        self.update_metadata(
            artifact_id.clone(),
            ReplicationStatus::Queued,
            target_nodes.clone().unwrap_or_default(),
        )
        .await;

        let task = AsyncReplicationTask {
            artifact_id: artifact_id.clone(),
            artifact_data,
            metadata,
            replication_factor,
            target_nodes,
            retry_count: 0,
            max_retries: self.config.async_retry_attempts,
        };

        tx.send(task)
            .await
            .map_err(|e| AppError::NetworkError(format!("Failed to queue replication task: {}", e)))?;

        info!("Queued async replication for artifact {}", artifact_id);
        Ok(())
    }

    /// Shutdown async replication workers
    pub async fn shutdown_async_replication(&mut self) -> Result<(), AppError> {
        // Close the sender to signal workers to stop
        self.async_queue_tx = None;

        // Wait for workers to finish
        if let Some(handle) = self.background_worker.take() {
            handle.await.map_err(|e| {
                AppError::ConfigError(format!("Error waiting for background worker: {:?}", e))
            })?;
        }

        info!("Async replication workers shut down");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raid::RaidConfig;
    use std::path::PathBuf;

    fn create_test_raid_manager() -> Arc<RwLock<crate::raid::RaidManager>> {
        let config = RaidConfig {
            mode: crate::raid::RaidMode::Local,
            base_path: PathBuf::from("./test_data/raid"),
            quota_bytes: Some(1024 * 1024 * 1024),
            retention_days: Some(30),
            gc_on_startup: false,
        };
        Arc::new(RwLock::new(crate::raid::RaidManager::new(config)))
    }

    #[tokio::test]
    async fn test_replication_engine_creation() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);
        
        assert_eq!(engine.config().default_replication_factor, 3);
        assert_eq!(engine.config().sync_timeout_seconds, 30);
    }

    #[tokio::test]
    async fn test_node_registration() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        engine.register_node(1, "http://node1:8080".to_string()).await;
        engine.register_node(2, "http://node2:8080".to_string()).await;

        let nodes = engine.get_available_nodes().await;
        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().any(|(id, _)| *id == 1));
        assert!(nodes.iter().any(|(id, _)| *id == 2));
    }

    #[tokio::test]
    async fn test_node_selection() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        // Register nodes
        for i in 1..=5 {
            engine
                .register_node(i, format!("http://node{}:8080", i))
                .await;
        }

        // Select 3 nodes
        let selected = engine.select_replication_nodes(3, None).await.unwrap();
        assert_eq!(selected.len(), 3);

        // Select with exclusions
        let selected = engine
            .select_replication_nodes(3, Some(vec![1, 2]))
            .await
            .unwrap();
        assert_eq!(selected.len(), 3);
        assert!(!selected.contains(&1));
        assert!(!selected.contains(&2));
    }

    #[tokio::test]
    async fn test_node_selection_insufficient_nodes() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        engine.register_node(1, "http://node1:8080".to_string()).await;

        // Request more nodes than available
        let result = engine.select_replication_nodes(5, None).await;
        assert!(result.is_ok()); // Should succeed but return only available nodes
        let selected = result.unwrap();
        assert_eq!(selected.len(), 1);
    }

    #[tokio::test]
    async fn test_node_selection_no_nodes() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        let result = engine.select_replication_nodes(3, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replication_metadata_initialization() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        let artifact_id = "test-artifact-123".to_string();
        engine.initialize_replication(artifact_id.clone(), 3).await.unwrap();

        let metadata = engine.get_replication_metadata(&artifact_id).await;
        assert!(metadata.is_some());
        let meta = metadata.unwrap();
        assert_eq!(meta.artifact_id, artifact_id);
        assert_eq!(meta.target_factor, 3);
        assert_eq!(meta.current_count, 0);
        assert!(matches!(meta.status, ReplicationStatus::Pending));
    }

    #[tokio::test]
    async fn test_replication_status() {
        let raid_manager = create_test_raid_manager();
        let engine = ReplicationEngine::with_defaults(raid_manager, None);

        let artifact_id = "test-artifact-123".to_string();
        engine.initialize_replication(artifact_id.clone(), 3).await.unwrap();

        let status = engine.get_replication_status(&artifact_id).await.unwrap();
        assert!(matches!(status, ReplicationStatus::Pending));

        // Update status
        engine
            .update_metadata(
                artifact_id.clone(),
                ReplicationStatus::Completed,
                vec![1, 2, 3],
            )
            .await;

        let status = engine.get_replication_status(&artifact_id).await.unwrap();
        assert!(matches!(status, ReplicationStatus::Completed));

        let is_fully_replicated = engine.is_fully_replicated(&artifact_id).await.unwrap();
        assert!(is_fully_replicated);
    }
}

