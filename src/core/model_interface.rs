//! Model Interface - Базовый интерфейс для всех моделей
//! 
//! Этот модуль предоставляет:
//! - Базовый интерфейс для всех моделей
//! - Управление запросами и ответами
//! - Метрики производительности
//! - GPU утилизация
//! - Управление памятью

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::core::error::AppError;
use crate::platform::gpu::GpuInfo;

/// Базовый интерфейс для всех моделей
#[async_trait]
pub trait ModelInterface: Send + Sync {
    /// Обработка запроса к модели
    async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError>;
    
    /// Получение информации о модели
    async fn get_model_info(&self) -> Result<ModelInfo, AppError>;
    
    /// Обновление конфигурации модели
    async fn update_config(&self, config: ModelConfig) -> Result<(), AppError>;
    
    /// Получение метрик модели
    async fn get_metrics(&self) -> Result<ModelMetrics, AppError>;
    
    /// Инициализация модели
    async fn initialize(&self) -> Result<(), AppError>;
    
    /// Остановка модели
    async fn shutdown(&self) -> Result<(), AppError>;
    
    /// Проверка состояния модели
    async fn health_check(&self) -> Result<ModelHealth, AppError>;
}

/// Запрос к модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub prompt: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Ответ модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub text: String,
    pub tokens_used: u32,
    pub finish_reason: Option<String>,
    pub model_name: String,
    pub processing_time: f64,
    pub confidence: Option<f32>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Информация о модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub model_type: ModelType,
    pub parameters: u64,
    pub context_length: u32,
    pub supported_features: Vec<ModelFeature>,
    pub hardware_requirements: HardwareRequirements,
    pub license: Option<String>,
    pub author: Option<String>,
}

/// Тип модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    LanguageModel,
    VisionModel,
    MultimodalModel,
    CodeModel,
    MathModel,
    Custom(String),
}

/// Возможности модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelFeature {
    TextGeneration,
    TextCompletion,
    TextClassification,
    SentimentAnalysis,
    Translation,
    Summarization,
    QuestionAnswering,
    CodeGeneration,
    ImageGeneration,
    ImageClassification,
    Custom(String),
}

/// Требования к железу
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRequirements {
    pub min_gpu_memory: u64, // MB
    pub recommended_gpu_memory: u64, // MB
    pub min_ram: u64, // MB
    pub recommended_ram: u64, // MB
    pub min_cpu_cores: u32,
    pub recommended_cpu_cores: u32,
    pub gpu_types: Vec<String>,
    pub supported_precisions: Vec<Precision>,
}

/// Точность вычислений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Precision {
    FP16,
    FP32,
    FP64,
    INT8,
    INT16,
    INT32,
    Mixed,
}

/// Конфигурация модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_path: Option<String>,
    pub device: DeviceConfig,
    pub performance: PerformanceConfig,
    pub memory: MemoryConfig,
    pub inference: InferenceConfig,
    pub optimization: OptimizationConfig,
}

/// Конфигурация устройства
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub device_type: DeviceType,
    pub device_id: Option<u32>,
    pub memory_fraction: f32,
    pub allow_growth: bool,
}

/// Тип устройства
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    CPU,
    GPU,
    ASIC,
    Auto,
}

/// Конфигурация производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub batch_size: u32,
    pub max_concurrent_requests: u32,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub enable_caching: bool,
    pub cache_size: u64,
}

/// Конфигурация памяти
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub max_memory_usage: u64, // MB
    pub memory_pool_size: u64, // MB
    pub enable_memory_optimization: bool,
    pub garbage_collection_threshold: f32,
}

/// Конфигурация инференса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub default_temperature: f32,
    pub default_max_tokens: u32,
    pub default_top_p: f32,
    pub enable_sampling: bool,
    pub enable_beam_search: bool,
    pub beam_width: u32,
}

/// Конфигурация оптимизации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub enable_quantization: bool,
    pub quantization_type: Option<Precision>,
    pub enable_pruning: bool,
    pub enable_distillation: bool,
    pub enable_compilation: bool,
    pub optimization_level: OptimizationLevel,
}

/// Уровень оптимизации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Basic,
    Advanced,
    Maximum,
}

/// Метрики модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub requests_processed: u64,
    pub requests_per_second: f64,
    pub average_response_time: f64,
    pub tokens_generated: u64,
    pub tokens_per_second: f64,
    pub memory_usage: u64, // MB
    pub gpu_usage: f64, // %
    pub cpu_usage: f64, // %
    pub error_rate: f64,
    pub cache_hit_rate: f64,
    pub active_sessions: u32,
    pub queue_length: u32,
    pub last_updated: u64,
}

/// Здоровье модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelHealth {
    pub status: HealthStatus,
    pub message: String,
    pub last_check: u64,
    pub uptime: u64,
    pub memory_usage: u64,
    pub gpu_usage: f64,
    pub error_count: u64,
    pub warning_count: u64,
}

/// Статус здоровья
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Offline,
}

/// Менеджер моделей
pub struct ModelManager {
    models: Arc<RwLock<HashMap<String, Arc<dyn ModelInterface>>>>,
    config: ModelManagerConfig,
}

/// Конфигурация менеджера моделей
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManagerConfig {
    pub max_models: u32,
    pub default_device: DeviceType,
    pub auto_load: bool,
    pub model_cache_size: u64,
    pub health_check_interval: u64,
}

