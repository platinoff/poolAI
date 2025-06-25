pub mod vm;
pub mod gpu;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VMConfig {
    pub max_vms: usize,
    pub default_memory_gb: f32,
    pub default_cpu_cores: usize,
    pub enable_gpu_passthrough: bool,
    pub enable_asic_passthrough: bool,
    pub storage_path: String,
    pub network_bridge: String,
}

#[derive(Debug, Clone)]
pub struct VMInstance {
    pub id: String,
    pub name: String,
    pub status: VMStatus,
    pub config: VMInstanceConfig,
    pub resources: VMResources,
    pub created_at: std::time::Instant,
    pub last_activity: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum VMStatus {
    Creating,
    Running,
    Stopped,
    Paused,
    Error,
    Destroyed,
}

#[derive(Debug, Clone)]
pub struct VMInstanceConfig {
    pub memory_gb: f32,
    pub cpu_cores: usize,
    pub disk_size_gb: f32,
    pub gpu_devices: Vec<String>,
    pub asic_devices: Vec<String>,
    pub network_interfaces: Vec<VMNetworkInterface>,
    pub os_image: String,
    pub startup_script: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VMNetworkInterface {
    pub name: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub bridge: String,
}

#[derive(Debug, Clone)]
pub struct VMResources {
    pub memory_used_gb: f32,
    pub cpu_usage_percent: f32,
    pub disk_used_gb: f32,
    pub network_rx_mbps: f32,
    pub network_tx_mbps: f32,
    pub gpu_utilization: HashMap<String, f32>,
    pub asic_utilization: HashMap<String, f32>,
}

pub struct VM {
    config: VMConfig,
    instances: Arc<RwLock<HashMap<String, VMInstance>>>,
    vm_manager: Arc<vm::VMManager>,
    gpu_manager: Arc<gpu::GPUManager>,
}

impl VM {
    pub fn new(config: VMConfig) -> Result<Self, AppError> {
        let vm_manager = Arc::new(vm::VMManager::new(config.clone())?);
        let gpu_manager = Arc::new(gpu::GPUManager::new()?);
        
        Ok(Self {
            config,
            instances: Arc::new(RwLock::new(HashMap::new())),
            vm_manager,
            gpu_manager,
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Initialize VM manager
        self.vm_manager.initialize().await?;
        
        // Initialize GPU manager
        self.gpu_manager.initialize().await?;
        
        // Restore existing VMs
        self.restore_vm_instances().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Stop all VMs
        let instances = self.instances.read().await;
        let vm_ids: Vec<String> = instances.keys().cloned().collect();
        
        for vm_id in vm_ids {
            self.stop_vm(&vm_id).await?;
        }
        
        // Shutdown managers
        self.vm_manager.shutdown().await?;
        self.gpu_manager.shutdown().await?;
        
        Ok(())
    }

    pub async fn create_vm(&self, name: String, config: VMInstanceConfig) -> Result<String, AppError> {
        // Check limits
        let instances = self.instances.read().await;
        if instances.len() >= self.config.max_vms {
            return Err(AppError::Resource("VM limit exceeded".to_string()));
        }
        drop(instances);
        
        // Generate VM ID
        let vm_id = self.generate_vm_id();
        
        // Create VM through manager
        self.vm_manager.create_vm(&vm_id, &name, &config).await?;
        
        // Create VM record
        let vm_instance = VMInstance {
            id: vm_id.clone(),
            name,
            status: VMStatus::Creating,
            config,
            resources: VMResources {
                memory_used_gb: 0.0,
                cpu_usage_percent: 0.0,
                disk_used_gb: 0.0,
                network_rx_mbps: 0.0,
                network_tx_mbps: 0.0,
                gpu_utilization: HashMap::new(),
                asic_utilization: HashMap::new(),
            },
            created_at: std::time::Instant::now(),
            last_activity: std::time::Instant::now(),
        };
        
        // Register VM
        {
            let mut instances = self.instances.write().await;
            instances.insert(vm_id.clone(), vm_instance);
        }
        
        // Start VM
        self.start_vm(&vm_id).await?;
        
        Ok(vm_id)
    }

    pub async fn start_vm(&self, vm_id: &str) -> Result<(), AppError> {
        // Start VM through manager
        self.vm_manager.start_vm(vm_id).await?;
        
        // Update status
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                instance.status = VMStatus::Running;
                instance.last_activity = std::time::Instant::now();
            }
        }
        
        Ok(())
    }

