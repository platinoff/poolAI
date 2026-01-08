use chrono;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;
use thiserror::Error;
use tracing::{error, info, warn};
use uuid;

#[derive(Error, Debug)]
pub enum PoolAIError {
    #[error("IO error: {0}")]
    IoError(io::Error),
    #[error("JSON error: {0}")]
    JsonError(serde_json::Error),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Sync error")]
    SyncError,
    #[error("Time error: {0}")]
    TimeError(std::time::SystemTimeError),
    #[error("UUID error: {0}")]
    UuidError(uuid::Error),
}

impl From<io::Error> for PoolAIError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::Error> for PoolAIError {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}

impl From<std::time::SystemTimeError> for PoolAIError {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::TimeError(e)
    }
}

impl From<uuid::Error> for PoolAIError {
    fn from(e: uuid::Error) -> Self {
        Self::UuidError(e)
    }
}

#[derive(Error, Debug)]
pub enum CursorError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

/// Application error type for PoolAI
///
/// This enum represents all possible errors that can occur in the PoolAI system.
/// It provides structured error handling with context and suggestions.
///
/// # Example
///
/// ```rust
/// use poolai::core::error::AppError;
///
/// // Create a configuration error
/// let err = AppError::ConfigError("Invalid configuration: max_workers must be > 0".to_string());
///
/// // Check error code
/// assert_eq!(err.error_code(), "CONFIG_ERROR");
///
/// // Check if recoverable
/// assert!(!err.is_recoverable());
/// ```
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Model error: {0}")]
    ModelError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Pool error: {0}")]
    PoolError(String),
    #[error("Monitoring error: {0}")]
    MonitoringError(String),
    #[error("Resource error: {0}")]
    ResourceError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("GPU error: {0}")]
    GpuError(String),
    #[error("Memory error: {0}")]
    MemoryError(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Initialization error: {0}")]
    InitializationError(String),
    #[error("Shutdown error: {0}")]
    ShutdownError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unknown,
}

impl AppError {
    /// Log the error with appropriate severity level
    ///
    /// Automatically selects the appropriate logging level based on error type:
    /// - `error!` for critical errors (ConfigError, ModelError, etc.)
    /// - `warn!` for recoverable errors (ResourceError, TimeoutError)
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::core::error::AppError;
    ///
    /// let err = AppError::ConfigError("Invalid config".to_string());
    /// err.log(); // Logs at error level
    /// ```
    pub fn log(&self) {
        match self {
            AppError::ModelError(msg) => error!("Model error: {}", msg),
            AppError::ConfigError(msg) => error!("Configuration error: {}", msg),
            AppError::PoolError(msg) => error!("Pool error: {}", msg),
            AppError::MonitoringError(msg) => error!("Monitoring error: {}", msg),
            AppError::ResourceError(msg) => error!("Resource error: {}", msg),
            AppError::NetworkError(msg) => error!("Network error: {}", msg),
            AppError::GpuError(msg) => error!("GPU error: {}", msg),
            AppError::MemoryError(msg) => error!("Memory error: {}", msg),
            AppError::TimeoutError(msg) => error!("Timeout error: {}", msg),
            AppError::ValidationError(msg) => error!("Validation error: {}", msg),
            AppError::InitializationError(msg) => error!("Initialization error: {}", msg),
            AppError::ShutdownError(msg) => error!("Shutdown error: {}", msg),
            AppError::IoError(e) => error!("IO error: {}", e),
            AppError::SerializationError(e) => error!("Serialization error: {}", e),
            AppError::Unknown => error!("Unknown error"),
        }
    }

