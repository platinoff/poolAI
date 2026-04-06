//! Administrative Control Plane for RAID strategies
//!
//! Provides administrative operations for managing RAID strategies:
//! - Strategy switching
//! - Configuration management
//! - Metrics and monitoring
//! - Rebalancing control
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::raid::admin::RaidAdmin;
//! use poolai::raid::{RaidManager, RaidConfig, RaidMode};
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
//! let admin = RaidAdmin::new(raid_manager);
//!
//! // Get strategy status
//! let status = admin.get_strategy_status().await?;
//! println!("Current strategy: {}", status.mode);
//!
//! // Trigger manual rebalancing
//! let result = admin.trigger_rebalance().await?;
//! println!("Rebalanced {} artifacts", result.artifacts_moved);
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use crate::raid::{RaidManager, RebalanceResult, StrategyStatus};
use std::sync::Arc;
use tracing::info;

/// Administrative control plane for RAID strategies
pub struct RaidAdmin {
    raid_manager: Arc<RaidManager>,
}

impl RaidAdmin {
    /// Create a new RAID admin instance
    ///
    /// # Arguments
    ///
    /// * `raid_manager` - The RAID manager to administer
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::admin::RaidAdmin;
    /// use poolai::raid::{RaidManager, RaidConfig};
    /// use std::sync::Arc;
    ///
    /// let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
    /// let admin = RaidAdmin::new(raid_manager);
    /// ```
    pub fn new(raid_manager: Arc<RaidManager>) -> Self {
        Self { raid_manager }
    }

    /// Get current strategy status
    ///
    /// Returns information about the active RAID strategy including:
    /// - Strategy mode (Local, BurstRaid, SmallWorld)
    /// - Initialization status
    /// - Rebalancing configuration
    /// - Last rebalance timestamp
    ///
    /// # Errors
    ///
    /// Returns `AppError` if the RAID manager is not initialized.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::raid::admin::RaidAdmin;
    /// # use poolai::raid::{RaidManager, RaidConfig};
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// # let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
    /// # let admin = RaidAdmin::new(raid_manager);
    /// let status = admin.get_strategy_status().await?;
    /// println!("Strategy: {}, Active: {}", status.mode, status.active);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_strategy_status(&self) -> Result<StrategyStatus, AppError> {
        self.raid_manager.get_strategy_status().await
    }

    /// Trigger manual rebalancing for the active strategy
    ///
    /// Manually triggers rebalancing of artifacts across nodes.
    /// This is useful for administrative control when automatic rebalancing is disabled
    /// or when immediate rebalancing is needed.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if:
    /// - RAID manager is not initialized
    /// - No strategy is active (Local mode)
    /// - Rebalancing fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::raid::admin::RaidAdmin;
    /// # use poolai::raid::{RaidManager, RaidConfig};
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// # let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
    /// # let admin = RaidAdmin::new(raid_manager);
    /// let result = admin.trigger_rebalance().await?;
    /// println!("Rebalanced {} artifacts", result.artifacts_moved);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trigger_rebalance(&self) -> Result<RebalanceResult, AppError> {
        info!("Administrative rebalance triggered");
        let result = self.raid_manager.trigger_rebalance().await?;
        info!(
            "Rebalancing completed: {} artifacts moved, success: {}",
            result.artifacts_moved, result.success
        );
        Ok(result)
    }

    /// Get BurstRAID metrics if BurstRAID strategy is active
    ///
    /// Returns detailed metrics about burst detection, replication factors, and artifact statistics.
    ///
    /// # Returns
    ///
    /// Returns `Some(BurstRaidMetrics)` if BurstRAID is active, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::raid::admin::RaidAdmin;
    /// # use poolai::raid::{RaidManager, RaidConfig};
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// # let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
    /// # let admin = RaidAdmin::new(raid_manager);
    /// if let Some(metrics) = admin.get_burst_raid_metrics().await {
    ///     println!("Artifacts in burst: {}", metrics.artifacts_in_burst);
    ///     println!("Total requests: {}", metrics.total_requests);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_burst_raid_metrics(
        &self,
    ) -> Option<crate::raid::burst_raid::BurstRaidMetrics> {
        self.raid_manager.get_burst_raid_metrics().await
    }

    /// Get SmallWorld metrics if SmallWorld strategy is active
    ///
    /// Returns detailed metrics about clustering coefficients, node statistics, and artifact distribution.
    ///
    /// # Returns
    ///
    /// Returns `Some(SmallWorldMetrics)` if SmallWorld is active, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::raid::admin::RaidAdmin;
    /// # use poolai::raid::{RaidManager, RaidConfig};
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// # let raid_manager = Arc::new(RaidManager::new(RaidConfig::default_for_platform()));
    /// # let admin = RaidAdmin::new(raid_manager);
    /// if let Some(metrics) = admin.get_small_world_metrics().await {
    ///     println!("Average clustering: {:.3}", metrics.avg_clustering_coefficient);
    ///     println!("Total nodes: {}", metrics.total_nodes);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_small_world_metrics(
        &self,
    ) -> Option<crate::raid::small_world::SmallWorldMetrics> {
        self.raid_manager.get_small_world_metrics().await
    }

    /// Get artifact burst stats for a specific artifact (BurstRAID only)
    ///
    /// Returns burst statistics for a specific artifact if BurstRAID strategy is active.
    ///
    /// # Arguments
    ///
    /// * `artifact_id` - UUID of the artifact
    ///
    /// # Returns
    ///
    /// Returns `Some(ArtifactBurstStats)` if artifact is tracked and BurstRAID is active, `None` otherwise.
    pub async fn get_artifact_burst_stats(
        &self,
        artifact_id: uuid::Uuid,
    ) -> Option<crate::raid::burst_raid::ArtifactBurstStats> {
        self.raid_manager
            .get_artifact_burst_stats(artifact_id)
            .await
    }

    /// Get clustering coefficient for a specific node (SmallWorld only)
    ///
    /// Returns the clustering coefficient for a specific node if SmallWorld strategy is active.
    ///
    /// # Arguments
    ///
    /// * `node_id` - ID of the node
    ///
    /// # Returns
    ///
    /// Returns `Some(f64)` if node exists and SmallWorld is active, `None` otherwise.
    pub async fn get_node_clustering_coefficient(&self, node_id: u64) -> Option<f64> {
        self.raid_manager
            .get_node_clustering_coefficient(node_id)
            .await
    }
}
