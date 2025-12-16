use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
// use std::collections::HashMap; // Not used in MVP
use crate::core::error::AppError;

/// PoolAI system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// System name
    pub name: String,
    /// System version
    pub version: String,
    /// Logging level
    pub log_level: String,
    /// Maximum number of workers
    pub max_workers: usize,
    /// Request queue size
    pub queue_size: usize,
    /// Metrics collection interval (seconds)
    pub metrics_interval: u64,
}

/// GPU configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Enable GPU
    pub enabled: bool,
    /// GPU memory limit (MB)
    pub memory_limit: u64,
    /// Temperature limit (°C)
    pub temperature_limit: u8,
    /// Power limit (Watts)
    pub power_limit: u16,
    /// Number of GPUs to use
    pub gpu_count: usize,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name
    pub name: String,
    /// Model path
    pub path: String,
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Memory limit for model (MB)
    pub memory_limit: u64,
    /// Generation temperature
    pub temperature: f32,
    /// Maximum number of tokens
    pub max_tokens: usize,
    /// Enable caching
    pub enable_cache: bool,
    /// Cache size (MB)
    pub cache_size: u64,
}

/// Pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of workers
    pub max_workers: usize,
    /// Request queue size
    pub queue_size: usize,
    /// Auto-scaling enabled
    pub auto_scaling: bool,
    /// Scaling threshold (0.0-1.0)
    pub scaling_threshold: f32,
    /// Request processing timeout (seconds)
    pub request_timeout: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics collection interval (seconds)
    pub metrics_interval: u64,
    /// Alert threshold (0.0-1.0)
    pub alert_threshold: f32,
    /// Number of days to retain metrics
    pub retention_days: u32,
    /// Enable detailed logging
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

/// Main PoolAI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAIConfig {
    /// System configuration
    pub system: SystemConfig,
    /// GPU configuration
    pub gpu: GpuConfig,
    /// Pool configuration
    pub pool: PoolConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Version configuration
    pub version: VersionConfig,
    /// Health check configuration
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
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, AppError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let config: PoolAIConfig = toml::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<(), AppError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Get model configuration
    pub fn get_model_config(&self, _model_name: &str) -> Option<&ModelConfig> {
        None
    }

    /// Add model configuration
    pub fn add_model_config(&mut self, _config: ModelConfig) {
        // This method is not used in the MVP
    }

    /// Remove model configuration
    pub fn remove_model_config(&mut self, _model_name: &str) -> Option<ModelConfig> {
        None
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), AppError> {
        // Validate system configuration
        if self.system.max_workers == 0 {
            return Err(AppError::ConfigError("max_workers must be greater than 0".to_string()));
        }

        if self.system.queue_size == 0 {
            return Err(AppError::ConfigError("queue_size must be greater than 0".to_string()));
        }

        // Validate GPU configuration
        if self.gpu.enabled && self.gpu.memory_limit == 0 {
            return Err(AppError::ConfigError("GPU memory_limit must be greater than 0".to_string()));
        }

        // Validate pool configuration
        if self.pool.max_workers == 0 {
            return Err(AppError::ConfigError("pool max_workers must be greater than 0".to_string()));
        }

        if self.pool.scaling_threshold < 0.0 || self.pool.scaling_threshold > 1.0 {
            return Err(AppError::ConfigError("scaling_threshold must be between 0.0 and 1.0".to_string()));
        }

        // Validate monitoring configuration
        if self.monitoring.metrics_interval == 0 {
            return Err(AppError::ConfigError("metrics_interval must be greater than 0".to_string()));
        }

        if self.monitoring.alert_threshold < 0.0 || self.monitoring.alert_threshold > 1.0 {
            return Err(AppError::ConfigError("alert_threshold must be between 0.0 and 1.0".to_string()));
        }

        Ok(())
    }
}

/// Global configuration instance
static CONFIG: OnceLock<PoolAIConfig> = OnceLock::new();

/// Initialize configuration
pub fn initialize_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;
    
    CONFIG.set(config).map_err(|_| {
        AppError::ConfigError("Configuration already initialized".to_string())
    })?;
    
    Ok(())
}

/// Get configuration
pub fn get_config() -> Result<PoolAIConfig, AppError> {
    CONFIG.get().cloned().ok_or_else(|| {
        AppError::ConfigError("Configuration not initialized".to_string())
    })
}

/// Update configuration
pub fn update_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;
    
    // OnceLock doesn't support updates, so we need to reinitialize
    // This is a limitation, but ensures thread safety
    // For true updates, consider using Arc<RwLock<PoolAIConfig>> instead
    CONFIG.set(config).map_err(|_| {
        AppError::ConfigError("Configuration already initialized. Use reinitialize_config() to update.".to_string())
    })?;
    
    Ok(())
} 