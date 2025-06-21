//! Config Manager - Управление конфигурацией системы

use crate::core::config::AppConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fs;
use std::path::Path;
use log::{info, warn, error};

/// Менеджер конфигурации
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    config_path: String,
}

impl ConfigManager {
    /// Создает новый менеджер конфигурации
    pub fn new(config_path: String) -> Self {
        Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            config_path,
        }
    }

    /// Загружает конфигурацию из файла
    pub async fn load_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Loading configuration from {}", self.config_path);
        
        if !Path::new(&self.config_path).exists() {
            log::warn!("ConfigManager: Configuration file not found, creating default");
            self.save_config().await?;
            return Ok(());
        }

        let config_content = fs::read_to_string(&self.config_path)?;
        let config: AppConfig = serde_json::from_str(&config_content)?;
        
        let mut current_config = self.config.write().await;
        *current_config = config;
        
        log::info!("ConfigManager: Configuration loaded successfully");
        Ok(())
    }

    /// Сохраняет конфигурацию в файл
    pub async fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Saving configuration to {}", self.config_path);
        
        let config = self.config.read().await;
        let config_content = serde_json::to_string_pretty(&*config)?;
        
        fs::write(&self.config_path, config_content)?;
        
        log::info!("ConfigManager: Configuration saved successfully");
        Ok(())
    }

    /// Получает текущую конфигурацию
    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }

    /// Обновляет конфигурацию
    pub async fn update_config(&self, new_config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Updating configuration");
        
        let mut config = self.config.write().await;
        *config = new_config;
        
        self.save_config().await?;
        
        log::info!("ConfigManager: Configuration updated successfully");
        Ok(())
    }

    /// Обновляет отдельные параметры конфигурации
    pub async fn update_config_section(&self, section: &str, values: HashMap<String, serde_json::Value>) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Updating configuration section: {}", section);
        
        let mut config = self.config.write().await;
        
        match section {
            "pool" => {
                if let Some(port) = values.get("port") {
                    if let Some(port_val) = port.as_u64() {
                        config.pool.port = port_val as u16;
                    }
                }
                if let Some(host) = values.get("host") {
                    if let Some(host_val) = host.as_str() {
                        config.pool.host = host_val.to_string();
                    }
                }
            }
            "database" => {
                if let Some(url) = values.get("url") {
                    if let Some(url_val) = url.as_str() {
                        config.database.url = url_val.to_string();
                    }
                }
            }
            "logging" => {
                if let Some(level) = values.get("level") {
                    if let Some(level_val) = level.as_str() {
                        config.logging.level = level_val.to_string();
                    }
                }
            }
            "security" => {
                if let Some(secret_key) = values.get("secret_key") {
                    if let Some(secret_key_val) = secret_key.as_str() {
                        config.security.secret_key = secret_key_val.to_string();
                    }
                }
            }
            _ => {
                return Err("Unknown configuration section".into());
            }
        }
        
        self.save_config().await?;
        
        log::info!("ConfigManager: Configuration section updated successfully");
        Ok(())
    }

    /// Получает значение параметра конфигурации
    pub async fn get_config_value(&self, section: &str, key: &str) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        
        match section {
            "pool" => {
                match key {
                    "port" => Ok(Some(serde_json::Value::Number(config.pool.port.into()))),
                    "host" => Ok(Some(serde_json::Value::String(config.pool.host.clone()))),
                    _ => Ok(None),
                }
            }
            "database" => {
                match key {
                    "url" => Ok(Some(serde_json::Value::String(config.database.url.clone()))),
                    _ => Ok(None),
                }
            }
            "logging" => {
                match key {
                    "level" => Ok(Some(serde_json::Value::String(config.logging.level.clone()))),
                    _ => Ok(None),
                }
            }
            "security" => {
                match key {
                    "secret_key" => Ok(Some(serde_json::Value::String(config.security.secret_key.clone()))),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    /// Устанавливает значение параметра конфигурации
    pub async fn set_config_value(&self, section: &str, key: &str, value: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Setting config value: {}.{}", section, key);
        
        let mut values = HashMap::new();
        values.insert(key.to_string(), value);
        
        self.update_config_section(section, values).await
    }

    /// Валидирует конфигурацию
    pub async fn validate_config(&self) -> Result<ConfigValidationResult, Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        let mut result = ConfigValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Проверка пула
        if config.pool.port == 0 {
            result.errors.push("Pool port cannot be 0".to_string());
            result.is_valid = false;
        }

        if config.pool.host.is_empty() {
            result.errors.push("Pool host cannot be empty".to_string());
            result.is_valid = false;
        }

        // Проверка базы данных
        if config.database.url.is_empty() {
            result.errors.push("Database URL cannot be empty".to_string());
            result.is_valid = false;
        }

        // Проверка безопасности
        if config.security.secret_key.is_empty() {
            result.warnings.push("Secret key is empty, using default".to_string());
        }

        // Проверка логирования
        let valid_levels = vec!["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&config.logging.level.as_str()) {
            result.errors.push(format!("Invalid log level: {}", config.logging.level));
            result.is_valid = false;
        }

        Ok(result)
    }

    /// Создает резервную копию конфигурации
    pub async fn backup_config(&self) -> Result<String, Box<dyn std::error::Error>> {
        let backup_path = format!("{}.backup.{}", self.config_path, chrono::Utc::now().timestamp());
        
        let config_content = fs::read_to_string(&self.config_path)?;
        fs::write(&backup_path, config_content)?;
        
        log::info!("ConfigManager: Configuration backed up to {}", backup_path);
        Ok(backup_path)
    }

    /// Восстанавливает конфигурацию из резервной копии
    pub async fn restore_config(&self, backup_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ConfigManager: Restoring configuration from {}", backup_path);
        
        if !Path::new(backup_path).exists() {
            return Err("Backup file not found".into());
        }

        let backup_content = fs::read_to_string(backup_path)?;
        let config: AppConfig = serde_json::from_str(&backup_content)?;
        
        self.update_config(config).await?;
        
        log::info!("ConfigManager: Configuration restored successfully");
        Ok(())
    }
}

/// Результат валидации конфигурации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Информация о конфигурации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    pub path: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub size: u64,
    pub is_valid: bool,
} 