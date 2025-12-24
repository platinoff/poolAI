//! Virtual Machines (VM) module
//!
//! Concept alignment:
//! - VM instance management (planned in `poolAI_concept.txt`)
//! - Isolation/security hooks (stubbed)
//! - Resource optimization primitives (basic)
//! - Resource limits enforcement (Week 2+)

use crate::core::error::AppError;
use crate::runtime::ProcessManager;

pub mod resources;
pub use resources::{ResourceLimits, ResourceUsage, ResourceLimiter};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// VM lifecycle status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmStatus {
    Creating,
    Running,
    Stopped,
    Failed(String),
}

/// Resource limits / requests for a VM instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmResources {
    pub cpu_cores: u16,
    pub memory_mb: u32,
    pub gpu_required: bool,
}

impl Default for VmResources {
    fn default() -> Self {
        Self {
            cpu_cores: 2,
            memory_mb: 2048,
            gpu_required: false,
        }
    }
}

/// Isolation/security policy placeholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmIsolation {
    /// Best-effort isolation using OS-level process sandboxing (planned).
    ProcessSandbox,
    /// Hardware virtualization (planned).
    HardwareVm,
}

/// A VM instance representation (in-memory for now).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInstance {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub status: VmStatus,
    pub resources: VmResources,
    pub isolation: VmIsolation,
    /// Process ID from ProcessManager (if running)
    pub process_id: Option<Uuid>,
    /// Command to execute when starting
    pub command: Option<String>,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory
    pub working_dir: Option<PathBuf>,
}

