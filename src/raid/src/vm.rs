use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use chrono::{DateTime, Utc};
use std::time::Duration;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;

#[derive(Error, Debug)]
pub enum Error {
    #[error("VM error: {0}")]
    VmError(String),
    #[error("Resource error: {0}")]
    ResourceError(String),
    #[error("Device error: {0}")]
    DeviceError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    pub id: String,
    pub name: String,
    pub cpu_cores: u32,
    pub memory_mb: u32,
    pub disk_gb: u32,
    pub image: String,
    pub status: VmStatus,
    pub ports: Vec<PortMapping>,
    pub max_restart_attempts: u32,
    pub restart_delay_ms: u64,
    pub health_check_interval_ms: u64,
    pub auto_restart: bool,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VmStatus {
    Stopped,
    Running,
    Paused,
    Error(String),
    Restarting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub host_port: u16,
    pub vm_port: u16,
    pub protocol: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    CPU,
    GPU,
    Memory,
    Storage,
    Network,
    USB,
    PCIe,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Available,
    InUse,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStats {
    pub uptime: Duration,
    pub cpu_usage: f32,
    pub memory_usage: u32,
    pub disk_usage: u32,
    pub network_in: u64,
    pub network_out: u64,
    pub last_health_check: Option<DateTime<Utc>>,
    pub restart_count: u32,
    pub last_error: Option<String>,
}

pub struct VmManager {
    vms: Arc<RwLock<HashMap<String, VmConfig>>>,
    stats: Arc<RwLock<HashMap<String, VmStats>>>,
    devices: Arc<RwLock<HashMap<String, Device>>>,
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            vms: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        // Initialize default devices
        self.init_default_devices().await?;
        Ok(())
    }

    async fn init_default_devices(&self) -> Result<(), Error> {
        let mut devices = self.devices.write().await;

        // Add CPU device
        devices.insert("cpu".to_string(), Device {
            id: "cpu".to_string(),
            name: "CPU".to_string(),
            device_type: DeviceType::CPU,
            status: DeviceStatus::Available,
        });

        // Add Memory device
        devices.insert("memory".to_string(), Device {
            id: "memory".to_string(),
            name: "Memory".to_string(),
            device_type: DeviceType::Memory,
            status: DeviceStatus::Available,
        });

        // Add Storage device
        devices.insert("storage".to_string(), Device {
            id: "storage".to_string(),
            name: "Storage".to_string(),
            device_type: DeviceType::Storage,
            status: DeviceStatus::Available,
        });

        Ok(())
    }

    pub async fn create_vm(&self, config: VmConfig) -> Result<(), Error> {
        let mut vms = self.vms.write().await;
        if vms.contains_key(&config.id) {
            return Err(Error::VmError(format!("VM with id {} already exists", config.id)));
        }

        // Validate VM configuration
        self.validate_vm_config(&config).await?;

        // Initialize VM stats
        let stats = VmStats {
            uptime: Duration::from_secs(0),
            cpu_usage: 0.0,
            memory_usage: 0,
            disk_usage: 0,
            network_in: 0,
            network_out: 0,
            last_health_check: None,
            restart_count: 0,
            last_error: None,
        };

        vms.insert(config.id.clone(), config);
        self.stats.write().await.insert(config.id.clone(), stats);
        Ok(())
    }

    async fn validate_vm_config(&self, config: &VmConfig) -> Result<(), Error> {
        if config.cpu_cores == 0 {
            return Err(Error::VmError("CPU cores must be greater than 0".to_string()));
        }
        if config.memory_mb == 0 {
            return Err(Error::VmError("Memory must be greater than 0".to_string()));
        }
        if config.disk_gb == 0 {
            return Err(Error::VmError("Disk size must be greater than 0".to_string()));
        }
        if config.max_restart_attempts == 0 {
            return Err(Error::VmError("Max restart attempts must be greater than 0".to_string()));
        }
        if config.restart_delay_ms == 0 {
            return Err(Error::VmError("Restart delay must be greater than 0".to_string()));
        }
        if config.health_check_interval_ms == 0 {
            return Err(Error::VmError("Health check interval must be greater than 0".to_string()));
        }

        // Check resource availability
        self.check_resource_availability(config).await?;

        Ok(())
    }

    async fn check_resource_availability(&self, config: &VmConfig) -> Result<(), Error> {
        let devices = self.devices.read().await;
        let vms = self.vms.read().await;

        // Check CPU availability
        let total_cpu: u32 = vms.values()
            .filter(|v| v.status == VmStatus::Running)
            .map(|v| v.cpu_cores)
            .sum();

        if total_cpu + config.cpu_cores > 16 { // Example: 16 CPU cores limit
            return Err(Error::ResourceError("Not enough CPU cores available".to_string()));
        }

        // Check memory availability
        let total_memory: u32 = vms.values()
            .filter(|v| v.status == VmStatus::Running)
            .map(|v| v.memory_mb)
            .sum();

        if total_memory + config.memory_mb > 16384 { // Example: 16GB total memory limit
            return Err(Error::ResourceError("Not enough memory available".to_string()));
        }

        // Check device availability
        for device in &config.devices {
            if let Some(d) = devices.get(&device.id) {
                if d.status != DeviceStatus::Available {
                    return Err(Error::DeviceError(format!("Device {} is not available", device.id)));
                }
            } else {
                return Err(Error::DeviceError(format!("Device {} not found", device.id)));
            }
        }

        Ok(())
    }

    pub async fn start_vm(&self, id: &str) -> Result<(), Error> {
        let mut vms = self.vms.write().await;
        let mut stats = self.stats.write().await;
        
        if let Some(vm) = vms.get_mut(id) {
            if vm.status == VmStatus::Running {
                return Err(Error::VmError("VM is already running".to_string()));
            }

            vm.status = VmStatus::Running;
            if let Some(vm_stats) = stats.get_mut(id) {
                vm_stats.uptime = Duration::from_secs(0);
                vm_stats.last_health_check = Some(Utc::now());
            }

            Ok(())
        } else {
            Err(Error::VmError(format!("VM with id {} not found", id)))
        }
    }

    pub async fn stop_vm(&self, id: &str) -> Result<(), Error> {
        let mut vms = self.vms.write().await;
        let mut stats = self.stats.write().await;
        
        if let Some(vm) = vms.get_mut(id) {
            if vm.status == VmStatus::Stopped {
                return Err(Error::VmError("VM is already stopped".to_string()));
            }

            vm.status = VmStatus::Stopped;
            if let Some(vm_stats) = stats.get_mut(id) {
                vm_stats.last_health_check = None;
            }

            Ok(())
        } else {
            Err(Error::VmError(format!("VM with id {} not found", id)))
        }
    }

    pub async fn get_vm(&self, id: &str) -> Option<VmConfig> {
        self.vms.read().await.get(id).cloned()
    }

    pub async fn get_vm_stats(&self, id: &str) -> Option<VmStats> {
        self.stats.read().await.get(id).cloned()
    }

    pub async fn list_vms(&self) -> Vec<VmConfig> {
        self.vms.read().await.values().cloned().collect()
    }

    pub async fn add_device(&self, device: Device) -> Result<(), Error> {
        let mut devices = self.devices.write().await;
        if devices.contains_key(&device.id) {
            return Err(Error::DeviceError(format!("Device with id {} already exists", device.id)));
        }
        devices.insert(device.id.clone(), device);
        Ok(())
    }

    pub async fn remove_device(&self, id: &str) -> Result<(), Error> {
        let mut devices = self.devices.write().await;
        if !devices.contains_key(id) {
            return Err(Error::DeviceError(format!("Device with id {} not found", id)));
        }
        devices.remove(id);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Stop all running VMs
        let vms = self.vms.read().await;
        for (id, vm) in vms.iter() {
            if vm.status == VmStatus::Running {
                self.stop_vm(id).await?;
            }
        }
        Ok(())
    }
} 