//! Integration tests for Context Memory Monitoring
//!
//! Tests the context memory monitoring functionality including
//! file tracking, metrics collection, and optimization suggestions.

use poolai::monitoring::context_memory::{ChangeType, ContextMemoryMonitor, ContextMetrics};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_context_memory_monitor_creation() {
    let monitor = ContextMemoryMonitor::new();
    let metrics = monitor.get_metrics().await;

    assert_eq!(metrics.current_size, 0);
    assert_eq!(metrics.max_size, 0);
    assert_eq!(metrics.file_count, 0);
    assert_eq!(metrics.changes_count, 0);
}

#[tokio::test]
async fn test_track_file_added() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/main.rs", 1024).await.unwrap();

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.current_size, 1024);
    assert_eq!(metrics.file_count, 1);
    assert_eq!(metrics.max_size, 1024);

    // Check changes
    let changes = monitor.get_recent_changes(10).await;
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].change_type, ChangeType::FileAdded);
    assert_eq!(changes[0].file_path, "src/main.rs");
    assert_eq!(changes[0].size_bytes, 1024);
}

#[tokio::test]
async fn test_track_file_modified() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/lib.rs", 512).await.unwrap();

    monitor
        .track_file_modified("src/lib.rs", 2048)
        .await
        .unwrap();

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.current_size, 2048);
    assert_eq!(metrics.file_count, 1);
    assert_eq!(metrics.max_size, 2048);

    // Check changes
    let changes = monitor.get_recent_changes(10).await;
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].change_type, ChangeType::FileModified);
    assert_eq!(changes[0].file_path, "src/lib.rs");
    assert_eq!(changes[0].size_bytes, 2048);
}

#[tokio::test]
async fn test_track_file_deleted() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/old.rs", 1024).await.unwrap();

    monitor.track_file_deleted("src/old.rs").await.unwrap();

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.current_size, 0);
    assert_eq!(metrics.file_count, 0);

    // Check changes
    let changes = monitor.get_recent_changes(10).await;
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].change_type, ChangeType::FileDeleted);
    assert_eq!(changes[0].file_path, "src/old.rs");
}

#[tokio::test]
async fn test_track_context_cleared() {
    let monitor = ContextMemoryMonitor::new();

    monitor
        .track_file_added("src/file1.rs", 1024)
        .await
        .unwrap();
    monitor
        .track_file_added("src/file2.rs", 2048)
        .await
        .unwrap();

    monitor.track_context_cleared().await.unwrap();

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.current_size, 0);
    assert_eq!(metrics.file_count, 0);

    // Check changes
    let changes = monitor.get_recent_changes(10).await;
    assert_eq!(changes.len(), 3);
    assert_eq!(changes[0].change_type, ChangeType::ContextCleared);
}

#[tokio::test]
async fn test_multiple_files() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/main.rs", 1024).await.unwrap();
    monitor.track_file_added("src/lib.rs", 2048).await.unwrap();
    monitor.track_file_added("src/utils.rs", 512).await.unwrap();

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.current_size, 3584); // 1024 + 2048 + 512
    assert_eq!(metrics.file_count, 3);
    assert_eq!(metrics.max_size, 3584);
}

#[tokio::test]
async fn test_max_size_tracking() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/small.rs", 100).await.unwrap();
    assert_eq!(monitor.get_metrics().await.max_size, 100);

    monitor
        .track_file_added("src/large.rs", 5000)
        .await
        .unwrap();
    assert_eq!(monitor.get_metrics().await.max_size, 5100);

    monitor.track_file_deleted("src/large.rs").await.unwrap();
    // Max size should remain at 5100 even after deletion
    assert_eq!(monitor.get_metrics().await.max_size, 5100);
}

#[tokio::test]
async fn test_average_size_calculation() {
    let monitor = ContextMemoryMonitor::new();

    monitor
        .track_file_added("src/file1.rs", 1000)
        .await
        .unwrap();
    monitor
        .track_file_added("src/file2.rs", 2000)
        .await
        .unwrap();
    monitor
        .track_file_added("src/file3.rs", 3000)
        .await
        .unwrap();

    let metrics = monitor.get_metrics().await;
    // Average should be calculated from history
    assert!(metrics.average_size > 0.0);
}

