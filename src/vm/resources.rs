//! Resource Limits Enforcement for VM instances
//!
//! Platform-specific implementations:
//! - Windows: Job Objects (TODO)
//! - Linux: cgroups v2 (TODO)
//! - Cross-platform: Basic validation and monitoring (current)

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;
use tracing::warn;

/// Resource limits configuration for a VM instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU cores limit (0 = unlimited)
    pub cpu_cores: u16,
    /// Memory limit in MB (0 = unlimited)
    pub memory_mb: u32,
    /// GPU device ID (None = no GPU)
    pub gpu_device: Option<usize>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_cores: 0, // Unlimited
            memory_mb: 0, // Unlimited
            gpu_device: None,
        }
    }
}

impl From<crate::vm::VmResources> for ResourceLimits {
    fn from(resources: crate::vm::VmResources) -> Self {
        Self {
            cpu_cores: resources.cpu_cores,
            memory_mb: resources.memory_mb,
            gpu_device: if resources.gpu_required { Some(0) } else { None },
        }
    }
}

/// Resource limiter trait for platform-specific implementations
#[async_trait::async_trait]
pub trait ResourceLimiter: Send + Sync {
    /// Apply resource limits to a command before spawning
    async fn apply_limits(
        &self,
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError>;

    /// Get current resource usage for a process
    async fn get_usage(&self, process_id: u32) -> Result<ResourceUsage, AppError>;

    /// Check if limits are enforced (platform support)
    fn is_supported(&self) -> bool;
}

/// Current resource usage for a process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_mb: u32,
    pub gpu_utilization: Option<f32>,
}

/// Platform-specific resource limiter implementation
pub struct PlatformResourceLimiter;

impl PlatformResourceLimiter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ResourceLimiter for PlatformResourceLimiter {
    async fn apply_limits(
        &self,
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // Platform-specific implementation
        #[cfg(target_os = "windows")]
        {
            windows::apply_windows_limits(command, limits).await
        }
        
        #[cfg(target_os = "linux")]
        {
            linux::apply_linux_limits(command, limits).await
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            // Fallback: basic validation only
            if limits.cpu_cores > 0 || limits.memory_mb > 0 {
                warn!("Resource limits not supported on this platform, only validation will be performed");
            }
            Ok(())
        }
    }

    async fn get_usage(&self, process_id: u32) -> Result<ResourceUsage, AppError> {
        #[cfg(target_os = "windows")]
        {
            windows::get_windows_usage(process_id).await
        }
        
        #[cfg(target_os = "linux")]
        {
            linux::get_linux_usage(process_id).await
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Err(AppError::ConfigError(
                "Resource usage monitoring not supported on this platform".to_string(),
            ))
        }
    }

    fn is_supported(&self) -> bool {
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            true
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            false
        }
    }
}

// Platform-specific modules (stubs for now)

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use tracing::warn;

    pub async fn apply_windows_limits(
        _command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // TODO: Implement Windows Job Objects for CPU/memory limits
        // Requires windows crate or winapi crate
        if limits.cpu_cores > 0 || limits.memory_mb > 0 {
            warn!(
                "Windows resource limits not yet implemented (CPU: {}, Memory: {} MB)",
                limits.cpu_cores, limits.memory_mb
            );
            // For now, just validate
            if limits.memory_mb > 0 && limits.memory_mb < 64 {
                return Err(AppError::ValidationError(
                    "Memory limit too low (minimum 64 MB)".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub async fn get_windows_usage(_process_id: u32) -> Result<ResourceUsage, AppError> {
        // TODO: Implement Windows process resource usage query
        // Requires windows crate or winapi crate
        Ok(ResourceUsage {
            cpu_percent: 0.0,
            memory_mb: 0,
            gpu_utilization: None,
        })
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use tracing::warn;

    pub async fn apply_linux_limits(
        _command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // TODO: Implement Linux cgroups v2 for CPU/memory limits
        // Requires cgroups-rs or manual cgroup manipulation
        if limits.cpu_cores > 0 || limits.memory_mb > 0 {
            warn!(
                "Linux resource limits not yet implemented (CPU: {}, Memory: {} MB)",
                limits.cpu_cores, limits.memory_mb
            );
            // For now, just validate
            if limits.memory_mb > 0 && limits.memory_mb < 64 {
                return Err(AppError::ValidationError(
                    "Memory limit too low (minimum 64 MB)".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub async fn get_linux_usage(_process_id: u32) -> Result<ResourceUsage, AppError> {
        // TODO: Implement Linux process resource usage query
        // Requires reading from /proc/{pid}/stat and /proc/{pid}/status
        Ok(ResourceUsage {
            cpu_percent: 0.0,
            memory_mb: 0,
            gpu_utilization: None,
        })
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
mod fallback {
    use super::*;
    use tracing::warn;

    pub async fn apply_fallback_limits(
        _command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        warn!("Resource limits not supported on this platform");
        if limits.memory_mb > 0 && limits.memory_mb < 64 {
            return Err(AppError::ValidationError(
                "Memory limit too low (minimum 64 MB)".to_string(),
            ));
        }
        Ok(())
    }
}