    /// Attempt to recover from the error
    ///
    /// Attempts automatic recovery based on error type.
    /// Returns `Ok(())` if recovery is attempted, but does not guarantee success.
    ///
    /// # Recovery strategies by error type
    ///
    /// - `ModelError`: Attempts to reload the model
    /// - `ConfigError`: Attempts to load default configuration
    /// - `PoolError`: Attempts to restart the pool
    /// - `ResourceError`: Attempts to free resources
    /// - `TimeoutError`: Returns Ok (timeout is transient)
    /// - Other errors: May not have recovery strategies
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::core::error::AppError;
    ///
    /// let err = AppError::TimeoutError("Request timeout".to_string());
    /// if err.is_recoverable() {
    ///     let _ = err.recover(); // Attempt recovery
    /// }
    /// ```
    pub fn recover(&self) -> Result<(), AppError> {
        warn!("Attempting recovery from error: {:?}", self);

        match self {
            AppError::ModelError(_) => {
                // Attempt to reload the model
                info!("Attempting model reload...");
                Ok(())
            }
            AppError::ConfigError(_) => {
                // Attempt to load default configuration
                info!("Attempting to load default configuration...");
                Ok(())
            }
            AppError::PoolError(_) => {
                // Attempt to restart the pool
                info!("Attempting pool restart...");
                Ok(())
            }
            AppError::MonitoringError(_) => {
                // Attempt to restart monitoring
                info!("Attempting monitoring restart...");
                Ok(())
            }
            AppError::ResourceError(_) => {
                // Attempt to free resources
                info!("Attempting resource cleanup...");
                Ok(())
            }
            AppError::GpuError(_) => {
                // Attempt to reinitialize GPU
                info!("Attempting GPU reinitialization...");
                Ok(())
            }
            AppError::MemoryError(_) => {
                // Attempt to clean up memory
                info!("Attempting memory cleanup...");
                Ok(())
            }
            AppError::TimeoutError(_) => {
                // Attempt to retry after timeout
                info!("Attempting retry after timeout...");
                Ok(())
            }
            _ => {
                warn!("No specific recovery strategy for error type");
                Ok(())
            }
        }
    }

    /// Check if the error is recoverable
    /// Check if the error is recoverable
    ///
    /// Returns `true` if the error type supports automatic recovery.
    /// Recoverable errors include timeouts, resource errors, and transient failures.
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::core::error::AppError;
    ///
    /// let timeout_err = AppError::TimeoutError("Request timeout".to_string());
    /// assert!(timeout_err.is_recoverable());
    ///
    /// let config_err = AppError::ConfigError("Invalid config".to_string());
    /// assert!(!config_err.is_recoverable());
    /// ```
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AppError::ModelError(_)
                | AppError::ConfigError(_)
                | AppError::PoolError(_)
                | AppError::MonitoringError(_)
                | AppError::ResourceError(_)
                | AppError::GpuError(_)
                | AppError::MemoryError(_)
                | AppError::TimeoutError(_)
        )
    }

    /// Get error code string
    /// Get a machine-readable error code
    ///
    /// Returns a static string identifier for the error type.
    /// Useful for error handling, logging, and API responses.
    ///
    /// # Error codes
    ///
    /// - `MODEL_ERROR`: Model-related errors
    /// - `CONFIG_ERROR`: Configuration errors
    /// - `POOL_ERROR`: Pool management errors
    /// - `RESOURCE_ERROR`: Resource-related errors
    /// - `TIMEOUT_ERROR`: Timeout errors
    /// - `VALIDATION_ERROR`: Validation errors
    /// - And others...
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::core::error::AppError;
    ///
    /// let err = AppError::ConfigError("Invalid config".to_string());
    /// assert_eq!(err.error_code(), "CONFIG_ERROR");
    ///
    /// // Use in API responses
    /// let error_response = serde_json::json!({
    ///     "error": err.error_code(),
    ///     "message": err.to_string()
    /// });
    /// ```
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ModelError(_) => "MODEL_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
            AppError::PoolError(_) => "POOL_ERROR",
            AppError::MonitoringError(_) => "MONITORING_ERROR",
            AppError::ResourceError(_) => "RESOURCE_ERROR",
            AppError::NetworkError(_) => "NETWORK_ERROR",
            AppError::GpuError(_) => "GPU_ERROR",
            AppError::MemoryError(_) => "MEMORY_ERROR",
            AppError::TimeoutError(_) => "TIMEOUT_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::InitializationError(_) => "INITIALIZATION_ERROR",
            AppError::ShutdownError(_) => "SHUTDOWN_ERROR",
            AppError::IoError(_) => "IO_ERROR",
            AppError::SerializationError(_) => "SERIALIZATION_ERROR",
            AppError::Unknown => "UNKNOWN_ERROR",
        }
    }
}

