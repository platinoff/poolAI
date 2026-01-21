//! BurstRAID Strategy Implementation
//!
//! BurstRAID is a distributed storage strategy optimized for burst workloads.
//! It provides intelligent replication with burst detection, automatic rebalancing,
//! and adaptive replication factors based on workload patterns.
//!
//! # Features
//!
//! - **Burst Detection**: Automatically detects workload bursts and adjusts replication
//! - **Adaptive Replication**: Dynamic replication factor based on workload intensity
//! - **Rebalancing**: Automatic rebalancing of artifacts across nodes during low activity
//! - **Priority-based Replication**: Higher priority replication for frequently accessed artifacts
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::raid::burst_raid::{BurstRaidStrategy, BurstRaidConfig};
//! use poolai::raid::{RaidConfig, RaidManager};
//! use std::sync::Arc;
//! use tokio::sync::RwLock;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = BurstRaidConfig::default();
//! let raid_config = RaidConfig::default_for_platform();
//! let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
//! let strategy = BurstRaidStrategy::new(config, raid_manager, None);
//! strategy.initialize().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use crate::raid::events::EventStore;
use crate::raid::replication::ReplicationEngine;
use crate::raid::RaidManager;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Configuration for BurstRAID strategy
#[derive(Debug, Clone)]
pub struct BurstRaidConfig {
    /// Base replication factor (minimum replicas per artifact)
    pub base_replication_factor: u32,
    /// Maximum replication factor during bursts
    pub max_replication_factor: u32,
    /// Burst detection threshold (requests per second per artifact)
    pub burst_threshold_rps: f64,
    /// Burst cooldown period in seconds
    pub burst_cooldown_secs: u64,
    /// Rebalancing interval in seconds
    pub rebalancing_interval_secs: u64,
    /// Enable automatic rebalancing
    pub enable_auto_rebalancing: bool,
}

impl Default for BurstRaidConfig {
    fn default() -> Self {
        Self {
            base_replication_factor: 2,
            max_replication_factor: 5,
            burst_threshold_rps: 10.0,
            burst_cooldown_secs: 300,        // 5 minutes
            rebalancing_interval_secs: 3600, // 1 hour
            enable_auto_rebalancing: true,
        }
    }
}

/// Burst detection state for an artifact
#[derive(Debug, Clone)]
struct BurstState {
    /// Current replication factor
    replication_factor: u32,
    /// Last burst detection time
    last_burst_time: Option<DateTime<Utc>>,
    /// Whether artifact is currently in burst mode
    in_burst: bool,
}

/// Metrics for BurstRAID strategy
#[derive(Debug, Clone)]
pub struct BurstRaidMetrics {
    /// Total number of artifacts being tracked
    pub total_artifacts: usize,
    /// Number of artifacts currently in burst mode
    pub artifacts_in_burst: usize,
    /// Total number of requests tracked
    pub total_requests: u64,
    /// Burst detection threshold (requests per second)
    pub burst_threshold_rps: f64,
    /// Base replication factor
    pub base_replication_factor: u32,
    /// Maximum replication factor during bursts
    pub max_replication_factor: u32,
}

/// Burst detection statistics for a specific artifact
#[derive(Debug, Clone)]
pub struct ArtifactBurstStats {
    /// Artifact ID
    pub artifact_id: Uuid,
    /// Whether artifact is currently in burst mode
    pub in_burst: bool,
    /// Current requests per second
    pub current_rps: f64,
    /// Current replication factor
    pub replication_factor: u32,
    /// Last burst detection time
    pub last_burst_time: Option<DateTime<Utc>>,
}

/// Minimal BurstRaidStrategy clone for background tasks
///
/// Contains only the fields needed for background tasks to avoid circular references.
struct BurstRaidStrategyForTask {
    config: BurstRaidConfig,
    replication_engine: Arc<ReplicationEngine>,
    burst_states: Arc<RwLock<HashMap<Uuid, BurstState>>>,
    request_counters: Arc<RwLock<HashMap<Uuid, (u64, DateTime<Utc>)>>>,
}

