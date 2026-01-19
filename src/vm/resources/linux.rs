//! Linux-specific resource limiting using cgroups
//!
//! Provides:
//! - CPU limits via cgroup v2 (cpu.max) or v1 (cpu.cfs_quota_us, cpu.cfs_period_us)
//! - Memory limits via cgroup v2 (memory.max) or v1 (memory.limit_in_bytes)
//! - Resource usage monitoring from cgroup stats

use crate::core::error::AppError;
use crate::vm::resources::{ResourceLimits, ResourceUsage};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// Linux cgroup manager for resource limiting
#[allow(dead_code)] // Used in resources.rs
pub struct LinuxCgroupLimiter {
    /// Base path for cgroup filesystem
    cgroup_root: PathBuf,
    /// Cgroup version (v1 or v2)
    cgroup_version: CgroupVersion,
}

#[derive(Debug, Clone, Copy)]
enum CgroupVersion {
    V1,
    V2,
}

impl LinuxCgroupLimiter {
    /// Create a new Linux cgroup limiter
    pub fn new() -> Result<Self, AppError> {
        // Detect cgroup version
        let cgroup_root = if Path::new("/sys/fs/cgroup/unified").exists() {
            // cgroup v2 (unified hierarchy)
            PathBuf::from("/sys/fs/cgroup")
        } else if Path::new("/sys/fs/cgroup/cpu").exists() {
            // cgroup v1 (separate controllers)
            PathBuf::from("/sys/fs/cgroup")
        } else {
            return Err(AppError::ConfigError(
                "cgroup filesystem not found".to_string(),
            ));
        };

        let cgroup_version = if Path::new("/sys/fs/cgroup/cgroup.controllers").exists() {
            CgroupVersion::V2
        } else {
            CgroupVersion::V1
        };

        Ok(Self {
            cgroup_root,
            cgroup_version,
        })
    }

    /// Get cgroup path for a process
    fn get_cgroup_path(&self, process_id: Uuid, controller: &str) -> PathBuf {
        let cgroup_name = format!("poolai-{}", process_id);
        match self.cgroup_version {
            CgroupVersion::V2 => {
                // cgroup v2: single unified hierarchy
                self.cgroup_root.join(&cgroup_name)
            }
            CgroupVersion::V1 => {
                // cgroup v1: separate controller directories
                self.cgroup_root.join(controller).join(&cgroup_name)
            }
        }
    }

    /// Create cgroup for a process
    async fn create_cgroup(&self, process_id: Uuid, controller: &str) -> Result<PathBuf, AppError> {
        let cgroup_path = self.get_cgroup_path(process_id, controller);
        
        // Create cgroup directory
        fs::create_dir_all(&cgroup_path).await.map_err(|e| {
            AppError::ConfigError(format!("Failed to create cgroup directory: {}", e))
        })?;

        Ok(cgroup_path)
    }

    /// Apply CPU limits using cgroup
    async fn apply_cpu_limits(
        &self,
        process_id: Uuid,
        cpu_cores: u16,
    ) -> Result<(), AppError> {
        match self.cgroup_version {
            CgroupVersion::V2 => {
                // cgroup v2: use cpu.max (format: "quota period" or "max" for unlimited)
                let cgroup_path = self.create_cgroup(process_id, "cpu").await?;
                let cpu_max_path = cgroup_path.join("cpu.max");
                
                // Calculate quota: cpu_cores * 100000 (period is 100000 microseconds = 0.1s)
                let period = 100000u64;
                let quota = (cpu_cores as u64) * period;
                let cpu_max = format!("{} {}", quota, period);
                
                fs::write(&cpu_max_path, cpu_max.as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write cpu.max: {}", e))
                })?;
            }
            CgroupVersion::V1 => {
                // cgroup v1: use cpu.cfs_quota_us and cpu.cfs_period_us
                let cgroup_path = self.create_cgroup(process_id, "cpu").await?;
                
                // Set period (100ms = 100000 microseconds)
                let period_path = cgroup_path.join("cpu.cfs_period_us");
                fs::write(&period_path, b"100000").await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write cpu.cfs_period_us: {}", e))
                })?;
                