    pub async fn stop_vm(&self, vm_id: &str) -> Result<(), AppError> {
        // Stop VM through manager
        self.vm_manager.stop_vm(vm_id).await?;
        
        // Update status
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                instance.status = VMStatus::Stopped;
            }
        }
        
        Ok(())
    }

    pub async fn destroy_vm(&self, vm_id: &str) -> Result<(), AppError> {
        // Stop VM if running
        {
            let instances = self.instances.read().await;
            if let Some(instance) = instances.get(vm_id) {
                if matches!(instance.status, VMStatus::Running) {
                    self.stop_vm(vm_id).await?;
                }
            }
        }
        
        // Destroy VM through manager
        self.vm_manager.destroy_vm(vm_id).await?;
        
        // Remove from instances
        {
            let mut instances = self.instances.write().await;
            instances.remove(vm_id);
        }
        
        Ok(())
    }

    pub async fn get_vm_info(&self, vm_id: &str) -> Option<VMInstance> {
        let instances = self.instances.read().await;
        instances.get(vm_id).cloned()
    }

    pub async fn list_vms(&self) -> Vec<VMInstance> {
        let instances = self.instances.read().await;
        instances.values().cloned().collect()
    }

    pub async fn update_vm_resources(&self, vm_id: &str, resources: VMResources) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;
        
        if let Some(instance) = instances.get_mut(vm_id) {
            instance.resources = resources;
            instance.last_activity = std::time::Instant::now();
            Ok(())
        } else {
            Err(AppError::Model(format!("VM '{}' not found", vm_id)))
        }
    }

    pub async fn attach_gpu(&self, vm_id: &str, gpu_device: &str) -> Result<(), AppError> {
        if !self.config.enable_gpu_passthrough {
            return Err(AppError::Model("GPU passthrough not enabled".to_string()));
        }
        
        // Attach GPU through GPU manager
        self.gpu_manager.attach_to_vm(gpu_device, vm_id).await?;
        
        // Update VM config
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                if !instance.config.gpu_devices.contains(&gpu_device.to_string()) {
                    instance.config.gpu_devices.push(gpu_device.to_string());
                }
            }
        }
        
        Ok(())
    }

    pub async fn detach_gpu(&self, vm_id: &str, gpu_device: &str) -> Result<(), AppError> {
        // Detach GPU through GPU manager
        self.gpu_manager.detach_from_vm(gpu_device, vm_id).await?;
        
        // Update VM config
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                instance.config.gpu_devices.retain(|device| device != gpu_device);
            }
        }
        
        Ok(())
    }

    pub async fn get_vm_metrics(&self, vm_id: &str) -> Result<VMResources, AppError> {
        let instances = self.instances.read().await;
        
        if let Some(instance) = instances.get(vm_id) {
            Ok(instance.resources.clone())
        } else {
            Err(AppError::Model(format!("VM '{}' not found", vm_id)))
        }
    }

    async fn restore_vm_instances(&self) -> Result<(), AppError> {
        // Restore VM instances from storage
        // In real implementation, this would load VM state from disk
        log::info!("Restoring VM instances");
        Ok(())
    }

    fn generate_vm_id(&self) -> String {
        use uuid::Uuid;
        format!("vm-{}", Uuid::new_v4().to_string().split('-').next().unwrap())
    }
} 