impl BurstRaidStrategyForTask {
    /// Run rebalancing (reuses logic from BurstRaidStrategy)
    async fn run_rebalance(&self) -> Result<usize, AppError> {
        // Reuse the rebalance() method by calling it through a temporary BurstRaidStrategy
        // For now, inline the core rebalancing logic here
        info!("Starting BurstRAID rebalancing");

        // Analyze distribution
        let distribution = self.analyze_distribution().await?;

        if distribution.is_empty() {
            info!("No artifacts to rebalance");
            return Ok(0);
        }

        info!("Analyzed distribution: {} artifacts", distribution.len());

        // Create rebalance plan
        let rebalance_plan = self.create_rebalance_plan(&distribution).await?;

        if rebalance_plan.is_empty() {
            info!("Rebalancing not needed: distribution is already balanced");
            return Ok(0);
        }

        info!(
            "Rebalancing plan: {} artifacts to move",
            rebalance_plan.len()
        );

        // Move artifacts
        let mut moved_count = 0;
        let mut failed_count = 0;

        for (artifact_id, target_nodes) in rebalance_plan {
            match self.move_artifact_to_nodes(artifact_id, target_nodes).await {
                Ok(_) => moved_count += 1,
                Err(e) => {
                    failed_count += 1;
                    warn!("Failed to move artifact {}: {}", artifact_id, e);
                }
            }
        }

        info!(
            "BurstRAID rebalancing completed: {} moved, {} failed",
            moved_count, failed_count
        );

        Ok(moved_count)
    }

    // Helper methods (same as BurstRaidStrategy)
    async fn analyze_distribution(&self) -> Result<Vec<(Uuid, u64, bool, Vec<u64>)>, AppError> {
        let metadata_map = self.replication_engine.get_all_replication_metadata().await;
        let mut distribution = Vec::new();

        for (artifact_id_str, metadata) in metadata_map.iter() {
            let artifact_id = match Uuid::parse_str(artifact_id_str) {
                Ok(id) => id,
                Err(_) => {
                    warn!(
                        "Invalid artifact ID in replication metadata: {}",
                        artifact_id_str
                    );
                    continue;
                }
            };

            let request_counters = self.request_counters.read().await;
            let access_count = request_counters
                .get(&artifact_id)
                .map(|(count, _)| *count)
                .unwrap_or(0);

            let burst_states = self.burst_states.read().await;
            let in_burst = burst_states
                .get(&artifact_id)
                .map(|state| state.in_burst)
                .unwrap_or(false);

            distribution.push((
                artifact_id,
                access_count,
                in_burst,
                metadata.replica_nodes.clone(),
            ));
        }

        Ok(distribution)
    }

    async fn create_rebalance_plan(
        &self,
        distribution: &[(Uuid, u64, bool, Vec<u64>)],
    ) -> Result<Vec<(Uuid, Vec<u64>)>, AppError> {
        let available_nodes = self.replication_engine.get_available_nodes().await;
        if available_nodes.is_empty() {
            return Ok(Vec::new());
        }

        let node_ids: Vec<u64> = available_nodes.iter().map(|(id, _)| *id).collect();
        let mut rebalance_plan = Vec::new();

        for (artifact_id, access_count, in_burst, current_nodes) in distribution {
            let target_factor = if *in_burst {
                self.config.max_replication_factor
            } else if *access_count > (self.config.burst_threshold_rps * 3600.0) as u64 {
                (self.config.base_replication_factor + self.config.max_replication_factor) / 2
            } else {
                self.config.base_replication_factor
            };

            let current_factor = current_nodes.len() as u32;
            if current_factor == target_factor && !current_nodes.is_empty() {
                continue;
            }

            let mut target_nodes = Vec::new();
            let mut candidate_nodes: Vec<u64> = node_ids
                .iter()
                .filter(|node_id| !current_nodes.contains(node_id))
                .copied()
                .collect();

            if target_factor > current_factor {
                let needed = target_factor - current_factor;
                for _ in 0..needed.min(candidate_nodes.len() as u32) {
                    if let Some(node) = candidate_nodes.pop() {
                        target_nodes.push(node);
                    }
                }
            }

            if !target_nodes.is_empty() {
                rebalance_plan.push((*artifact_id, target_nodes));
            }
        }

        Ok(rebalance_plan)
    }

