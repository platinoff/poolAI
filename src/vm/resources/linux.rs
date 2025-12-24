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
        
        // For now, return 0.0 as placeholder
        // TODO: Read from cpu.stat or cpuacct.usage (cgroup v1) or cpu.stat (cgroup v2)
        // This requires parsing and calculating usage over time
        
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
        if let Some(cpu_cores) = limits.cpu_cores {
            self.apply_cpu_limits(process_id, cpu_cores).await?;
        }

        // Apply memory limits
        if let Some(memory_mb) = limits.memory_mb {
            self.apply_memory_limits(process_id, memory_mb).await?;
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
            gpu_percent: None, // TODO: GPU usage monitoring
            gpu_memory_mb: None,
        })
    }
}

