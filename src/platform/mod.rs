pub mod windows;
pub mod linux;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub platform_type: PlatformType,
    pub gpu_devices: Vec<GpuDevice>,
    pub cpu_cores: usize,
    pub memory_gb: f32,
    pub enable_power_management: bool,
    pub temperature_threshold_celsius: f32,
}

#[derive(Debug, Clone)]
pub enum PlatformType {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub device_id: String,
    pub name: String,
    pub memory_mb: f32,
    pub compute_capability: String,
    pub driver_version: String,
    pub is_available: bool,
}

#[derive(Debug, Clone)]
pub struct SystemResources {
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub memory_total_mb: f32,
    pub disk_usage_percent: f32,
    pub network_throughput_mbps: f32,
    pub temperature_celsius: f32,
    pub power_consumption_watts: f32,
}

#[derive(Debug, Clone)]
pub struct GpuResources {
    pub device_id: String,
    pub utilization_percent: f32,
    pub memory_used_mb: f32,
    pub memory_total_mb: f32,
    pub temperature_celsius: f32,
    pub power_consumption_watts: f32,
    pub fan_speed_percent: f32,
}

pub struct Platform {
    config: PlatformConfig,
    system_resources: Arc<RwLock<SystemResources>>,
    gpu_resources: Arc<RwLock<HashMap<String, GpuResources>>>,
    platform_impl: Box<dyn PlatformInterface + Send + Sync>,
}

impl Platform {
    pub fn new(config: PlatformConfig) -> Result<Self, AppError> {
        let platform_impl: Box<dyn PlatformInterface + Send + Sync> = match config.platform_type {
            PlatformType::Windows => Box::new(windows::WindowsPlatform::new()?),
            PlatformType::Linux => Box::new(linux::LinuxPlatform::new()?),
            PlatformType::MacOS => {
                return Err(AppError::Model("Unsupported platform".to_string()));
            }
            PlatformType::Unknown => {
                return Err(AppError::Model("Unknown platform".to_string()));
            }
        };
        
        let system_resources = SystemResources {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            memory_total_mb: config.memory_gb * 1024.0,
            disk_usage_percent: 0.0,
            network_throughput_mbps: 0.0,
            temperature_celsius: 0.0,
            power_consumption_watts: 0.0,
        };
        
        let mut gpu_resources = HashMap::new();
        for gpu in &config.gpu_devices {
            gpu_resources.insert(gpu.device_id.clone(), GpuResources {
                device_id: gpu.device_id.clone(),
                utilization_percent: 0.0,
                memory_used_mb: 0.0,
                memory_total_mb: gpu.memory_mb,
                temperature_celsius: 0.0,
                power_consumption_watts: 0.0,
                fan_speed_percent: 0.0,
            });
        }
        
        Ok(Self {
            config,
            system_resources: Arc::new(RwLock::new(system_resources)),
            gpu_resources: Arc::new(RwLock::new(gpu_resources)),
            platform_impl,
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Initialize platform
        self.platform_impl.initialize().await?;
        
        // Start resource monitoring
        self.start_resource_monitoring().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Shutdown platform
        self.platform_impl.shutdown().await?;
        
        Ok(())
    }

    pub async fn get_system_resources(&self) -> SystemResources {
        self.system_resources.read().await.clone()
    }

    pub async fn get_gpu_resources(&self, device_id: &str) -> Option<GpuResources> {
        let gpu_resources = self.gpu_resources.read().await;
        gpu_resources.get(device_id).cloned()
    }

    pub async fn get_all_gpu_resources(&self) -> Vec<GpuResources> {
        let gpu_resources = self.gpu_resources.read().await;
        gpu_resources.values().cloned().collect()
    }

    pub async fn allocate_gpu_memory(&self, device_id: &str, size_mb: f32) -> Result<(), AppError> {
        let mut gpu_resources = self.gpu_resources.write().await;
        
        if let Some(gpu) = gpu_resources.get_mut(device_id) {
            let available_memory = gpu.memory_total_mb - gpu.memory_used_mb;
            
            if available_memory >= size_mb {
                gpu.memory_used_mb += size_mb;
                Ok(())
            } else {
                Err(AppError::Resource("Insufficient GPU memory".to_string()))
            }
        } else {
            Err(AppError::Model(format!("GPU device '{}' not found", device_id)))
        }
    }

    pub async fn release_gpu_memory(&self, device_id: &str, size_mb: f32) -> Result<(), AppError> {
        let mut gpu_resources = self.gpu_resources.write().await;
        
        if let Some(gpu) = gpu_resources.get_mut(device_id) {
            gpu.memory_used_mb = gpu.memory_used_mb.saturating_sub(size_mb);
            Ok(())
        } else {
            Err(AppError::Model(format!("GPU device '{}' not found", device_id)))
        }
    }

    pub async fn optimize_resources(&self) -> Result<(), AppError> {
        // Optimize resources
        self.platform_impl.optimize_resources().await?;
        
        Ok(())
    }

    pub async fn check_temperature(&self) -> Result<bool, AppError> {
        let system_resources = self.system_resources.read().await;
        let gpu_resources = self.gpu_resources.read().await;
        
        // Check system temperature
        if system_resources.temperature_celsius > self.config.temperature_threshold_celsius {
            return Ok(false);
        }
        
        // Check GPU temperature
        for gpu in gpu_resources.values() {
            if gpu.temperature_celsius > self.config.temperature_threshold_celsius {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    pub async fn get_available_gpus(&self) -> Vec<GpuDevice> {
        self.config.gpu_devices
            .iter()
            .filter(|gpu| gpu.is_available)
            .cloned()
            .collect()
    }

    async fn start_resource_monitoring(&self) -> Result<(), AppError> {
        let system_resources = self.system_resources.clone();
        let gpu_resources = self.gpu_resources.clone();
        let platform_impl = self.platform_impl.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Update system resources
                if let Ok(resources) = platform_impl.get_system_resources().await {
                    *system_resources.write().await = resources;
                }
                
                // Update GPU resources
                if let Ok(gpu_res) = platform_impl.get_gpu_resources().await {
                    *gpu_resources.write().await = gpu_res;
                }
            }
        });
        
        Ok(())
    }
}

pub trait PlatformInterface {
    async fn initialize(&self) -> Result<(), AppError>;
    async fn shutdown(&self) -> Result<(), AppError>;
    async fn get_system_resources(&self) -> Result<SystemResources, AppError>;
    async fn get_gpu_resources(&self) -> Result<HashMap<String, GpuResources>, AppError>;
    async fn optimize_resources(&self) -> Result<(), AppError>;
    fn clone(&self) -> Box<dyn PlatformInterface + Send + Sync>;
} 