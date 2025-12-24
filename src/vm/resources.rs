//! Resource Limits and Management for VM instances
//!
//! Provides:
//! - Resource limits definition (CPU, memory, GPU)
//! - Resource usage monitoring
//! - Platform-specific resource limiting (trait-based)
//! - Integration with VM Manager

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

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
    
    /// Register a process PID (optional - only needed for some platforms like Linux)
    /// 
    /// # Arguments
    /// * `process_id` - Process ID (UUID)
    /// * `pid` - Platform-specific process ID (PID)
    /// 
    /// Default implementation does nothing (for platforms that don't need PID registration)
    async fn register_process_pid(&self, _process_id: Uuid, _pid: u32) {
        // Default: no-op (for platforms that don't need PID registration)
    }
    
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
    /// Linux cgroup limiter (only on Linux)
    #[cfg(target_os = "linux")]
    linux_limiter: Option<linux::LinuxCgroupLimiter>,
    /// Windows Job Object limiter (only on Windows)
    #[cfg(target_os = "windows")]
    windows_limiter: Option<windows::WindowsJobObjectLimiter>,
    /// Mapping from process_id (Uuid) to PID
    /// This is needed because apply_limits only receives process_id, not PID
    process_pid_map: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<Uuid, u32>>>,
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
        
        #[cfg(target_os = "linux")]
        let linux_limiter: Option<linux::LinuxCgroupLimiter> = linux::LinuxCgroupLimiter::new().ok();
        
        #[cfg(not(target_os = "linux"))]
        let _linux_limiter: Option<()> = None;

        #[cfg(target_os = "windows")]
        let windows_limiter: Option<windows::WindowsJobObjectLimiter> = windows::WindowsJobObjectLimiter::new().ok();
        
        #[cfg(not(target_os = "windows"))]
        let _windows_limiter: Option<()> = None;
        
        Self {
            platform,
            #[cfg(target_os = "linux")]
            linux_limiter,
            #[cfg(target_os = "windows")]
            windows_limiter,
            process_pid_map: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }


    /// Get PID for a process
    #[allow(dead_code)]
    async fn get_pid_for_process(&self, process_id: Uuid) -> Option<u32> {
        let map = self.process_pid_map.read().await;
        map.get(&process_id).copied()
    }
}

#[async_trait::async_trait]
impl ResourceLimiter for PlatformResourceLimiter {
    async fn register_process_pid(&self, process_id: Uuid, pid: u32) {
        let mut map = self.process_pid_map.write().await;
        map.insert(process_id, pid);
    }
    async fn apply_limits(&self, process_id: Uuid, limits: &ResourceLimits) -> Result<(), AppError> {
        // Validate limits first
        limits.validate()?;
        
        // Platform-specific implementation
        #[cfg(target_os = "linux")]
        {
            // Get PID for this process
            let pid = self.get_pid_for_process(process_id).await
                .ok_or_else(|| AppError::ResourceError(
                    format!("PID not found for process {}", process_id)
                ))?;

            if let Some(ref limiter) = self.linux_limiter {
                limiter.apply_limits(process_id, pid, limits).await?;
                tracing::info!(
                    "Applied Linux cgroup limits to process {} (PID {}): cpu={:?}, memory={:?}MB",
                    process_id, pid, limits.cpu_cores, limits.memory_mb
                );
            } else {
                tracing::warn!(
                    "Linux cgroup limiter not available, limits not enforced for process {}",
                    process_id
                );
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Get PID for this process
            let pid = self.get_pid_for_process(process_id).await
                .ok_or_else(|| AppError::ResourceError(
                    format!("PID not found for process {}", process_id)
                ))?;

            if let Some(ref limiter) = self.windows_limiter {
                limiter.apply_limits(process_id, pid, limits).await?;
                tracing::info!(
                    "Applied Windows Job Object limits to process {} (PID {}): cpu={:?}, memory={:?}MB",
                    process_id, pid, limits.cpu_cores, limits.memory_mb
                );
            } else {
                tracing::warn!(
                    "Windows Job Object limiter not available, limits not enforced for process {}",
                    process_id
                );
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            // Suppress unused variable warning
            let _ = process_id;
            tracing::warn!(
                "Resource limits not supported on this platform"
            );
        }
        
        Ok(())
    }
    
    async fn get_usage(&self, process_id: Uuid) -> Result<ResourceUsage, AppError> {
        #[cfg(target_os = "linux")]
        {
            if let Some(ref limiter) = self.linux_limiter {
                return limiter.get_usage(process_id).await;
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(ref limiter) = self.windows_limiter {
                return limiter.get_usage(process_id).await;
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let _ = process_id;
        }

        Ok(ResourceUsage::default())
    }
    
    fn is_supported(&self) -> bool {
        #[cfg(target_os = "linux")]
        {
            return self.linux_limiter.is_some();
        }

        #[cfg(target_os = "windows")]
        {
            // Windows implementation is available (placeholder for now)
            // Will return true once full Windows API integration is complete
            return self.windows_limiter.is_some();
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            return false;
        }
    }
}

impl Default for PlatformResourceLimiter {
    fn default() -> Self {
        Self::new()
    }
}