#[tokio::test]
async fn test_get_recent_changes() {
    let monitor = ContextMemoryMonitor::new();

    for i in 0..20 {
        monitor
            .track_file_added(&format!("src/file{}.rs", i), 100)
            .await
            .unwrap();
    }

    let changes = monitor.get_recent_changes(10).await;
    assert_eq!(changes.len(), 10);

    // Most recent should be last
    assert_eq!(changes[0].file_path, "src/file19.rs");
}

#[tokio::test]
async fn test_get_changes_in_window() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/file1.rs", 100).await.unwrap();

    sleep(Duration::from_millis(100)).await;

    monitor.track_file_added("src/file2.rs", 200).await.unwrap();

    // Get changes in last second
    let changes = monitor.get_changes_in_window(Duration::from_secs(1)).await;
    assert!(changes.len() >= 2);

    // Get changes in last millisecond (should be empty or very few)
    let changes = monitor
        .get_changes_in_window(Duration::from_millis(1))
        .await;
    // Should have at least the most recent change
    assert!(changes.len() >= 1);
}

#[tokio::test]
async fn test_memory_usage_tracking() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/main.rs", 1024).await.unwrap();

    let metrics = monitor.get_metrics().await;
    assert!(metrics.memory_usage.ram_bytes > 0);
    let _disk_bytes = metrics.memory_usage.disk_bytes;
    let _cache_bytes = metrics.memory_usage.cache_bytes;
}

#[tokio::test]
async fn test_memory_usage_history() {
    let monitor = ContextMemoryMonitor::new();

    monitor.track_file_added("src/file1.rs", 100).await.unwrap();
    sleep(Duration::from_millis(50)).await;

    monitor.track_file_added("src/file2.rs", 200).await.unwrap();

    let history = monitor
        .get_memory_usage_history(Duration::from_secs(1))
        .await;
    assert!(history.len() >= 2);
}

#[tokio::test]
async fn test_suggest_optimizations() {
    let monitor = ContextMemoryMonitor::new();

    // Small context - should have no or few suggestions
    monitor
        .track_file_added("src/small.rs", 1000)
        .await
        .unwrap();
    let suggestions = monitor.suggest_optimizations().await;
    // Should have few or no suggestions for small context
    assert!(suggestions.len() <= 2);

    // Large context - should suggest optimizations
    for i in 0..150 {
        monitor
            .track_file_added(&format!("src/file{}.rs", i), 100000)
            .await
            .unwrap();
    }

    let suggestions = monitor.suggest_optimizations().await;
    // Should have suggestions for large context
    assert!(!suggestions.is_empty());

    // Check that suggestions contain relevant keywords
    let suggestion_text = suggestions.join(" ");
    assert!(suggestion_text.contains("large") || suggestion_text.contains("Many files"));
}

#[tokio::test]
async fn test_optimization_suggestions_for_large_size() {
    let monitor = ContextMemoryMonitor::new();

    // Add a very large file (>10MB)
    monitor
        .track_file_added("src/huge.rs", 15_000_000)
        .await
        .unwrap();

    let suggestions = monitor.suggest_optimizations().await;
    assert!(!suggestions.is_empty());
    assert!(suggestions
        .iter()
        .any(|s| s.contains("large") || s.contains("10MB")));
}

#[tokio::test]
async fn test_optimization_suggestions_for_many_files() {
    let monitor = ContextMemoryMonitor::new();

    // Add many files (>100)
    for i in 0..150 {
        monitor
            .track_file_added(&format!("src/file{}.rs", i), 1000)
            .await
            .unwrap();
    }

    let suggestions = monitor.suggest_optimizations().await;
    assert!(!suggestions.is_empty());
    assert!(suggestions
        .iter()
        .any(|s| s.contains("Many files") || s.contains("100")));
}

#[tokio::test]
async fn test_change_history_limits() {
    let monitor = ContextMemoryMonitor::new();

    // Add many changes to test history limits
    for i in 0..2000 {
        monitor
            .track_file_added(&format!("src/file{}.rs", i), 100)
            .await
            .unwrap();
    }

    // Should still be able to get metrics
    let metrics = monitor.get_metrics().await;
    assert!(metrics.changes_count > 0);

    // Should be able to get recent changes (limited)
    let changes = monitor.get_recent_changes(100).await;
    assert!(changes.len() <= 100);
}
