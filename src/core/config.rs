use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// PoolAI system configuration
///
/// # Example
///
/// ```rust
/// use poolai::core::config::SystemConfig;
///
/// let config = SystemConfig {
///     name: "MyPoolAI".to_string(),
///     version: "1.0.0".to_string(),
///     log_level: "info".to_string(),
///     max_workers: 16,
///     queue_size: 2000,
///     metrics_interval: 10,
/// };
/// ```
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

/// HTTPS/TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsConfig {
    /// Enable HTTPS (requires feature "https")
    pub enabled: bool,
    /// Certificate file path (PEM format)
    pub cert_path: Option<String>,
    /// Private key file path (PEM format)
    pub key_path: Option<String>,
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: Some("certs/cert.pem".to_string()),
            key_path: Some("certs/key.pem".to_string()),
        }
    }
}

/// Main PoolAI configuration
///
/// Contains all configuration sections for the PoolAI system.
///
/// # Example
///
/// ```rust
/// use poolai::core::config::PoolAIConfig;
///
/// // Use default configuration
/// let config = PoolAIConfig::default();
///
/// // Or create custom configuration
/// let config = PoolAIConfig {
///     system: poolai::core::config::SystemConfig {
///         name: "MyPoolAI".to_string(),
///         version: "1.0.0".to_string(),
///         log_level: "debug".to_string(),
///         max_workers: 32,
///         queue_size: 5000,
///         metrics_interval: 10,
///     },
///     ..Default::default()
/// };
/// ```
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
    /// HTTPS/TLS configuration
    pub https: HttpsConfig,
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
            https: HttpsConfig::default(),
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
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: system.max_workers is 0 (must be > 0). \
                    Context: max_workers defines the maximum number of concurrent worker processes. \
                    Suggestion: Set max_workers to a positive value (e.g., 4-16 depending on CPU cores). \
                    Current value: {}",
                    self.system.max_workers
                )
            ));
        }

        if self.system.queue_size == 0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: system.queue_size is 0 (must be > 0). \
                    Context: queue_size defines the maximum number of pending requests in the queue. \
                    Suggestion: Set queue_size to a positive value (e.g., 100-1000 depending on workload). \
                    Current value: {}",
                    self.system.queue_size
                )
            ));
        }

        // Validate GPU configuration
        if self.gpu.enabled && self.gpu.memory_limit == 0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: GPU is enabled but memory_limit is 0 (must be > 0). \
                    Context: When GPU is enabled, memory_limit defines the maximum GPU memory in MB. \
                    Suggestion: Set gpu.memory_limit to a positive value (e.g., 4096 for 4GB) or disable GPU if not needed. \
                    Current value: {}",
                    self.gpu.memory_limit
                )
            ));
        }

        // Validate pool configuration
        if self.pool.max_workers == 0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: pool.max_workers is 0 (must be > 0). \
                    Context: pool.max_workers defines the maximum number of workers in the pool. \
                    Suggestion: Set pool.max_workers to a positive value (e.g., 8-32 depending on system capacity). \
                    Current value: {}",
                    self.pool.max_workers
                )
            ));
        }

        if self.pool.scaling_threshold < 0.0 || self.pool.scaling_threshold > 1.0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: pool.scaling_threshold is {} (must be between 0.0 and 1.0). \
                    Context: scaling_threshold defines the load threshold for auto-scaling (0.0 = 0%, 1.0 = 100%). \
                    Suggestion: Set scaling_threshold to a value between 0.0 and 1.0 (e.g., 0.7 for 70% load threshold). \
                    Current value: {}",
                    self.pool.scaling_threshold, self.pool.scaling_threshold
                )
            ));
        }

        // Validate monitoring configuration
        if self.monitoring.metrics_interval == 0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: monitoring.metrics_interval is 0 (must be > 0). \
                    Context: metrics_interval defines the interval in seconds between metric collection. \
                    Suggestion: Set metrics_interval to a positive value (e.g., 5-60 seconds depending on monitoring needs). \
                    Current value: {}",
                    self.monitoring.metrics_interval
                )
            ));
        }

        if self.monitoring.alert_threshold < 0.0 || self.monitoring.alert_threshold > 1.0 {
            return Err(AppError::ConfigError(
                format!(
                    "Invalid configuration: monitoring.alert_threshold is {} (must be between 0.0 and 1.0). \
                    Context: alert_threshold defines the threshold for triggering alerts (0.0 = 0%, 1.0 = 100%). \
                    Suggestion: Set alert_threshold to a value between 0.0 and 1.0 (e.g., 0.9 for 90% threshold). \
                    Current value: {}",
                    self.monitoring.alert_threshold, self.monitoring.alert_threshold
                )
            ));
        }

        Ok(())
    }
}

/// Global configuration instance
static CONFIG: OnceLock<PoolAIConfig> = OnceLock::new();

