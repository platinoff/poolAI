//! Integration tests for RAID Administrative Control Plane
//!
//! Tests administrative operations:
//! - Strategy status retrieval
//! - Manual rebalancing
//! - Metrics retrieval
//! - Artifact and node statistics

use poolai::core::error::AppError;
use poolai::raid::admin::RaidAdmin;
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Helper function to create a test RAID manager
async fn create_test_raid_manager() -> Arc<RwLock<RaidManager>> {
    let temp_dir = TempDir::new().unwrap();
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = Arc::new(RwLock::new(RaidManager::new(config)));
    manager.write().await.initialize().await.unwrap();
    manager
}

#[tokio::test]
async fn test_admin_get_strategy_status() -> Result<(), AppError> {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    let status = admin.get_strategy_status().await?;
    assert_eq!(status.mode, "Local");
    assert!(status.initialized);
    assert!(status.active);
    assert!(!status.rebalancing_enabled);

    Ok(())
}

#[tokio::test]
async fn test_admin_get_burst_raid_metrics_none() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    // Local mode doesn't have BurstRAID metrics
    let metrics = admin.get_burst_raid_metrics().await;
    assert!(metrics.is_none());
}

#[tokio::test]
async fn test_admin_get_small_world_metrics_none() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    // Local mode doesn't have SmallWorld metrics
    let metrics = admin.get_small_world_metrics().await;
    assert!(metrics.is_none());
}

#[tokio::test]
async fn test_admin_get_artifact_burst_stats_none() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    let artifact_id = Uuid::new_v4();
    let stats = admin.get_artifact_burst_stats(artifact_id).await;
    assert!(stats.is_none());
}

#[tokio::test]
async fn test_admin_get_node_clustering_none() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    let coeff = admin.get_node_clustering_coefficient(1).await;
    assert!(coeff.is_none());
}

#[tokio::test]
async fn test_admin_trigger_rebalance_local_mode() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager);

    // Rebalancing not available for Local mode
    let result = admin.trigger_rebalance().await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Rebalancing not available for Local mode"));
    }
}