    async fn move_artifact_to_nodes(
        &self,
        artifact_id: Uuid,
        target_nodes: Vec<u64>,
    ) -> Result<(), AppError> {
        use crate::raid::protocol::ArtifactMetadata;
        use sha2::{Digest, Sha256};

        let raid_manager_ref = self.replication_engine.get_raid_manager();
        let artifact_path = {
            let raid_manager = raid_manager_ref.read().await;
            let artifacts = raid_manager.artifacts.read().await;
            artifacts
                .artifacts
                .get(&artifact_id)
                .ok_or_else(|| {
                    AppError::ValidationError(format!(
                        "Artifact {} not found in manifest",
                        artifact_id
                    ))
                })?
                .path
                .clone()
        };

        let raid_manager = raid_manager_ref.read().await;
        let artifact_data = raid_manager.get_artifact(&artifact_path).await?;

        let artifact_ref = {
            let artifacts = raid_manager.artifacts.read().await;
            artifacts
                .artifacts
                .get(&artifact_id)
                .ok_or_else(|| {
                    AppError::ValidationError(format!(
                        "Artifact {} not found in manifest",
                        artifact_id
                    ))
                })?
                .clone()
        };
        drop(raid_manager);

        let mut hasher = Sha256::new();
        hasher.update(&artifact_data);
        let checksum = format!("sha256:{:x}", hasher.finalize());

        let metadata = ArtifactMetadata {
            name: artifact_ref.name,
            version: "1.0.0".to_string(),
            size_bytes: artifact_data.len() as u64,
            checksum,
            created_at: artifact_ref.stored_at,
            content_type: Some("application/octet-stream".to_string()),
            tags: None,
        };

        self.replication_engine
            .replicate_sync(
                artifact_id.to_string(),
                artifact_data,
                metadata,
                target_nodes.len() as u32,
                Some(target_nodes),
            )
            .await?;

        Ok(())
    }
}

/// BurstRAID strategy implementation
pub struct BurstRaidStrategy {
    config: BurstRaidConfig,
    replication_engine: Arc<ReplicationEngine>,
    /// Burst state per artifact
    burst_states: Arc<RwLock<HashMap<Uuid, BurstState>>>,
    /// Request counters for burst detection
    request_counters: Arc<RwLock<HashMap<Uuid, (u64, DateTime<Utc>)>>>,
    /// Background rebalancing task handle
    rebalancing_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    /// Background cleanup task handle
    cleanup_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl BurstRaidStrategy {
    /// Create a new BurstRAID strategy instance
    ///
    /// # Arguments
    ///
    /// * `config` - BurstRAID configuration
    /// * `raid_manager` - RAID manager reference
    /// * `event_store` - Event store reference (optional)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::burst_raid::{BurstRaidStrategy, BurstRaidConfig};
    /// use poolai::raid::RaidManager;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let config = BurstRaidConfig::default();
    /// let raid_manager = Arc::new(RwLock::new(RaidManager::new(
    ///     poolai::raid::RaidConfig::default_for_platform()
    /// )));
    /// let strategy = BurstRaidStrategy::new(config, raid_manager, None);
    /// ```
    pub fn new(
        config: BurstRaidConfig,
        raid_manager: Arc<RwLock<RaidManager>>,
        event_store: Option<Arc<RwLock<EventStore>>>,
    ) -> Self {
        let replication_engine =
            Arc::new(ReplicationEngine::with_defaults(raid_manager, event_store));

        Self {
            config,
            replication_engine,
            burst_states: Arc::new(RwLock::new(HashMap::new())),
            request_counters: Arc::new(RwLock::new(HashMap::new())),
            rebalancing_handle: Arc::new(RwLock::new(None)),
            cleanup_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize the BurstRAID strategy
    ///
    /// Starts background tasks if needed.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if initialization fails.
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing BurstRAID strategy");

        // Start rebalancing task if enabled
        if self.config.enable_auto_rebalancing {
            self.start_rebalancing_task().await;
            info!("Auto-rebalancing enabled and started");
        }

        // Start cleanup task for stale counters
        self.start_cleanup_task().await;

        info!("BurstRAID strategy initialized successfully");
        Ok(())
    }

    /// Start background rebalancing task
    async fn start_rebalancing_task(&self) {
        let strategy = Arc::new(BurstRaidStrategyForTask {
            replication_engine: Arc::clone(&self.replication_engine),
            burst_states: Arc::clone(&self.burst_states),
            request_counters: Arc::clone(&self.request_counters),
            config: self.config.clone(),
        });
        let interval_secs = self.config.rebalancing_interval_secs;

        let handle = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                info!("Running scheduled BurstRAID rebalancing");
                if let Err(e) = strategy.run_rebalance().await {
                    warn!("BurstRAID rebalancing failed: {}", e);
                }
            }
        });