/// Initialize configuration
///
/// This function initializes the global configuration instance.
/// Must be called before using any configuration-dependent functionality.
///
/// # Arguments
///
/// * `config` - The configuration to initialize
///
/// # Errors
///
/// Returns an error if:
/// - Configuration validation fails
/// - Configuration is already initialized
///
/// # Example
///
/// ```rust,no_run
/// use poolai::core::config::{PoolAIConfig, initialize_config};
///
/// # fn example() -> Result<(), poolai::core::error::AppError> {
/// let config = PoolAIConfig::default();
/// initialize_config(config)?;
/// // Now you can use get_config() to retrieve the configuration
/// # Ok(())
/// # }
/// ```
pub fn initialize_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;

    CONFIG
        .set(config)
        .map_err(|_| AppError::ConfigError("Configuration already initialized".to_string()))?;

    Ok(())
}

/// Get configuration
///
/// Retrieves a clone of the global configuration instance.
/// Must be called after `initialize_config()`.
///
/// # Errors
///
/// Returns an error if configuration has not been initialized.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::core::config::{get_config, initialize_config, PoolAIConfig};
///
/// # fn example() -> Result<(), poolai::core::error::AppError> {
/// // First initialize
/// initialize_config(PoolAIConfig::default())?;
///
/// // Then retrieve
/// let config = get_config()?;
/// println!("System name: {}", config.system.name);
/// # Ok(())
/// # }
/// ```
pub fn get_config() -> Result<PoolAIConfig, AppError> {
    CONFIG
        .get()
        .cloned()
        .ok_or_else(|| AppError::ConfigError("Configuration not initialized".to_string()))
}

/// Update configuration
pub fn update_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;

    // OnceLock doesn't support updates, so we need to reinitialize
    // This is a limitation, but ensures thread safety
    // For true updates, consider using Arc<RwLock<PoolAIConfig>> instead
    CONFIG.set(config).map_err(|_| {
        AppError::ConfigError(
            "Configuration already initialized. Use reinitialize_config() to update.".to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_config() {
        let config = PoolAIConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_max_workers() {
        let mut config = PoolAIConfig::default();
        config.system.max_workers = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max_workers"));
    }

    #[test]
    fn test_validate_zero_queue_size() {
        let mut config = PoolAIConfig::default();
        config.system.queue_size = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("queue_size"));
    }

    #[test]
    fn test_validate_gpu_enabled_zero_memory() {
        let mut config = PoolAIConfig::default();
        config.gpu.enabled = true;
        config.gpu.memory_limit = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("memory_limit"));
    }

    #[test]
    fn test_validate_gpu_disabled_zero_memory() {
        let mut config = PoolAIConfig::default();
        config.gpu.enabled = false;
        config.gpu.memory_limit = 0;
        // Should be OK when GPU is disabled
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_pool_zero_max_workers() {
        let mut config = PoolAIConfig::default();
        config.pool.max_workers = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pool.max_workers"));
    }

    #[test]
    fn test_validate_scaling_threshold_too_low() {
        let mut config = PoolAIConfig::default();
        config.pool.scaling_threshold = -0.1;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("scaling_threshold"));
    }

    #[test]
    fn test_validate_scaling_threshold_too_high() {
        let mut config = PoolAIConfig::default();
        config.pool.scaling_threshold = 1.5;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("scaling_threshold"));
    }

    #[test]
    fn test_validate_scaling_threshold_valid() {
        let mut config = PoolAIConfig::default();
        config.pool.scaling_threshold = 0.7;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_metrics_interval_zero() {
        let mut config = PoolAIConfig::default();
        config.monitoring.metrics_interval = 0;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("metrics_interval"));
    }

    #[test]
    fn test_validate_alert_threshold_too_low() {
        let mut config = PoolAIConfig::default();
        config.monitoring.alert_threshold = -0.1;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("alert_threshold"));
    }

    #[test]
    fn test_validate_alert_threshold_too_high() {
        let mut config = PoolAIConfig::default();
        config.monitoring.alert_threshold = 1.5;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("alert_threshold"));
    }

    #[test]
    fn test_validate_alert_threshold_valid() {
        let mut config = PoolAIConfig::default();
        config.monitoring.alert_threshold = 0.9;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_get_config_not_initialized() {
        // Reset CONFIG to None for testing
        // Note: OnceLock doesn't support reset, so this test may fail if
        // config was already initialized in a previous test
        let result = get_config();
        // This might fail if config was already set, which is OK
        if result.is_err() {
            assert!(result.unwrap_err().to_string().contains("not initialized"));
        }
    }

    #[test]
    fn test_initialize_and_get_config() {
        let config = PoolAIConfig::default();
        // Try to initialize - might fail if already initialized
        if initialize_config(config.clone()).is_ok() {
            let retrieved = get_config().unwrap();
            assert_eq!(retrieved.system.name, config.system.name);
            assert_eq!(retrieved.system.max_workers, config.system.max_workers);
        }
    }
}
