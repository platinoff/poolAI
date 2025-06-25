pub mod instance;

use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse, ModelConfig};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub max_instances: usize,
    pub instance_timeout_ms: u64,
    pub auto_scaling: bool,
    pub scaling_threshold: f32,
    pub resource_limit_mb: usize,
}

#[derive(Debug, Clone)]
pub struct InstanceInfo {
    pub instance_id: String,
    pub model_name: String,
    pub status: InstanceStatus,
    pub created_at: std::time::Instant,
    pub last_activity: std::time::Instant,
    pub resource_usage_mb: f32,
    pub request_count: u64,
}

#[derive(Debug, Clone)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

pub struct Runtime {
    config: RuntimeConfig,
    instances: Arc<RwLock<HashMap<String, instance::Instance>>>,
    instance_info: Arc<RwLock<HashMap<String, InstanceInfo>>>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            instances: Arc::new(RwLock::new(HashMap::new())),
            instance_info: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_instance(&self, model_config: ModelConfig) -> Result<String, AppError> {
        let instance_id = self.generate_instance_id();
        
        // Проверка лимитов
        let instances = self.instances.read().await;
        if instances.len() >= self.config.max_instances {
            return Err(AppError::ResourceLimitExceeded);
        }
        drop(instances);
        
        // Создание экземпляра
        let instance = instance::Instance::new(instance_id.clone(), model_config).await?;
        
        // Регистрация экземпляра
        {
            let mut instances = self.instances.write().await;
            instances.insert(instance_id.clone(), instance);
        }
        
        // Создание информации об экземпляре
        let instance_info = InstanceInfo {
            instance_id: instance_id.clone(),
            model_name: model_config.model_path.clone(),
            status: InstanceStatus::Starting,
            created_at: std::time::Instant::now(),
            last_activity: std::time::Instant::now(),
            resource_usage_mb: 0.0,
            request_count: 0,
        };
        
        {
            let mut info = self.instance_info.write().await;
            info.insert(instance_id.clone(), instance_info);
        }
        
        Ok(instance_id)
    }

    pub async fn destroy_instance(&self, instance_id: &str) -> Result<(), AppError> {
        // Остановка экземпляра
        if let Some(instance) = self.instances.read().await.get(instance_id) {
            instance.shutdown().await?;
        }
        
        // Удаление из регистра
        {
            let mut instances = self.instances.write().await;
            instances.remove(instance_id);
        }
        
        {
            let mut info = self.instance_info.write().await;
            info.remove(instance_id);
        }
        
        Ok(())
    }

    pub async fn process_request(&self, instance_id: &str, request: ModelRequest) -> Result<ModelResponse, AppError> {
        let instance = {
            let instances = self.instances.read().await;
            instances.get(instance_id)
                .ok_or(AppError::InstanceNotFound)?
                .clone()
        };
        
        // Обновление активности
        {
            let mut info = self.instance_info.write().await;
            if let Some(instance_info) = info.get_mut(instance_id) {
                instance_info.last_activity = std::time::Instant::now();
                instance_info.request_count += 1;
                instance_info.status = InstanceStatus::Running;
            }
        }
        
        // Обработка запроса
        let response = instance.process_request(request).await?;
        
        Ok(response)
    }

    pub async fn get_instance_info(&self, instance_id: &str) -> Option<InstanceInfo> {
        let info = self.instance_info.read().await;
        info.get(instance_id).cloned()
    }

    pub async fn list_instances(&self) -> Vec<InstanceInfo> {
        let info = self.instance_info.read().await;
        info.values().cloned().collect()
    }

    pub async fn scale_up(&self) -> Result<(), AppError> {
        if !self.config.auto_scaling {
            return Ok(());
        }
        
        // Логика масштабирования
        let instances = self.instances.read().await;
        if instances.len() < self.config.max_instances {
            // Создание нового экземпляра
            todo!("Implement scaling logic")
        }
        
        Ok(())
    }

    pub async fn scale_down(&self) -> Result<(), AppError> {
        if !self.config.auto_scaling {
            return Ok(());
        }
        
        // Поиск неактивных экземпляров для остановки
        let mut instances_to_stop = Vec::new();
        
        {
            let info = self.instance_info.read().await;
            let now = std::time::Instant::now();
            
            for (instance_id, instance_info) in info.iter() {
                if now.duration_since(instance_info.last_activity).as_secs() > 300 { // 5 минут
                    instances_to_stop.push(instance_id.clone());
                }
            }
        }
        
        // Остановка неактивных экземпляров
        for instance_id in instances_to_stop {
            self.destroy_instance(&instance_id).await?;
        }
        
        Ok(())
    }

    pub async fn health_check(&self) -> Result<(), AppError> {
        let mut instances_to_remove = Vec::new();
        
        {
            let instances = self.instances.read().await;
            let mut info = self.instance_info.write().await;
            
            for (instance_id, instance_info) in info.iter_mut() {
                // Проверка таймаута
                let now = std::time::Instant::now();
                if now.duration_since(instance_info.last_activity).as_millis() > self.config.instance_timeout_ms as u128 {
                    instances_to_remove.push(instance_id.clone());
                    continue;
                }
                
                // Проверка здоровья экземпляра
                if let Some(instance) = instances.get(instance_id) {
                    match instance.health_check().await {
                        Ok(true) => {
                            instance_info.status = InstanceStatus::Running;
                        }
                        Ok(false) => {
                            instance_info.status = InstanceStatus::Error;
                            instances_to_remove.push(instance_id.clone());
                        }
                        Err(_) => {
                            instance_info.status = InstanceStatus::Error;
                            instances_to_remove.push(instance_id.clone());
                        }
                    }
                }
            }
        }
        
        // Удаление проблемных экземпляров
        for instance_id in instances_to_remove {
            self.destroy_instance(&instance_id).await?;
        }
        
        Ok(())
    }

    fn generate_instance_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        rand::random::<u64>().hash(&mut hasher);
        
        format!("instance_{:x}", hasher.finish())
    }
} 