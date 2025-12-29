//! Load Tests for Distributed Replication
//!
//! Tests for high-load scenarios:
//! - Concurrent replication operations
//! - High-throughput replication
//! - Concurrent node registrations
//! - Concurrent metadata operations
//! - Stress testing with many artifacts
//! - Concurrent read operations

use poolai::raid::{
    events::{EventStore, RaidEvent},
    protocol::ArtifactMetadata,
    replication::{
        ConflictResolutionStrategy, ReadConsistencyLevel, ReplicationConfig, ReplicationEngine,
    },
    RaidConfig, RaidManager, RaidMode,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tempfile::TempDir;
use chrono::Utc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

/// Helper function to create a replication engine wrapped in Arc
async fn create_replication_engine(
    raid_manager: Arc<RwLock<RaidManager>>,
    event_store: Option<Arc<RwLock<EventStore>>>,
) -> Arc<ReplicationEngine> {
    Arc::new(ReplicationEngine::with_defaults(raid_manager, event_store))
}

#[tokio::test]
async fn test_concurrent_node_registration() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Concurrent node registration
    let node_count = 100;
    let mut handles = Vec::new();

    for i in 1..=node_count {
        let engine_clone = engine.clone();
        let handle = tokio::spawn(async move {
            engine_clone
                .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
                .await;
        });
        handles.push(handle);
    }

    // Wait for all registrations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all nodes are registered
    let selected = engine.select_replication_nodes(100, None).await.unwrap();
    assert_eq!(selected.len(), 100);
}

#[tokio::test]
async fn test_concurrent_metadata_initialization() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Concurrent metadata initialization
    let artifact_count = 1000;
    let mut handles = Vec::new();

    for i in 0..artifact_count {
        let engine_clone = engine.clone();
        let artifact_id = format!("artifact-{}", i);
        let handle = tokio::spawn(async move {
            engine_clone
                .initialize_replication(artifact_id, 3)
                .await
                .unwrap();
        });
        handles.push(handle);
    }

    // Wait for all initializations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify metadata was created
    for i in 0..artifact_count {
        let artifact_id = format!("artifact-{}", i);
        let metadata = engine.get_replication_metadata(&artifact_id).await;
        assert!(metadata.is_some(), "Metadata should exist for artifact {}", artifact_id);
    }
}

#[tokio::test]
async fn test_concurrent_node_selection() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Register nodes
    for i in 1..=50 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Concurrent node selection
    let selection_count = 1000;
    let mut handles = Vec::new();

    for _ in 0..selection_count {
        let engine_clone = engine.clone();
        let handle = tokio::spawn(async move {
            let selected = engine_clone.select_replication_nodes(3, None).await.unwrap();
            assert_eq!(selected.len(), 3);
        });
        handles.push(handle);
    }

    // Wait for all selections to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_concurrent_metadata_retrieval() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Initialize metadata
    let artifact_count = 100;
    for i in 0..artifact_count {
        let artifact_id = format!("artifact-{}", i);
        engine
            .initialize_replication(artifact_id, 3)
            .await
            .unwrap();
    }

    // Concurrent metadata retrieval
    let retrieval_count = 1000;
    let mut handles = Vec::new();

    for i in 0..retrieval_count {
        let engine_clone = engine.clone();
        // Use deterministic selection based on iteration
        let artifact_id = format!("artifact-{}", i % artifact_count);
        let handle = tokio::spawn(async move {
            let metadata = engine_clone.get_replication_metadata(&artifact_id).await;
            assert!(metadata.is_some(), "Metadata should exist");
        });
        handles.push(handle);
    }

    // Wait for all retrievals to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_high_throughput_replication_metadata() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // High-throughput metadata operations
    let operations = 10000;
    let start = std::time::Instant::now();

    for i in 0..operations {
        let artifact_id = format!("artifact-{}", i);
        engine
            .initialize_replication(artifact_id, 3)
            .await
            .unwrap();
    }

    let duration = start.elapsed();
    let throughput = operations as f64 / duration.as_secs_f64();

    println!(
        "High-throughput test: {} operations in {:?}, throughput: {:.2} ops/sec",
        operations, duration, throughput
    );

    // Performance assertion: should handle high throughput
    assert!(
        throughput > 1000.0,
        "Should handle at least 1000 ops/sec, got {:.2}",
        throughput
    );
}

#[tokio::test]
async fn test_stress_many_artifacts() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Stress test with many artifacts
    let artifact_count = 10000;
    let start = std::time::Instant::now();

    for i in 0..artifact_count {
        let artifact_id = format!("artifact-{}", i);
        engine
            .initialize_replication(artifact_id, 3)
            .await
            .unwrap();
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() as f64 / artifact_count as f64;

    println!(
        "Stress test: {} artifacts in {:?}, avg: {:.2}μs per artifact",
        artifact_count, duration, avg_time
    );

    // Verify all artifacts have metadata
    let mut verified = 0;
    for i in 0..artifact_count.min(100) {
        // Sample check (checking all 10000 would be too slow)
        let artifact_id = format!("artifact-{}", i);
        if engine.get_replication_metadata(&artifact_id).await.is_some() {
            verified += 1;
        }
    }

    assert!(
        verified == 100,
        "Should have metadata for all sampled artifacts"
    );
}

#[tokio::test]
async fn test_concurrent_quorum_calculations() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Concurrent quorum calculations
    let calculation_count = 10000;
    let mut handles = Vec::new();

    for i in 0..calculation_count {
        let engine_clone = engine.clone();
        // Use deterministic factor based on iteration
        let replication_factor = ((i as u32) % 100) + 1;
        let handle = tokio::spawn(async move {
            let quorum = engine_clone.calculate_quorum(replication_factor);
            assert!(quorum > 0);
            assert!(quorum <= replication_factor);
        });
        handles.push(handle);
    }

    // Wait for all calculations to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_mixed_workload() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Register nodes
    for i in 1..=20 {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Mixed workload: concurrent operations of different types
    let mut handles = Vec::new();

    // Node selections
    for _ in 0..100 {
        let engine_clone = engine.clone();
        let handle = tokio::spawn(async move {
            let _selected = engine_clone.select_replication_nodes(3, None).await.unwrap();
        });
        handles.push(handle);
    }

    // Metadata initializations
    for i in 0..100 {
        let engine_clone = engine.clone();
        let artifact_id = format!("artifact-{}", i);
        let handle = tokio::spawn(async move {
            engine_clone
                .initialize_replication(artifact_id, 3)
                .await
                .unwrap();
        });
        handles.push(handle);
    }

    // Quorum calculations
    for _ in 0..100 {
        let engine_clone = engine.clone();
        let handle = tokio::spawn(async move {
            let _quorum = engine_clone.calculate_quorum(5);
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

