use std::fmt;
use thiserror::Error;
use std::io;
use serde_json;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid;
use cursor_codes::core::config::AppConfig;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Worker error: {0}")]
    Worker(String),

    #[error("VM error: {0}")]
    VM(String),

    #[error("Bridge error: {0}")]
    Bridge(String),

    #[error("Router error: {0}")]
    Router(String),

    #[error("Reward error: {0}")]
    Reward(String),

    #[error("Pool error: {0}")]
    Pool(String),

    #[error("Telegram error: {0}")]
    Telegram(String),

    #[error("Admin error: {0}")]
    Admin(String),

    #[error("Library error: {0}")]
    Library(String),

    #[error("Tuning error: {0}")]
    Tuning(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl AppError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Network(_) | AppError::Timeout(_) | AppError::Database(_)
        )
    }

    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            AppError::Auth(_) | AppError::Authorization(_) | AppError::Config(_)
        )
    }

    pub fn to_string(&self) -> String {
        match self {
            AppError::Io(e) => format!("IO error: {}", e),
            AppError::Serialization(e) => format!("Serialization error: {}", e),
            AppError::Config(msg) => format!("Configuration error: {}", msg),
            AppError::Auth(msg) => format!("Authentication error: {}", msg),
            AppError::Authorization(msg) => format!("Authorization error: {}", msg),
            AppError::NotFound(msg) => format!("Resource not found: {}", msg),
            AppError::InvalidInput(msg) => format!("Invalid input: {}", msg),
            AppError::Database(msg) => format!("Database error: {}", msg),
            AppError::Network(msg) => format!("Network error: {}", msg),
            AppError::Timeout(msg) => format!("Timeout error: {}", msg),
            AppError::Worker(msg) => format!("Worker error: {}", msg),
            AppError::VM(msg) => format!("VM error: {}", msg),
            AppError::Bridge(msg) => format!("Bridge error: {}", msg),
            AppError::Router(msg) => format!("Router error: {}", msg),
            AppError::Reward(msg) => format!("Reward error: {}", msg),
            AppError::Pool(msg) => format!("Pool error: {}", msg),
            AppError::Telegram(msg) => format!("Telegram error: {}", msg),
            AppError::Admin(msg) => format!("Admin error: {}", msg),
            AppError::Library(msg) => format!("Library error: {}", msg),
            AppError::Tuning(msg) => format!("Tuning error: {}", msg),
            AppError::Unknown(msg) => format!("Unknown error: {}", msg),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub error_type: String,
    pub severity: String,
    pub retry_count: u32,
    pub retry_delay: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStats {
    pub total_errors: u64,
    pub total_retries: u64,
    pub total_resolved: u64,
    pub total_unresolved: u64,
    pub last_error_time: Option<DateTime<Utc>>,
    pub last_resolution_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub config: ErrorConfig,
    pub stats: ErrorStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub id: String,
    pub error_type: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
    pub resolution_time: Option<DateTime<Utc>>,
    pub retry_count: u32,
}

pub struct ErrorSystem {
    errors: Arc<Mutex<HashMap<String, ErrorMetrics>>>,
    events: Arc<Mutex<HashMap<String, Vec<ErrorEvent>>>>,
}

impl ErrorSystem {
    pub fn new() -> Self {
        Self {
            errors: Arc::new(Mutex::new(HashMap::new())),
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_error(&self, config: ErrorConfig) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        
        if errors.contains_key(&config.id) {
            return Err(format!("Error type '{}' already exists", config.id));
        }

        errors.insert(
            config.id.clone(),
            ErrorMetrics {
                config: config.clone(),
                stats: ErrorStats {
                    total_errors: 0,
                    total_retries: 0,
                    total_resolved: 0,
                    total_unresolved: 0,
                    last_error_time: None,
                    last_resolution_time: None,
                    last_error: None,
                },
            },
        );

        info!("Added new error type: {}", config.id);
        Ok(())
    }

    pub async fn remove_error(&self, id: &str) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        let mut events = self.events.lock().await;
        
        if errors.remove(id).is_some() {
            events.remove(id);
            info!("Removed error type: {}", id);
            Ok(())
        } else {
            Err(format!("Error type '{}' not found", id))
        }
    }

    pub async fn record_error(
        &self,
        error_type: &str,
        message: &str,
        stack_trace: Option<String>,
    ) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        let mut events = self.events.lock().await;
        
        let metrics = errors
            .get_mut(error_type)
            .ok_or_else(|| format!("Error type '{}' not found", error_type))?;

        if !metrics.config.active {
            return Err("Error type is not active".to_string());
        }

        let event = ErrorEvent {
            id: uuid::Uuid::new_v4().to_string(),
            error_type: error_type.to_string(),
            message: message.to_string(),
            stack_trace,
            timestamp: Utc::now(),
            resolved: false,
            resolution_time: None,
            retry_count: 0,
        };

        metrics.stats.total_errors += 1;
        metrics.stats.total_unresolved += 1;
        metrics.stats.last_error_time = Some(event.timestamp);
        metrics.stats.last_error = Some(message.to_string());

        events
            .entry(error_type.to_string())
            .or_insert_with(Vec::new)
            .push(event.clone());

        if self.is_critical_error(error_type, message)? {
            error!("Critical error detected: {} - {}", error_type, message);
            self.handle_critical_error(error_type, &event).await?;
        }

        if metrics.config.retry_count > 0 {
            self.schedule_retry(error_type, &event).await?;
        }

        info!("Recorded error: {} - {}", error_type, message);
        Ok(())
    }

    pub async fn resolve_error(&self, error_type: &str, event_id: &str) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        let mut events = self.events.lock().await;
        
        let metrics = errors
            .get_mut(error_type)
            .ok_or_else(|| format!("Error type '{}' not found", error_type))?;

        let event_list = events
            .get_mut(error_type)
            .ok_or_else(|| format!("No events found for error type '{}'", error_type))?;

        let event = event_list
            .iter_mut()
            .find(|e| e.id == event_id)
            .ok_or_else(|| format!("Event '{}' not found", event_id))?;

        if event.resolved {
            return Err("Event is already resolved".to_string());
        }

        event.resolved = true;
        event.resolution_time = Some(Utc::now());

        metrics.stats.total_resolved += 1;
        metrics.stats.total_unresolved -= 1;
        metrics.stats.last_resolution_time = Some(Utc::now());

        info!("Resolved error: {} - {}", error_type, event.message);
        Ok(())
    }

    pub async fn retry_error(&self, error_type: &str, event_id: &str) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        let mut events = self.events.lock().await;
        
        let metrics = errors
            .get_mut(error_type)
            .ok_or_else(|| format!("Error type '{}' not found", error_type))?;

        if !metrics.config.active {
            return Err("Error type is not active".to_string());
        }

        let event_list = events
            .get_mut(error_type)
            .ok_or_else(|| format!("No events found for error type '{}'", error_type))?;

        let event = event_list
            .iter_mut()
            .find(|e| e.id == event_id)
            .ok_or_else(|| format!("Event '{}' not found", event_id))?;

        if event.resolved {
            return Err("Cannot retry resolved event".to_string());
        }

        if event.retry_count >= metrics.config.retry_count {
            return Err("Maximum retry count reached".to_string());
        }

        event.retry_count += 1;
        metrics.stats.total_retries += 1;

        self.schedule_retry(error_type, event).await?;

        info!("Retrying error: {} - {} (attempt {})", 
            error_type, event.message, event.retry_count);
        Ok(())
    }

    pub async fn get_error(&self, id: &str) -> Result<ErrorMetrics, String> {
        let errors = self.errors.lock().await;
        
        errors
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Error type '{}' not found", id))
    }

    pub async fn get_all_errors(&self) -> Vec<ErrorMetrics> {
        let errors = self.errors.lock().await;
        errors.values().cloned().collect()
    }

    pub async fn get_active_errors(&self) -> Vec<ErrorMetrics> {
        let errors = self.errors.lock().await;
        errors
            .values()
            .filter(|e| e.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_events(&self, error_type: &str) -> Result<Vec<ErrorEvent>, String> {
        let events = self.events.lock().await;
        
        events
            .get(error_type)
            .cloned()
            .ok_or_else(|| format!("No events found for error type '{}'", error_type))
    }

    pub async fn set_error_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        
        let metrics = errors
            .get_mut(id)
            .ok_or_else(|| format!("Error type '{}' not found", id))?;

        metrics.config.active = active;
        info!(
            "Error type '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_error_config(&self, id: &str, new_config: ErrorConfig) -> Result<(), String> {
        let mut errors = self.errors.lock().await;
        
        if !errors.contains_key(id) {
            return Err(format!("Error type '{}' not found", id));
        }

        let metrics = errors.get_mut(id).unwrap();
        metrics.config = new_config;

        info!("Updated error type: {}", id);
        Ok(())
    }

    fn is_critical_error(&self, error_type: &str, message: &str) -> Result<bool, String> {
        let critical_types = ["auth", "authorization", "config"];
        let critical_patterns = [
            "unauthorized", "permission denied", "invalid config",
            "security breach", "data corruption"
        ];

        Ok(critical_types.contains(&error_type) || 
           critical_patterns.iter().any(|&p| message.to_lowercase().contains(p)))
    }

    async fn handle_critical_error(&self, error_type: &str, event: &ErrorEvent) -> Result<(), String> {
        self.notify_admins(error_type, event).await?;
        self.log_critical_error(error_type, event).await?;
        self.attempt_recovery(error_type, event).await?;
        Ok(())
    }

    async fn schedule_retry(&self, error_type: &str, event: &ErrorEvent) -> Result<(), String> {
        let errors = self.errors.lock().await;
        let metrics = errors
            .get(error_type)
            .ok_or_else(|| format!("Error type '{}' not found", error_type))?;

        let delay = metrics.config.retry_delay * (1 << event.retry_count);
        
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            // Здесь будет логика retry
        });

        Ok(())
    }

    async fn notify_admins(&self, error_type: &str, event: &ErrorEvent) -> Result<(), String> {
        // Реализация оповещения администраторов
        Ok(())
    }

    async fn log_critical_error(&self, error_type: &str, event: &ErrorEvent) -> Result<(), String> {
        // Реализация логирования критических ошибок
        Ok(())
    }

    async fn attempt_recovery(&self, error_type: &str, event: &ErrorEvent) -> Result<(), String> {
        // Реализация попытки восстановления
        Ok(())
    }
} 