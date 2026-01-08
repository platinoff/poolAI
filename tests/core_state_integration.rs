//! Integration tests for core::state module
//!
//! Tests application state management, worker tracking, and system state operations.

use chrono::Utc;
use poolai::core::state::{
    AppState, SystemMetrics, SystemStatus, Worker, WorkerMetrics, WorkerStatus,
};

#[tokio::test]
async fn test_app_state_creation() {
    let state = AppState::new();
    assert!(!state.is_ready());
}

#[tokio::test]
async fn test_app_state_initialization() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");
    assert!(state.is_ready());
}

#[tokio::test]
async fn test_app_state_cleanup() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");
    state.cleanup().await.expect("Cleanup should succeed");
    assert!(!state.is_ready());
}

#[tokio::test]
async fn test_add_worker() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let worker = Worker {
        id: "worker-1".to_string(),
        address: "127.0.0.1:8080".to_string(),
        mining_power: 100.0,
        status: WorkerStatus::Active,
        last_seen: Utc::now(),
        metrics: WorkerMetrics::default(),
        active_models: vec![],
    };

    state.add_worker(worker.clone()).expect("Should add worker");
    let retrieved = state.get_worker("worker-1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "worker-1");
}

#[tokio::test]
async fn test_remove_worker() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let worker = Worker {
        id: "worker-1".to_string(),
        address: "127.0.0.1:8080".to_string(),
        mining_power: 100.0,
        status: WorkerStatus::Active,
        last_seen: Utc::now(),
        metrics: WorkerMetrics::default(),
        active_models: vec![],
    };

    state.add_worker(worker).expect("Should add worker");
    state
        .remove_worker("worker-1")
        .expect("Should remove worker");
    assert!(state.get_worker("worker-1").is_none());
}

#[tokio::test]
async fn test_update_worker_status() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let worker = Worker {
        id: "worker-1".to_string(),
        address: "127.0.0.1:8080".to_string(),
        mining_power: 100.0,
        status: WorkerStatus::Active,
        last_seen: Utc::now(),
        metrics: WorkerMetrics::default(),
        active_models: vec![],
    };

    state.add_worker(worker).expect("Should add worker");
    state
        .update_worker_status("worker-1", WorkerStatus::Inactive)
        .expect("Should update status");

    let updated = state.get_worker("worker-1").unwrap();
    assert!(matches!(updated.status, WorkerStatus::Inactive));
}

#[tokio::test]
async fn test_update_worker_metrics() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let worker = Worker {
        id: "worker-1".to_string(),
        address: "127.0.0.1:8080".to_string(),
        mining_power: 100.0,
        status: WorkerStatus::Active,
        last_seen: Utc::now(),
        metrics: WorkerMetrics::default(),
        active_models: vec![],
    };

    state.add_worker(worker).expect("Should add worker");

    let new_metrics = WorkerMetrics {
        cpu_utilization: 75.0,
        memory_usage_mb: 1024.0,
        gpu_utilization: 50.0,
        gpu_temperature: 60.0,
        requests_processed: 100,
        avg_processing_time_ms: 250.0,
        error_count: 5,
    };

    state
        .update_worker_metrics("worker-1", new_metrics.clone())
        .expect("Should update metrics");

    let updated = state.get_worker("worker-1").unwrap();
    assert_eq!(updated.metrics.cpu_utilization, 75.0);
    assert_eq!(updated.metrics.memory_usage_mb, 1024.0);
}

#[tokio::test]
async fn test_get_all_workers() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    for i in 1..=5 {
        let worker = Worker {
            id: format!("worker-{}", i),
            address: format!("127.0.0.1:{}", 8080 + i),
            mining_power: 100.0 * i as f64,
            status: WorkerStatus::Active,
            last_seen: Utc::now(),
            metrics: WorkerMetrics::default(),
            active_models: vec![],
        };
        state.add_worker(worker).expect("Should add worker");
    }

    let all_workers = state.get_all_workers();
    assert_eq!(all_workers.len(), 5);
}

#[tokio::test]
async fn test_get_system_state() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let system_state = state.get_system_state();
    assert!(matches!(system_state.status, SystemStatus::Running));
}

#[tokio::test]
async fn test_update_system_metrics() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let metrics = SystemMetrics {
        total_cpu_utilization: 50.0,
        total_memory_usage_mb: 4096.0,
        total_gpu_utilization: 60.0,
        total_requests: 1000,
        avg_latency_ms: 200.0,
        throughput_rps: 50.0,
        error_count: 10,
    };

    state
        .update_system_metrics(metrics.clone())
        .expect("Should update metrics");

    let system_state = state.get_system_state();
    assert_eq!(system_state.system_metrics.total_cpu_utilization, 50.0);
    assert_eq!(system_state.system_metrics.total_memory_usage_mb, 4096.0);
}

#[tokio::test]
async fn test_get_uptime() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    // Wait a bit to ensure some uptime
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let uptime = state.get_uptime();
    assert!(uptime.as_millis() >= 10);
}

#[tokio::test]
async fn test_remove_nonexistent_worker() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let result = state.remove_worker("nonexistent");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_worker_status() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let result = state.update_worker_status("nonexistent", WorkerStatus::Active);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_worker_metrics() {
    let state = AppState::new();
    state
        .initialize()
        .await
        .expect("Initialization should succeed");

    let result = state.update_worker_metrics("nonexistent", WorkerMetrics::default());
    assert!(result.is_err());
}
