use crate::core::error::AppError;
use crate::core::config::ModelConfig as ConfigModelConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Запрос к модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    /// Входные данные (промпт)
    pub input: String,
    /// Параметры генерации
    pub parameters: ModelParameters,
    /// ID сессии для кэширования
    pub session_id: Option<String>,
    /// Приоритет запроса (1-10, где 10 - высший)
    pub priority: u8,
    /// Таймаут запроса (секунды)
    pub timeout: Option<u64>,
}

/// Ответ модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    /// Сгенерированный текст
    pub output: String,
    /// Метрики обработки
    pub metrics: ModelMetrics,
    /// ID сессии
    pub session_id: Option<String>,
    /// Статус обработки
    pub status: ResponseStatus,
    /// Ошибки (если есть)
    pub errors: Vec<String>,
}

/// Статус ответа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Partial,
    Error,
    Timeout,
}

/// Параметры модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Температура генерации (0.0-2.0)
    pub temperature: f32,
    /// Максимальное количество токенов
    pub max_tokens: usize,
    /// Top-p sampling (0.0-1.0)
    pub top_p: f32,
    /// Frequency penalty (-2.0-2.0)
    pub frequency_penalty: f32,
    /// Presence penalty (-2.0-2.0)
    pub presence_penalty: f32,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    /// Seed для воспроизводимости
    pub seed: Option<u64>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 100,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: vec![],
            seed: None,
        }
    }
}

/// Информация о модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Название модели
    pub name: String,
    /// Версия модели
    pub version: String,
    /// Возможности модели
    pub capabilities: Vec<String>,
    /// Максимальное количество токенов
    pub max_tokens: usize,
    /// Поддерживаемые параметры
    pub supported_parameters: Vec<String>,
    /// Размер модели (MB)
    pub model_size_mb: u64,
    /// Поддерживаемые языки
    pub supported_languages: Vec<String>,
    /// Требования к GPU
    pub gpu_requirements: GpuRequirements,
}

/// Требования к GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRequirements {
    /// Минимальная память GPU (MB)
    pub min_memory_mb: u64,
    /// Рекомендуемая память GPU (MB)
    pub recommended_memory_mb: u64,
    /// Поддерживаемые архитектуры GPU
    pub supported_architectures: Vec<String>,
    /// Требуется CUDA
    pub requires_cuda: bool,
}

/// Метрики модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Время обработки (мс)
    pub processing_time_ms: u64,
    /// Количество сгенерированных токенов
    pub tokens_generated: usize,
    /// Утилизация GPU (%)
    pub gpu_utilization: f32,
    /// Использование памяти (MB)
    pub memory_usage_mb: f32,
    /// Пропускная способность (токенов/сек)
    pub throughput_tokens_per_sec: f32,
    /// Загрузка CPU (%)
    pub cpu_utilization: f32,
    /// Температура GPU (°C)
    pub gpu_temperature: f32,
    /// Мощность GPU (Watts)
    pub gpu_power_watts: f32,
    /// Количество запросов в очереди
    pub queue_length: usize,
    /// Средняя задержка (мс)
    pub average_latency_ms: f32,
}

/// Состояние модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelState {
    /// Статус модели
    pub status: ModelStatus,
    /// Количество активных запросов
    pub active_requests: usize,
    /// Общее количество обработанных запросов
    pub total_requests: u64,
    /// Время последней активности
    pub last_activity: chrono::DateTime<chrono::Utc>,
    /// Ошибки (если есть)
    pub errors: Vec<String>,
    /// Метрики производительности
    pub metrics: ModelMetrics,
}

/// Статус модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Initializing,
    Ready,
    Busy,
    Error,
    Shutdown,
}

/// Конфигурация модели (для интерфейса)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Путь к модели
    pub model_path: String,
    /// GPU устройство
    pub gpu_device: Option<usize>,
    /// Максимальный размер батча
    pub max_batch_size: usize,
    /// Лимит памяти (MB)
    pub memory_limit_mb: usize,
    /// Включить кэширование
    pub enable_caching: bool,
    /// Размер кэша (MB)
    pub cache_size_mb: usize,
    /// Параметры по умолчанию
    pub default_parameters: ModelParameters,
    /// Настройки производительности
    pub performance_settings: PerformanceSettings,
}

