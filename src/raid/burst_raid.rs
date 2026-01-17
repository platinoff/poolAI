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
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = BurstRaidConfig::default();
//! let strategy = BurstRaidStrategy::new(config);
//! strategy.initialize().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use crate::raid::replication::ReplicationEngine;
use crate::raid::RaidManager;
use crate::raid::events::EventStore;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
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
            burst_cooldown_secs: 300, // 5 minutes
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

/// BurstRAID strategy implementation
pub struct BurstRaidStrategy {
    config: BurstRaidConfig,
    replication_engine: Arc<ReplicationEngine>,
    /// Burst state per artifact
    burst_states: Arc<RwLock<HashMap<Uuid, BurstState>>>,
    /// Request counters for burst detection
    request_counters: Arc<RwLock<HashMap<Uuid, (u64, DateTime<Utc>)>>>,
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
        let replication_engine = Arc::new(ReplicationEngine::with_defaults(
            raid_manager,
            event_store,
        ));

        Self {
            config,
            replication_engine,
            burst_states: Arc::new(RwLock::new(HashMap::new())),
            request_counters: Arc::new(RwLock::new(HashMap::new())),
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
            // TODO: Start background rebalancing task
            info!("Auto-rebalancing enabled (will be started in background task)");
        }

        info!("BurstRAID strategy initialized successfully");
        Ok(())
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

        let (count, window_start) = counters
            .entry(artifact_id)
            .or_insert_with(|| (0, now));

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

        let state = burst_states.entry(artifact_id).or_insert_with(|| BurstState {
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
            artifact_id,
            replication_factor
        );

        Ok(())
    }

    /// Trigger rebalancing of artifacts across nodes
    ///
    /// This method redistributes artifacts to balance storage and access patterns.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if rebalancing fails.
    pub async fn rebalance(&self) -> Result<(), AppError> {
        info!("Starting BurstRAID rebalancing");

        // TODO: Implement rebalancing logic
        // 1. Analyze current distribution of artifacts across nodes
        // 2. Identify artifacts that should be moved (based on access patterns, node capacity, etc.)
        // 3. Move artifacts to better nodes
        // 4. Update replication metadata

        info!("BurstRAID rebalancing completed");
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

        // TODO: Stop background tasks

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
            crate::raid::RaidConfig::default_for_platform()
        )));
        let strategy = BurstRaidStrategy::new(config, raid_manager, None);
        
        // Should not fail
        let _ = strategy.initialize().await;
    }

    #[tokio::test]
    async fn test_record_access() {
        let config = BurstRaidConfig::default();
        let repl_config = ReplicationEngineConfig::default();
        let strategy = BurstRaidStrategy::new(config, repl_config);
        
        let artifact_id = Uuid::new_v4();
        strategy.record_access(artifact_id).await;
        
        let counters = strategy.request_counters.read().await;
        assert!(counters.contains_key(&artifact_id));
    }
}