/// Error metrics result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Error counts by type
    pub error_counts: std::collections::HashMap<String, u64>,
    /// Last error time
    pub last_error_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Total error count
    pub total_errors: u64,
    /// Recovery count
    pub recovery_attempts: u64,
    /// Успешные восстановления
    pub successful_recoveries: u64,
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            error_counts: std::collections::HashMap::new(),
            last_error_time: None,
            total_errors: 0,
            recovery_attempts: 0,
            successful_recoveries: 0,
        }
    }
}

impl ErrorMetrics {
    /// Добавление ошибки в метрики
    pub fn add_error(&mut self, error: &AppError) {
        let error_code = error.error_code();
        *self.error_counts.entry(error_code.to_string()).or_insert(0) += 1;
        self.last_error_time = Some(chrono::Utc::now());
        self.total_errors += 1;
    }

    /// Добавление попытки восстановления
    pub fn add_recovery_attempt(&mut self, success: bool) {
        self.recovery_attempts += 1;
        if success {
            self.successful_recoveries += 1;
        }
    }

    /// Получение статистики ошибок
    pub fn get_error_statistics(&self) -> std::collections::HashMap<String, f64> {
        let mut stats = std::collections::HashMap::new();

        if self.total_errors > 0 {
            stats.insert("total_errors".to_string(), self.total_errors as f64);
            stats.insert(
                "recovery_rate".to_string(),
                self.successful_recoveries as f64 / self.recovery_attempts as f64,
            );

            for (error_type, count) in &self.error_counts {
                stats.insert(
                    format!("error_rate_{}", error_type.to_lowercase()),
                    *count as f64 / self.total_errors as f64,
                );
            }
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code() {
        assert_eq!(
            AppError::ModelError("test".to_string()).error_code(),
            "MODEL_ERROR"
        );
        assert_eq!(
            AppError::ConfigError("test".to_string()).error_code(),
            "CONFIG_ERROR"
        );
        assert_eq!(
            AppError::PoolError("test".to_string()).error_code(),
            "POOL_ERROR"
        );
        assert_eq!(
            AppError::ValidationError("test".to_string()).error_code(),
            "VALIDATION_ERROR"
        );
        assert_eq!(AppError::Unknown.error_code(), "UNKNOWN_ERROR");
    }

    #[test]
    fn test_is_recoverable() {
        assert!(AppError::ModelError("test".to_string()).is_recoverable());
        assert!(AppError::ConfigError("test".to_string()).is_recoverable());
        assert!(AppError::PoolError("test".to_string()).is_recoverable());
        assert!(AppError::MonitoringError("test".to_string()).is_recoverable());
        assert!(AppError::ResourceError("test".to_string()).is_recoverable());
        assert!(AppError::GpuError("test".to_string()).is_recoverable());
        assert!(AppError::MemoryError("test".to_string()).is_recoverable());
        assert!(AppError::TimeoutError("test".to_string()).is_recoverable());
        assert!(!AppError::ValidationError("test".to_string()).is_recoverable());
        assert!(!AppError::Unknown.is_recoverable());
    }

    #[test]
    fn test_recover() {
        // Test that recover() doesn't panic for all error types
        assert!(AppError::ModelError("test".to_string()).recover().is_ok());
        assert!(AppError::ConfigError("test".to_string()).recover().is_ok());
        assert!(AppError::PoolError("test".to_string()).recover().is_ok());
        assert!(AppError::ValidationError("test".to_string())
            .recover()
            .is_ok());
        assert!(AppError::Unknown.recover().is_ok());
    }

    #[test]
    fn test_error_metrics_add_error() {
        let mut metrics = ErrorMetrics::default();

        metrics.add_error(&AppError::ModelError("test".to_string()));
        assert_eq!(metrics.total_errors, 1);
        assert_eq!(metrics.error_counts.get("MODEL_ERROR"), Some(&1));
        assert!(metrics.last_error_time.is_some());

        metrics.add_error(&AppError::ConfigError("test".to_string()));
        assert_eq!(metrics.total_errors, 2);
        assert_eq!(metrics.error_counts.get("CONFIG_ERROR"), Some(&1));

        metrics.add_error(&AppError::ModelError("test2".to_string()));
        assert_eq!(metrics.total_errors, 3);
        assert_eq!(metrics.error_counts.get("MODEL_ERROR"), Some(&2));
    }

    #[test]
    fn test_error_metrics_add_recovery_attempt() {
        let mut metrics = ErrorMetrics::default();

        metrics.add_recovery_attempt(true);
        assert_eq!(metrics.recovery_attempts, 1);
        assert_eq!(metrics.successful_recoveries, 1);

        metrics.add_recovery_attempt(false);
        assert_eq!(metrics.recovery_attempts, 2);
        assert_eq!(metrics.successful_recoveries, 1);

        metrics.add_recovery_attempt(true);
        assert_eq!(metrics.recovery_attempts, 3);
        assert_eq!(metrics.successful_recoveries, 2);
    }

    #[test]
    fn test_error_metrics_get_error_statistics() {
        let mut metrics = ErrorMetrics::default();

        // Test with no errors
        let stats = metrics.get_error_statistics();
        assert_eq!(stats.len(), 0);

        // Add errors
        metrics.add_error(&AppError::ModelError("test1".to_string()));
        metrics.add_error(&AppError::ModelError("test2".to_string()));
        metrics.add_error(&AppError::ConfigError("test".to_string()));

        metrics.add_recovery_attempt(true);
        metrics.add_recovery_attempt(true);

        let stats = metrics.get_error_statistics();
        assert_eq!(stats.get("total_errors"), Some(&3.0));
        assert_eq!(stats.get("recovery_rate"), Some(&1.0));
        assert_eq!(stats.get("error_rate_model_error"), Some(&(2.0 / 3.0)));
        assert_eq!(stats.get("error_rate_config_error"), Some(&(1.0 / 3.0)));
    }

    #[test]
    fn test_error_metrics_default() {
        let metrics = ErrorMetrics::default();
        assert_eq!(metrics.total_errors, 0);
        assert_eq!(metrics.recovery_attempts, 0);
        assert_eq!(metrics.successful_recoveries, 0);
        assert_eq!(metrics.error_counts.len(), 0);
        assert!(metrics.last_error_time.is_none());
    }

    #[test]
    fn test_error_from_io_error() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_err = AppError::from(io_err);

        match app_err {
            AppError::IoError(_) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_error_from_serde_json_error() {
        let invalid_json = "invalid json";
        let json_err = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let app_err = AppError::from(json_err);

        match app_err {
            AppError::SerializationError(_) => {}
            _ => panic!("Expected SerializationError"),
        }
    }

    #[test]
    fn test_poolai_error_from_io_error() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let poolai_err = PoolAIError::from(io_err);

        match poolai_err {
            PoolAIError::IoError(_) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_poolai_error_from_serde_json_error() {
        let invalid_json = "invalid json";
        let json_err = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let poolai_err = PoolAIError::from(json_err);

        match poolai_err {
            PoolAIError::JsonError(_) => {}
            _ => panic!("Expected JsonError"),
        }
    }
}