impl ModelManager {
    /// Создает новый менеджер моделей
    pub fn new(config: ModelManagerConfig) -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Регистрирует модель
    pub async fn register_model(&self, name: String, model: Arc<dyn ModelInterface>) -> Result<(), AppError> {
        let mut models = self.models.write().await;
        
        if models.len() >= self.config.max_models as usize {
            return Err(AppError::ResourceLimitExceeded("Maximum number of models reached".to_string()));
        }
        
        // Инициализируем модель
        model.initialize().await?;
        
        models.insert(name, model);
        Ok(())
    }

    /// Получает модель по имени
    pub async fn get_model(&self, name: &str) -> Option<Arc<dyn ModelInterface>> {
        let models = self.models.read().await;
        models.get(name).cloned()
    }

    /// Удаляет модель
    pub async fn remove_model(&self, name: &str) -> Result<(), AppError> {
        let mut models = self.models.write().await;
        
        if let Some(model) = models.remove(name) {
            model.shutdown().await?;
        }
        
        Ok(())
    }

    /// Получает список всех моделей
    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }

    /// Получает метрики всех моделей
    pub async fn get_all_metrics(&self) -> HashMap<String, ModelMetrics> {
        let models = self.models.read().await;
        let mut metrics = HashMap::new();
        
        for (name, model) in models.iter() {
            if let Ok(model_metrics) = model.get_metrics().await {
                metrics.insert(name.clone(), model_metrics);
            }
        }
        
        metrics
    }

    /// Проверяет здоровье всех моделей
    pub async fn health_check_all(&self) -> HashMap<String, ModelHealth> {
        let models = self.models.read().await;
        let mut health = HashMap::new();
        
        for (name, model) in models.iter() {
            if let Ok(model_health) = model.health_check().await {
                health.insert(name.clone(), model_health);
            }
        }
        
        health
    }

    /// Обрабатывает запрос к модели
    pub async fn process_request(&self, model_name: &str, request: ModelRequest) -> Result<ModelResponse, AppError> {
        let model = self.get_model(model_name).await
            .ok_or_else(|| AppError::NotFound(format!("Model '{}' not found", model_name)))?;
        
        model.process_request(request).await
    }

    /// Получает информацию о модели
    pub async fn get_model_info(&self, model_name: &str) -> Result<ModelInfo, AppError> {
        let model = self.get_model(model_name).await
            .ok_or_else(|| AppError::NotFound(format!("Model '{}' not found", model_name)))?;
        
        model.get_model_info().await
    }

    /// Обновляет конфигурацию модели
    pub async fn update_model_config(&self, model_name: &str, config: ModelConfig) -> Result<(), AppError> {
        let model = self.get_model(model_name).await
            .ok_or_else(|| AppError::NotFound(format!("Model '{}' not found", model_name)))?;
        
        model.update_config(config).await
    }
}

/// Базовая реализация модели
pub struct BaseModel {
    info: ModelInfo,
    config: ModelConfig,
    metrics: Arc<RwLock<ModelMetrics>>,
    gpu_info: Arc<RwLock<GpuInfo>>,
}

impl BaseModel {
    /// Создает новую базовую модель
    pub fn new(info: ModelInfo, config: ModelConfig) -> Self {
        Self {
            info,
            config,
            metrics: Arc::new(RwLock::new(ModelMetrics {
                requests_processed: 0,
                requests_per_second: 0.0,
                average_response_time: 0.0,
                tokens_generated: 0,
                tokens_per_second: 0.0,
                memory_usage: 0,
                gpu_usage: 0.0,
                cpu_usage: 0.0,
                error_rate: 0.0,
                cache_hit_rate: 0.0,
                active_sessions: 0,
                queue_length: 0,
                last_updated: 0,
            })),
            gpu_info: Arc::new(RwLock::new(GpuInfo::default())),
        }
    }

    /// Обновляет метрики
    async fn update_metrics(&self, processing_time: f64, tokens_generated: u32) {
        let mut metrics = self.metrics.write().await;
        metrics.requests_processed += 1;
        metrics.tokens_generated += tokens_generated as u64;
        metrics.average_response_time = 
            (metrics.average_response_time * (metrics.requests_processed - 1) as f64 + processing_time) 
            / metrics.requests_processed as f64;
        metrics.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[async_trait]
impl ModelInterface for BaseModel {
    async fn process_request(&self, _request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Базовая реализация - должна быть переопределена в конкретных моделях
        Err(AppError::NotImplemented("Base model cannot process requests".to_string()))
    }

    async fn get_model_info(&self) -> Result<ModelInfo, AppError> {
        Ok(self.info.clone())
    }

    async fn update_config(&self, config: ModelConfig) -> Result<(), AppError> {
        // Обновляем конфигурацию
        // В реальной реализации здесь должна быть логика валидации и применения
        Ok(())
    }

    async fn get_metrics(&self) -> Result<ModelMetrics, AppError> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    async fn initialize(&self) -> Result<(), AppError> {
        // Базовая инициализация
        log::info!("Initializing model: {}", self.info.name);
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), AppError> {
        // Базовая остановка
        log::info!("Shutting down model: {}", self.info.name);
        Ok(())
    }

    async fn health_check(&self) -> Result<ModelHealth, AppError> {
        let metrics = self.metrics.read().await;
        let gpu_info = self.gpu_info.read().await;
        
        let status = if metrics.error_rate < 0.1 {
            HealthStatus::Healthy
        } else if metrics.error_rate < 0.3 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        
        Ok(ModelHealth {
            status,
            message: "Model is operational".to_string(),
            last_check: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uptime: metrics.last_updated,
            memory_usage: metrics.memory_usage,
            gpu_usage: gpu_info.usage.unwrap_or(0.0),
            error_count: (metrics.requests_processed as f64 * metrics.error_rate) as u64,
            warning_count: 0,
        })
    }
} 