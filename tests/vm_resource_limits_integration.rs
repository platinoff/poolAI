//! Integration tests for VM Resource Limits
//!
//! Tests:
//! - Resource limits validation
//! - Resource limits application to commands
//! - Resource limits support detection
//! - Resource usage query (stub)

use poolai::vm::{
    PlatformResourceLimiter, ResourceLimiter, ResourceLimits, VmIsolation, VmManager, VmResources,
};
use tokio::process::Command;

#[tokio::test]
async fn test_resource_limits_from_vm_resources() {
    let resources = VmResources {
        cpu_cores: 4,
        memory_mb: 4096,
        gpu_required: true,
        gpu_scheduling_policy: None,
    };

    let limits = ResourceLimits::from(resources);

    assert_eq!(limits.cpu_cores, 4);
    assert_eq!(limits.memory_mb, 4096);
    assert_eq!(limits.gpu_device, Some(0));
}

#[tokio::test]
async fn test_resource_limits_default() {
    let limits = ResourceLimits::default();

    assert_eq!(limits.cpu_cores, 0); // Unlimited
    assert_eq!(limits.memory_mb, 0); // Unlimited
    assert_eq!(limits.gpu_device, None);
}

#[tokio::test]
async fn test_resource_limits_validation() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    // Create instance with valid limits
    let resources = VmResources {
        cpu_cores: 2,
        memory_mb: 1024,
        gpu_required: false,
        gpu_scheduling_policy: None,
    };

    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Test applying resource limits
    let mut command = Command::new("echo");
    let result = manager
        .apply_resource_limits(&mut command, instance.id)
        .await;

    // Should not error (validation only for now)
    assert!(result.is_ok(), "Valid limits should be accepted");
}

#[tokio::test]
async fn test_resource_limits_too_low_memory() {
    // Test with memory limit too low - direct ResourceLimits usage
    let limiter = PlatformResourceLimiter::new();
    let mut command = Command::new("echo");
    let limits = ResourceLimits {
        cpu_cores: 1,
        memory_mb: 32, // Too low (minimum 64 MB)
        gpu_device: None,
    };

    // Should error on validation
    let result = limiter.apply_limits(&mut command, &limits).await;
    assert!(result.is_err(), "Memory limit too low should be rejected");
}

#[tokio::test]
async fn test_resource_limits_support_detection() {
    let manager = VmManager::new();

    // Check if resource limits are supported on this platform
    let supported = manager.is_resource_limits_supported();

    // Should be true on Windows/Linux, false on other platforms
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    assert!(
        supported,
        "Resource limits should be supported on Windows/Linux"
    );

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    assert!(
        !supported,
        "Resource limits should not be supported on other platforms"
    );
}

#[tokio::test]
async fn test_apply_resource_limits_to_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_required: false,
        gpu_scheduling_policy: None,
    };

    let instance = manager
        .create_instance(
            "test-vm".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Test applying resource limits to a command
    let mut command = Command::new("echo");
    let result = manager
        .apply_resource_limits(&mut command, instance.id)
        .await;

    // Should succeed (validation only for now)
    assert!(result.is_ok(), "Applying resource limits should succeed");
}

#[tokio::test]
async fn test_get_instance_resource_usage_not_implemented() {
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

    // Resource usage query should fail (process spawning not yet implemented)
    let result = manager.get_instance_resource_usage(instance.id).await;
    assert!(
        result.is_err(),
        "Resource usage query should fail until process spawning is implemented"
    );
}
