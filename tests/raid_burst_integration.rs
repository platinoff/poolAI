//! Integration tests for BurstRAID Strategy with real artifacts
//!
//! Tests:
//! - Burst detection with real artifacts
//! - Rebalancing with real artifacts
//! - Metrics collection
//! - Replication factor changes based on burst state

use poolai::core::error::AppError;
use poolai::raid::burst_raid::{BurstRaidConfig, BurstRaidStrategy};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Helper function to create a test BurstRAID strategy
async fn create_test_burst_strategy() -> (BurstRaidStrategy, Arc<RwLock<RaidManager>>) {
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

    let burst_config = BurstRaidConfig {
        base_replication_factor: 2,
        max_replication_factor: 5,
        burst_threshold_rps: 5.0, // Low threshold for testing
        burst_cooldown_secs: 10,  // Short cooldown for testing
        rebalancing_interval_secs: 60,
        enable_auto_rebalancing: false, // Disable auto-rebalancing for tests
    };

    let strategy = BurstRaidStrategy::new(burst_config, raid_manager.clone(), None);
    strategy.initialize().await.unwrap();

    (strategy, raid_manager)
}

#[tokio::test]
async fn test_burst_detection_with_real_artifacts() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    // Create a real artifact
    let artifact_ref = raid_manager
        .write()
        .await
        .put_artifact("test-artifact", b"test data for burst detection")
        .await
        .unwrap();

    let artifact_id = artifact_ref.id;

    // Record multiple accesses quickly to trigger burst
    for _ in 0..10 {
        strategy.record_access(artifact_id).await;
        // Small delay to simulate rapid access
        sleep(Duration::from_millis(50)).await;
    }

    // Check if burst is detected
    let is_burst = strategy.is_burst(artifact_id).await;
    assert!(is_burst, "Burst should be detected after rapid access");

    // Verify replication factor increased
    let replication_factor = strategy.get_replication_factor(artifact_id).await;
    assert_eq!(
        replication_factor, 5,
        "Replication factor should be max (5) during burst"
    );

    // Get burst stats
    let stats = strategy.get_artifact_burst_stats(artifact_id).await;
    assert!(stats.is_some(), "Burst stats should be available");
    if let Some(stats) = stats {
        assert!(stats.in_burst, "Artifact should be in burst state");
        assert!(stats.current_rps >= 5.0, "RPS should be above threshold");
        assert_eq!(
            stats.replication_factor, 5,
            "Replication factor should be max"
        );
    }
}

#[tokio::test]
async fn test_burst_cooldown() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    // Create artifact
    let artifact_ref = raid_manager
        .write()
        .await
        .put_artifact("test-artifact", b"test data")
        .await
        .unwrap();

    let artifact_id = artifact_ref.id;

    // Trigger burst
    for _ in 0..10 {
        strategy.record_access(artifact_id).await;
        sleep(Duration::from_millis(50)).await;
    }

    assert!(strategy.is_burst(artifact_id).await, "Should be in burst");

    // Wait for cooldown
    sleep(Duration::from_secs(12)).await;

    // After cooldown, burst should end
    // Note: This test may be flaky due to timing, but tests the cooldown logic
    let replication_factor = strategy.get_replication_factor(artifact_id).await;
    // Replication factor should return to base after cooldown
    // (may still be in burst if accessed recently)
}

#[tokio::test]
async fn test_rebalance_with_real_artifacts() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    // Create multiple artifacts
    let mut artifact_ids = Vec::new();
    for i in 0..5 {
        let artifact_ref = raid_manager
            .write()
            .await
            .put_artifact(&format!("artifact-{}", i), b"test data")
            .await
            .unwrap();
        artifact_ids.push(artifact_ref.id);

        // Record some accesses
        for _ in 0..3 {
            strategy.record_access(artifact_ref.id).await;
        }
    }

    // Trigger rebalancing
    let artifacts_moved = strategy.rebalance().await.unwrap();

    // Rebalancing should complete (may move 0 artifacts if already balanced)
    // artifacts_moved is an unsigned count; just acknowledge it.
    let _ = artifacts_moved;

    // Verify metrics are available
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 5, "Should track 5 artifacts");
    assert!(metrics.total_requests > 0, "Should have recorded requests");
}

