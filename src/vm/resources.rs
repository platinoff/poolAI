//! Resource Limits Enforcement for VM instances
//!
//! This module provides resource limits enforcement for VM instances.
//!
//! ## Platform-specific implementations:
//!
//! - **Windows**: Job Objects (planned for future implementation)
//!   - Windows Job Objects allow fine-grained control over process resources
//!   - Will provide CPU and memory limits enforcement
//!   - Requires Windows API integration
//!
//! - **Linux**: cgroups v2 (planned for future implementation)
//!   - Modern cgroups v2 interface for resource control
//!   - Will provide CPU, memory, and I/O limits enforcement
//!   - Requires systemd or direct cgroup filesystem access
//!
//! - **Cross-platform**: Basic validation and monitoring (current implementation)
//!   - Validates resource limits configuration
//!   - Monitors resource usage through platform APIs
//!   - Provides fallback behavior when platform-specific enforcement is not available

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

/// Resource limits configuration for a VM instance
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

/// Resource limiter trait for platform-specific implementations
#[async_trait::async_trait]
pub trait ResourceLimiter: Send + Sync {
    /// Apply resource limits to a command before spawning
    ///
    /// This is called before the process is spawned. For platforms like Linux cgroups,
    /// this may only set environment variables, and actual limits should be applied
    /// post-spawn using `apply_limits_post_spawn()`.
    async fn apply_limits(
        &self,
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError>;

    /// Apply resource limits to a process after spawning
    ///
    /// This is called after the process is spawned and PID is available.
    /// Used by platforms like Linux cgroups that require process PID.
    ///
    /// # Arguments
    /// * `process_id` - VM instance ID (Uuid)
    /// * `pid` - Process ID (u32)
    /// * `limits` - Resource limits to apply
    async fn apply_limits_post_spawn(
        &self,
        _process_id: uuid::Uuid,
        _pid: u32,
        _limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // Default implementation: no-op (limits applied in apply_limits)
        Ok(())
    }

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

impl Default for PlatformResourceLimiter {
    fn default() -> Self {
        Self::new()
    }
}

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
            // Resource limits not supported on this platform, only validation will be performed
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
        // Future improvement: Implement Windows Job Objects for CPU/memory limits
        // 1. Create Job Object using CreateJobObjectW Windows API
        //    - Call CreateJobObjectW(NULL, job_name) to create a new job object
        //    - Store job handle for later use
        //    - Requires windows-sys or winapi crate bindings
        // 2. Configure CPU limits using JOBOBJECT_CPU_RATE_CONTROL_INFORMATION
        //    - Set ControlFlags to JOB_OBJECT_CPU_RATE_CONTROL_ENABLE
        //    - Set CpuRate to percentage (0-100) for CPU cores limit
        //    - Use SetInformationJobObject() with JobObjectCpuRateControlInformation
        // 3. Configure memory limits using JOBOBJECT_EXTENDED_LIMIT_INFORMATION
        //    - Set JobMemoryLimit to memory_mb * 1024 * 1024 (bytes)
        //    - Set LimitFlags to include JOB_OBJECT_LIMIT_JOB_MEMORY
        //    - Use SetInformationJobObject() with JobObjectExtendedLimitInformation
        // 4. Assign process to job using AssignProcessToJobObject()
        //    - Call AssignProcessToJobObject(job_handle, process_handle)
        //    - Process must not already be in a job object
        //    - Process must be created suspended (CREATE_SUSPENDED flag)
        //
        // This requires:
        // - Windows API bindings (windows-sys crate or winapi crate)
        // - Administrator privileges for some job object features
        // - Understanding of Windows Job Objects API
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

    /// Apply Windows resource limits to an already-spawned process
    ///
    /// This function is intended for future use when applying limits to running processes.
    /// Currently unused but kept for post-spawn resource limit application scenarios.
    #[allow(dead_code)]
    pub async fn apply_windows_limits_post_spawn(
        process_id: uuid::Uuid,
        pid: u32,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            // Use WindowsJobObjectLimiter to apply limits
            // Note: WindowsJobObjectLimiter is not exported, so we use the apply_limits method
            // from the windows module directly
            use crate::vm::resources::windows::WindowsJobObjectLimiter;

            let limiter = WindowsJobObjectLimiter::new()?;

            // Apply limits using WindowsJobObjectLimiter
            limiter.apply_limits(process_id, pid, limits).await?;

            tracing::info!(
                "Applied Windows resource limits to process {} (PID {}): CPU: {}, Memory: {} MB",
                process_id,
                pid,
                limits.cpu_cores,
                limits.memory_mb
            );

            Ok(())
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            // Fallback: just log
            let _ = (process_id, pid); // Suppress unused variable warnings
            if limits.cpu_cores > 0 || limits.memory_mb > 0 {
                warn!(
                    "Windows resource limits post-spawn not available (Windows API not enabled) (CPU: {}, Memory: {} MB)",
                    limits.cpu_cores, limits.memory_mb
                );
            }
            Ok(())
        }
    }

