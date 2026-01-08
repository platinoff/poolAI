//! Integration tests for Monitoring Metrics Module

use poolai::monitoring::metrics::{MetricsCollector, ModelMetrics, ResourceMetrics};
use std::time::Duration;

#[tokio::test]
async fn test_metrics_collector_creation() {
    let collector = MetricsCollector::new();
    // Just verify it can be created
    let _ = collector;
}

#[tokio::test]
async fn test_metrics_collection() -> Result<(), Box<dyn std::error::Error>> {
    let collector = MetricsCollector::new();
    let metrics = collector.collect().await?;

    // Verify metrics structure
    assert!(metrics.gpu_utilization >= 0.0);
    assert!(metrics.memory_usage_mb >= 0.0);
    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.disk_usage_percent >= 0.0);
    assert!(metrics.network_throughput_mbps >= 0.0);
    assert!(metrics.average_response_time_ms >= 0.0);
    assert!(metrics.requests_per_second >= 0.0);
    assert!(metrics.error_rate >= 0.0);

    Ok(())
}

#[tokio::test]
async fn test_model_metrics_management() {
    let mut collector = MetricsCollector::new();

    let model_metrics = ModelMetrics {
        model_name: "test-model".to_string(),
        processing_time_ms: 150,
        tokens_generated: 1000,
        tokens_per_second: 6.67,
        gpu_memory_usage_mb: 8192.0,
        gpu_utilization: 85.5,
        cache_hit_rate: 0.92,
        error_count: 0,
        success_count: 100,
    };

    collector
        .update_model_metrics("test-model".to_string(), model_metrics.clone())
        .await;

    let retrieved = collector.get_model_metrics("test-model").await;
    assert!(retrieved.is_some());
    let retrieved_metrics = retrieved.unwrap();
    assert_eq!(retrieved_metrics.model_name, "test-model");
    assert_eq!(retrieved_metrics.tokens_generated, 1000);
}

#[tokio::test]
async fn test_resource_metrics_management() {
    let mut collector = MetricsCollector::new();

    let resource_metrics = ResourceMetrics {
        gpu_count: 2,
        total_gpu_memory_mb: 16384.0,
        available_gpu_memory_mb: 8192.0,
        cpu_cores: 16,
        total_ram_mb: 32768.0,
        available_ram_mb: 16384.0,
        disk_space_gb: 1000.0,
        available_disk_space_gb: 500.0,
    };

    collector
        .update_resource_metrics(resource_metrics.clone())
        .await;

    let retrieved = collector.get_resource_metrics().await;
    assert_eq!(retrieved.gpu_count, 2);
    assert_eq!(retrieved.cpu_cores, 16);
    assert_eq!(retrieved.total_ram_mb, 32768.0);
}

#[tokio::test]
async fn test_historical_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();

    // Collect and add metrics to history
    let metrics1 = collector.collect().await?;
    collector.add_metrics_to_history(metrics1).await;

    let metrics2 = collector.collect().await?;
    collector.add_metrics_to_history(metrics2).await;

    // Get historical metrics
    let historical = collector
        .get_historical_metrics(Duration::from_secs(60))
        .await;
    assert!(historical.len() >= 2);

    Ok(())
}

#[tokio::test]
async fn test_historical_metrics_time_filter() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();

    let metrics = collector.collect().await?;
    collector.add_metrics_to_history(metrics).await;

    // Get metrics from a very short time window (should include recent metrics)
    let recent = collector
        .get_historical_metrics(Duration::from_secs(1))
        .await;
    assert!(recent.len() >= 1);

    // Get metrics from a time window in the past (should be empty)
    let old = collector
        .get_historical_metrics(Duration::from_secs(0))
        .await;
    assert_eq!(old.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_model_metrics_not_found() {
    let collector = MetricsCollector::new();

    let retrieved = collector.get_model_metrics("non-existent-model").await;
    assert!(retrieved.is_none());
}