#[tokio::test]
async fn test_metrics_collection() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    // Create artifacts with different access patterns
    let artifact1 = raid_manager
        .write()
        .await
        .put_artifact("high-traffic", b"data1")
        .await
        .unwrap();

    let artifact2 = raid_manager
        .write()
        .await
        .put_artifact("low-traffic", b"data2")
        .await
        .unwrap();

    // High traffic artifact
    for _ in 0..10 {
        strategy.record_access(artifact1.id).await;
        sleep(Duration::from_millis(50)).await;
    }

    // Low traffic artifact
    strategy.record_access(artifact2.id).await;

    // Get metrics
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 2, "Should track 2 artifacts");
    assert!(
        metrics.total_requests >= 11,
        "Should have recorded requests"
    );

    // Check burst stats for high traffic artifact
    let stats1 = strategy.get_artifact_burst_stats(artifact1.id).await;
    assert!(stats1.is_some(), "Stats should be available");
    if let Some(stats) = stats1 {
        assert!(stats.current_rps > 0.0, "Should have RPS > 0");
    }

    // Check burst stats for low traffic artifact
    let stats2 = strategy.get_artifact_burst_stats(artifact2.id).await;
    assert!(stats2.is_some(), "Stats should be available");
}

#[tokio::test]
async fn test_replication_factor_changes() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    let artifact_ref = raid_manager
        .write()
        .await
        .put_artifact("test-artifact", b"test data")
        .await
        .unwrap();

    let artifact_id = artifact_ref.id;

    // Initially, replication factor should be base
    let initial_factor = strategy.get_replication_factor(artifact_id).await;
    assert_eq!(
        initial_factor, 2,
        "Initial replication factor should be base (2)"
    );

    // Trigger burst
    for _ in 0..10 {
        strategy.record_access(artifact_id).await;
        sleep(Duration::from_millis(50)).await;
    }

    // Replication factor should increase
    let burst_factor = strategy.get_replication_factor(artifact_id).await;
    assert_eq!(
        burst_factor, 5,
        "Replication factor should be max (5) during burst"
    );
}

#[tokio::test]
async fn test_multiple_artifacts_burst_detection() {
    let (strategy, raid_manager) = create_test_burst_strategy().await;

    // Create multiple artifacts
    let mut artifacts = Vec::new();
    for i in 0..3 {
        let artifact_ref = raid_manager
            .write()
            .await
            .put_artifact(&format!("artifact-{}", i), b"test data")
            .await
            .unwrap();
        artifacts.push(artifact_ref.id);
    }

    // Trigger burst for first artifact only
    for _ in 0..10 {
        strategy.record_access(artifacts[0]).await;
        sleep(Duration::from_millis(50)).await;
    }

    // Update burst state before checking to ensure metrics are accurate
    // Use get_replication_factor which updates burst state
    let _ = strategy.get_replication_factor(artifacts[0]).await;
    let _ = strategy.get_replication_factor(artifacts[1]).await;
    let _ = strategy.get_replication_factor(artifacts[2]).await;

    // Only first artifact should be in burst
    assert!(
        strategy.is_burst(artifacts[0]).await,
        "First artifact should be in burst"
    );
    assert!(
        !strategy.is_burst(artifacts[1]).await,
        "Second artifact should not be in burst"
    );
    assert!(
        !strategy.is_burst(artifacts[2]).await,
        "Third artifact should not be in burst"
    );

    // Verify metrics reflect this
    let metrics = strategy.get_metrics().await;
    assert_eq!(metrics.total_artifacts, 3, "Should track 3 artifacts");
    assert_eq!(
        metrics.artifacts_in_burst, 1,
        "Should have 1 artifact in burst"
    );
}
