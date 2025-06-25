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
        // Инициализация RAID менеджера
        self.raid_manager.initialize().await?;
        
        // Сканирование существующих RAID массивов
        self.scan_existing_arrays().await?;
        
        // Создание RAID массива если указаны устройства
        if !self.config.devices.is_empty() {
            self.create_raid_array("poolai_raid", self.config.clone()).await?;
        }
        
        // Запуск мониторинга
        if self.config.enable_monitoring {
            self.start_monitoring().await?;
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Остановка мониторинга
        // Выключение RAID менеджера
        self.raid_manager.shutdown().await?;
        
        Ok(())
    }

    pub async fn create_raid_array(&self, name: &str, config: RAIDConfig) -> Result<(), AppError> {
        // Создание RAID массива через менеджер
        self.raid_manager.create_array(name, &config).await?;
        
        // Создание записи массива
        let devices: Vec<RAIDDevice> = config.devices.iter().map(|device_path| {
            RAIDDevice {
                device_path: device_path.clone(),
                status: DeviceStatus::Online,
                capacity_gb: 1000.0, // Будет получено из системы
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
        
        // Регистрация массива
        {
            let mut arrays = self.arrays.write().await;
            arrays.insert(name.to_string(), raid_array);
        }
        
        // Инициализация метрик
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
        // Уничтожение RAID массива через менеджер
        self.raid_manager.destroy_array(name).await?;
        
        // Удаление записи массива
        {
            let mut arrays = self.arrays.write().await;
            arrays.remove(name);
        }
        
        // Удаление метрик
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
        // Добавление устройства в RAID массив
        self.raid_manager.add_device(array_name, device_path).await?;
        
        // Обновление записи массива
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                let new_device = RAIDDevice {
                    device_path: device_path.to_string(),
                    status: DeviceStatus::Online,
                    capacity_gb: 1000.0, // Будет получено из системы
                    health_percent: 100.0,
                    last_error: None,
                };
                
                array.devices.push(new_device);
                
                // Пересчет общей емкости
                array.total_capacity_gb = self.calculate_total_capacity(&array.raid_level, &array.devices);
            }
        }
        
        Ok(())
    }

    pub async fn remove_device(&self, array_name: &str, device_path: &str) -> Result<(), AppError> {
        // Удаление устройства из RAID массива
        self.raid_manager.remove_device(array_name, device_path).await?;
        
        // Обновление записи массива
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                array.devices.retain(|device| device.device_path != device_path);
                
                // Пересчет общей емкости
                array.total_capacity_gb = self.calculate_total_capacity(&array.raid_level, &array.devices);
            }
        }
        
        Ok(())
    }

    pub async fn start_rebuild(&self, array_name: &str) -> Result<(), AppError> {
        // Запуск перестроения RAID массива
        self.raid_manager.start_rebuild(array_name).await?;
        
        // Обновление статуса
        {
            let mut arrays = self.arrays.write().await;
            if let Some(array) = arrays.get_mut(array_name) {
                array.status = RAIDStatus::Rebuilding;
            }
        }
        
        Ok(())
    }

    pub async fn get_rebuild_progress(&self, array_name: &str) -> Result<f32, AppError> {
        // Получение прогресса перестроения
        self.raid_manager.get_rebuild_progress(array_name).await
    }

    pub async fn check_health(&self, array_name: &str) -> Result<f32, AppError> {
        // Проверка здоровья RAID массива
        self.raid_manager.check_health(array_name).await
    }

    async fn scan_existing_arrays(&self) -> Result<(), AppError> {
        // Сканирование существующих RAID массивов
        // В реальной реализации здесь будет:
        // - Чтение /proc/mdstat
        // - Анализ mdadm --detail
        // - Восстановление информации о массивах
        
        Ok(())
    }

    async fn start_monitoring(&self) -> Result<(), AppError> {
        let arrays = self.arrays.clone();
        let metrics = self.metrics.clone();
        let raid_manager = self.raid_manager.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Обновление метрик для всех массивов
                let array_names: Vec<String> = {
                    let arrays_read = arrays.read().await;
                    arrays_read.keys().cloned().collect()
                };
                
                for array_name in array_names {
                    if let Ok(array_metrics) = raid_manager.get_metrics(&array_name).await {
                        let mut metrics_write = metrics.write().await;
                        if let Some(metric) = metrics_write.get_mut(&array_name) {
                            metric.read_throughput_mbps = array_metrics.read_throughput_mbps;
                            metric.write_throughput_mbps = array_metrics.write_throughput_mbps;
                            metric.io_operations_per_sec = array_metrics.io_operations_per_sec;
                            metric.average_response_time_ms = array_metrics.average_response_time_ms;
                            metric.error_count = array_metrics.error_count;
                            metric.rebuild_progress_percent = array_metrics.rebuild_progress_percent;
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    fn calculate_total_capacity(&self, raid_level: &RAIDLevel, devices: &[RAIDDevice]) -> f32 {
        let total_devices = devices.len() as f32;
        let device_capacity = devices.iter().map(|d| d.capacity_gb).sum::<f32>();
        
        match raid_level {
            RAIDLevel::RAID0 => device_capacity,
            RAIDLevel::RAID1 => device_capacity / total_devices,
            RAIDLevel::RAID5 => device_capacity * (total_devices - 1.0) / total_devices,
            RAIDLevel::RAID6 => device_capacity * (total_devices - 2.0) / total_devices,
            RAIDLevel::RAID10 => device_capacity / 2.0,
            RAIDLevel::JBOD => device_capacity,
        }
    }
} 