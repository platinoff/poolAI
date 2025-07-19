use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::error::AppError;

/// Конфигурация системы PoolAI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Название системы
    pub name: String,
    /// Версия системы
    pub version: String,
    /// Уровень логирования
    pub log_level: String,
    /// Максимальное количество воркеров
    pub max_workers: usize,
    /// Размер очереди запросов
    pub queue_size: usize,
    /// Интервал сбора метрик (секунды)
    pub metrics_interval: u64,
}

/// Конфигурация GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Включить GPU
    pub enabled: bool,
    /// Лимит памяти GPU (MB)
    pub memory_limit: u64,
    /// Лимит температуры (°C)
    pub temperature_limit: u8,
    /// Лимит мощности (Watts)
    pub power_limit: u16,
    /// Количество GPU для использования
    pub gpu_count: usize,
}

/// Конфигурация модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Название модели
    pub name: String,
    /// Путь к модели
    pub path: String,
    /// Максимальный размер батча
    pub max_batch_size: usize,
    /// Лимит памяти для модели (MB)
    pub memory_limit: u64,
    /// Температура генерации
    pub temperature: f32,
    /// Максимальное количество токенов
    pub max_tokens: usize,
    /// Включить кэширование
    pub enable_cache: bool,
    /// Размер кэша (MB)
    pub cache_size: u64,
}

/// Конфигурация пула
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Максимальное количество воркеров
    pub max_workers: usize,
    /// Размер очереди запросов
    pub queue_size: usize,
    /// Автоматическое масштабирование
    pub auto_scaling: bool,
    /// Порог для масштабирования (0.0-1.0)
    pub scaling_threshold: f32,
    /// Таймаут обработки запроса (секунды)
    pub request_timeout: u64,
}

/// Конфигурация мониторинга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Интервал сбора метрик (секунды)
    pub metrics_interval: u64,
    /// Порог для алертов (0.0-1.0)
    pub alert_threshold: f32,
    /// Количество дней хранения метрик
    pub retention_days: u32,
    /// Включить детальное логирование
    pub detailed_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConfig {
    pub app_version: String,
    pub build_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub expected_workers: usize,
}

/// Основная конфигурация PoolAI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAIConfig {
    /// Конфигурация системы
    pub system: SystemConfig,
    /// Конфигурация GPU
    pub gpu: GpuConfig,
    /// Конфигурация пула
    pub pool: PoolConfig,
    /// Конфигурация мониторинга
    pub monitoring: MonitoringConfig,
    /// Конфигурация версии
    pub version: VersionConfig,
    /// Конфигурация healthcheck
    pub health: HealthConfig,
}

impl Default for PoolAIConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig {
                name: "PoolAI".to_string(),
                version: "MVP_v1".to_string(),
                log_level: "info".to_string(),
                max_workers: 8,
                queue_size: 1000,
                metrics_interval: 5,
            },
            gpu: GpuConfig {
                enabled: true,
                memory_limit: 8192,
                temperature_limit: 85,
                power_limit: 200,
                gpu_count: 1,
            },
            pool: PoolConfig {
                max_workers: 8,
                queue_size: 1000,
                auto_scaling: true,
                scaling_threshold: 0.8,
                request_timeout: 30,
            },
            monitoring: MonitoringConfig {
                metrics_interval: 5,
                alert_threshold: 0.9,
                retention_days: 30,
                detailed_logging: true,
            },
            version: VersionConfig {
                app_version: "0.1.0".to_string(),
                build_time: "2023-10-27T10:00:00Z".to_string(),
            },
            health: HealthConfig {
                expected_workers: 8,
            },
        }
    }
}

impl PoolAIConfig {
    /// Загрузка конфигурации из файла
    pub fn from_file(path: &str) -> Result<Self, AppError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let config: PoolAIConfig = toml::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    /// Сохранение конфигурации в файл
    pub fn save_to_file(&self, path: &str) -> Result<(), AppError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Получение конфигурации модели
    pub fn get_model_config(&self, model_name: &str) -> Option<&ModelConfig> {
        None
    }

    /// Добавление конфигурации модели
    pub fn add_model_config(&mut self, config: ModelConfig) {
        // This method is not used in the MVP
    }

    /// Удаление конфигурации модели
    pub fn remove_model_config(&mut self, model_name: &str) -> Option<ModelConfig> {
        None
    }

    /// Валидация конфигурации
    pub fn validate(&self) -> Result<(), AppError> {
        // Проверка системной конфигурации
        if self.system.max_workers == 0 {
            return Err(AppError::ConfigError("max_workers must be greater than 0".to_string()));
        }

        if self.system.queue_size == 0 {
            return Err(AppError::ConfigError("queue_size must be greater than 0".to_string()));
        }

        // Проверка GPU конфигурации
        if self.gpu.enabled && self.gpu.memory_limit == 0 {
            return Err(AppError::ConfigError("GPU memory_limit must be greater than 0".to_string()));
        }

        // Проверка пула
        if self.pool.max_workers == 0 {
            return Err(AppError::ConfigError("pool max_workers must be greater than 0".to_string()));
        }

        if self.pool.scaling_threshold < 0.0 || self.pool.scaling_threshold > 1.0 {
            return Err(AppError::ConfigError("scaling_threshold must be between 0.0 and 1.0".to_string()));
        }

        // Проверка мониторинга
        if self.monitoring.metrics_interval == 0 {
            return Err(AppError::ConfigError("metrics_interval must be greater than 0".to_string()));
        }

        if self.monitoring.alert_threshold < 0.0 || self.monitoring.alert_threshold > 1.0 {
            return Err(AppError::ConfigError("alert_threshold must be between 0.0 and 1.0".to_string()));
        }

        Ok(())
    }
}

/// Глобальный экземпляр конфигурации
static mut CONFIG: Option<PoolAIConfig> = None;

/// Инициализация конфигурации
pub fn initialize_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;
    
    unsafe {
        CONFIG = Some(config);
    }
    
    Ok(())
}

/// Получение конфигурации
pub fn get_config() -> Result<PoolAIConfig, AppError> {
    unsafe {
        CONFIG.clone().ok_or_else(|| {
            AppError::ConfigError("Configuration not initialized".to_string())
        })
    }
}

/// Обновление конфигурации
pub fn update_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;
    
    unsafe {
        CONFIG = Some(config);
    }
    
    Ok(())
} 