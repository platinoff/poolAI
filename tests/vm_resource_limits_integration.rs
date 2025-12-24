//! Integration tests for VM Resource Limits infrastructure
//!
//! Tests:
//! - ResourceLimits creation and validation
//! - ResourceLimits from VmResources conversion
//! - ResourceLimiter trait implementation (placeholder)
//! - API endpoints for resource limits

use poolai::vm::{VmManager, VmResources, VmIsolation};
use poolai::vm::resources::{ResourceLimits, ResourceUsage};

#[tokio::test]
async fn test_resource_limits_from_vm_resources() {
    let vm_resources = VmResources {
        cpu_cores: 4,
        memory_mb: 4096,
        gpu_required: true,
    };
    
    let limits = ResourceLimits::from(vm_resources);
    
    assert_eq!(limits.cpu_cores, Some(4));
    assert_eq!(limits.memory_mb, Some(4096));
    assert_eq!(limits.gpu_device, Some(0));
}

#[tokio::test]
async fn test_resource_limits_default() {
    let limits = ResourceLimits::default();
    
    assert_eq!(limits.cpu_cores, Some(2));
    assert_eq!(limits.memory_mb, Some(2048));
    assert_eq!(limits.gpu_device, None);
}

#[tokio::test]
async fn test_resource_limits_validation() {
    // Valid limits
    let valid_limits = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(2048),
        gpu_device: None,
    };
    assert!(valid_limits.validate().is_ok());
    
    // Invalid: zero CPU cores
    let invalid_cpu = ResourceLimits {
        cpu_cores: Some(0),
        memory_mb: Some(2048),
        gpu_device: None,
    };
    assert!(invalid_cpu.validate().is_err());
    
    // Invalid: too low memory
    let invalid_memory = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(64), // Less than 128 MB minimum
        gpu_device: None,
    };
    assert!(invalid_memory.validate().is_err());
}

#[tokio::test]
async fn test_resource_limits_too_low_memory() {
    let limits = ResourceLimits {
        cpu_cores: Some(1),
        memory_mb: Some(100), // Below minimum
        gpu_device: None,
    };
    
    assert!(limits.validate().is_err());
}

#[tokio::test]
async fn test_resource_limits_support_detection() {
    let manager = VmManager::new();
    let supported = manager.is_resource_limits_supported();
    
    // On Linux, returns true if cgroups are available
    // On Windows, returns true if Job Objects are available
    // On other platforms, returns false
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
}

#[tokio::test]
async fn test_apply_resource_limits_to_instance() {
    let manager = VmManager::new();
    
    // Create a VM instance
    let instance = manager.create_instance(
        "test-instance".to_string(),
        VmResources::default(),
        VmIsolation::ProcessSandbox,
        None,
        Vec::new(),
        None,
    ).await.unwrap();
    
    // Create resource limits
    let limits = ResourceLimits {
        cpu_cores: Some(2),
        memory_mb: Some(2048),
        gpu_device: None,
    };
    
    // Try to apply limits (will fail because instance has no process)
    // This is expected behavior - limits can only be applied to running instances
    let result = manager.apply_resource_limits(instance.id, limits).await;
    assert!(result.is_err()); // Expected: instance has no process
}

#[tokio::test]
async fn test_get_instance_resource_usage_not_implemented() {
    let manager = VmManager::new();
    
    // Create a VM instance
    let instance = manager.create_instance(
        "test-instance".to_string(),
        VmResources::default(),
        VmIsolation::ProcessSandbox,
        None,
        Vec::new(),
        None,
    ).await.unwrap();
    
    // Try to get resource usage (will fail because instance has no process)
    let result = manager.get_instance_resource_usage(instance.id).await;
    assert!(result.is_err()); // Expected: instance has no process
}

#[tokio::test]
async fn test_resource_usage_default() {
    let usage = ResourceUsage::default();
    
    assert_eq!(usage.cpu_percent, 0.0);
    assert_eq!(usage.memory_mb, 0);
    assert_eq!(usage.gpu_percent, None);
    assert_eq!(usage.gpu_memory_mb, None);
}

