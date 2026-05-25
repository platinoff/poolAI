//! Windows-specific resource limiting using Job Objects (post-spawn).

use crate::core::error::AppError;
use crate::vm::resources::{validate_resource_limits, ResourceLimits, ResourceUsage};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
#[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
    SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_JOB_MEMORY,
};
#[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
};

/// Windows Job Object state tracking.
#[derive(Debug, Clone)]
pub struct JobObjectState {
    pub handle: usize,
    pub process_id: Uuid,
    pub cpu_cores: Option<u16>,
    pub memory_mb: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub process_assigned: bool,
}

impl JobObjectState {
    pub fn new(handle: usize, process_id: Uuid) -> Self {
        Self {
            handle,
            process_id,
            cpu_cores: None,
            memory_mb: None,
            created_at: Utc::now(),
            process_assigned: false,
        }
    }
}

/// Windows Job Object manager — limits are applied after spawn when PID is known.
pub struct WindowsJobObjectLimiter {
    job_objects:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<Uuid, JobObjectState>>>,
}

impl Default for WindowsJobObjectLimiter {
    fn default() -> Self {
        Self::new().expect("WindowsJobObjectLimiter init")
    }
}

impl WindowsJobObjectLimiter {
    pub fn new() -> Result<Self, AppError> {
        #[cfg(not(target_os = "windows"))]
        {
            return Err(AppError::ConfigError(
                "Windows Job Objects are only available on Windows".to_string(),
            ));
        }

        #[cfg(target_os = "windows")]
        {
            Ok(Self {
                job_objects: std::sync::Arc::new(tokio::sync::RwLock::new(
                    std::collections::HashMap::new(),
                )),
            })
        }
    }

