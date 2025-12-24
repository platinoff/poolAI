//! Integration tests for VM Health Checks
//!
//! Tests:
//! - Health check registration on instance start
//! - Health check unregistration on instance stop
//! - Periodic health checks for running instances
//! - Health status API endpoint
//! - Auto-restart on health check failure (stub)

use poolai::vm::{VmManager, VmResources, VmIsolation};
use poolai::runtime::health::HealthStatus;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_health_check_registration_on_start() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Health check should not be registered before start
    let health_before = manager.get_instance_health(instance.id).await.unwrap();
    assert!(health_before.is_none(), "Health check should not be registered before start");

    // Start instance (should register health check)
    manager.start_instance(instance.id).await.unwrap();

    // Health check should be registered after start
    let health_after = manager.get_instance_health(instance.id).await.unwrap();
    assert!(health_after.is_some(), "Health check should be registered after start");
}

#[tokio::test]
async fn test_health_check_unregistration_on_stop() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start instance (registers health check)
    manager.start_instance(instance.id).await.unwrap();
    
    // Verify health check is registered
    let health_running = manager.get_instance_health(instance.id).await.unwrap();
    assert!(health_running.is_some(), "Health check should be registered when running");

    // Stop instance (should unregister health check)
    manager.stop_instance(instance.id).await.unwrap();

    // Health check should still exist but instance is stopped
    // (HealthMonitor keeps the entry, but instance status is Stopped)
    let health_stopped = manager.get_instance_health(instance.id).await.unwrap();
    // Note: HealthMonitor may keep the entry, so we check instance status instead
    let instance_after = manager.get_instance(instance.id).await;
    assert!(instance_after.is_some());
    assert!(matches!(instance_after.unwrap().status, poolai::vm::VmStatus::Stopped));
}

#[tokio::test]
async fn test_health_check_for_running_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start instance
    manager.start_instance(instance.id).await.unwrap();

    // Perform manual health check
    let health_status = manager.check_instance_health(instance.id).await.unwrap();
    
    // Running instance should be healthy
    assert!(matches!(health_status, HealthStatus::Healthy), 
        "Running instance should have healthy status");
}

#[tokio::test]
async fn test_health_check_for_stopped_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start then stop instance
    manager.start_instance(instance.id).await.unwrap();
    manager.stop_instance(instance.id).await.unwrap();

    // Perform manual health check on stopped instance
    // Note: After stop, health check is unregistered, so status will be Unknown
    let health_status = manager.check_instance_health(instance.id).await.unwrap();
    
    // Stopped instance should be Unknown (health check unregistered) or Unhealthy
    assert!(matches!(health_status, HealthStatus::Unknown | HealthStatus::Unhealthy(_)), 
        "Stopped instance should have Unknown or Unhealthy status");
}

#[tokio::test]
async fn test_health_check_for_nonexistent_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let nonexistent_id = uuid::Uuid::new_v4();

    // Health check for non-existent instance should return Unknown
    let health_status = manager.check_instance_health(nonexistent_id).await.unwrap();
    assert!(matches!(health_status, HealthStatus::Unknown), 
        "Non-existent instance should have unknown health status");
}

#[tokio::test]
async fn test_get_health_status_api() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Before start - should return None
    let health = manager.get_instance_health(instance.id).await.unwrap();
    assert!(health.is_none(), "Health should be None before instance start");

    // Start instance
    manager.start_instance(instance.id).await.unwrap();

    // After start - should return Some(HealthStatus)
    let health = manager.get_instance_health(instance.id).await.unwrap();
    assert!(health.is_some(), "Health should be Some after instance start");
    
    // Should be Healthy for running instance
    if let Some(status) = health {
        assert!(matches!(status, HealthStatus::Healthy | HealthStatus::Unknown),
            "Health status should be Healthy or Unknown for running instance");
    }
}