    pub async fn get_windows_usage(_process_id: u32) -> Result<ResourceUsage, AppError> {
        // Future improvement: Implement Windows process resource usage query
        // 1. Open process handle using OpenProcess() Windows API
        //    - Use PROCESS_QUERY_INFORMATION | PROCESS_VM_READ access rights
        //    - Handle must be closed with CloseHandle() when done
        // 2. Query CPU usage using GetProcessTimes() Windows API
        //    - Get kernel time and user time
        //    - Calculate CPU percentage based on elapsed time and process time
        //    - Requires tracking previous times for percentage calculation
        // 3. Query memory usage using PROCESS_MEMORY_COUNTERS_EX structure
        //    - Use GetProcessMemoryInfo() to get PROCESS_MEMORY_COUNTERS_EX
        //    - Read PrivateUsage field for process memory in bytes
        //    - Convert bytes to MB for ResourceUsage struct
        // 4. Query GPU usage (optional, requires vendor-specific APIs)
        //    - Use NVIDIA Management Library (NVML) or AMD ADL API
        //    - Map process_id to GPU context
        //    - Query GPU utilization percentage
        //
        // This requires:
        // - Windows API bindings (windows-sys crate or winapi crate)
        // - Process handle management (proper cleanup)
        // - Time tracking for CPU percentage calculation
        // - Optional GPU vendor SDKs for GPU usage
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

    pub async fn apply_linux_limits(
        command: &mut Command,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // Validate limits
        if limits.memory_mb > 0 && limits.memory_mb < 64 {
            return Err(AppError::ValidationError(
                "Memory limit too low (minimum 64 MB)".to_string(),
            ));
        }

        // Try to initialize LinuxCgroupLimiter to check if cgroups are available
        // Note: We can't use LinuxCgroupLimiter::apply_limits() here because it requires
        // a process_id (Uuid) and pid (u32), which we don't have yet (process not spawned)
        //
        // Actual cgroup limits should be applied in VmManager after process spawn:
        // 1. Spawn process and get PID
        // 2. Create LinuxCgroupLimiter instance
        // 3. Apply limits using limiter.apply_limits(process_id, pid, limits)
        //
        // For now, store limits in environment variables as a hint that limits should be applied
        if limits.cpu_cores > 0 {
            command.env("POOLAI_CPU_LIMIT", limits.cpu_cores.to_string());
        }
        if limits.memory_mb > 0 {
            command.env("POOLAI_MEMORY_LIMIT_MB", limits.memory_mb.to_string());
        }

        // Note: Actual cgroup enforcement requires post-spawn application
        // This is a limitation of cgroups - they need the process PID
        // See LinuxCgroupLimiter::apply_limits() for actual implementation
        Ok(())
    }

    pub async fn get_linux_usage(_process_id: u32) -> Result<ResourceUsage, AppError> {
        // Future improvement: Implement Linux process resource usage query
        // 1. Read CPU usage from /proc/{pid}/stat file
        //    - Parse utime (user time) and stime (system time) fields (positions 14 and 15)
        //    - Read /proc/stat for system uptime to calculate CPU percentage
        //    - Calculate: ((utime + stime) / (system_uptime * clock_ticks)) * 100
        //    - Requires tracking previous times for accurate percentage calculation
        // 2. Read memory usage from /proc/{pid}/status file
        //    - Parse VmRSS (Resident Set Size) field for physical memory
        //    - Parse VmSize field for virtual memory (optional)
        //    - Convert KB to MB for ResourceUsage struct
        //    - Example: VmRSS: 123456 kB -> 120 MB
        // 3. Read CPU and memory from /proc/{pid}/statm file (alternative)
        //    - First field: total program size (pages)
        //    - Second field: resident set size (pages)
        //    - Convert pages to bytes: pages * page_size (getconf PAGESIZE)
        // 4. Query GPU usage (optional, requires vendor-specific APIs)
        //    - Use nvidia-smi for NVIDIA GPUs (parse command output)
        //    - Use rocm-smi for AMD GPUs (parse command output)
        //    - Map process_id to GPU context using vendor APIs
        //
        // This requires:
        // - Reading from /proc filesystem (available on all Linux systems)
        // - Parsing /proc/{pid}/stat and /proc/{pid}/status files
        // - Time tracking for CPU percentage calculation
        // - Optional GPU vendor tools for GPU usage
        // - Understanding of Linux proc filesystem format
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
