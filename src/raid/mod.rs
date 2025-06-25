pub mod raid;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RAIDConfig {
    pub raid_level: RAIDLevel,
    pub devices: Vec<String>,
    pub chunk_size_kb: usize,
    pub enable_monitoring: bool,
    pub auto_rebuild: bool,
    pub spare_devices: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum RAIDLevel {
    RAID0,
    RAID1,
    RAID5,
    RAID6,
    RAID10,
    JBOD,
}

#[derive(Debug, Clone)]
pub struct RAIDArray {
    pub name: String,
    pub raid_level: RAIDLevel,
    pub devices: Vec<RAIDDevice>,
    pub status: RAIDStatus,
    pub total_capacity_gb: f32,
    pub used_capacity_gb: f32,
    pub created_at: std::time::Instant,
}

#[derive(Debug, Clone)]
pub struct RAIDDevice {
    pub device_path: String,
    pub status: DeviceStatus,
    pub capacity_gb: f32,
    pub health_percent: f32,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RAIDStatus {
    Optimal,
    Degraded,
    Rebuilding,
    Failed,
    Offline,
}

#[derive(Debug, Clone)]
pub enum DeviceStatus {
    Online,
    Offline,
    Failed,
    Rebuilding,
    Spare,
}

#[derive(Debug, Clone)]
pub struct RAIDMetrics {
    pub array_name: String,
    pub read_throughput_mbps: f32,
    pub write_throughput_mbps: f32,
    pub io_operations_per_sec: f64,
    pub average_response_time_ms: f64,
    pub error_count: u64,
    pub rebuild_progress_percent: f32,
}

pub struct RAID {
    config: RAIDConfig,
    arrays: Arc<RwLock<HashMap<String, RAIDArray>>>,
    metrics: Arc<RwLock<HashMap<String, RAIDMetrics>>>,
    raid_manager: Arc<raid::RAIDManager>,
}

impl RAID {
    pub fn new(config: RAIDConfig) -> Result<Self, AppError> {
        let raid_manager = Arc::new(raid::RAIDManager::new(config.clone())?);
        
        Ok(Self {
            config,
            arrays: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            raid_manager,
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Initialize RAID manager
        self.raid_manager.initialize().await?;
        
        // Scan existing RAID arrays
        self.scan_existing_arrays().await?;
        
        // Create RAID array if devices are specified
        if !self.config.devices.is_empty() {
            self.create_raid_array("poolai_raid", self.config.clone()).await?;
        }
        
        // Start monitoring
        if self.config.enable_monitoring {
            self.start_monitoring().await?;
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Stop monitoring
        // Shutdown RAID manager
        self.raid_manager.shutdown().await?;
        
        Ok(())
    }

    pub async fn create_raid_array(&self, name: &str, config: RAIDConfig) -> Result<(), AppError> {
        // Create RAID array through manager
        self.raid_manager.create_array(name, &config).await?;
        
        // Create array record
        let devices: Vec<RAIDDevice> = config.devices.iter().map(|device_path| {
            RAIDDevice {
                device_path: device_path.clone(),
                status: DeviceStatus::Online,
                capacity_gb: 1000.0, // Will be obtained from system
                health_percent: 100.0,
                last_error: None,
            }
        }).collect();
        
        let total_capacity = self.calculate_total_capacity(&config.raid_level, &devices);
        
        let raid_array = RAIDArray {
            name: name.to_string(),
            raid_level: config.raid_level.clone(),
            devices,
            status: RAIDStatus::Optimal,
            total_capacity_gb: total_capacity,
            used_capacity_gb: 0.0,
            created_at: std::time::Instant::now(),
        };
        
        // Register array
        {
            let mut arrays = self.arrays.write().await;
            arrays.insert(name.to_string(), raid_array);
        }
        
        // Initialize metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.insert(name.to_string(), RAIDMetrics {
                array_name: name.to_string(),
                read_throughput_mbps: 0.0,
                write_throughput_mbps: 0.0,
                io_operations_per_sec: 0.0,
                average_response_time_ms: 0.0,
                error_count: 0,
                rebuild_progress_percent: 0.0,
            });
        }
        
        Ok(())
    }

    pub async fn destroy_raid_array(&self, name: &str) -> Result<(), AppError> {
        // Destroy RAID array through manager
        self.raid_manager.destroy_array(name).await?;
        
        // Remove array record
        {
            let mut arrays = self.arrays.write().await;
            arrays.remove(name);
        }
        
        // Remove metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.remove(name);
        }
        
        Ok(())
    }

    pub async fn get_raid_info(&self, name: &str) -> Option<RAIDArray> {
        let arrays = self.arrays.read().await;
        arrays.get(name).cloned()
    }

    pub async fn list_raid_arrays(&self) -> Vec<RAIDArray> {
        let arrays = self.arrays.read().await;
        arrays.values().cloned().collect()
    }

    pub async fn get_raid_metrics(&self, name: &str) -> Option<RAIDMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(name).cloned()
    }

    pub async fn add_device(&self, array_name: &str, device_path: &str) -> Result<(), AppError> {
        // Add device through manager
        self.raid_manager.add_device(array_name, device_path).await?;
        
        // Update array record
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                let new_device = RAIDDevice {
                    device_path: device_path.to_string(),
                    status: DeviceStatus::Online,
                    capacity_gb: 1000.0,
                    health_percent: 100.0,
                    last_error: None,
                };
                array.devices.push(new_device);
                
                // Recalculate total capacity
                array.total_capacity_gb = self.calculate_total_capacity(&array.raid_level, &array.devices);
            }
        }
        
        Ok(())
    }

    pub async fn remove_device(&self, array_name: &str, device_path: &str) -> Result<(), AppError> {
        // Remove device through manager
        self.raid_manager.remove_device(array_name, device_path).await?;
        
        // Update array record
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                array.devices.retain(|device| device.device_path != device_path);
                
                // Recalculate total capacity
                array.total_capacity_gb = self.calculate_total_capacity(&array.raid_level, &array.devices);
            }
        }
        
        Ok(())
    }

    pub async fn start_rebuild(&self, array_name: &str) -> Result<(), AppError> {
        // Start rebuild through manager
        self.raid_manager.start_rebuild(array_name).await?;
        
        // Update array status
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                array.status = RAIDStatus::Rebuilding;
            }
        }
        
        Ok(())
    }

    pub async fn get_rebuild_progress(&self, array_name: &str) -> Result<f32, AppError> {
        self.raid_manager.get_rebuild_progress(array_name).await
    }

    pub async fn check_health(&self, array_name: &str) -> Result<f32, AppError> {
        self.raid_manager.check_health(array_name).await
    }

    async fn scan_existing_arrays(&self) -> Result<(), AppError> {
        // Scan existing RAID arrays
        // In real implementation, this would scan system for existing arrays
        log::info!("Scanning existing RAID arrays");
        Ok(())
    }

    async fn start_monitoring(&self) -> Result<(), AppError> {
        let arrays = self.arrays.clone();
        let metrics = self.metrics.clone();
        let raid_manager = self.raid_manager.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let array_names: Vec<String> = {
                    let arrays = arrays.read().await;
                    arrays.keys().cloned().collect()
                };
                
                for array_name in array_names {
                    // Update metrics
                    if let Ok(array_metrics) = raid_manager.get_metrics(&array_name).await {
                        let mut metrics = metrics.write().await;
                        if let Some(existing_metrics) = metrics.get_mut(&array_name) {
                            existing_metrics.read_throughput_mbps = array_metrics.read_throughput_mbps;
                            existing_metrics.write_throughput_mbps = array_metrics.write_throughput_mbps;
                            existing_metrics.io_operations_per_sec = array_metrics.io_operations_per_sec;
                            existing_metrics.average_response_time_ms = array_metrics.average_response_time_ms;
                            existing_metrics.error_count = array_metrics.error_count;
                            existing_metrics.rebuild_progress_percent = array_metrics.rebuild_progress_percent;
                        }
                    }
                    
                    // Check health
                    if let Ok(health) = raid_manager.check_health(&array_name).await {
                        if health < 50.0 {
                            log::warn!("RAID array {} health is low: {}%", array_name, health);
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    fn calculate_total_capacity(&self, raid_level: &RAIDLevel, devices: &[RAIDDevice]) -> f32 {
        let total_capacity: f32 = devices.iter().map(|d| d.capacity_gb).sum();
        
        match raid_level {
            RAIDLevel::RAID0 => total_capacity,
            RAIDLevel::RAID1 => total_capacity / 2.0,
            RAIDLevel::RAID5 => total_capacity * (devices.len() as f32 - 1.0) / devices.len() as f32,
            RAIDLevel::RAID6 => total_capacity * (devices.len() as f32 - 2.0) / devices.len() as f32,
            RAIDLevel::RAID10 => total_capacity / 2.0,
            RAIDLevel::JBOD => total_capacity,
        }
    }
} 