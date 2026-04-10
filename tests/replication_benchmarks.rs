//! Performance Benchmarks for Distributed Replication
//!
//! Benchmarks for measuring replication performance:
//! - Replication latency measurements
//! - Throughput measurements
//! - Node selection performance
//! - Metadata operations performance
//! - Conflict resolution performance

use poolai::raid::{
    events::EventStore,
    replication::{ConflictResolutionStrategy, ReadConsistencyLevel, ReplicationEngine},
    RaidConfig, RaidManager, RaidMode,
};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;

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
) -> ReplicationEngine {
    ReplicationEngine::with_defaults(raid_manager, event_store)
}

// Note: create_test_metadata helper removed as it's not used in current benchmarks

#[tokio::test]
async fn benchmark_node_selection_performance() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Register many nodes for performance testing
    let node_count = 100;
    for i in 1..=node_count {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    // Benchmark node selection
    let start = std::time::Instant::now();
    let iterations = 1000;

    for _ in 0..iterations {
        let _selected = engine.select_replication_nodes(3, None).await.unwrap();
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() as f64 / iterations as f64;

    println!(
        "Node selection benchmark: {} iterations in {:?}, avg: {:.2}μs per selection",
        iterations, duration, avg_time
    );

    // Performance assertion: should complete in reasonable time
    assert!(duration.as_millis() < 1000, "Node selection should be fast");
}

#[tokio::test]
async fn benchmark_quorum_calculation_performance() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Benchmark quorum calculation
    let start = std::time::Instant::now();
    let iterations = 10000;

    for i in 1..=iterations {
        let _quorum = engine.calculate_quorum((i % 100) + 1);
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;

    println!(
        "Quorum calculation benchmark: {} iterations in {:?}, avg: {:.2}ns per calculation",
        iterations, duration, avg_time
    );

    // Performance assertion: should be very fast (nanoseconds)
    assert!(
        duration.as_millis() < 100,
        "Quorum calculation should be very fast"
    );
}

#[tokio::test]
async fn benchmark_replication_metadata_operations() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Benchmark metadata initialization
    let start = std::time::Instant::now();
    let artifact_count = 1000;

    for i in 0..artifact_count {
        let artifact_id = format!("artifact-{}", i);
        engine.initialize_replication(artifact_id, 3).await.unwrap();
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() as f64 / artifact_count as f64;

    println!(
        "Metadata initialization benchmark: {} artifacts in {:?}, avg: {:.2}μs per artifact",
        artifact_count, duration, avg_time
    );

    // Performance assertion: should complete in reasonable time
    assert!(
        duration.as_millis() < 5000,
        "Metadata initialization should be reasonably fast"
    );
}

#[tokio::test]
async fn benchmark_metadata_retrieval_performance() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Initialize metadata for many artifacts
    let artifact_count = 1000;
    for i in 0..artifact_count {
        let artifact_id = format!("artifact-{}", i);
        engine.initialize_replication(artifact_id, 3).await.unwrap();
    }

    // Benchmark metadata retrieval
    let start = std::time::Instant::now();
    let iterations = 10000;

    for i in 0..iterations {
        let artifact_id = format!("artifact-{}", i % artifact_count);
        let _metadata = engine.get_replication_metadata(&artifact_id).await;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() as f64 / iterations as f64;

    println!(
        "Metadata retrieval benchmark: {} retrievals in {:?}, avg: {:.2}μs per retrieval",
        iterations, duration, avg_time
    );

    // Performance assertion: should be fast
    assert!(
        duration.as_millis() < 2000,
        "Metadata retrieval should be fast"
    );
}

#[tokio::test]
async fn benchmark_node_registration_performance() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Benchmark node registration
    let start = std::time::Instant::now();
    let node_count = 1000;

    for i in 1..=node_count {
        engine
            .register_node(i, format!("http://127.0.0.1:{}", 8080 + i))
            .await;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() as f64 / node_count as f64;

    println!(
        "Node registration benchmark: {} nodes in {:?}, avg: {:.2}μs per node",
        node_count, duration, avg_time
    );

    // Performance assertion: should be fast
    assert!(
        duration.as_millis() < 1000,
        "Node registration should be fast"
    );
}

#[tokio::test]
async fn benchmark_configuration_access_performance() {
    let temp_dir = TempDir::new().unwrap();

    let raid_manager = create_test_raid_manager(&temp_dir, 1);
    raid_manager.write().await.initialize().await.unwrap();

    let event_store = create_test_event_store(&temp_dir, 1);
    event_store.write().await.initialize().await.unwrap();

    let engine = create_replication_engine(raid_manager, Some(event_store)).await;

    // Benchmark configuration access
    let start = std::time::Instant::now();
    let iterations = 100000;

    for _ in 0..iterations {
        let _config = engine.config();
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;

    println!(
        "Configuration access benchmark: {} accesses in {:?}, avg: {:.2}ns per access",
        iterations, duration, avg_time
    );

    // Performance assertion: should be very fast (nanoseconds)
    assert!(
        duration.as_millis() < 100,
        "Configuration access should be very fast"
    );
}

#[tokio::test]
async fn benchmark_consistency_level_comparison() {
    // Benchmark enum comparison operations
    let start = std::time::Instant::now();
    let iterations = 1000000;

    for _ in 0..iterations {
        let eventual = ReadConsistencyLevel::Eventual;
        let quorum = ReadConsistencyLevel::Quorum;
        let strong = ReadConsistencyLevel::Strong;

        let _ = eventual == quorum;
        let _ = quorum == strong;
        let _ = eventual == strong;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;

    println!(
        "Consistency level comparison benchmark: {} comparisons in {:?}, avg: {:.2}ns per comparison",
        iterations, duration, avg_time
    );

    // Performance assertion: should be extremely fast
    assert!(
        duration.as_millis() < 100,
        "Enum comparison should be extremely fast"
    );
}

#[tokio::test]
async fn benchmark_conflict_resolution_strategy_comparison() {
    // Benchmark enum comparison operations
    let start = std::time::Instant::now();
    let iterations = 1000000;

    for _ in 0..iterations {
        let last_write = ConflictResolutionStrategy::LastWriteWins;
        let first_write = ConflictResolutionStrategy::FirstWriteWins;
        let manual = ConflictResolutionStrategy::Manual;
        let vector_clock = ConflictResolutionStrategy::VectorClock;

        let _ = last_write == first_write;
        let _ = first_write == manual;
        let _ = manual == vector_clock;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;

    println!(
        "Conflict resolution strategy comparison benchmark: {} comparisons in {:?}, avg: {:.2}ns per comparison",
        iterations, duration, avg_time
    );

    // Performance assertion: should be extremely fast
    assert!(
        duration.as_millis() < 100,
        "Enum comparison should be extremely fast"
    );
}