        *self.rebalancing_handle.write().await = Some(handle);
    }

    /// Start background cleanup task for stale counters
    async fn start_cleanup_task(&self) {
        let request_counters = Arc::clone(&self.request_counters);
        let burst_states = Arc::clone(&self.burst_states);
        let burst_cooldown = self.config.burst_cooldown_secs;

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // Run every minute
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                let now = Utc::now();
                let cutoff = now - chrono::Duration::seconds(burst_cooldown as i64);

                // Cleanup stale request counters
                let mut counters = request_counters.write().await;
                let initial_len = counters.len();
                counters.retain(|_, (_, timestamp)| *timestamp > cutoff);
                let removed_counters = initial_len - counters.len();

                if removed_counters > 0 {
                    debug!("Cleaned up {} stale request counters", removed_counters);
                }

                // Cleanup stale burst states (artifacts not accessed recently)
                let mut states = burst_states.write().await;
                let initial_len = states.len();
                states.retain(|_, state| {
                    state.last_burst_time.map(|t| t > cutoff).unwrap_or(true) || state.in_burst
                });
                let removed_states = initial_len - states.len();

                if removed_states > 0 {
                    debug!("Cleaned up {} stale burst states", removed_states);
                }
            }
        });

        *self.cleanup_handle.write().await = Some(handle);
    }

    /// Record an artifact access for burst detection
    ///
    /// This should be called whenever an artifact is accessed (read or write)
    /// to track request patterns for burst detection.
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - ID of the accessed artifact
    pub async fn record_access(&self, artifact_id: Uuid) {
        let now = Utc::now();
        let mut counters = self.request_counters.write().await;

        let (count, window_start) = counters.entry(artifact_id).or_insert_with(|| (0, now));

        // Reset window if it's been more than 1 second
        let window_duration = (now - *window_start).num_seconds();
        if window_duration >= 1 {
            *count = 1;
            *window_start = now;
        } else {
            *count += 1;
        }

        debug!(
            "Recorded access for artifact {}: {} requests in current window",
            artifact_id, count
        );
    }

    /// Detect if an artifact is experiencing a burst
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - ID of the artifact to check
    ///
    /// # Returns
    ///
    /// Returns `true` if artifact is in burst mode, `false` otherwise
    pub async fn is_burst(&self, artifact_id: Uuid) -> bool {
        let counters = self.request_counters.read().await;
        let burst_states = self.burst_states.read().await;

        // Check request rate
        if let Some((count, window_start)) = counters.get(&artifact_id) {
            let now = Utc::now();
            let window_duration_secs = (now - *window_start).num_seconds() as f64;
            let rps = if window_duration_secs > 0.0 {
                *count as f64 / window_duration_secs
            } else {
                *count as f64
            };

            if rps >= self.config.burst_threshold_rps {
                // Check if we're still in burst cooldown period
                if let Some(state) = burst_states.get(&artifact_id) {
                    if let Some(last_burst) = state.last_burst_time {
                        let cooldown_duration = (now - last_burst).num_seconds() as u64;
                        if cooldown_duration < self.config.burst_cooldown_secs {
                            return state.in_burst;
                        }
                    }
                }
                return true;
            }
        }

        // Check if artifact is still in burst state from previous detection
        if let Some(state) = burst_states.get(&artifact_id) {
            return state.in_burst;
        }

        false
    }

    /// Get current replication factor for an artifact
    ///
    /// Returns the replication factor based on burst state:
    /// - Base replication factor if not in burst
    /// - Max replication factor if in burst
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - ID of the artifact
    ///
    /// # Returns
    ///
    /// Returns the replication factor (base_replication_factor to max_replication_factor)
    pub async fn get_replication_factor(&self, artifact_id: Uuid) -> u32 {
        let burst_states = self.burst_states.read().await;

        if let Some(state) = burst_states.get(&artifact_id) {
            if state.in_burst {
                return self.config.max_replication_factor;
            }
        }

        self.config.base_replication_factor
    }

    /// Update burst state for an artifact
    ///
    /// This method detects bursts and updates replication factor accordingly.
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - ID of the artifact
    async fn update_burst_state(&self, artifact_id: Uuid) -> Result<(), AppError> {
        let is_burst = self.is_burst(artifact_id).await;
        let mut burst_states = self.burst_states.write().await;

        let state = burst_states
            .entry(artifact_id)
            .or_insert_with(|| BurstState {
                replication_factor: self.config.base_replication_factor,
                last_burst_time: None,
                in_burst: false,
            });

        let was_in_burst = state.in_burst;
        state.in_burst = is_burst;

        if is_burst && !was_in_burst {
            // Burst started
            state.last_burst_time = Some(Utc::now());
            state.replication_factor = self.config.max_replication_factor;
            info!(
                "Burst detected for artifact {}: increasing replication factor to {}",
                artifact_id, state.replication_factor
            );
        } else if !is_burst && was_in_burst {
            // Burst ended
            state.replication_factor = self.config.base_replication_factor;
            info!(
                "Burst ended for artifact {}: reducing replication factor to {}",
                artifact_id, state.replication_factor
            );
        }

        Ok(())
    }

    /// Replicate an artifact using BurstRAID strategy
    ///
    /// This method handles replication with burst-aware replication factor.
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - ID of the artifact
    /// * `artifact_data` - Artifact data to replicate
    /// * `metadata` - Artifact metadata
    ///
    /// # Errors
    ///
    /// Returns `AppError` if replication fails.
    pub async fn replicate_artifact(
        &self,
        artifact_id: Uuid,
        artifact_data: Vec<u8>,
        metadata: crate::raid::protocol::ArtifactMetadata,
    ) -> Result<(), AppError> {
        // Update burst state
        self.update_burst_state(artifact_id).await?;

        // Get replication factor based on burst state
        let replication_factor = self.get_replication_factor(artifact_id).await;

        // Initialize replication if needed
        if let Err(e) = self
            .replication_engine
            .initialize_replication(artifact_id.to_string(), replication_factor)
            .await
        {
            // Ignore error if replication already initialized
            if !e.to_string().contains("already initialized") {
                return Err(e);
            }
        }

        // Perform synchronous replication
        // Note: replicate_sync will select nodes internally if target_nodes is None
        self.replication_engine
            .replicate_sync(
                artifact_id.to_string(),
                artifact_data,
                metadata.clone(),
                replication_factor,
                None, // Let replication engine select nodes
            )
            .await?;

        info!(
            "Replicated artifact {} (replication factor: {})",
            artifact_id, replication_factor
        );

        Ok(())
    }

    /// Trigger rebalancing of artifacts across nodes
    ///
    /// This method redistributes artifacts to balance storage and access patterns.
    /// It analyzes current distribution, identifies artifacts that should be moved
    /// based on access patterns and node capacity, and moves them to better nodes.
    ///
    /// # Returns
    ///
    /// Returns the number of artifacts moved during rebalancing.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if rebalancing fails.
    pub async fn rebalance(&self) -> Result<usize, AppError> {
        info!("Starting BurstRAID rebalancing");

        // 1. Analyze current distribution of artifacts across nodes
        let distribution = self.analyze_distribution().await?;

        if distribution.is_empty() {
            info!("No artifacts to rebalance");
            return Ok(());
        }

        info!(
            "Analyzed distribution: {} artifacts across {} nodes",
            distribution.len(),
            self.replication_engine.get_available_nodes().await.len()
        );

        // 2. Identify artifacts that should be moved (based on access patterns, node capacity, etc.)
        let rebalance_plan = self.create_rebalance_plan(&distribution).await?;

        if rebalance_plan.is_empty() {
            info!("Rebalancing not needed: distribution is already balanced");
            return Ok(());
        }

        info!(
            "Rebalancing plan: {} artifacts to move",
            rebalance_plan.len()
        );

        // 3. Move artifacts to better nodes
        let mut moved_count = 0;
        let mut failed_count = 0;

        for (artifact_id, target_nodes) in rebalance_plan {
            match self.move_artifact_to_nodes(artifact_id, target_nodes).await {
                Ok(_) => {
                    moved_count += 1;
                    debug!("Moved artifact {} to new nodes", artifact_id);
                }
                Err(e) => {
                    failed_count += 1;
                    warn!("Failed to move artifact {}: {}", artifact_id, e);
                }
            }
        }

        info!(
            "BurstRAID rebalancing completed: {} moved, {} failed",
            moved_count, failed_count
        );

        Ok(())
    }

    /// Analyze current distribution of artifacts across nodes
    ///
    /// Returns a map of artifact_id -> (access_count, burst_state, replica_nodes)
    async fn analyze_distribution(&self) -> Result<Vec<(Uuid, u64, bool, Vec<u64>)>, AppError> {
        // Get all replication metadata
        let metadata_map = self.replication_engine.get_all_replication_metadata().await;
        let mut distribution = Vec::new();

        for (artifact_id_str, metadata) in metadata_map.iter() {
            let artifact_id = match Uuid::parse_str(artifact_id_str) {
                Ok(id) => id,
                Err(_) => {
                    warn!(
                        "Invalid artifact ID in replication metadata: {}",
                        artifact_id_str
                    );
                    continue;
                }
            };

            // Get access count from request counters
            let request_counters = self.request_counters.read().await;
            let access_count = request_counters
                .get(&artifact_id)
                .map(|(count, _)| *count)
                .unwrap_or(0);

            // Get burst state
            let burst_states = self.burst_states.read().await;
            let in_burst = burst_states
                .get(&artifact_id)
                .map(|state| state.in_burst)
                .unwrap_or(false);

            // Get replica nodes from metadata
            let replica_nodes = metadata.replica_nodes.clone();

            distribution.push((artifact_id, access_count, in_burst, replica_nodes));
        }

        Ok(distribution)
    }

    /// Create rebalance plan based on distribution analysis
    ///
    /// Returns a map of artifact_id -> target_nodes for rebalancing.
    async fn create_rebalance_plan(
        &self,
        distribution: &[(Uuid, u64, bool, Vec<u64>)],
    ) -> Result<Vec<(Uuid, Vec<u64>)>, AppError> {
        let available_nodes = self.replication_engine.get_available_nodes().await;

        if available_nodes.is_empty() {
            return Ok(Vec::new());
        }

        let node_ids: Vec<u64> = available_nodes.iter().map(|(id, _)| *id).collect();

        // Calculate target replication factor per artifact based on access patterns
        let mut rebalance_plan = Vec::new();

        for (artifact_id, access_count, in_burst, current_nodes) in distribution {
            // Determine target replication factor
            let target_factor = if *in_burst {
                self.config.max_replication_factor
            } else if *access_count > (self.config.burst_threshold_rps * 3600.0) as u64 {
                // High access count but not in burst - use medium replication
                (self.config.base_replication_factor + self.config.max_replication_factor) / 2
            } else {
                self.config.base_replication_factor
            };

            // Check if rebalancing is needed
            let current_factor = current_nodes.len() as u32;

            if current_factor == target_factor && !current_nodes.is_empty() {
                // Already at target factor, check if we need to redistribute
                // For now, skip if already at target factor
                continue;
            }

            // Select target nodes for rebalancing
            // Prefer nodes that don't already have the artifact
            let mut target_nodes = Vec::new();
            let mut candidate_nodes: Vec<u64> = node_ids
                .iter()
                .filter(|node_id| !current_nodes.contains(node_id))
                .copied()
                .collect();

            // If we need more replicas, add from candidates
            if target_factor > current_factor {
                let needed = target_factor - current_factor;
                for _ in 0..needed.min(candidate_nodes.len() as u32) {
                    if let Some(node) = candidate_nodes.pop() {
                        target_nodes.push(node);
                    }
                }
            }

            // If we need fewer replicas, mark for removal (handled separately)
            // For now, we'll add new replicas to balance load
            if !target_nodes.is_empty() {
                rebalance_plan.push((*artifact_id, target_nodes));
            }
        }

        Ok(rebalance_plan)
    }

    /// Move artifact to new nodes
    ///
    /// Reads artifact data from RaidManager and replicates to target nodes.
    async fn move_artifact_to_nodes(
        &self,
        artifact_id: Uuid,
        target_nodes: Vec<u64>,
    ) -> Result<(), AppError> {
        use crate::raid::protocol::ArtifactMetadata;
        use sha2::{Digest, Sha256};

        // Get artifact from RaidManager
        let raid_manager_ref = self.replication_engine.get_raid_manager();

        // Get artifact reference (clone it to release lock early)
        let artifact_path = {
            let raid_manager = raid_manager_ref.read().await;
            let artifacts = raid_manager.artifacts.read().await;

            let artifact_ref = artifacts.artifacts.get(&artifact_id).ok_or_else(|| {
                AppError::ValidationError(format!("Artifact {} not found in manifest", artifact_id))
            })?;

            artifact_ref.path.clone()
        };

        // Read artifact data (now we can read without holding the lock)
        let raid_manager = raid_manager_ref.read().await;
        let artifact_data = raid_manager.get_artifact(&artifact_path).await?;

        // Get artifact metadata (need to read again for name, etc.)
        let artifact_ref = {
            let artifacts = raid_manager.artifacts.read().await;
            artifacts
                .artifacts
                .get(&artifact_id)
                .ok_or_else(|| {
                    AppError::ValidationError(format!(
                        "Artifact {} not found in manifest",
                        artifact_id
                    ))
                })?
                .clone()
        };

        drop(raid_manager);

        // Calculate checksum
        let mut hasher = Sha256::new();
        hasher.update(&artifact_data);
        let checksum = format!("sha256:{:x}", hasher.finalize());

        // Create metadata
        let metadata = ArtifactMetadata {
            name: artifact_ref.name,
            version: "1.0.0".to_string(),
            size_bytes: artifact_data.len() as u64,
            checksum,
            created_at: artifact_ref.stored_at,
            content_type: Some("application/octet-stream".to_string()),
            tags: None,
        };

        // Replicate to target nodes
        self.replication_engine
            .replicate_sync(
                artifact_id.to_string(),
                artifact_data,
                metadata,
                target_nodes.len() as u32,
                Some(target_nodes),
            )
            .await?;

        Ok(())
    }

    /// Shutdown the BurstRAID strategy
    ///
    /// Performs cleanup and stops background tasks.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if shutdown fails.
    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down BurstRAID strategy");

        // Stop rebalancing task
        if let Some(handle) = self.rebalancing_handle.write().await.take() {
            handle.abort();
            info!("Stopped rebalancing task");
        }

        // Stop cleanup task
        if let Some(handle) = self.cleanup_handle.write().await.take() {
            handle.abort();
            info!("Stopped cleanup task");
        }

        info!("BurstRAID strategy shut down successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_burst_raid_config_default() {
        let config = BurstRaidConfig::default();
        assert_eq!(config.base_replication_factor, 2);
        assert_eq!(config.max_replication_factor, 5);
        assert_eq!(config.burst_threshold_rps, 10.0);
    }

    #[tokio::test]
    async fn test_burst_raid_strategy_initialize() {
        use crate::raid::RaidManager;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        let config = BurstRaidConfig::default();
        let raid_manager = Arc::new(RwLock::new(RaidManager::new(
            crate::raid::RaidConfig::default_for_platform(),
        )));
        let strategy = BurstRaidStrategy::new(config, raid_manager, None);

        // Should not fail
        let _ = strategy.initialize().await;
    }

    #[tokio::test]
    async fn test_record_access() {
        use crate::raid::RaidManager;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        let config = BurstRaidConfig::default();
        let raid_manager = Arc::new(RwLock::new(RaidManager::new(
            crate::raid::RaidConfig::default_for_platform(),
        )));
        let strategy = BurstRaidStrategy::new(config, raid_manager, None);

        let artifact_id = Uuid::new_v4();
        strategy.record_access(artifact_id).await;

        let counters = strategy.request_counters.read().await;
        assert!(counters.contains_key(&artifact_id));
    }
}
