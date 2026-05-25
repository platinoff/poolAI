//! Resource limits enforcement for VM instances.
//!
//! Pre-spawn: validation (and Linux env hints). Post-spawn: platform limiters
//! (Windows Job Objects, Linux cgroups) when PID is available.

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::LinuxCgroupLimiter;
#[cfg(target_os = "windows")]
pub use windows::{JobObjectState, WindowsJobObjectLimiter};

/// Minimum enforced memory limit (MB) when a finite limit is set.
pub const MIN_MEMORY_LIMIT_MB: u32 = 64;

/// Resource limits configuration for a VM instance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// CPU cores limit (0 = unlimited)
    pub cpu_cores: u16,
    /// Memory limit in MB (0 = unlimited)
    pub memory_mb: u32,
    /// GPU device ID (None = no GPU)
    pub gpu_device: Option<usize>,
}

impl From<crate::vm::VmResources> for ResourceLimits {
    fn from(resources: crate::vm::VmResources) -> Self {
        Self {
            cpu_cores: resources.cpu_cores,
            memory_mb: resources.memory_mb,
            gpu_device: if resources.gpu_required {
                Some(0)
            } else {
                None
            },
        }
    }
}

/// Validate limit values (shared pre/post spawn).
pub fn validate_resource_limits(limits: &ResourceLimits) -> Result<(), AppError> {
    if limits.memory_mb > 0 && limits.memory_mb < MIN_MEMORY_LIMIT_MB {
        return Err(AppError::ValidationError(format!(
            "Memory limit too low (minimum {MIN_MEMORY_LIMIT_MB} MB)"
        )));
    }
    Ok(())
}

/// Current resource usage for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_mb: u32,
    pub gpu_utilization: Option<f32>,
}

/// Platform-specific resource limiter.
#[async_trait::async_trait]
pub trait ResourceLimiter: Send + Sync {
    /// Apply resource limits to a command before spawning (validation / env hints).
    async fn apply_limits(
        &self,
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError>;

    /// Apply resource limits after spawn when PID is known.
    async fn apply_limits_post_spawn(
        &self,
        process_id: uuid::Uuid,
        pid: u32,
        limits: &ResourceLimits,
    ) -> Result<(), AppError>;

    async fn get_usage(&self, process_id: u32) -> Result<ResourceUsage, AppError>;

    fn is_supported(&self) -> bool;
}

/// Default limiter — delegates to OS-specific backends.
pub struct PlatformResourceLimiter {
    #[cfg(target_os = "windows")]
    windows: std::sync::Arc<WindowsJobObjectLimiter>,
    #[cfg(target_os = "linux")]
    linux: std::sync::Arc<LinuxCgroupLimiter>,
}

impl Default for PlatformResourceLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformResourceLimiter {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            windows: std::sync::Arc::new(
                WindowsJobObjectLimiter::new().expect("Windows Job Object limiter"),
            ),
            #[cfg(target_os = "linux")]
            linux: std::sync::Arc::new(LinuxCgroupLimiter::new().unwrap_or_else(|e| {
                tracing::warn!("Linux cgroup limiter unavailable: {e}");
                LinuxCgroupLimiter::disabled()
            })),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn windows_limiter(&self) -> &WindowsJobObjectLimiter {
        &self.windows
    }
}

#[async_trait::async_trait]
impl ResourceLimiter for PlatformResourceLimiter {
    async fn apply_limits(
        &self,
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        validate_resource_limits(limits)?;

        #[cfg(target_os = "linux")]
        {
            linux::apply_linux_pre_spawn(command, limits).await?;
        }

        #[cfg(target_os = "windows")]
        {
            let _ = command;
            // Limits are enforced post-spawn via Job Objects.
        }

        Ok(())
    }

    async fn apply_limits_post_spawn(
        &self,
        process_id: uuid::Uuid,
        pid: u32,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        #[cfg(target_os = "windows")]
        {
            return self.windows.apply_limits(process_id, pid, limits).await;
        }

        #[cfg(target_os = "linux")]
        {
            if self.linux.is_available() {
                return self.linux.apply_limits(process_id, pid, limits).await;
            }
            validate_resource_limits(limits)?;
            let _ = (process_id, pid);
            return Ok(());
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            validate_resource_limits(limits)?;
            let _ = (process_id, pid);
            Ok(())
        }
    }

    async fn get_usage(&self, process_id: u32) -> Result<ResourceUsage, AppError> {
        #[cfg(target_os = "windows")]
        {
            return Ok(ResourceUsage {
                cpu_percent: 0.0,
                memory_mb: 0,
                gpu_utilization: None,
            });
        }

        #[cfg(target_os = "linux")]
        {
            let _ = process_id;
            return Ok(ResourceUsage {
                cpu_percent: 0.0,
                memory_mb: 0,
                gpu_utilization: None,
            });
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
