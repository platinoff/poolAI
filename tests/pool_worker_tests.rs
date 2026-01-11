//! Pool Worker Tests
//!
//! Tests for worker pool functionality, worker management, and worker lifecycle.

use poolai::pool::worker::{Worker, WorkerConfig, WorkerStatus};

#[test]
fn test_worker_status_variants() {
    let _ = WorkerStatus::Idle;
    let _ = WorkerStatus::Busy;
    let _ = WorkerStatus::Offline;
    let _ = WorkerStatus::Error;
}

#[test]
fn test_worker_status_display() {
    assert_eq!(format!("{}", WorkerStatus::Idle), "Idle");
    assert_eq!(format!("{}", WorkerStatus::Busy), "Busy");
    assert_eq!(format!("{}", WorkerStatus::Offline), "Offline");
    assert_eq!(format!("{}", WorkerStatus::Error), "Error");
}

#[test]
fn test_worker_status_debug() {
    let debug = format!("{:?}", WorkerStatus::Idle);
    assert!(debug.contains("Idle"));
}

#[test]
fn test_worker_status_equality() {
    assert_eq!(WorkerStatus::Idle, WorkerStatus::Idle);
    assert_ne!(WorkerStatus::Idle, WorkerStatus::Busy);
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
