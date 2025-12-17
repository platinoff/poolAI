//! Virtual Machines (VM) module
//!
//! Concept alignment:
//! - VM instance management (planned in `poolAI_concept.txt`)
//! - Isolation/security hooks (stubbed)
//! - Resource optimization primitives (basic)

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::info;
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
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
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
        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        inst.status = VmStatus::Running;
        Ok(())
    }

    pub async fn stop_instance(&self, id: Uuid) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!("VM instance {} not found", id)))?;

        inst.status = VmStatus::Stopped;
        Ok(())
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


