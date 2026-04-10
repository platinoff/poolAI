//! Integration tests for SmallWorld Network Strategy
//!
//! Tests the SmallWorld distributed storage strategy including:
//! - Network topology-based replication
//! - Clustering coefficient calculation
//! - Short-path routing for artifact placement
//! - Automatic rebalancing

use poolai::pool::topology::{initialize_global_topology_manager, TopologyManager};
use poolai::raid::replication::ReplicationEngine;
use poolai::raid::small_world::{SmallWorldConfig, SmallWorldStrategy};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Helper function to create a test RaidManager
fn create_test_raid_manager() -> Arc<RwLock<RaidManager>> {
    Arc::new(RwLock::new(RaidManager::new(RaidConfig {
        mode: RaidMode::Local,
        base_path: PathBuf::from("./test_data/raid_small_world"),
        quota_bytes: Some(10 * 1024 * 1024 * 1024), // 10 GB
        retention_days: Some(30),
        gc_on_startup: false,
    })))
}

/// Helper function to create a test topology manager
fn create_test_topology_manager() -> Arc<RwLock<TopologyManager>> {
    Arc::new(RwLock::new(TopologyManager::new(None)))
}

/// Helper function to create a test SmallWorld strategy
async fn create_test_strategy() -> SmallWorldStrategy {
    let raid_manager = create_test_raid_manager();
    let topology_manager = create_test_topology_manager();

    // Initialize topology manager
    let _ = initialize_global_topology_manager(None);

    let replication_config = poolai::raid::replication::ReplicationConfig::default();
    let replication_engine = Arc::new(ReplicationEngine::new(
        raid_manager,
        None, // No event store for tests
        replication_config,
    ));

    let config = SmallWorldConfig::default();
    SmallWorldStrategy::new(
        config,
        replication_engine,
        topology_manager,
        None, // No event store for tests
    )
}

#[tokio::test]
async fn test_small_world_strategy_creation() {
    let _strategy = create_test_strategy().await;
}

#[tokio::test]
async fn test_small_world_strategy_initialize() {
    let strategy = create_test_strategy().await;
    let result = strategy.initialize().await;
    assert!(result.is_ok(), "Strategy initialization should succeed");
}

#[tokio::test]
async fn test_replicate_artifact_no_nodes() {
    let strategy = create_test_strategy().await;
    strategy.initialize().await.unwrap();

    // Try to replicate artifact with no nodes available
    let artifact_id = Uuid::new_v4();
    let artifact_data = b"test artifact data";
    let artifact_name = "test-artifact";

    let result = strategy
        .replicate_artifact(artifact_id, artifact_data.to_vec(), artifact_name)
        .await;

    // Should return error when no nodes are available
    assert!(
        result.is_err(),
        "Should return error when no nodes available"
    );
}

#[tokio::test]
async fn test_rebalance_empty_distribution() {
    let strategy = create_test_strategy().await;
    strategy.initialize().await.unwrap();

    // Rebalance with empty distribution should succeed
    let result = strategy.rebalance().await;
    assert!(
        result.is_ok(),
        "Rebalancing empty distribution should succeed"
    );
}

#[tokio::test]
async fn test_small_world_config_default() {
    let config = SmallWorldConfig::default();
    assert_eq!(config.base_replication_factor, 3);
    assert_eq!(config.target_clustering_coefficient, 0.6);
    assert_eq!(config.max_path_length, 3);
    assert_eq!(config.proximity_threshold_ms, 50.0);
    assert!(config.enable_cluster_aware);
    assert!(config.enable_auto_rebalancing);
}

#[tokio::test]
async fn test_small_world_strategy_shutdown() {
    let strategy = create_test_strategy().await;
    strategy.initialize().await.unwrap();

    // Shutdown should succeed
    strategy.shutdown().await;
}
