//! Integration tests for Runtime Worker Module

use poolai::runtime::worker::{Worker, WorkerConfig, WorkerMetrics, WorkerStatus};
use chrono::Utc;

#[tokio::test]
async fn test_worker_creation() {
    let worker = Worker::new(1);
    // Just verify it can be created
    let _ = worker;
}

#[tokio::test]
async fn test_worker_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = Worker::new(1);
    worker.initialize().await?;
    
    let status = worker.get_status().await;
    assert_eq!(status, WorkerStatus::Ready);
    
    Ok(())
}

#[tokio::test]
async fn test_worker_status_transitions() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = Worker::new(1);
    
    // Initially should be Initializing
    let initial_status = worker.get_status().await;
    assert_eq!(initial_status, WorkerStatus::Initializing);
    
    // After initialization, should be Ready
    worker.initialize().await?;
    let ready_status = worker.get_status().await;
    assert_eq!(ready_status, WorkerStatus::Ready);
    
    // After shutdown, should be Shutdown
    worker.shutdown().await?;
    let shutdown_status = worker.get_status().await;
    assert_eq!(shutdown_status, WorkerStatus::Shutdown);
    
    Ok(())
}

#[tokio::test]
async fn test_worker_metrics_update() {
    let worker = Worker::new(1);
    
    let new_metrics = WorkerMetrics {
        cpu_usage: 50.0,
        memory_usage_mb: 1024.0,
        gpu_usage: Some(75.0),
        tasks_completed: 100,
        tasks_failed: 2,
        avg_task_duration_ms: 250.0,
        last_activity: Utc::now(),
    };
    
    worker.update_metrics(new_metrics.clone()).await;
    
    let retrieved_metrics = worker.get_metrics().await;
    assert_eq!(retrieved_metrics.cpu_usage, 50.0);
    assert_eq!(retrieved_metrics.memory_usage_mb, 1024.0);
    assert_eq!(retrieved_metrics.gpu_usage, Some(75.0));
    assert_eq!(retrieved_metrics.tasks_completed, 100);
}

#[tokio::test]
async fn test_worker_metrics_default() {
    let metrics = WorkerMetrics::default();
    
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.memory_usage_mb, 0.0);
    assert_eq!(metrics.gpu_usage, None);
    assert_eq!(metrics.tasks_completed, 0);
    assert_eq!(metrics.tasks_failed, 0);
}

#[tokio::test]
async fn test_worker_config_default() {
    let config = WorkerConfig::default();
    
    assert_eq!(config.id, "default-worker");
    assert_eq!(config.max_memory_mb, 2048);
    assert_eq!(config.cpu_priority, 5);
    assert_eq!(config.gpu_device, None);
    assert!(config.auto_restart);
    assert_eq!(config.health_check_interval, 30);
}

#[tokio::test]
async fn test_worker_active_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = Worker::new(1);
    
    // Initially should be 0 (Initializing)
    let count_initial = worker.get_active_count().await;
    assert_eq!(count_initial, 0);
    
    // After initialization, should be 1 (Ready)
    worker.initialize().await?;
    let count_ready = worker.get_active_count().await;
    assert_eq!(count_ready, 1);
    
    // After shutdown, should be 0 (Shutdown)
    worker.shutdown().await?;
    let count_shutdown = worker.get_active_count().await;
    assert_eq!(count_shutdown, 0);
    
    Ok(())
}