    async fn create_job_object(&self, process_id: Uuid) -> Result<usize, AppError> {
        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            let job_handle = unsafe { CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()) };
            if job_handle == INVALID_HANDLE_VALUE || job_handle.is_null() {
                return Err(AppError::ConfigError(format!(
                    "Failed to create Job Object for VM instance {process_id}"
                )));
            }
            let handle_id = job_handle as usize;
            let mut job_objects = self.job_objects.write().await;
            job_objects.insert(process_id, JobObjectState::new(handle_id, process_id));
            tracing::info!("Created Job Object for VM instance {process_id} (handle: {handle_id})");
            return Ok(handle_id);
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            let handle_id = process_id.as_u128() as usize;
            let mut job_objects = self.job_objects.write().await;
            job_objects.insert(process_id, JobObjectState::new(handle_id, process_id));
            tracing::debug!(
                "Job Object placeholder for VM instance {process_id} (enable `vm-isolation-windows` for native enforcement)"
            );
            Ok(handle_id)
        }
    }

    async fn job_handle_for(&self, process_id: Uuid) -> Result<usize, AppError> {
        let job_objects = self.job_objects.read().await;
        if let Some(state) = job_objects.get(&process_id) {
            return Ok(state.handle);
        }
        drop(job_objects);
        self.create_job_object(process_id).await
    }

    async fn apply_cpu_limits(
        &self,
        process_id: Uuid,
        pid: u32,
        cpu_cores: u16,
    ) -> Result<(), AppError> {
        let _ = pid;
        if !self.job_objects.read().await.contains_key(&process_id) {
            self.create_job_object(process_id).await?;
        }
        let mut job_objects = self.job_objects.write().await;
        if let Some(state) = job_objects.get_mut(&process_id) {
            state.cpu_cores = Some(cpu_cores);
        }
        tracing::info!(
            "Windows CPU limit recorded for VM instance {process_id}: {cpu_cores} cores (rate control via Job Object — follow-up)"
        );
        Ok(())
    }

    async fn apply_memory_limits(
        &self,
        process_id: Uuid,
        pid: u32,
        memory_mb: u32,
    ) -> Result<(), AppError> {
        let _handle_id = self.job_handle_for(process_id).await?;
        let memory_bytes = (memory_mb as u64) * 1024 * 1024;

        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            let job_handle = _handle_id as HANDLE;
            let mut limit_info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
                BasicLimitInformation:
                    windows_sys::Win32::System::JobObjects::JOBOBJECT_BASIC_LIMIT_INFORMATION {
                        LimitFlags: JOB_OBJECT_LIMIT_JOB_MEMORY,
                        ..Default::default()
                    },
                ProcessMemoryLimit: memory_bytes,
                ..Default::default()
            };
            let result = unsafe {
                SetInformationJobObject(
                    job_handle,
                    JobObjectExtendedLimitInformation,
                    &limit_info as *const _ as *const _,
                    std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                )
            };
            if result == 0 {
                return Err(AppError::ConfigError(format!(
                    "SetInformationJobObject memory limit failed for VM instance {process_id} ({memory_mb} MB)"
                )));
            }
            tracing::info!(
                "Applied Windows memory limit for VM instance {process_id}: {memory_mb} MB"
            );
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            let _ = (pid, memory_bytes);
            tracing::info!(
                "Windows memory limit recorded for VM instance {process_id} (PID {pid}): {memory_mb} MB (enable `vm-isolation-windows` for native enforcement)"
            );
        }

        let mut job_objects = self.job_objects.write().await;
        if let Some(state) = job_objects.get_mut(&process_id) {
            state.memory_mb = Some(memory_mb);
        }
        Ok(())
    }

    async fn assign_process_to_job(&self, process_id: Uuid, pid: u32) -> Result<(), AppError> {
        if pid == 0 {
            return Err(AppError::ValidationError(
                "Cannot assign Job Object: invalid PID 0".to_string(),
            ));
        }

        let handle_id = self.job_handle_for(process_id).await?;

        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            let job_handle = handle_id as HANDLE;
            let access = PROCESS_SET_QUOTA | PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION;
            let process_handle = unsafe { OpenProcess(access, 0, pid) };
            if process_handle == 0 || process_handle.is_null() {
                return Err(AppError::ConfigError(format!(
                    "OpenProcess failed for PID {pid} (VM instance {process_id})"
                )));
            }
            let assigned = unsafe { AssignProcessToJobObject(job_handle, process_handle) };
            unsafe {
                CloseHandle(process_handle);
            }
            if assigned == 0 {
                return Err(AppError::ConfigError(format!(
                    "AssignProcessToJobObject failed for PID {pid} (VM instance {process_id})"
                )));
            }
            tracing::info!("Assigned PID {pid} to Job Object for VM instance {process_id}");
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            tracing::debug!(
                "Job Object assignment recorded for VM instance {process_id} (PID {pid}); enable `vm-isolation-windows` for native AssignProcessToJobObject"
            );
        }

        let mut job_objects = self.job_objects.write().await;
        if let Some(state) = job_objects.get_mut(&process_id) {
            state.process_assigned = true;
        }
        Ok(())
    }

    /// Apply CPU/memory limits and assign the process to the Job Object (post-spawn).
    pub async fn apply_limits(
        &self,
        process_id: Uuid,
        pid: u32,
        limits: &ResourceLimits,
    ) -> Result<(), AppError> {
        validate_resource_limits(limits)?;

        if limits.cpu_cores > 0 {
            self.apply_cpu_limits(process_id, pid, limits.cpu_cores)
                .await?;
        }
        if limits.memory_mb > 0 {
            self.apply_memory_limits(process_id, pid, limits.memory_mb)
                .await?;
        }
        if limits.cpu_cores > 0 || limits.memory_mb > 0 {
            self.assign_process_to_job(process_id, pid).await?;
        }
        Ok(())
    }

    pub async fn get_job_object_state(&self, process_id: Uuid) -> Option<JobObjectState> {
        let job_objects = self.job_objects.read().await;
        job_objects.get(&process_id).cloned()
    }

    pub async fn get_usage(&self, process_id: Uuid) -> Result<ResourceUsage, AppError> {
        let job_objects = self.job_objects.read().await;
        if !job_objects.contains_key(&process_id) {
            return Err(AppError::ValidationError(format!(
                "Job Object not found for VM instance {process_id}"
            )));
        }
        Ok(ResourceUsage {
            cpu_percent: 0.0,
            memory_mb: 0,
            gpu_utilization: None,
        })
    }
}
