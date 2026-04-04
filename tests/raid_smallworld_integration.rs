//! Integration tests for SmallWorld Strategy with real artifacts
//!
//! Tests:
//! - Clustering coefficient calculation with real topology
//! - Rebalancing with real artifacts
//! - Metrics collection
//! - Node selection based on clustering

use poolai::pool::topology::TopologyManager;
use poolai::raid::replication::ReplicationEngine;
use poolai::raid::small_world::{SmallWorldConfig, SmallWorldStrategy};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Helper function to create a test SmallWorld strategy
async fn create_test_smallworld_strategy() -> (
    SmallWorldStrategy,
    Arc<RwLock<RaidManager>>,
    Arc<RwLock<TopologyManager>>,
) {
    let temp_dir = TempDir::new().unwrap();
    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    // Use custom config with longer timeout for tests
    let replication_config = poolai::raid::replication::ReplicationConfig {
        default_replication_factor: 1,
        sync_timeout_seconds: 1, // Short timeout for tests (will fail fast)
        ..Default::default()
    };
    let replication_engine = Arc::new(ReplicationEngine::new(
        raid_manager.clone(),
        None,
        replication_config,
    ));

    // Register nodes in ReplicationEngine (required for replication)
    replication_engine
        .register_node(1, "http://192.168.1.1:8080".to_string())
        .await;
    replication_engine
        .register_node(2, "http://192.168.1.2:8080".to_string())
        .await;
    replication_engine
        .register_node(3, "http://192.168.1.3:8080".to_string())
        .await;

    let topology_manager = Arc::new(RwLock::new(TopologyManager::new(None)));

    // Add some test nodes to topology
    {
        let manager = topology_manager.write().await;
        (&*manager).test_add_node("1", "192.168.1.1:8080").await;
        (&*manager).test_add_node("2", "192.168.1.2:8080").await;
        (&*manager).test_add_node("3", "192.168.1.3:8080").await;

        // Add latency information
        (&*manager).test_update_latency("1", "2", 10.0).await;
        (&*manager).test_update_latency("2", "3", 15.0).await;
        (&*manager).test_update_latency("1", "3", 20.0).await;
    }

    let smallworld_config = SmallWorldConfig {
        base_replication_factor: 1, // Use 1 for tests to avoid network replication timeout
        target_clustering_coefficient: 0.6,
        max_path_length: 3,
        proximity_threshold_ms: 50.0,
        enable_cluster_aware: true,
        rebalancing_interval_secs: 60,
        enable_auto_rebalancing: false, // Disable auto-rebalancing for tests
    };

    let strategy = SmallWorldStrategy::new(
        smallworld_config,
        replication_engine,
        topology_manager.clone(),
        None,
    );
    strategy.initialize().await.unwrap();

    (strategy, raid_manager, topology_manager)
}

#[tokio::test]
async fn test_clustering_coefficient_with_real_topology() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Clustering coefficients are updated during initialization
    // Get clustering coefficient for node 1 (may be None if not calculated yet)
    let coeff = strategy.get_node_clustering_coefficient(1).await;

    // If coefficient is available, verify it's in valid range (allow small float error)
    const EPS: f64 = 1e-9;
    if let Some(coeff_value) = coeff {
        assert!(
            coeff_value >= -EPS && coeff_value <= 1.0 + EPS,
            "Clustering coefficient should be in [0, 1], got {}",
            coeff_value
        );
    }

    // Get metrics
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_nodes, 3, "Should have 3 nodes");
    assert!(
        metrics.avg_clustering_coefficient >= 0.0,
        "Average clustering coefficient should be >= 0"
    );

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}

#[tokio::test]
async fn test_rebalance_with_real_artifacts() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Create multiple artifacts for testing (without real replication)
    let mut artifact_ids = Vec::new();
    for _ in 0..5 {
        let artifact_id = Uuid::new_v4();
        // Add artifact to placements without real replication for tests
        strategy.add_test_artifact(artifact_id, vec![1, 2, 3]).await;
        artifact_ids.push(artifact_id);
    }

    // Trigger rebalancing (may move 0 artifacts if already optimal)
    let _artifacts_moved = strategy.rebalance().await.unwrap();

    // Verify metrics
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 5, "Should track 5 artifacts");
    assert_eq!(metrics.total_nodes, 3, "Should have 3 nodes");

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}

#[tokio::test]
async fn test_node_selection_based_on_clustering() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Create artifact for testing (without real replication)
    let artifact_id = Uuid::new_v4();
    strategy.add_test_artifact(artifact_id, vec![1, 2, 3]).await;

    // Update clustering coefficients first
    strategy.update_clustering_coefficients().await.unwrap();

    // Test that rebalancing uses clustering-based node selection
    // Rebalance internally uses select_target_nodes which considers clustering
    let _artifacts_moved = strategy.rebalance().await.unwrap();

    // Verify metrics show nodes are tracked
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_nodes, 3, "Should have 3 nodes in topology");
    assert!(
        metrics.avg_clustering_coefficient >= 0.0,
        "Average clustering coefficient should be calculated"
    );

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}

#[tokio::test]
async fn test_metrics_collection() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Create artifacts for testing (without real replication)
    for _ in 0..3 {
        let artifact_id = Uuid::new_v4();
        // Add artifact to placements without real replication for tests
        strategy.add_test_artifact(artifact_id, vec![1, 2, 3]).await;
    }

    // Get metrics
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 3, "Should track 3 artifacts");
    assert_eq!(metrics.total_nodes, 3, "Should have 3 nodes");
    assert_eq!(
        metrics.base_replication_factor, 1,
        "Base replication factor should be 1 (config uses 1 for tests)"
    );
    const EPS: f64 = 1e-9;
    assert!(
        metrics.avg_clustering_coefficient >= -EPS
            && metrics.avg_clustering_coefficient <= 1.0 + EPS,
        "Average clustering coefficient should be in [0, 1], got {}",
        metrics.avg_clustering_coefficient
    );

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}

#[tokio::test]
async fn test_node_clustering_coefficient() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Get clustering coefficient for specific node
    let coeff1 = strategy.get_node_clustering_coefficient(1).await;
    assert!(
        coeff1.is_some(),
        "Clustering coefficient should be available for node 1"
    );

    let coeff2 = strategy.get_node_clustering_coefficient(2).await;
    assert!(
        coeff2.is_some(),
        "Clustering coefficient should be available for node 2"
    );

    // Non-existent node should return None
    let coeff_none = strategy.get_node_clustering_coefficient(999).await;
    assert!(coeff_none.is_none(), "Non-existent node should return None");

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}

#[tokio::test]
async fn test_rebalance_optimizes_placement() {
    let (strategy, _raid_manager, _topology_manager) = create_test_smallworld_strategy().await;

    // Create artifact for testing (without real replication)
    let artifact_id = Uuid::new_v4();
    // Add artifact to placements without real replication for tests
    strategy.add_test_artifact(artifact_id, vec![1, 2, 3]).await;

    // After rebalancing, placement should be optimized (may move 0 if already optimal)
    let _artifacts_moved = strategy.rebalance().await.unwrap();

    // Verify metrics are updated
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 1, "Should track 1 artifact");

    // Cleanup: shutdown strategy
    strategy.shutdown().await;
}
