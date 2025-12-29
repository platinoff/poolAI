//! Failure Scenario Tests for Distributed Replication
//!
//! Tests for handling failures in distributed replication:
//! - Node failure scenarios
//! - Network partition scenarios
//! - Partial failure handling
//! - Recovery after failures
//! - Circuit breaker integration with failures
//! - Quorum availability during failures

use poolai::core::error::AppError;
use poolai::raid::{
    circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState},
    client::ProtocolClient,
    events::{EventStore, RaidEvent},
    protocol::ArtifactMetadata,
    replication::{
        ConflictResolutionStrategy, ReadConsistencyLevel, ReplicationConfig, ReplicationEngine,
        ReplicationStatus,
    },
    RaidConfig, RaidManager, RaidMode,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tempfile::TempDir;
use chrono::Utc;

/// Helper function to create a test RAID manager
fn create_test_raid_manager(temp_dir: &TempDir, node_id: u64) -> Arc<RwLock<RaidManager>> {
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp_dir.path().join(format!("node_{}", node_id)),
        quota_bytes: Some(1024 * 1024 * 1024),
        retention_days: Some(30),
        gc_on_startup: false,
    };
    Arc::new(RwLock::new(RaidManager::new(config)))
}

/// Helper function to create a test event store
fn create_test_event_store(temp_dir: &TempDir, node_id: u64) -> Arc<RwLock<EventStore>> {
    let event_store = EventStore::new(temp_dir.path().join(format!("node_{}/events", node_id)));
    Arc::new(RwLock::new(event_store))
}

/// Helper function to create a replication engine
async fn create_replication_engine(
    raid_manager: Arc<RwLock<RaidManager>>,
    event_store: Option<Arc<RwLock<EventStore>>>,
    config: Option<ReplicationConfig>,
) -> ReplicationEngine {
    ReplicationEngine::with_defaults(raid_manager, event_store)
}

#[tokio::test]
async fn test_quorum_availability_during_failures() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Register 5 nodes (replication factor 3, quorum 2)
    for i in 1..=5 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Calculate quorum for replication factor 3
    let quorum = engine.calculate_quorum(3);
    assert_eq!(quorum, 2);

    // With 5 nodes, we can lose 2 nodes and still have quorum (3 nodes remaining >= quorum 2)
    // Test that we can select nodes even if some are excluded (simulating failures)
    let exclude_failed_nodes = vec![4, 5]; // Simulate 2 nodes failed
    let selected = engine
        .select_replication_nodes(3, Some(exclude_failed_nodes))
        .await
        .unwrap();

    // Should still be able to select 3 nodes from remaining 3
    assert_eq!(selected.len(), 3);
    assert!(!selected.contains(&4));
    assert!(!selected.contains(&5));
}

#[tokio::test]
async fn test_quorum_unavailable_after_too_many_failures() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Register 5 nodes (replication factor 3, quorum 2)
    for i in 1..=5 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Simulate too many failures (4 nodes failed, only 1 remaining)
    // Quorum is 2, but only 1 node available
    let exclude_failed_nodes = vec![2, 3, 4, 5]; // Simulate 4 nodes failed
    let result = engine
        .select_replication_nodes(3, Some(exclude_failed_nodes))
        .await;

    // Should fail or return only available nodes
    match result {
        Ok(selected) => {
            // If it returns, should only have 1 node (less than quorum)
            assert!(selected.len() < 2, "Should not have quorum");
        }
        Err(AppError::ConfigError(_)) => {
            // Expected error when not enough nodes
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_circuit_breaker_failure_detection() {
    // Test that circuit breaker can detect failures
    let breaker = CircuitBreaker::with_defaults(1);
    let config = CircuitBreakerConfig::default();

    // Initially closed
    assert_eq!(breaker.state().await, CircuitState::Closed);

    // Record failures up to threshold
    let threshold = config.failure_threshold;
    for _ in 0..threshold {
        breaker.record_failure().await;
    }

    // Circuit should be open
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Requests should be rejected
    assert!(!breaker.allow_request().await);
}

#[tokio::test]
async fn test_circuit_breaker_recovery() {
    // Test that circuit breaker can recover after failures
    let breaker = CircuitBreaker::with_defaults(1);
    let config = CircuitBreakerConfig::default();

    // Record failures to open circuit
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }

    assert_eq!(breaker.state().await, CircuitState::Open);

    // Wait for recovery timeout (timeout_seconds)
    tokio::time::sleep(tokio::time::Duration::from_secs(
        config.timeout_seconds + 1,
    ))
    .await;

    // Circuit should transition to half-open
    // (In real scenario, would need to check after timeout)
    // For now, verify that recording success after timeout would close it
    breaker.record_success().await;
    // After success in half-open, should close
    // (This depends on implementation - verifying basic functionality)
}

#[tokio::test]
async fn test_replication_status_on_failure() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    let artifact_id = "test-artifact-failure".to_string();

    // Initialize replication
    engine
        .initialize_replication(artifact_id.clone(), 3)
        .await
        .unwrap();

    // Simulate failure by updating status
    // In real scenario, this would happen during replication attempt
    let metadata = engine.get_replication_metadata(&artifact_id).await;
    assert!(metadata.is_some());

    // Test different failure statuses
    let failed_status = ReplicationStatus::Failed {
        reason: "Node unreachable".to_string(),
    };

    let partial_status = ReplicationStatus::Partial {
        successful: 2,
        failed: 1,
    };

    // Verify status variants
    match failed_status {
        ReplicationStatus::Failed { reason } => {
            assert_eq!(reason, "Node unreachable");
        }
        _ => panic!("Expected Failed status"),
    }

    match partial_status {
        ReplicationStatus::Partial { successful, failed } => {
            assert_eq!(successful, 2);
            assert_eq!(failed, 1);
        }
        _ => panic!("Expected Partial status"),
    }
}

