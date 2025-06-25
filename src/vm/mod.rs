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
        // Инициализация VM менеджера
        self.vm_manager.initialize().await?;
        
        // Инициализация GPU менеджера
        self.gpu_manager.initialize().await?;
        
        // Восстановление существующих VM
        self.restore_vm_instances().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Остановка всех VM
        let instances = self.instances.read().await;
        let vm_ids: Vec<String> = instances.keys().cloned().collect();
        
        for vm_id in vm_ids {
            self.stop_vm(&vm_id).await?;
        }
        
        // Выключение менеджеров
        self.vm_manager.shutdown().await?;
        self.gpu_manager.shutdown().await?;
        
        Ok(())
    }

    pub async fn create_vm(&self, name: String, config: VMInstanceConfig) -> Result<String, AppError> {
        // Проверка лимитов
        let instances = self.instances.read().await;
        if instances.len() >= self.config.max_vms {
            return Err(AppError::ResourceLimitExceeded);
        }
        drop(instances);
        
        // Генерация ID VM
        let vm_id = self.generate_vm_id();
        
        // Создание VM через менеджер
        self.vm_manager.create_vm(&vm_id, &name, &config).await?;
        
        // Создание записи VM
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
        
        // Регистрация VM
        {
            let mut instances = self.instances.write().await;
            instances.insert(vm_id.clone(), vm_instance);
        }
        
        // Запуск VM
        self.start_vm(&vm_id).await?;
        
        Ok(vm_id)
    }

    pub async fn start_vm(&self, vm_id: &str) -> Result<(), AppError> {
        // Запуск VM через менеджер
        self.vm_manager.start_vm(vm_id).await?;
        
        // Обновление статуса
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
        // Остановка VM через менеджер
        self.vm_manager.stop_vm(vm_id).await?;
        
        // Обновление статуса
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                instance.status = VMStatus::Stopped;
            }
        }
        
        Ok(())
    }

    pub async fn destroy_vm(&self, vm_id: &str) -> Result<(), AppError> {
        // Остановка VM если запущена
        {
            let instances = self.instances.read().await;
            if let Some(instance) = instances.get(vm_id) {
                if matches!(instance.status, VMStatus::Running) {
                    self.stop_vm(vm_id).await?;
                }
            }
        }
        
        // Уничтожение VM через менеджер
        self.vm_manager.destroy_vm(vm_id).await?;
        
        // Удаление записи
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
        }
        
        Ok(())
    }

    pub async fn attach_gpu(&self, vm_id: &str, gpu_device: &str) -> Result<(), AppError> {
        // Проверка доступности GPU
        if !self.gpu_manager.is_gpu_available(gpu_device).await? {
            return Err(AppError::DeviceNotAvailable);
        }
        
        // Присоединение GPU к VM
        self.vm_manager.attach_gpu(vm_id, gpu_device).await?;
        
        // Обновление конфигурации VM
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
        // Отсоединение GPU от VM
        self.vm_manager.detach_gpu(vm_id, gpu_device).await?;
        
        // Обновление конфигурации VM
        {
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.get_mut(vm_id) {
                instance.config.gpu_devices.retain(|device| device != gpu_device);
            }
        }
        
        Ok(())
    }

    pub async fn get_vm_metrics(&self, vm_id: &str) -> Result<VMResources, AppError> {
        // Получение метрик VM через менеджер
        let resources = self.vm_manager.get_vm_resources(vm_id).await?;
        
        // Обновление локальных данных
        self.update_vm_resources(vm_id, resources.clone()).await?;
        
        Ok(resources)
    }

    async fn restore_vm_instances(&self) -> Result<(), AppError> {
        // Восстановление VM из сохраненного состояния
        // В реальной реализации здесь будет:
        // - Чтение конфигураций VM
        // - Восстановление состояния
        // - Запуск VM если необходимо
        
        Ok(())
    }

    fn generate_vm_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        rand::random::<u64>().hash(&mut hasher);
        
        format!("vm_{:x}", hasher.finish())
    }
} 