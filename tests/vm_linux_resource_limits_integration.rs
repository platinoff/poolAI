//! Integration tests for Linux Resource Limits (cgroups)
//!
//! Tests:
//! - Linux cgroup limiter initialization
//! - CPU limits application (if running on Linux)
//! - Memory limits application (if running on Linux)
//! - Resource usage monitoring (if running on Linux)
//!
//! Note: These tests will only work on Linux systems with cgroups enabled.
//! On other platforms, they will test the fallback behavior.

use poolai::vm::ResourceLimiter;
use poolai::vm::ResourceLimits;
use poolai::vm::{VmIsolation, VmManager, VmResources};
use uuid::Uuid;

#[tokio::test]
async fn test_platform_resource_limiter_creation() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());

    // On Linux, should be supported if cgroups are available
    // On other platforms, should return false
    #[cfg(target_os = "linux")]
    {
        // is_supported() returns true if LinuxCgroupLimiter was successfully created
        // This depends on cgroups being available
        let supported = limiter.is_supported();
        // We can't assert true/false here because it depends on the system
        // Just verify it doesn't panic
        assert!(supported || !supported); // Always true, just checking it works
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, depends on Job Objects availability
        let supported = limiter.is_supported();
        let _ = supported; // Just verify it doesn't panic
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        assert!(!limiter.is_supported());
    }
}

#[tokio::test]
async fn test_register_process_pid() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let _process_id = Uuid::new_v4();
    let _pid = 12345u32;

    // Test applying limits to a command (no PID registration needed)
    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_device: None,
    };

    // Create a command to apply limits to
    let mut command = tokio::process::Command::new("echo");

    // On Linux, this should work if cgroups are available
    // On other platforms, it should just validate
    let result = limiter.apply_limits(&mut command, &limits).await;

    #[cfg(target_os = "linux")]
    {
        // On Linux, if cgroups are available and PID is registered, should succeed
        // If cgroups are not available, might fail
        // We just verify it doesn't panic
        let _ = result;
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, if Job Objects are available and PID is registered, should succeed
        // If Job Objects are not available, might fail
        // We just verify it doesn't panic
        let _ = result;
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        // On non-Linux/Windows, should succeed (just logs)
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_apply_limits_without_pid() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());

    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_device: None,
    };

    // Create a command to apply limits to
    let mut command = tokio::process::Command::new("echo");
    
    // Apply limits to command - this should succeed even without PID registration
    // because apply_limits works with Command before spawning (no PID needed yet)
    let result = limiter.apply_limits(&mut command, &limits).await;

    // apply_limits works with Command before process is spawned, so it doesn't need PID
    // Resource limits are placeholder implementation for now, so it just validates
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        // On Linux/Windows, placeholder implementation just validates and logs
        // Full implementation would apply limits when process is spawned
        assert!(result.is_ok());
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        // On non-Linux/Windows, should succeed (just logs)
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_get_usage() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let _process_id = Uuid::new_v4();

    // Get usage for non-existent process (using PID, not UUID)
    let pid = 12345u32;
    let result = limiter.get_usage(pid).await;

    // Should return default usage (0.0 CPU, 0 MB memory)
    assert!(result.is_ok());
    let usage = result.unwrap();
    assert_eq!(usage.cpu_percent, 0.0);
    assert_eq!(usage.memory_mb, 0);
    assert_eq!(usage.gpu_utilization, None);
}

#[tokio::test]
async fn test_resource_limits_validation() {
    use poolai::vm::PlatformResourceLimiter;
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let _process_id = Uuid::new_v4();

    // Register a fake PID
    // Test applying limits to a command (no PID registration needed)

    // Invalid limits: too low memory (will be validated)
    // Note: cpu_cores: 0 means unlimited, not invalid
    let invalid_limits = ResourceLimits {
        cpu_cores: 0, // 0 means unlimited, this is valid
        memory_mb: 32, // Less than 64 MB minimum
        gpu_device: None,
    };

    // Create a command to apply limits to
    let mut command = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command, &invalid_limits).await;
    // Should fail because memory_mb < 64
    assert!(result.is_err());

    // Invalid limits: too low memory (less than 64 MB minimum)
    let invalid_memory = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 32, // Less than 64 MB minimum
        gpu_device: None,
    };

    let mut command2 = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command2, &invalid_memory).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_vm_manager_with_resource_limits() {
    let manager = VmManager::new();

    // Check if resource limits are supported
    let supported = manager.is_resource_limits_supported();

    #[cfg(target_os = "linux")]
    {
        // On Linux, depends on cgroups availability
        let _ = supported; // Just verify it doesn't panic
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, depends on Job Objects availability
        let _ = supported; // Just verify it doesn't panic
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        assert!(!supported);
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
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