/// VM Manager - central orchestrator for VM instances.
pub struct VmManager {
    instances: Arc<RwLock<HashMap<Uuid, VmInstance>>>,
    process_manager: Arc<RwLock<ProcessManager>>,
    resource_limiter: Arc<dyn ResourceLimiter + Send + Sync>,
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            process_manager: Arc::new(RwLock::new(ProcessManager::new())),
            resource_limiter: Arc::new(resources::PlatformResourceLimiter::new()) as Arc<dyn ResourceLimiter + Send + Sync>,
        }
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing VM manager");
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down VM manager");
        Ok(())
    }

    pub async fn create_instance(
        &self,
        name: String,
        resources: VmResources,
        isolation: VmIsolation,
        command: Option<String>,
        args: Vec<String>,
        working_dir: Option<PathBuf>,
    ) -> Result<VmInstance, AppError> {
        let id = Uuid::new_v4();
        let instance = VmInstance {
            id,
            name: name.clone(),
            created_at: Utc::now(),
            status: VmStatus::Creating,
            resources,
            isolation,
            process_id: None,
            command,
            args,
            working_dir,
        };

        self.instances.write().await.insert(id, instance.clone());
        info!("Created VM instance {}: {}", id, name);
        Ok(instance)
    }

    pub async fn list_instances(&self) -> Vec<VmInstance> {
        self.instances
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    pub async fn get_instance(&self, id: Uuid) -> Option<VmInstance> {
        self.instances.read().await.get(&id).cloned()
    }

    pub async fn start_instance(&self, id: Uuid) -> Result<(), AppError> {
        // Get command and args before spawning process
        let (command, args, working_dir, memory_mb) = {
            let instances = self.instances.read().await;
            let inst = instances
                .get(&id)
                .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;
            
            (inst.command.clone(), inst.args.clone(), inst.working_dir.clone(), inst.resources.memory_mb)
        };

        // If instance has a command, spawn it via ProcessManager
        if let Some(cmd) = command {
            let config = crate::runtime::ProcessConfig {
                command: cmd,
                args,
                working_dir,
                env: HashMap::new(), // TODO: Add environment variables support
                timeout_seconds: Some(3600), // 1 hour default timeout
                cpu_limit_percent: None, // TODO: Map from resources.cpu_cores
                memory_limit_mb: Some(memory_mb),
                capture_logs: true,
            };

            let process_id = {
                let pm = self.process_manager.write().await;
                pm.spawn_process(config).await?
            };

            // Apply resource limits after process is spawned
            {
                // Get PID from ProcessManager and register it
                let pid = {
                    let pm = self.process_manager.read().await;
                    pm.get_process_pid(process_id).await.ok().flatten()
                };

                if let Some(pid) = pid {
                    // Register PID in resource limiter (needed for Linux cgroups)
                    self.resource_limiter.register_process_pid(process_id, pid).await;
                    
                    // Get limits and apply them
                    let instances = self.instances.read().await;
                    let inst = instances.get(&id).ok_or_else(|| {
                        AppError::ValidationError(format!("VM instance {} not found after spawn", id))
                    })?;
                    
                    let limits = ResourceLimits::from(inst.resources.clone());
                    drop(instances); // Release lock before async call
                    
                    // Apply limits (PID is now registered)
                    if let Err(e) = self.resource_limiter.apply_limits(process_id, &limits).await {
                        warn!("Failed to apply resource limits to process {}: {}", process_id, e);
                        // Continue anyway - limits are not critical for basic operation
                    }
                } else {
                    warn!("Could not get PID for process {}, skipping resource limits", process_id);
                }
            }

            // Update instance with process_id
            let mut instances = self.instances.write().await;
            let inst = instances.get_mut(&id).unwrap();
            inst.process_id = Some(process_id);
            inst.status = VmStatus::Running;
            info!("Started VM instance {} with process {}", id, process_id);
        } else {
            // No command specified, just mark as running
            let mut instances = self.instances.write().await;
            let inst = instances.get_mut(&id).unwrap();
            inst.status = VmStatus::Running;
            info!("Started VM instance {} (no command)", id);
        }

        Ok(())
    }

    pub async fn stop_instance(&self, id: Uuid) -> Result<(), AppError> {
        // Get process_id before stopping
        let process_id = {
            let instances = self.instances.read().await;
            let inst = instances
                .get(&id)
                .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;
            inst.process_id
        };

        // Stop process if running
        if let Some(pid) = process_id {
            let pm = self.process_manager.write().await;
            if let Err(e) = pm.stop_process(pid).await {
                warn!("Failed to stop process {} for VM {}: {}", pid, id, e);
            }
        }

        // Update instance status
        let mut instances = self.instances.write().await;
        let inst = instances.get_mut(&id).unwrap();
        inst.process_id = None;
        inst.status = VmStatus::Stopped;
        info!("Stopped VM instance {}", id);
        Ok(())
    }

    /// Get process logs for a VM instance
    pub async fn get_instance_logs(&self, id: Uuid) -> Result<crate::runtime::ProcessLogs, AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        let process_id = inst.process_id
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} has no process", id)))?;

        drop(instances); // Release lock before async call

        let pm = self.process_manager.read().await;
        pm.get_process_logs(process_id).await
    }

    /// Get process status for a VM instance
    pub async fn get_instance_process_status(&self, id: Uuid) -> Result<crate::runtime::ProcessStatus, AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        let process_id = inst.process_id
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} has no process", id)))?;

        drop(instances); // Release lock before async call

        let pm = self.process_manager.read().await;
        pm.get_process_status(process_id).await
    }

    /// Apply resource limits to a VM instance
    pub async fn apply_resource_limits(&self, id: Uuid, limits: ResourceLimits) -> Result<(), AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        let process_id = inst.process_id
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} has no process", id)))?;

        drop(instances); // Release lock before async call

        self.resource_limiter.apply_limits(process_id, &limits).await
    }

    /// Get resource usage for a VM instance
    pub async fn get_instance_resource_usage(&self, id: Uuid) -> Result<ResourceUsage, AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        let process_id = inst.process_id
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} has no process", id)))?;

        drop(instances); // Release lock before async call

        self.resource_limiter.get_usage(process_id).await
    }

    /// Check if resource limits are supported on this platform
    pub fn is_resource_limits_supported(&self) -> bool {
        self.resource_limiter.is_supported()
    }
}

static VM_MANAGER: OnceLock<Arc<VmManager>> = OnceLock::new();

/// Get global VM manager instance.
pub fn get_global_manager() -> Arc<VmManager> {
    VM_MANAGER
        .get_or_init(|| Arc::new(VmManager::new()))
        .clone()
}

/// Initialize the VM module.
pub async fn initialize() -> Result<(), AppError> {
    get_global_manager().initialize().await
}

/// Shutdown the VM module.
pub async fn shutdown() -> Result<(), AppError> {
    get_global_manager().shutdown().await
}


