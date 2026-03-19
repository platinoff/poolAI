//! Cross-strategy integration tests for RAID
//!
//! Tests:
//! - Switching between strategies
//! - Strategy initialization and shutdown
//! - Metrics comparison between strategies

use poolai::pool::topology::initialize_global_topology_manager;
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_strategy_switching() {
    let temp_dir = TempDir::new().unwrap();

    // Start with Local mode
    let config_local = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_local = RaidManager::new(config_local);
    manager_local.initialize().await.unwrap();

    // Create artifact in Local mode
    let artifact1 = manager_local
        .put_artifact("artifact-local", b"local data")
        .await
        .unwrap();

    // Verify artifact exists
    let data1 = manager_local.get_artifact(&artifact1.path).await.unwrap();
    assert_eq!(data1, b"local data");

    // Switch to BurstRaid mode (new manager instance)
    let config_burst = RaidConfig {
        mode: RaidMode::BurstRaid,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_burst = RaidManager::new(config_burst);
    manager_burst.initialize().await.unwrap();

    // Create artifact in BurstRaid mode
    let artifact2 = manager_burst
        .put_artifact("artifact-burst", b"burst data")
        .await
        .unwrap();

    // Verify artifact exists
    let data2 = manager_burst.get_artifact(&artifact2.path).await.unwrap();
    assert_eq!(data2, b"burst data");

    // Verify both artifacts are accessible
    let artifacts_local = manager_local.list_artifacts().await;
    let artifacts_burst = manager_burst.list_artifacts().await;

    assert!(
        artifacts_local.len() >= 1,
        "Local manager should have artifacts"
    );
    assert!(
        artifacts_burst.len() >= 1,
        "BurstRaid manager should have artifacts"
    );
}

#[tokio::test]
async fn test_strategy_status() {
    let temp_dir = TempDir::new().unwrap();

    // Test Local mode status
    let config_local = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_local = RaidManager::new(config_local);
    manager_local.initialize().await.unwrap();

    let status_local = manager_local.get_strategy_status().await.unwrap();
    assert_eq!(status_local.mode, "Local");
    assert!(status_local.initialized);
    assert!(status_local.active);
    assert!(!status_local.rebalancing_enabled);

    // Test BurstRaid mode status (after creating artifact)
    let config_burst = RaidConfig {
        mode: RaidMode::BurstRaid,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_burst = RaidManager::new(config_burst);
    manager_burst.initialize().await.unwrap();

    // Create artifact to initialize strategy
    manager_burst.put_artifact("test", b"data").await.unwrap();

    let status_burst = manager_burst.get_strategy_status().await.unwrap();
    assert_eq!(status_burst.mode, "BurstRaid");
    assert!(status_burst.initialized);
    assert!(status_burst.active);
}

#[tokio::test]
async fn test_rebalance_across_strategies() {
    // Initialize topology manager (required for SmallWorld strategy)
    let _ = initialize_global_topology_manager();

    let temp_dir = TempDir::new().unwrap();

    // Test rebalancing in BurstRaid mode
    let config_burst = RaidConfig {
        mode: RaidMode::BurstRaid,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_burst = RaidManager::new(config_burst);
    manager_burst.initialize().await.unwrap();

    // Create artifacts
    for i in 0..3 {
        manager_burst
            .put_artifact(&format!("artifact-{}", i), b"test data")
            .await
            .unwrap();
    }

    // Rebalance should succeed even if no nodes are available (will return 0 moved)
    // This tests that rebalance() handles empty distribution gracefully
    let rebalance_result = manager_burst.trigger_rebalance().await.unwrap();
    assert!(rebalance_result.success, "Rebalancing should succeed");
    // artifacts_moved is an unsigned count; just acknowledge it without absurd comparisons.
    let _artifacts_moved_burst = rebalance_result.artifacts_moved;

    // Test rebalancing in SmallWorld mode
    let config_smallworld = RaidConfig {
        mode: RaidMode::SmallWorld,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager_smallworld = RaidManager::new(config_smallworld);
    manager_smallworld.initialize().await.unwrap();

    // Create artifacts
    for i in 0..3 {
        manager_smallworld
            .put_artifact(&format!("artifact-{}", i), b"test data")
            .await
            .unwrap();
    }

    // Rebalance should succeed even if no nodes are available (will return 0 moved)
    // This tests that rebalance() handles empty distribution gracefully
    let rebalance_result = manager_smallworld.trigger_rebalance().await.unwrap();
    assert!(rebalance_result.success, "Rebalancing should succeed");
    // artifacts_moved is an unsigned count; just acknowledge it without absurd comparisons.
    let _artifacts_moved_smallworld = rebalance_result.artifacts_moved;

    // Cleanup: shutdown managers to stop background tasks
    manager_burst.shutdown().await.unwrap();
    manager_smallworld.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_local_mode_no_rebalance() {
    let temp_dir = TempDir::new().unwrap();

    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager = RaidManager::new(config);
    manager.initialize().await.unwrap();

    // Rebalancing should not be available for Local mode
    let result = manager.trigger_rebalance().await;
    assert!(result.is_err(), "Rebalancing should fail for Local mode");
    if let Err(e) = result {
        assert!(
            e.to_string().contains("not available"),
            "Error should mention rebalancing not available"
        );
    }
}

#[tokio::test]
async fn test_strategy_shutdown() {
    let temp_dir = TempDir::new().unwrap();

    let config = RaidConfig {
        mode: RaidMode::BurstRaid,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let manager = RaidManager::new(config);
    manager.initialize().await.unwrap();

    // Create artifact to initialize strategy
    manager.put_artifact("test", b"data").await.unwrap();

    // Shutdown should succeed
    let result = manager.shutdown().await;
    assert!(result.is_ok(), "Shutdown should succeed");
}
