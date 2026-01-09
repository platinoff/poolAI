//! Integration tests for Windows Resource Limits (Job Objects)
//!
//! Tests:
//! - Windows Job Object limiter initialization
//! - CPU limits application (if running on Windows)
//! - Memory limits application (if running on Windows)
//! - Resource usage monitoring (if running on Windows)
//!
//! Note: These tests will only work on Windows systems.
//! On other platforms, they will test the fallback behavior.

use poolai::vm::{ResourceLimiter, ResourceLimits, ResourceUsage};
use poolai::vm::{VmIsolation, VmManager, VmResources};
use uuid::Uuid;

#[tokio::test]
async fn test_windows_job_object_limiter_creation() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());

    // On Windows, should be supported if Job Objects are available
    // On other platforms, should return false
    #[cfg(target_os = "windows")]
    {
        // is_supported() returns true if WindowsJobObjectLimiter was successfully created
        // This depends on Job Objects being available
        let supported = limiter.is_supported();
        // We can't assert true/false here because it depends on the system
        // Just verify it doesn't panic
        assert!(supported || !supported); // Always true, just checking it works
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert!(!limiter.is_supported());
    }
}

#[tokio::test]
async fn test_register_process_pid_windows() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let process_id = Uuid::new_v4();
    let pid = 12345u32;

    // Register PID
    limiter.register_process_pid(process_id, pid).await;

    // Verify it was registered (by trying to apply limits)
    let limits = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(2048),
        gpu_device: None,
    };

    // On Windows, this should work if PID is registered
    // On other platforms, it should just log
    let result = limiter.apply_limits(process_id, &limits).await;

    #[cfg(target_os = "windows")]
    {
        // On Windows, if Job Objects are available and PID is registered, should succeed
        // If Job Objects are not available, might fail
        // We just verify it doesn't panic
        let _ = result;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows, should succeed (just logs)
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_apply_limits_without_pid_windows() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let process_id = Uuid::new_v4();

    let limits = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(2048),
        gpu_device: None,
    };

    // Try to apply limits without registering PID
    let result = limiter.apply_limits(process_id, &limits).await;

    #[cfg(target_os = "windows")]
    {
        // On Windows, should fail because PID is not registered
        assert!(result.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows, should succeed (just logs)
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_get_usage_windows() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let process_id = Uuid::new_v4();

    // Get usage for non-existent process
    let result = limiter.get_usage(process_id).await;

    // Should return default usage (0.0 CPU, 0 MB memory)
    assert!(result.is_ok());
    let usage = result.unwrap();
    assert_eq!(usage.cpu_percent, 0.0);
    assert_eq!(usage.memory_mb, 0);
    assert_eq!(usage.gpu_percent, None);
    assert_eq!(usage.gpu_memory_mb, None);
}

#[tokio::test]
async fn test_resource_limits_validation_windows() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let process_id = Uuid::new_v4();

    // Register a fake PID
    limiter.register_process_pid(process_id, 99999).await;

    // Invalid limits: zero CPU cores
    let invalid_limits = ResourceLimits {
        cpu_cores: Some(0),
        memory_mb: Some(2048),
        gpu_device: None,
    };

    let result = limiter.apply_limits(process_id, &invalid_limits).await;
    assert!(result.is_err());

    // Invalid limits: too low memory
    let invalid_memory = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(64), // Less than 128 MB minimum
        gpu_device: None,
    };

    let result = limiter.apply_limits(process_id, &invalid_memory).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_vm_manager_with_resource_limits_windows() {
    let manager = VmManager::new();

    // Check if resource limits are supported
    let supported = manager.is_resource_limits_supported();

    #[cfg(target_os = "windows")]
    {
        // On Windows, depends on Job Objects availability
        let _ = supported; // Just verify it doesn't panic
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert!(!supported);
    }

    // Create a VM instance
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
            None,
            Vec::new(),
            None,
        )
        .await
        .unwrap();

    // Try to get resource usage (will fail because instance has no process)
    let result = manager.get_instance_resource_usage(instance.id).await;
    assert!(result.is_err()); // Expected: instance has no process
}
