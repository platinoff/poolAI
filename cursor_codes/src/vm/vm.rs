use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use log::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use chrono::{DateTime, Utc};

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
    health_check_handles: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            vms: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            health_check_handles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_vm(&self, config: VmConfig) -> Result<(), String> {
        let mut vms = self.vms.write();
        if vms.contains_key(&config.id) {
            return Err(format!("VM with id {} already exists", config.id));
        }

        // Validate VM configuration
        self.validate_vm_config(&config)?;

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
        self.stats.write().insert(config.id.clone(), stats);
        info!("Created new VM: {}", config.id);
        Ok(())
    }

    fn validate_vm_config(&self, config: &VmConfig) -> Result<(), String> {
        if config.cpu_cores == 0 {
            return Err("CPU cores must be greater than 0".to_string());
        }
        if config.memory_mb == 0 {
            return Err("Memory must be greater than 0".to_string());
        }
        if config.disk_gb == 0 {
            return Err("Disk size must be greater than 0".to_string());
        }
        if config.max_restart_attempts == 0 {
            return Err("Max restart attempts must be greater than 0".to_string());
        }
        if config.restart_delay_ms == 0 {
            return Err("Restart delay must be greater than 0".to_string());
        }
        if config.health_check_interval_ms == 0 {
            return Err("Health check interval must be greater than 0".to_string());
        }
        Ok(())
    }

    pub async fn start_vm(&self, id: &str) -> Result<(), String> {
        let mut vms = self.vms.write();
        let mut stats = self.stats.write();
        
        if let Some(vm) = vms.get_mut(id) {
            if vm.status == VmStatus::Running {
                return Err("VM is already running".to_string());
            }

            // Check resource availability
            self.check_resource_availability(vm)?;

            vm.status = VmStatus::Running;
            if let Some(vm_stats) = stats.get_mut(id) {
                vm_stats.uptime = Duration::from_secs(0);
                vm_stats.last_health_check = Some(Utc::now());
            }

            // Start health check
            self.start_health_check(id).await?;

            info!("Started VM: {}", id);
            Ok(())
        } else {
            Err(format!("VM with id {} not found", id))
        }
    }

    pub async fn stop_vm(&self, id: &str) -> Result<(), String> {
        let mut vms = self.vms.write();
        let mut stats = self.stats.write();
        let mut handles = self.health_check_handles.write();
        
        if let Some(vm) = vms.get_mut(id) {
            if vm.status == VmStatus::Stopped {
                return Err("VM is already stopped".to_string());
            }

            vm.status = VmStatus::Stopped;
            if let Some(vm_stats) = stats.get_mut(id) {
                vm_stats.last_health_check = None;
            }

            // Stop health check
            if let Some(handle) = handles.remove(id) {
                handle.abort();
            }

            info!("Stopped VM: {}", id);
            Ok(())
        } else {
            Err(format!("VM with id {} not found", id))
        }
    }

    fn check_resource_availability(&self, vm: &VmConfig) -> Result<(), String> {
        // Check if there are enough resources available
        let total_memory: u32 = self.vms.read().values()
            .filter(|v| v.status == VmStatus::Running)
            .map(|v| v.memory_mb)
            .sum();

        if total_memory + vm.memory_mb > 16384 { // Example: 16GB total memory limit
            return Err("Not enough memory available".to_string());
        }

        let total_cpu: u32 = self.vms.read().values()
            .filter(|v| v.status == VmStatus::Running)
            .map(|v| v.cpu_cores)
            .sum();

        if total_cpu + vm.cpu_cores > 16 { // Example: 16 CPU cores limit
            return Err("Not enough CPU cores available".to_string());
        }

        Ok(())
    }

    async fn start_health_check(&self, id: &str) -> Result<(), String> {
        let vms = self.vms.clone();
        let stats = self.stats.clone();
        let handles = self.health_check_handles.clone();

        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(
                    vms.read().get(id).unwrap().health_check_interval_ms
                )).await;

                let mut vms = vms.write();
                let mut stats = stats.write();

                if let Some(vm) = vms.get_mut(id) {
                    if vm.status != VmStatus::Running {
                        break;
                    }

                    if let Some(vm_stats) = stats.get_mut(id) {
                        // Perform health check
                        if !Self::perform_health_check(vm, vm_stats) {
                            if vm.auto_restart && vm_stats.restart_count < vm.max_restart_attempts {
                                vm.status = VmStatus::Restarting;
                                vm_stats.restart_count += 1;
                                
                                // Wait before restart
                                tokio::time::sleep(Duration::from_millis(vm.restart_delay_ms)).await;
                                
                                vm.status = VmStatus::Running;
                                vm_stats.uptime = Duration::from_secs(0);
                            } else {
                                vm.status = VmStatus::Error("Health check failed".to_string());
                            }
                        }
                        vm_stats.last_health_check = Some(Utc::now());
                    }
                }
            }
        });

        handles.write().insert(id.to_string(), handle);
        Ok(())
    }

    fn perform_health_check(vm: &VmConfig, stats: &mut VmStats) -> bool {
        // Simulate health check
        let cpu_usage = rand::random::<f32>();
        let memory_usage = (rand::random::<u32>() % vm.memory_mb) + 1;
        
        stats.cpu_usage = cpu_usage;
        stats.memory_usage = memory_usage;
        
        // Consider VM healthy if CPU usage is below 90% and memory usage is below 95%
        cpu_usage < 0.9 && memory_usage < vm.memory_mb * 95 / 100
    }

    pub fn get_vm(&self, id: &str) -> Option<VmConfig> {
        self.vms.read().get(id).cloned()
    }

    pub fn get_vm_stats(&self, id: &str) -> Option<VmStats> {
        self.stats.read().get(id).cloned()
    }

    pub fn list_vms(&self) -> Vec<VmConfig> {
        self.vms.read().values().cloned().collect()
    }

    pub fn add_port_mapping(&self, id: &str, mapping: PortMapping) -> Result<(), String> {
        let mut vms = self.vms.write();
        if let Some(vm) = vms.get_mut(id) {
            // Check if port is already in use
            if vm.ports.iter().any(|p| p.host_port == mapping.host_port) {
                return Err(format!("Port {} is already mapped", mapping.host_port));
            }
            vm.ports.push(mapping);
            info!("Added port mapping for VM: {}", id);
            Ok(())
        } else {
            Err(format!("VM with id {} not found", id))
        }
    }

    pub fn remove_port_mapping(&self, id: &str, host_port: u16) -> Result<(), String> {
        let mut vms = self.vms.write();
        if let Some(vm) = vms.get_mut(id) {
            vm.ports.retain(|m| m.host_port != host_port);
            info!("Removed port mapping for VM: {}", id);
            Ok(())
        } else {
            Err(format!("VM with id {} not found", id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        let manager = VmManager::new();
        let config = VmConfig {
            id: "test".to_string(),
            name: "Test VM".to_string(),
            cpu_cores: 2,
            memory_mb: 2048,
            disk_gb: 20,
            image: "ubuntu:latest".to_string(),
            status: VmStatus::Stopped,
            ports: Vec::new(),
            max_restart_attempts: 3,
            restart_delay_ms: 5000,
            health_check_interval_ms: 10000,
            auto_restart: true,
        };
        assert!(manager.create_vm(config).is_ok());
    }

    #[test]
    fn test_vm_start_stop() {
        let manager = VmManager::new();
        let config = VmConfig {
            id: "test".to_string(),
            name: "Test VM".to_string(),
            cpu_cores: 2,
            memory_mb: 2048,
            disk_gb: 20,
            image: "ubuntu:latest".to_string(),
            status: VmStatus::Stopped,
            ports: Vec::new(),
            max_restart_attempts: 3,
            restart_delay_ms: 5000,
            health_check_interval_ms: 10000,
            auto_restart: true,
        };
        manager.create_vm(config).unwrap();
        assert!(manager.start_vm("test").is_ok());
        assert_eq!(manager.get_vm("test").unwrap().status, VmStatus::Running);
        assert!(manager.stop_vm("test").is_ok());
        assert_eq!(manager.get_vm("test").unwrap().status, VmStatus::Stopped);
    }
} 