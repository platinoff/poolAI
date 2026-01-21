//! Integration tests for SQLite persistence in Enterprise Monitoring
//!
//! Tests cover:
//! - Metrics persistence to SQLite database
//! - Historical metrics queries with filters
//! - Automatic cleanup of old metrics
//! - Fallback to in-memory history when database is unavailable

use poolai::core::error::AppError;
use poolai::enterprise::monitoring::{MetricDataPoint, MonitoringManager};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use uuid::Uuid;

/// Helper function to create a temporary database path
fn create_temp_db_path() -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_metrics.db");
    (temp_dir, db_path.to_string_lossy().to_string())
}

#[tokio::test]
async fn test_sqlite_persistence_record_metric() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    // Initialize database
    manager.initialize().await?;
    
    // Create test metric
    let tenant_id = Uuid::new_v4();
    let mut tags = HashMap::new();
    tags.insert("host".to_string(), "test-host".to_string());
    tags.insert("region".to_string(), "us-east-1".to_string());
    
    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "cpu_usage".to_string(),
        value: 75.5,
        tags: tags.clone(),
        tenant_id: Some(tenant_id),
    };
    
    // Record metric
    manager.record_metric(data_point.clone()).await?;
    
    // Wait a bit for async database operation
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Query metric history
    let history = manager.get_metric_history(
        Some("cpu_usage"),
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].metric, "cpu_usage");
    assert_eq!(history[0].value, 75.5);
    assert_eq!(history[0].tags, tags);
    assert_eq!(history[0].tenant_id, Some(tenant_id));
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_multiple_metrics() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_id = Uuid::new_v4();
    let now = Utc::now();
    
    // Record multiple metrics
    for i in 0..10 {
        let data_point = MetricDataPoint {
            timestamp: now + Duration::seconds(i),
            metric: format!("metric_{}", i % 3),
            value: i as f64 * 10.0,
            tags: HashMap::new(),
            tenant_id: Some(tenant_id),
        };
        manager.record_metric(data_point).await?;
    }
    
    // Wait for async operations
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Query all metrics
    let history = manager.get_metric_history(
        None,
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    assert_eq!(history.len(), 10);
    
    // Query specific metric
    let metric_0_history = manager.get_metric_history(
        Some("metric_0"),
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    assert_eq!(metric_0_history.len(), 4); // metric_0 appears at indices 0, 3, 6, 9
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_time_range_filter() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_id = Uuid::new_v4();
    let base_time = Utc::now() - Duration::hours(2);
    
    // Record metrics at different times
    for i in 0..5 {
        let data_point = MetricDataPoint {
            timestamp: base_time + Duration::minutes(i * 30),
            metric: "test_metric".to_string(),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_id),
        };
        manager.record_metric(data_point).await?;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Query with time range (last hour)
    let start_time = base_time + Duration::hours(1);
    let end_time = base_time + Duration::hours(3);
    
    let history = manager.get_metric_history(
        Some("test_metric"),
        Some(start_time),
        Some(end_time),
        Some(tenant_id),
        None,
    ).await?;
    
    // Should get metrics at indices 2, 3, 4 (within the time range)
    assert_eq!(history.len(), 3);
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_limit_filter() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_id = Uuid::new_v4();
    let now = Utc::now();
    
    // Record 20 metrics
    for i in 0..20 {
        let data_point = MetricDataPoint {
            timestamp: now + Duration::seconds(i),
            metric: "test_metric".to_string(),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_id),
        };
        manager.record_metric(data_point).await?;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    
    // Query with limit
    let history = manager.get_metric_history(
        Some("test_metric"),
        None,
        None,
        Some(tenant_id),
        Some(5),
    ).await?;
    
    assert_eq!(history.len(), 5);
    // Should be ordered by timestamp DESC (newest first)
    assert_eq!(history[0].value, 19.0);
    assert_eq!(history[4].value, 15.0);
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_tenant_filter() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_1 = Uuid::new_v4();
    let tenant_2 = Uuid::new_v4();
    let now = Utc::now();
    
    // Record metrics for tenant_1
    for i in 0..5 {
        let data_point = MetricDataPoint {
            timestamp: now + Duration::seconds(i),
            metric: "test_metric".to_string(),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_1),
        };
        manager.record_metric(data_point).await?;
    }
    
    // Record metrics for tenant_2
    for i in 0..3 {
        let data_point = MetricDataPoint {
            timestamp: now + Duration::seconds(i + 10),
            metric: "test_metric".to_string(),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_2),
        };
        manager.record_metric(data_point).await?;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Query tenant_1 metrics
    let tenant_1_history = manager.get_metric_history(
        Some("test_metric"),
        None,
        None,
        Some(tenant_1),
        None,
    ).await?;
    
    assert_eq!(tenant_1_history.len(), 5);
    
    // Query tenant_2 metrics
    let tenant_2_history = manager.get_metric_history(
        Some("test_metric"),
        None,
        None,
        Some(tenant_2),
        None,
    ).await?;
    
    assert_eq!(tenant_2_history.len(), 3);
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_tags_serialization() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let mut tags = HashMap::new();
    tags.insert("host".to_string(), "server-1".to_string());
    tags.insert("region".to_string(), "us-west-2".to_string());
    tags.insert("environment".to_string(), "production".to_string());
    
    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "test_metric".to_string(),
        value: 42.0,
        tags: tags.clone(),
        tenant_id: None,
    };
    
    manager.record_metric(data_point).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let history = manager.get_metric_history(
        Some("test_metric"),
        None,
        None,
        None,
        None,
    ).await?;
    
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].tags.len(), 3);
    assert_eq!(history[0].tags.get("host"), Some(&"server-1".to_string()));
    assert_eq!(history[0].tags.get("region"), Some(&"us-west-2".to_string()));
    assert_eq!(history[0].tags.get("environment"), Some(&"production".to_string()));
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_fallback_to_memory() -> Result<(), AppError> {
    // Create manager without persistence (in-memory only)
    let manager = MonitoringManager::new();
    manager.initialize().await?;
    
    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "test_metric".to_string(),
        value: 50.0,
        tags: HashMap::new(),
        tenant_id: None,
    };
    
    // Record metric (should work with in-memory)
    manager.record_metric(data_point.clone()).await?;
    
    // Query should return from in-memory history
    let history = manager.get_metric_history(
        Some("test_metric"),
        None,
        None,
        None,
        None,
    ).await?;
    
    // Should have at least the metric we just recorded
    assert!(!history.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_invalid_db_path() -> Result<(), AppError> {
    // Use invalid database path (non-existent parent directory)
    let invalid_path = "/nonexistent/path/test.db";
    let manager = MonitoringManager::new_with_persistence(Some(invalid_path.to_string()));
    
    // Initialize should fail gracefully
    let result = manager.initialize().await;
    // Should either succeed (if directory creation works) or fail gracefully
    // In either case, record_metric should not panic
    if result.is_err() {
        // If initialization fails, that's okay - we test graceful degradation
        return Ok(());
    }
    
    // Try to record metric - should not panic even if DB is unavailable
    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "test_metric".to_string(),
        value: 50.0,
        tags: HashMap::new(),
        tenant_id: None,
    };
    
    // Should not panic, but may log warnings
    let _ = manager.record_metric(data_point).await;
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_cleanup_old_metrics() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_id = Uuid::new_v4();
    let old_time = Utc::now() - Duration::days(31); // Older than 30 days
    let recent_time = Utc::now() - Duration::days(10); // Within 30 days
    
    // Record old metric
    let old_data_point = MetricDataPoint {
        timestamp: old_time,
        metric: "old_metric".to_string(),
        value: 10.0,
        tags: HashMap::new(),
        tenant_id: Some(tenant_id),
    };
    manager.record_metric(old_data_point).await?;
    
    // Record recent metric
    let recent_data_point = MetricDataPoint {
        timestamp: recent_time,
        metric: "recent_metric".to_string(),
        value: 20.0,
        tags: HashMap::new(),
        tenant_id: Some(tenant_id),
    };
    manager.record_metric(recent_data_point).await?;
    
    // Trigger cleanup by recording 1000 metrics (cleanup happens every 1000th insert)
    for i in 0..1000 {
        let data_point = MetricDataPoint {
            timestamp: Utc::now() + Duration::seconds(i),
            metric: "cleanup_trigger".to_string(),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_id),
        };
        manager.record_metric(data_point).await?;
    }
    
    // Wait for cleanup to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Query old metric - should not be found (cleaned up)
    let old_history = manager.get_metric_history(
        Some("old_metric"),
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    // Old metric should be cleaned up
    assert_eq!(old_history.len(), 0);
    
    // Query recent metric - should still be there
    let recent_history = manager.get_metric_history(
        Some("recent_metric"),
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    assert_eq!(recent_history.len(), 1);
    assert_eq!(recent_history[0].value, 20.0);
    
    Ok(())
}

#[tokio::test]
async fn test_sqlite_persistence_empty_filters() -> Result<(), AppError> {
    let (_temp_dir, db_path) = create_temp_db_path();
    let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
    
    manager.initialize().await?;
    
    let tenant_id = Uuid::new_v4();
    let now = Utc::now();
    
    // Record metrics with different names
    for i in 0..5 {
        let data_point = MetricDataPoint {
            timestamp: now + Duration::seconds(i),
            metric: format!("metric_{}", i),
            value: i as f64,
            tags: HashMap::new(),
            tenant_id: Some(tenant_id),
        };
        manager.record_metric(data_point).await?;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Query with no filters (should return all metrics)
    let all_history = manager.get_metric_history(
        None,
        None,
        None,
        Some(tenant_id),
        None,
    ).await?;
    
    assert_eq!(all_history.len(), 5);
    
    Ok(())
}
