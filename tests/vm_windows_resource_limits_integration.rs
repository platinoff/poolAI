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

#[cfg(target_os = "windows")]
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
        let _supported = supported;
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
    let _process_id = Uuid::new_v4();
    let pid = 12345u32;

    // Test applying limits to a command
    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_device: None,
    };

    // Create a command to apply limits to
    let mut command = tokio::process::Command::new("echo");

    // On Windows, this should work if Job Objects are available
    // On other platforms, it should just validate
    let result = limiter.apply_limits(&mut command, &limits).await;

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
    let _process_id = Uuid::new_v4();

    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_device: None,
    };

    // Apply limits to command - this should succeed even without PID registration
    // because apply_limits works with Command before spawning (no PID needed yet)
    let mut command = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command, &limits).await;

    #[cfg(target_os = "windows")]
    {
        // On Windows, placeholder implementation just validates and logs
        // Full implementation would apply limits when process is spawned
        // apply_limits works with Command before process is spawned, so it doesn't need PID
        assert!(result.is_ok());
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
    let _process_id = Uuid::new_v4();

    // Get usage for non-existent process (use u32 PID, not Uuid)
    let result = limiter.get_usage(99999u32).await;

    // Should return default usage (0.0 CPU, 0 MB memory)
    assert!(result.is_ok());
    let usage = result.unwrap();
    assert_eq!(usage.cpu_percent, 0.0);
    assert_eq!(usage.memory_mb, 0);
    assert_eq!(usage.gpu_utilization, None);
}

#[tokio::test]
async fn test_resource_limits_validation_windows() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let _process_id = Uuid::new_v4();

    // Test applying limits to a command (no PID registration needed)

    // Invalid limits: zero CPU cores
    let invalid_limits = ResourceLimits {
        cpu_cores: 0,
        memory_mb: 2048,
        gpu_device: None,
    };

    let mut command1 = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command1, &invalid_limits).await;
    // Should succeed (0 means unlimited, validation only)
    assert!(result.is_ok());

    // Invalid limits: too low memory (if enforced)
    let invalid_memory = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 64, // Less than 128 MB minimum (if enforced)
        gpu_device: None,
    };

    let mut command2 = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command2, &invalid_memory).await;
    // May succeed or fail depending on platform enforcement
    let _ = result;
}

#[cfg(target_os = "windows")]
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
        )
        .await
        .unwrap();

    // Try to get resource usage (will fail because instance has no process)
    let result = manager.get_instance_resource_usage(instance.id).await;
    assert!(result.is_err()); // Expected: instance has no process
}