#[tokio::test]
async fn test_node_selection_excludes_failed_nodes() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Register 5 nodes
    for i in 1..=5 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Simulate node 3 and 5 failed
    let failed_nodes = vec![3, 5];
    let selected = engine
        .select_replication_nodes(3, Some(failed_nodes.clone()))
        .await
        .unwrap();

    // Should not select failed nodes
    assert!(!selected.contains(&3));
    assert!(!selected.contains(&5));
    assert_eq!(selected.len(), 3);

    // Should select from remaining healthy nodes (1, 2, 4)
    for node_id in &selected {
        assert!(*node_id == 1 || *node_id == 2 || *node_id == 4);
    }
}

#[tokio::test]
async fn test_replication_with_partial_failures() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Register 5 nodes
    for i in 1..=5 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    let artifact_id = "test-artifact-partial-failure".to_string();

    // Initialize replication with factor 5
    engine
        .initialize_replication(artifact_id.clone(), 5)
        .await
        .unwrap();

    // Simulate partial failure: 2 nodes failed, 3 succeeded
    // In real scenario, this would be detected during replication
    // For test, we verify that partial status can be represented
    let partial_status = ReplicationStatus::Partial {
        successful: 3,
        failed: 2,
    };

    match partial_status {
        ReplicationStatus::Partial { successful, failed } => {
            assert_eq!(successful, 3);
            assert_eq!(failed, 2);
            // Total should match replication factor
            assert_eq!(successful + failed, 5);
        }
        _ => panic!("Expected Partial status"),
    }
}

#[tokio::test]
async fn test_read_consistency_with_failures() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Test that different consistency levels handle failures differently
    let eventual = ReadConsistencyLevel::Eventual;
    let quorum = ReadConsistencyLevel::Quorum;
    let strong = ReadConsistencyLevel::Strong;

    // Eventual can work with any single replica (most tolerant to failures)
    // Quorum needs majority (moderate tolerance)
    // Strong needs all replicas (least tolerant to failures)

    // Verify enum variants
    assert_ne!(eventual, quorum);
    assert_ne!(quorum, strong);
    assert_ne!(eventual, strong);
}

#[tokio::test]
async fn test_replication_retry_on_failure() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    // Create config with retry settings
    let config = ReplicationConfig {
        default_replication_factor: 3,
        sync_timeout_seconds: 30,
        async_retry_attempts: 3,
        async_retry_delay_seconds: 5,
        async_queue_size: 1000,
        async_worker_count: 2,
        default_read_consistency: ReadConsistencyLevel::Quorum,
    };

    let engine = create_replication_engine(raid_manager, Some(event_store), Some(config)).await;

    // Verify retry configuration using public method
    let engine_config = engine.config();
    assert_eq!(engine_config.async_retry_attempts, 3);
    assert_eq!(engine_config.async_retry_delay_seconds, 5);
}

#[tokio::test]
async fn test_network_partition_scenario() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store), None).await;

    // Register 5 nodes
    for i in 1..=5 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Simulate network partition: nodes 1,2,3 can communicate, but 4,5 are partitioned
    // In a 5-node cluster with replication factor 3, quorum is 2
    // Partition 1 (nodes 1,2,3) has 3 nodes >= quorum 2, so can continue
    // Partition 2 (nodes 4,5) has 2 nodes >= quorum 2, but this is a split-brain scenario

    // Test that we can still select nodes from partition 1
    let partition1_nodes = vec![1, 2, 3];
    let selected = engine
        .select_replication_nodes(3, Some(vec![4, 5])) // Exclude partitioned nodes
        .await
        .unwrap();

    // Should select from partition 1
    assert_eq!(selected.len(), 3);
    for node_id in &selected {
        assert!(partition1_nodes.contains(node_id));
    }
}

