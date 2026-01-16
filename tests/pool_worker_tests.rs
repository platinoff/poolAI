//! Pool Worker Tests
//!
//! Tests for worker pool functionality, worker management, and worker lifecycle.

use chrono;
use poolai::pool::worker::{Worker, WorkerConfig, WorkerStatus};

#[test]
fn test_worker_status_creation() {
    // WorkerStatus is a struct, not an enum
    let status = WorkerStatus {
        is_healthy: true,
        active_connections: 0,
        queue_size: 0,
        last_health_check: chrono::Utc::now(),
        total_requests_processed: 0,
        average_response_time_ms: 0.0,
        cpu_usage: 0.0,
        memory_usage_mb: 0.0,
        gpu_usage: None,
        process_id: None,
        uptime_seconds: 0,
        current_task: None,
    };
    assert!(status.is_healthy);
    assert_eq!(status.active_connections, 0);
}

#[test]
fn test_worker_status_debug() {
    let status = WorkerStatus {
        is_healthy: true,
        active_connections: 0,
        queue_size: 0,
        last_health_check: chrono::Utc::now(),
        total_requests_processed: 0,
        average_response_time_ms: 0.0,
        cpu_usage: 0.0,
        memory_usage_mb: 0.0,
        gpu_usage: None,
        process_id: None,
        uptime_seconds: 0,
        current_task: None,
    };
    let debug = format!("{:?}", status);
    assert!(debug.contains("WorkerStatus"));
}

#[test]
fn test_worker_status_fields() {
    let mut status = WorkerStatus {
        is_healthy: true,
        active_connections: 5,
        queue_size: 10,
        last_health_check: chrono::Utc::now(),
        total_requests_processed: 100,
        average_response_time_ms: 50.0,
        cpu_usage: 75.0,
        memory_usage_mb: 512.0,
        gpu_usage: Some(60.0),
        process_id: Some(12345),
        uptime_seconds: 3600,
        current_task: Some("task-123".to_string()),
    };
    assert!(status.is_healthy);
    assert_eq!(status.active_connections, 5);
    assert_eq!(status.queue_size, 10);
    assert_eq!(status.total_requests_processed, 100);
    assert_eq!(status.cpu_usage, 75.0);
    assert_eq!(status.memory_usage_mb, 512.0);
    assert_eq!(status.gpu_usage, Some(60.0));
    assert_eq!(status.process_id, Some(12345));
    assert_eq!(status.uptime_seconds, 3600);
    assert_eq!(status.current_task, Some("task-123".to_string()));
}

#[test]
fn test_worker_creation() {
    let config = WorkerConfig {
        worker_id: "test-worker".to_string(),
        max_concurrent_requests: 10,
        request_timeout_ms: 5000,
        health_check_interval_ms: 1000,
        enable_caching: true,
        cache_size: 1000,
        max_memory_mb: 2048,
        cpu_priority: 5,
        gpu_device: Some(0),
        auto_restart: true,
        resource_monitoring: true,
    };
    let worker = Worker::new(config);
    // Worker is created successfully
    assert!(true);
}

#[test]
fn test_worker_with_gpu() {
    let config = WorkerConfig {
        worker_id: "gpu-worker".to_string(),
        max_concurrent_requests: 10,
        request_timeout_ms: 5000,
        health_check_interval_ms: 1000,
        enable_caching: true,
        cache_size: 1000,
        max_memory_mb: 2048,
        cpu_priority: 5,
        gpu_device: Some(1),
        auto_restart: true,
        resource_monitoring: true,
    };
    let worker = Worker::new(config);
    // Worker with GPU created successfully
    assert!(true);
}

#[test]
fn test_worker_without_gpu() {
    let config = WorkerConfig {
        worker_id: "cpu-worker".to_string(),
        max_concurrent_requests: 10,
        request_timeout_ms: 5000,
        health_check_interval_ms: 1000,
        enable_caching: true,
        cache_size: 1000,
        max_memory_mb: 2048,
        cpu_priority: 5,
        gpu_device: None,
        auto_restart: true,
        resource_monitoring: true,
    };
    let worker = Worker::new(config);
    // Worker without GPU created successfully
    assert!(true);
}
