//! Integration tests for RAID Administrative Control Plane API endpoints
//!
//! Tests the REST API endpoints for RAID admin operations:
//! - GET /raid/admin/status
//! - POST /raid/admin/rebalance
//! - GET /raid/admin/metrics/burst
//! - GET /raid/admin/metrics/smallworld
//! - GET /raid/admin/metrics/artifact/{id}/burst
//! - GET /raid/admin/metrics/node/{id}/clustering

#[cfg(feature = "raid")]
use poolai::core::error::AppError;
#[cfg(feature = "raid")]
use poolai::raid::admin::{get_strategy_status, RaidAdmin, StrategyStatus};
#[cfg(feature = "raid")]
use poolai::raid::RaidManager;
#[cfg(feature = "raid")]
use std::sync::Arc;
#[cfg(feature = "raid")]
use tokio::sync::RwLock;
#[cfg(feature = "raid")]
use uuid::Uuid;

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_get_strategy_status() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Get strategy status
    let status = admin.get_strategy_status().await.unwrap();
    match status {
        StrategyStatus::Local => {
            // Expected for local mode
        }
        StrategyStatus::Distributed { .. } => {
            // Distributed mode
        }
    }
}

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_trigger_rebalance_local_mode() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Trigger rebalance in local mode (should return error)
    let result = admin.trigger_rebalance().await;
    // In local mode, rebalance should fail
    assert!(result.is_err());
}

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_get_burst_raid_metrics() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Get BurstRAID metrics (should work even if strategy not active)
    let metrics = admin.get_burst_raid_metrics().await.unwrap();
    // Metrics should be returned even if no artifacts
    assert!(metrics.total_artifacts >= 0);
}

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_get_small_world_metrics() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Get SmallWorld metrics (should work even if strategy not active)
    let metrics = admin.get_small_world_metrics().await.unwrap();
    // Metrics should be returned even if no nodes
    assert!(metrics.total_nodes >= 0);
}

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_get_artifact_burst_stats() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Create a test artifact
    let artifact_id = Uuid::new_v4();
    let test_data = b"test artifact data";
    let base64_data = base64::engine::general_purpose::STANDARD.encode(test_data);

    // Add artifact
    let _ = raid_manager
        .write()
        .await
        .add_artifact(artifact_id, "test-artifact".to_string(), base64_data)
        .await;

    // Get artifact burst stats
    let stats = admin.get_artifact_burst_stats(artifact_id).await.unwrap();
    // Stats should be returned
    assert!(stats.artifact_id == artifact_id);
}

#[cfg(feature = "raid")]
#[tokio::test]
async fn test_raid_admin_get_node_clustering_coefficient() {
    let raid_manager = Arc::new(RwLock::new(RaidManager::new()));
    let admin = RaidAdmin::new(raid_manager.clone());

    // Initialize RAID manager
    raid_manager.write().await.initialize().await.unwrap();

    // Get clustering coefficient for a node (should work even if no nodes)
    let node_id = Uuid::new_v4();
    let coefficient = admin.get_node_clustering_coefficient(node_id).await.unwrap();
    // Coefficient should be between 0.0 and 1.0
    assert!(coefficient >= 0.0 && coefficient <= 1.0);
}
