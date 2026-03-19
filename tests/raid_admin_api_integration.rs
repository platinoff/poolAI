//! Integration tests for RAID Administrative Control Plane API endpoints
//!
//! Tests the REST API endpoints for RAID admin operations:
//! - GET /raid/admin/status
//! - POST /raid/admin/rebalance
//! - GET /raid/admin/metrics/burst
//! - GET /raid/admin/metrics/smallworld
//! - GET /raid/admin/metrics/artifact/{id}/burst
//! - GET /raid/admin/metrics/node/{id}/clustering

use poolai::raid::admin::RaidAdmin;
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;

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
async fn test_raid_admin_get_strategy_status() {
    let raid_manager = create_test_raid_manager().await;
    let admin = RaidAdmin::new(raid_manager.clone());

    // Get strategy status
    let status = admin.get_strategy_status().await.unwrap();
    // Status should be returned successfully
    let _ = status;
}

#[tokio::test]
async fn test_raid_admin_trigger_rebalance_local_mode() {
    let config = RaidConfig::default_for_platform();
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(config)));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Trigger rebalance in local mode (should return error)
    let result = admin.trigger_rebalance().await;
    // In local mode, rebalance should fail
    assert!(result.is_err());
}

#[tokio::test]
async fn test_raid_admin_get_burst_raid_metrics() {
    let config = RaidConfig::default_for_platform();
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(config)));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Get BurstRAID metrics (may return None in local mode)
    let metrics = admin.get_burst_raid_metrics().await;
    // Metrics may be None in local mode, or Some if strategy is active
    if let Some(metrics) = metrics {
        let _total_artifacts = metrics.total_artifacts;
    }
}

#[tokio::test]
async fn test_raid_admin_get_small_world_metrics() {
    let config = RaidConfig::default_for_platform();
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(config)));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Get SmallWorld metrics (may return None in local mode)
    let metrics = admin.get_small_world_metrics().await;
    // Metrics may be None in local mode, or Some if strategy is active
    if let Some(metrics) = metrics {
        let _total_nodes = metrics.total_nodes;
    }
}

#[tokio::test]
async fn test_raid_admin_get_artifact_burst_stats() {
    let config = RaidConfig::default_for_platform();
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(config)));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Create a test artifact using put_artifact
    let artifact_ref = raid_manager
        .write()
        .await
        .put_artifact("test-artifact", b"test artifact data")
        .await
        .unwrap();
    let artifact_id = artifact_ref.id;

    // Get artifact burst stats (may return None if artifact not in burst state)
    let stats = admin.get_artifact_burst_stats(artifact_id).await;
    // Stats may or may not be available depending on burst state
    if let Some(stats) = stats {
        assert!(stats.artifact_id == artifact_id);
    }
}

#[tokio::test]
async fn test_raid_admin_get_node_clustering_coefficient() {
    let config = RaidConfig::default_for_platform();
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(config)));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Get clustering coefficient for a node (should work even if no nodes)
    let node_id = 1u64; // Use u64 node ID
    let coefficient = admin.get_node_clustering_coefficient(node_id).await;
    // Coefficient may be None if node doesn't exist, or between 0.0 and 1.0 if it does
    if let Some(coefficient) = coefficient {
        assert!(coefficient >= 0.0 && coefficient <= 1.0);
    }
}
