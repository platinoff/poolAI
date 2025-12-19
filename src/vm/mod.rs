//! Virtual Machines (VM) module
//!
//! Concept alignment:
//! - VM instance management (planned in `poolAI_concept.txt`)
//! - Isolation/security hooks (stubbed)
//! - Resource optimization primitives (basic)

use crate::core::error::AppError;
use crate::runtime::health::{HealthMonitor, HealthStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::time::Duration;
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
}

/// VM Manager - central orchestrator for VM instances.
pub struct VmManager {
    instances: Arc<RwLock<HashMap<Uuid, VmInstance>>>,
    health_monitor: Arc<RwLock<HealthMonitor>>,
    #[allow(dead_code)] // Used for periodic health checks
    health_check_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl VmManager {
    pub fn new() -> Self {
        let health_monitor = Arc::new(RwLock::new(HealthMonitor::new(30))); // 30 second interval
        
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            health_monitor,
            health_check_task: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing VM manager");
        
        // Initialize health monitor
        {
            let mut hm = self.health_monitor.write().await;
            hm.initialize().await
                .map_err(|e| AppError::ConfigError(format!("Failed to initialize health monitor: {}", e)))?;
            hm.start().await
                .map_err(|e| AppError::ConfigError(format!("Failed to start health monitor: {}", e)))?;
        }
        
        // Start periodic health checks for running VM instances
        self.start_periodic_health_checks().await;
        
        Ok(())
    }
    
    /// Start periodic health checks for running VM instances
    async fn start_periodic_health_checks(&self) {
        let instances = Arc::clone(&self.instances);
        let health_monitor = Arc::clone(&self.health_monitor);
        
        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Get all running instances
                let running_instances: Vec<(Uuid, String)> = {
                    let insts = instances.read().await;
                    insts.values()
                        .filter(|inst| matches!(inst.status, VmStatus::Running))
                        .map(|inst| (inst.id, inst.name.clone()))
                        .collect()
                };
                
                // Perform health check for each running instance
                for (id, name) in running_instances {
                    let instances_clone = Arc::clone(&instances);
                    let health_status = {
                        let hm = health_monitor.read().await;
                        hm.check_process_health(id, move || {
                            let instances = Arc::clone(&instances_clone);
                            Box::pin(async move {
                                // Check if instance is still running
                                let insts = instances.read().await;
                                match insts.get(&id) {
                                    Some(inst) if matches!(inst.status, VmStatus::Running) => {
                                        Ok(())
                                    }
                                    _ => {
                                        Err(AppError::ValidationError(format!("VM instance {} is not running", id)))
                                    }
                                }
                            })
                        }).await
                    };
                    
                    // Handle unhealthy status
                    if matches!(health_status, HealthStatus::Unhealthy(_)) {
                        warn!("VM instance {} ({}) health check failed", id, name);
                        
                        // Auto-restart if configured
                        let should_restart = {
                            let hm = health_monitor.read().await;
                            // Check failure count and auto-restart config
                            // For now, we'll restart after 3 failures
                            true // TODO: Get from config
                        };
                        
                        if should_restart {
                            warn!("Auto-restarting VM instance {} ({})", id, name);
                            // TODO: Implement restart logic
                            // For now, just mark as failed
                            let mut insts = instances.write().await;
                            if let Some(inst) = insts.get_mut(&id) {
                                inst.status = VmStatus::Failed("Health check failed".to_string());
                            }
                        }
                    }
                }
            }
        });
        
        *self.health_check_task.write().await = Some(task);
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down VM manager");
        
        // Stop health check task
        if let Some(task) = self.health_check_task.write().await.take() {
            task.abort();
        }
        
        // Shutdown health monitor
        {
            let mut hm = self.health_monitor.write().await;
            hm.shutdown().await
                .map_err(|e| AppError::ConfigError(format!("Failed to shutdown health monitor: {}", e)))?;
        }
        
        Ok(())
    }

    pub async fn create_instance(
        &self,
        name: String,
        resources: VmResources,
        isolation: VmIsolation,
    ) -> Result<VmInstance, AppError> {
        let id = Uuid::new_v4();
        let instance = VmInstance {
            id,
            name,
            created_at: Utc::now(),
            status: VmStatus::Creating,
            resources,
            isolation,
        };

        self.instances.write().await.insert(id, instance.clone());
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
        let name = {
            let mut instances = self.instances.write().await;
            let inst = instances
                .get_mut(&id)
                .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

            inst.status = VmStatus::Running;
            inst.name.clone()
        };
        
        // Register health check for this instance
        {
            let hm = self.health_monitor.write().await;
            hm.register_check(id, name).await;
        }
        
        info!("VM instance {} started and registered for health checks", id);
        Ok(())
    }

    pub async fn stop_instance(&self, id: Uuid) -> Result<(), AppError> {
        // Unregister health check
        {
            let hm = self.health_monitor.write().await;
            hm.unregister_check(id).await;
        }
        
        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        inst.status = VmStatus::Stopped;
        info!("VM instance {} stopped and unregistered from health checks", id);
        Ok(())
    }
    
    /// Get health status for a VM instance
    pub async fn get_instance_health(&self, id: Uuid) -> Result<Option<HealthStatus>, AppError> {
        let hm = self.health_monitor.read().await;
        Ok(hm.get_health_status(id).await)
    }
    
    /// Perform manual health check for a VM instance
    pub async fn check_instance_health(&self, id: Uuid) -> Result<HealthStatus, AppError> {
        let instances = Arc::clone(&self.instances);
        let health_monitor = Arc::clone(&self.health_monitor);
        
        let status = {
            let hm = health_monitor.read().await;
            let instances_clone = Arc::clone(&instances);
            hm.check_process_health(id, move || {
                let instances = Arc::clone(&instances_clone);
                Box::pin(async move {
                    // Check if instance is still running
                    let insts = instances.read().await;
                    match insts.get(&id) {
                        Some(inst) if matches!(inst.status, VmStatus::Running) => {
                            Ok(())
                        }
                        Some(_) => {
                            Err(AppError::ValidationError(format!("VM instance {} is not running", id)))
                        }
                        None => {
                            Err(AppError::ValidationError(format!("VM instance {} not found", id)))
                        }
                    }
                })
            }).await
        };
        
        Ok(status)
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


