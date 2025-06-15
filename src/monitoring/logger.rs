use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use cursor_codes::core::error::CursorError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub log_level: String,
    pub log_file: String,
    pub max_file_size: u64,
    pub max_files: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerStats {
    pub total_logs: u64,
    pub info_logs: u64,
    pub warn_logs: u64,
    pub error_logs: u64,
    pub current_file_size: u64,
    pub current_file_count: u32,
    pub last_log_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerMetrics {
    pub config: LoggerConfig,
    pub stats: LoggerStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub logger_id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

pub struct LoggerSystem {
    loggers: Arc<Mutex<HashMap<String, LoggerMetrics>>>,
    entries: Arc<Mutex<HashMap<String, LogEntry>>>,
}

impl LoggerSystem {
    pub fn new() -> Self {
        Self {
            loggers: Arc::new(Mutex::new(HashMap::new())),
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_logger(&self, config: LoggerConfig) -> Result<(), String> {
        let mut loggers = self.loggers.lock().await;
        
        if loggers.contains_key(&config.id) {
            return Err(format!("Logger '{}' already exists", config.id));
        }

        let metrics = LoggerMetrics {
            config,
            stats: LoggerStats {
                total_logs: 0,
                info_logs: 0,
                warn_logs: 0,
                error_logs: 0,
                current_file_size: 0,
                current_file_count: 0,
                last_log_time: None,
                last_error: None,
            },
        };

        loggers.insert(metrics.config.id.clone(), metrics);
        info!("Added new logger: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_logger(&self, id: &str) -> Result<(), String> {
        let mut loggers = self.loggers.lock().await;
        let mut entries = self.entries.lock().await;
        
        if !loggers.contains_key(id) {
            return Err(format!("Logger '{}' not found", id));
        }

        // Remove associated entries
        entries.retain(|_, e| e.logger_id != id);
        
        loggers.remove(id);
        info!("Removed logger: {}", id);
        Ok(())
    }

    pub async fn log(
        &self,
        logger_id: &str,
        level: &str,
        message: &str,
        metadata: HashMap<String, String>,
    ) -> Result<String, String> {
        let mut loggers = self.loggers.lock().await;
        let mut entries = self.entries.lock().await;
        
        let logger = loggers
            .get_mut(logger_id)
            .ok_or_else(|| format!("Logger '{}' not found", logger_id))?;

        if !logger.config.active {
            return Err("Logger is not active".to_string());
        }

        if !self.is_valid_log_level(level, &logger.config.log_level) {
            return Err("Invalid log level".to_string());
        }

        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            logger_id: logger_id.to_string(),
            timestamp: Utc::now(),
            level: level.to_string(),
            message: message.to_string(),
            metadata,
        };

        entries.insert(entry.id.clone(), entry.clone());
        logger.stats.total_logs += 1;
        logger.stats.last_log_time = Some(entry.timestamp);

        match level {
            "info" => logger.stats.info_logs += 1,
            "warn" => logger.stats.warn_logs += 1,
            "error" => logger.stats.error_logs += 1,
            _ => {}
        }

        self.write_log_entry(&logger.config, &entry).await?;

        info!(
            "Logged message: {} with level: {} to logger: {}",
            entry.id, level, logger_id
        );
        Ok(entry.id)
    }

    async fn write_log_entry(&self, config: &LoggerConfig, entry: &LogEntry) -> Result<(), String> {
        let log_path = Path::new(&config.log_file);
        let log_dir = log_path.parent().ok_or("Invalid log file path")?;

        if !log_dir.exists() {
            std::fs::create_dir_all(log_dir)
                .map_err(|e| format!("Failed to create log directory: {}", e))?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;

        let log_line = format!(
            "[{}] [{}] {} - {}\n",
            entry.timestamp, entry.level, entry.logger_id, entry.message
        );

        file.write_all(log_line.as_bytes())
            .map_err(|e| format!("Failed to write to log file: {}", e))?;

        Ok(())
    }

    fn is_valid_log_level(&self, level: &str, config_level: &str) -> bool {
        match (level, config_level) {
            ("error", _) => true,
            ("warn", "warn" | "info") => true,
            ("info", "info") => true,
            _ => false,
        }
    }

    pub async fn get_logger(&self, id: &str) -> Result<LoggerMetrics, String> {
        let loggers = self.loggers.lock().await;
        
        loggers
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Logger '{}' not found", id))
    }

    pub async fn get_all_loggers(&self) -> Vec<LoggerMetrics> {
        let loggers = self.loggers.lock().await;
        loggers.values().cloned().collect()
    }

    pub async fn get_active_loggers(&self) -> Vec<LoggerMetrics> {
        let loggers = self.loggers.lock().await;
        loggers
            .values()
            .filter(|l| l.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_entries(&self, logger_id: &str) -> Vec<LogEntry> {
        let entries = self.entries.lock().await;
        entries
            .values()
            .filter(|e| e.logger_id == logger_id)
            .cloned()
            .collect()
    }

    pub async fn set_logger_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut loggers = self.loggers.lock().await;
        
        let logger = loggers
            .get_mut(id)
            .ok_or_else(|| format!("Logger '{}' not found", id))?;

        logger.config.active = active;
        info!(
            "Logger '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_logger_config(&self, id: &str, new_config: LoggerConfig) -> Result<(), String> {
        let mut loggers = self.loggers.lock().await;
        
        let logger = loggers
            .get_mut(id)
            .ok_or_else(|| format!("Logger '{}' not found", id))?;

        logger.config = new_config;
        info!("Updated logger configuration: {}", id);
        Ok(())
    }
} 