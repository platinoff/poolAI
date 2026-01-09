//! Pool Worker Tests
//!
//! Tests for worker pool functionality, worker management, and worker lifecycle.

use poolai::pool::worker::{Worker, WorkerStatus};

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
    let worker = Worker {
        id: "test-worker".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    assert_eq!(worker.id, "test-worker");
    assert_eq!(worker.status, WorkerStatus::Idle);
}

#[test]
fn test_worker_with_gpu() {
    let worker = Worker {
        id: "gpu-worker".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(1),
        current_task: None,
    };
    assert_eq!(worker.gpu_id, Some(1));
}

#[test]
fn test_worker_without_gpu() {
    let worker = Worker {
        id: "cpu-worker".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: None,
        current_task: None,
    };
    assert_eq!(worker.gpu_id, None);
}

#[test]
fn test_worker_with_task() {
    let worker = Worker {
        id: "busy-worker".to_string(),
        status: WorkerStatus::Busy,
        gpu_id: Some(0),
        current_task: Some("task-123".to_string()),
    };
    assert_eq!(worker.status, WorkerStatus::Busy);
    assert_eq!(worker.current_task, Some("task-123".to_string()));
}

#[test]
fn test_worker_status_transitions() {
    let mut worker = Worker {
        id: "worker".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: None,
        current_task: None,
    };
    
    worker.status = WorkerStatus::Busy;
    assert_eq!(worker.status, WorkerStatus::Busy);
    
    worker.status = WorkerStatus::Idle;
    assert_eq!(worker.status, WorkerStatus::Idle);
}

#[test]
fn test_worker_debug_format() {
    let worker = Worker {
        id: "test".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    let debug = format!("{:?}", worker);
    assert!(debug.contains("test"));
    assert!(debug.contains("Idle"));
}

#[test]
fn test_worker_clone() {
    let worker = Worker {
        id: "test".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    let cloned = worker.clone();
    assert_eq!(worker.id, cloned.id);
    assert_eq!(worker.status, cloned.status);
}

#[test]
fn test_worker_equality() {
    let worker1 = Worker {
        id: "test".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    let worker2 = Worker {
        id: "test".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    assert_eq!(worker1, worker2);
}

#[test]
fn test_worker_inequality() {
    let worker1 = Worker {
        id: "test1".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    let worker2 = Worker {
        id: "test2".to_string(),
        status: WorkerStatus::Idle,
        gpu_id: Some(0),
        current_task: None,
    };
    assert_ne!(worker1, worker2);
}