/// Настройки производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Количество потоков
    pub num_threads: usize,
    /// Использовать GPU
    pub use_gpu: bool,
    /// Оптимизация памяти
    pub memory_optimization: bool,
    /// Параллельная обработка
    pub parallel_processing: bool,
}

/// Основной интерфейс модели согласно концепту MVP
#[async_trait::async_trait]
pub trait ModelInterface {
    /// Обработка запроса к модели
    async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError>;
    
    /// Получение информации о модели
    async fn get_model_info(&self) -> Result<ModelInfo, AppError>;
    
    /// Обновление конфигурации модели
    async fn update_config(&self, config: ModelConfig) -> Result<(), AppError>;
    
    /// Получение метрик модели
    async fn get_metrics(&self) -> Result<ModelMetrics, AppError>;
    
    /// Получение состояния модели
    async fn get_state(&self) -> Result<ModelState, AppError>;
    
    /// Инициализация модели
    async fn initialize(&self) -> Result<(), AppError>;
    
    /// Остановка модели
    async fn shutdown(&self) -> Result<(), AppError>;
    
    /// Проверка здоровья модели
    async fn health_check(&self) -> Result<(), AppError>;
    
    /// Очистка кэша
    async fn clear_cache(&self) -> Result<(), AppError>;
    
    /// Получение статистики
    async fn get_statistics(&self) -> Result<HashMap<String, f64>, AppError>;
}

/// Менеджер моделей для MVP
pub struct ModelManager {
    models: HashMap<String, Box<dyn ModelInterface + Send + Sync>>,
    config: ConfigModelConfig,
}

impl ModelManager {
    /// Создание нового менеджера моделей
    pub fn new(config: ConfigModelConfig) -> Self {
        Self {
            models: HashMap::new(),
            config,
        }
    }

    /// Регистрация модели
    pub async fn register_model(
        &mut self,
        name: String,
        model: Box<dyn ModelInterface + Send + Sync>,
    ) -> Result<(), AppError> {
        // Инициализация модели
        model.initialize().await?;
        
        // Проверка здоровья
        model.health_check().await?;
        
        self.models.insert(name, model);
        Ok(())
    }

    /// Удаление модели
    pub async fn unregister_model(&mut self, name: &str) -> Result<(), AppError> {
        if let Some(model) = self.models.remove(name) {
            model.shutdown().await?;
        }
        Ok(())
    }

    /// Получение модели
    pub fn get_model(&self, name: &str) -> Option<&Box<dyn ModelInterface + Send + Sync>> {
        self.models.get(name)
    }

    /// Получение всех моделей
    pub fn get_all_models(&self) -> &HashMap<String, Box<dyn ModelInterface + Send + Sync>> {
        &self.models
    }

    /// Обработка запроса через конкретную модель
    pub async fn process_request(
        &self,
        model_name: &str,
        request: ModelRequest,
    ) -> Result<ModelResponse, AppError> {
        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| AppError::ModelError(format!("Model {} not found", model_name)))?;

        model.process_request(request).await
    }

    /// Получение метрик всех моделей
    pub async fn get_all_metrics(&self) -> Result<HashMap<String, ModelMetrics>, AppError> {
        let mut metrics = HashMap::new();
        
        for (name, model) in &self.models {
            let model_metrics = model.get_metrics().await?;
            metrics.insert(name.clone(), model_metrics);
        }
        
        Ok(metrics)
    }

    /// Получение состояния всех моделей
    pub async fn get_all_states(&self) -> Result<HashMap<String, ModelState>, AppError> {
        let mut states = HashMap::new();
        
        for (name, model) in &self.models {
            let model_state = model.get_state().await?;
            states.insert(name.clone(), model_state);
        }
        
        Ok(states)
    }
} 