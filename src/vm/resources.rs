//! Resource Limits and Management for VM instances
//!
//! Provides:
//! - Resource limits definition (CPU, memory, GPU)
//! - Resource usage monitoring
//! - Platform-specific resource limiting (trait-based)
//! - Integration with VM Manager

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Resource limits for a VM instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum CPU cores (None = unlimited)
    pub cpu_cores: Option<u16>,
    /// Maximum memory in MB (None = unlimited)
    pub memory_mb: Option<u32>,
    /// GPU device ID (None = no GPU required)
    pub gpu_device: Option<u32>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_cores: Some(2),
            memory_mb: Some(2048),
            gpu_device: None,
        }
    }
}

impl From<crate::vm::VmResources> for ResourceLimits {
    fn from(resources: crate::vm::VmResources) -> Self {
        Self {
            cpu_cores: Some(resources.cpu_cores),
            memory_mb: Some(resources.memory_mb),
            gpu_device: if resources.gpu_required { Some(0) } else { None },
        }
    }
}

impl ResourceLimits {
    /// Validate resource limits
    pub fn validate(&self) -> Result<(), AppError> {
        if let Some(cpu) = self.cpu_cores {
            if cpu == 0 {
                return Err(AppError::ValidationError(
                    "CPU cores must be greater than 0".to_string(),
                ));
            }
        }
        
        if let Some(memory) = self.memory_mb {
            if memory < 128 {
                return Err(AppError::ValidationError(
                    "Memory must be at least 128 MB".to_string(),
                ));
            }
        }
        
        Ok(())
    }
}

/// Current resource usage for a process/VM instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Current CPU usage percentage (0-100)
    pub cpu_percent: f64,
    /// Current memory usage in MB
    pub memory_mb: u32,
    /// GPU usage percentage (0-100, if GPU is used)
    pub gpu_percent: Option<f64>,
    /// GPU memory usage in MB (if GPU is used)
    pub gpu_memory_mb: Option<u32>,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0,
            gpu_percent: None,
            gpu_memory_mb: None,
        }
    }
}

/// Trait for platform-specific resource limiting
#[async_trait::async_trait]
pub trait ResourceLimiter: Send + Sync {
    /// Apply resource limits to a process
    /// 
    /// # Arguments
    /// * `process_id` - Process ID (platform-specific)
    /// * `limits` - Resource limits to apply
    /// 
    /// # Returns
    /// `Ok(())` if limits were applied successfully, `Err` otherwise
    async fn apply_limits(&self, process_id: Uuid, limits: &ResourceLimits) -> Result<(), AppError>;
    
    /// Get current resource usage for a process
    /// 
    /// # Arguments
    /// * `process_id` - Process ID (platform-specific)
    /// 
    /// # Returns
    /// Current resource usage, or `Err` if unable to retrieve
    async fn get_usage(&self, process_id: Uuid) -> Result<ResourceUsage, AppError>;
    
    /// Check if resource limiting is supported on this platform
    /// 
    /// # Returns
    /// `true` if resource limiting is supported, `false` otherwise
    fn is_supported(&self) -> bool;
}

/// Platform-specific resource limiter implementation
/// 
/// This is a placeholder that will be implemented in Week 3-4:
/// - Week 3: Linux implementation using cgroups
/// - Week 4: Windows implementation using Job Objects
pub struct PlatformResourceLimiter {
    /// Platform-specific implementation state
    #[allow(dead_code)]
    platform: PlatformType,
}

#[derive(Debug, Clone, Copy)]
enum PlatformType {
    Linux,
    Windows,
    Unknown,
}

impl PlatformResourceLimiter {
    /// Create a new platform resource limiter
    pub fn new() -> Self {
        let platform = if cfg!(target_os = "linux") {
            PlatformType::Linux
        } else if cfg!(target_os = "windows") {
            PlatformType::Windows
        } else {
            PlatformType::Unknown
        };
        
        Self { platform }
    }
}

#[async_trait::async_trait]
impl ResourceLimiter for PlatformResourceLimiter {
    async fn apply_limits(&self, _process_id: Uuid, limits: &ResourceLimits) -> Result<(), AppError> {
        // Validate limits first
        limits.validate()?;
        
        // Placeholder implementation - will be implemented in Week 3-4
        // For now, just log the limits
        tracing::info!(
            "Resource limits requested (not yet enforced): cpu={:?}, memory={:?}MB, gpu={:?}",
            limits.cpu_cores,
            limits.memory_mb,
            limits.gpu_device
        );
        
        // TODO: Week 3-4 - Implement platform-specific resource limiting
        // Linux: Use cgroups
        // Windows: Use Job Objects
        
        Ok(())
    }
    
    async fn get_usage(&self, _process_id: Uuid) -> Result<ResourceUsage, AppError> {
        // Placeholder implementation - will be implemented in Week 3-4
        // For now, return default (zero usage)
        
        // TODO: Week 3-4 - Implement platform-specific resource usage monitoring
        // Linux: Read from /proc or cgroup stats
        // Windows: Use performance counters or WMI
        
        Ok(ResourceUsage::default())
    }
    
    fn is_supported(&self) -> bool {
        // Will return true once platform-specific implementation is complete (Week 3-4)
        // For now, return false to indicate it's not yet implemented
        false
    }
}

impl Default for PlatformResourceLimiter {
    fn default() -> Self {
        Self::new()
    }
}