                // Set quota (cpu_cores * period)
                let quota = (cpu_cores as u64) * 100000;
                let quota_path = cgroup_path.join("cpu.cfs_quota_us");
                fs::write(&quota_path, quota.to_string().as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write cpu.cfs_quota_us: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Apply memory limits using cgroup
    async fn apply_memory_limits(
        &self,
        process_id: Uuid,
        memory_mb: u32,
    ) -> Result<(), AppError> {
        let memory_bytes = (memory_mb as u64) * 1024 * 1024;
        let cgroup_path = self.create_cgroup(process_id, "memory").await?;

        match self.cgroup_version {
            CgroupVersion::V2 => {
                // cgroup v2: use memory.max
                let memory_max_path = cgroup_path.join("memory.max");
                fs::write(&memory_max_path, memory_bytes.to_string().as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write memory.max: {}", e))
                })?;
            }
            CgroupVersion::V1 => {
                // cgroup v1: use memory.limit_in_bytes
                let memory_limit_path = cgroup_path.join("memory.limit_in_bytes");
                fs::write(&memory_limit_path, memory_bytes.to_string().as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write memory.limit_in_bytes: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Add process to cgroup
    async fn add_process_to_cgroup(
        &self,
        process_id: Uuid,
        pid: u32,
    ) -> Result<(), AppError> {
        let cgroup_path = self.get_cgroup_path(process_id, "cpu");
        
        match self.cgroup_version {
            CgroupVersion::V2 => {
                // cgroup v2: use cgroup.procs
                let procs_path = cgroup_path.join("cgroup.procs");
                fs::write(&procs_path, pid.to_string().as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write cgroup.procs: {}", e))
                })?;
            }
            CgroupVersion::V1 => {
                // cgroup v1: use tasks (or cgroup.procs for thread groups)
                let tasks_path = cgroup_path.join("tasks");
                fs::write(&tasks_path, pid.to_string().as_bytes()).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to write tasks: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Get CPU usage from cgroup
    async fn get_cpu_usage(&self, process_id: Uuid) -> Result<f64, AppError> {
        let cgroup_path = self.get_cgroup_path(process_id, "cpu");
        
        // Future improvement: Read CPU usage from cgroup files
        // 1. Detect cgroup version (v1 or v2)
        //    - Check if /sys/fs/cgroup/cpu exists (v1) or /sys/fs/cgroup/cgroup.controllers exists (v2)
        //    - Use appropriate path based on version
        // 2. For cgroup v1 (cpuacct controller):
        //    - Read /sys/fs/cgroup/cpu,cpuacct/<cgroup_path>/cpuacct.usage (nanoseconds since boot)
        //    - Parse as u64 (nanoseconds)
        //    - Compare with previous measurement to calculate CPU percentage
        //    - CPU % = (current_usage - previous_usage) / (elapsed_time * num_cores * 1_000_000_000) * 100
        // 3. For cgroup v2:
        //    - Read /sys/fs/cgroup/<cgroup_path>/cpu.stat
        //    - Parse lines: "usage_usec <value>" (microseconds)
        //    - Calculate CPU percentage similarly to v1
        // 4. Track previous measurements for rate calculation
        //    - Store previous measurements in HashMap (process_id -> (previous_usage, previous_timestamp))
        //    - Use Instant::now() for elapsed time calculation
        //    - Handle first measurement (return 0.0 if no previous measurement)
        // 5. Error handling
        //    - Handle file read errors gracefully
        //    - Handle parse errors (invalid format)
        //    - Handle cgroup not found errors
        //    - Return 0.0 if unable to retrieve CPU usage
        // Example (cgroup v2):
        //    let cpu_stat_path = cgroup_path.join("cpu.stat");
        //    let content = tokio::fs::read_to_string(&cpu_stat_path).await?;
        //    let usage_usec = parse_cpu_stat(&content)?;
        //    let cpu_percent = calculate_cpu_percentage(usage_usec, previous_measurement, elapsed_time, num_cores)?;
        // For now, return 0.0 as placeholder
        
        Ok(0.0)
    }

    /// Get memory usage from cgroup
    async fn get_memory_usage(&self, process_id: Uuid) -> Result<u32, AppError> {
        let cgroup_path = self.get_cgroup_path(process_id, "memory");
        
        match self.cgroup_version {
            CgroupVersion::V2 => {
                // cgroup v2: read memory.current
                let memory_current_path = cgroup_path.join("memory.current");
                let usage_bytes = fs::read_to_string(&memory_current_path).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to read memory.current: {}", e)))?
                    .trim()
                    .parse::<u64>()
                    .map_err(|e| AppError::ConfigError(format!("Failed to parse memory.current: {}", e)))?;
                
                Ok((usage_bytes / 1024 / 1024) as u32)
            }
            CgroupVersion::V1 => {
                // cgroup v1: read memory.usage_in_bytes
                let memory_usage_path = cgroup_path.join("memory.usage_in_bytes");
                let usage_bytes = fs::read_to_string(&memory_usage_path).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to read memory.usage_in_bytes: {}", e)))?
                    .trim()
                    .parse::<u64>()
                    .map_err(|e| AppError::ConfigError(format!("Failed to parse memory.usage_in_bytes: {}", e)))?;
                
                Ok((usage_bytes / 1024 / 1024) as u32)
            }
        }
    }

    /// Apply resource limits to a process
    pub async fn apply_limits(
        &self,
        process_id: Uuid,
        pid: u32,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        // Apply CPU limits
        if limits.cpu_cores > 0 {
            self.apply_cpu_limits(process_id, limits.cpu_cores).await?;
        }

        // Apply memory limits
        if limits.memory_mb > 0 {
            self.apply_memory_limits(process_id, limits.memory_mb).await?;
        }

        // Add process to cgroup
        self.add_process_to_cgroup(process_id, pid).await?;

        Ok(())
    }

    /// Get resource usage for a process
    pub async fn get_usage(&self, process_id: Uuid) -> Result<ResourceUsage, AppError> {
        let cpu_percent = self.get_cpu_usage(process_id).await.unwrap_or(0.0);
        let memory_mb = self.get_memory_usage(process_id).await.unwrap_or(0);

        Ok(ResourceUsage {
            cpu_percent,
            memory_mb,
            // Future improvement: GPU usage monitoring on Linux
            // 1. Option A: Use NVIDIA Management Library (NVML) for NVIDIA GPUs
            //    - Link against libnvidia-ml.so (requires NVIDIA driver)
            //    - Use nvmlDeviceGetUtilizationRates for GPU utilization
            //    - Use nvmlDeviceGetProcessUtilization for per-process GPU usage
            //    - Requires nvml-sys crate or FFI bindings
            //    - Path: /usr/lib/x86_64-linux-gnu/libnvidia-ml.so or similar
            // 2. Option B: Read from /sys/class/drm/ (generic GPU info, limited per-process support)
            //    - Read /sys/class/drm/card*/device/uevent for GPU identification
            //    - Read /sys/class/drm/card*/gt/gt*/rps_act_freq_mhz for current frequency
            //    - Less accurate, no per-process GPU usage
            // 3. Option C: Use nvidia-smi command (NVIDIA GPUs)
            //    - Parse output of 'nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader'
            //    - Use 'nvidia-smi pmon' for per-process GPU usage
            //    - Requires parsing command output (slower, less reliable)
            // 4. Option D: Use rocm-smi command (AMD GPUs)
            //    - Parse output of 'rocm-smi --showuse --showmemuse' for AMD GPUs
            //    - Similar to nvidia-smi approach
            // 5. Option E: Use Intel GPU Tools (Intel GPUs)
            //    - Use intel_gpu_top or similar tools for Intel GPU monitoring
            //    - Parse output or use library bindings
            // 6. Implementation considerations
            //    - Detect available GPU vendor and use appropriate API
            //    - Handle cases where GPU is not available or unsupported
            //    - Cache GPU usage queries (expensive operations)
            //    - Return None if GPU monitoring is not available
            // Example (NVML - most accurate):
            //    use nvml_sys::*;
            //    let device = nvmlDeviceGetHandleByIndex_v2(0)?;
            //    let mut utilization = nvmlUtilization_t::default();
            //    nvmlDeviceGetUtilizationRates(device, &mut utilization)?;
            //    Some(utilization.gpu as f32)
            // Example (nvidia-smi - fallback):
            //    let output = Command::new("nvidia-smi")
            //        .args(&["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"])
            //        .output().await?;
            //    let gpu_percent = String::from_utf8(output.stdout)?.trim().parse::<f32>()?;
            gpu_percent: None,
            gpu_memory_mb: None,
        })
    }
}

