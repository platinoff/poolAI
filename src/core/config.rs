use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::net::IpAddr;
use std::str::FromStr;
use thiserror::Error;
use std::path::Path;
use std::fs;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub http_port: u16,
    pub https_port: u16,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub tls_version: String,
    pub cipher_suites: Vec<String>,
    pub enable_http2: bool,
    pub enable_ocsp_stapling: bool,
    pub cert_chain_path: Option<PathBuf>,
    pub bind_address: IpAddr,
    pub max_connections: usize,
    pub keep_alive: u64,
    pub client_timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaidConfig {
    pub raid_level: u8,
    pub min_disks: u8,
    pub stripe_size: usize,
    pub redundancy: u8,
    pub health_check_interval: u64,
    pub rebuild_priority: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub source_chain: String,
    pub target_chain: String,
    pub min_amount: f64,
    pub fee_percentage: f64,
    pub max_amount: f64,
    pub confirmation_blocks: u32,
    pub retry_attempts: u32,
    pub retry_delay: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub raid: RaidConfig,
    pub bridge: BridgeConfig,
    pub solana_rpc_url: String,
    pub log_level: String,
    pub environment: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                http_port: 8080,
                https_port: 8443,
                cert_path: PathBuf::from("cert.pem"),
                key_path: PathBuf::from("key.pem"),
                tls_version: "1.3".to_string(),
                cipher_suites: vec![
                    "TLS_AES_256_GCM_SHA384".to_string(),
                    "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                    "TLS_AES_128_GCM_SHA256".to_string(),
                ],
                enable_http2: true,
                enable_ocsp_stapling: true,
                cert_chain_path: None,
                bind_address: IpAddr::from_str("0.0.0.0").unwrap(),
                max_connections: 10000,
                keep_alive: 75,
                client_timeout: 30,
            },
            raid: RaidConfig {
                raid_level: 1,
                min_disks: 2,
                stripe_size: 1024 * 1024,
                redundancy: 1,
                health_check_interval: 60,
                rebuild_priority: 1,
            },
            bridge: BridgeConfig {
                source_chain: "ethereum".to_string(),
                target_chain: "solana".to_string(),
                min_amount: 0.1,
                fee_percentage: 0.01,
                max_amount: 1000.0,
                confirmation_blocks: 12,
                retry_attempts: 3,
                retry_delay: 5000,
            },
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            log_level: "info".to_string(),
            environment: "development".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
        
        let metadata = std::fs::metadata(&config_path)?;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(ConfigError::InvalidConfig(
                "Configuration file has unsafe permissions".to_string()
            ));
        }

        if std::path::Path::new(&config_path).exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            
            if let Some(signature) = std::env::var("CONFIG_SIGNATURE").ok() {
                if !Self::verify_config_signature(&contents, &signature)? {
                    return Err(ConfigError::InvalidConfig(
                        "Configuration signature verification failed".to_string()
                    ));
                }
            }

            let mut config: AppConfig = toml::from_str(&contents)?;
            config.validate()?;
            Ok(config)
        } else {
            let config = AppConfig::default();
            let contents = toml::to_string_pretty(&config)?;
            
            let mut file = std::fs::File::create(&config_path)?;
            file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
            file.write_all(contents.as_bytes())?;
            
            Ok(config)
        }
    }

    fn validate(&self) -> Result<(), ConfigError> {
        // Validate server configuration
        if self.server.http_port == self.server.https_port {
            return Err(ConfigError::InvalidConfig("HTTP and HTTPS ports must be different".to_string()));
        }

        if self.server.http_port == 0 || self.server.https_port == 0 {
            return Err(ConfigError::InvalidConfig("Port numbers must be greater than 0".to_string()));
        }

        if !self.server.cert_path.exists() {
            return Err(ConfigError::InvalidConfig("Certificate file not found".to_string()));
        }

        if !self.server.key_path.exists() {
            return Err(ConfigError::InvalidConfig("Key file not found".to_string()));
        }

        if let Some(chain_path) = &self.server.cert_chain_path {
            if !chain_path.exists() {
                return Err(ConfigError::InvalidConfig("Certificate chain file not found".to_string()));
            }
        }

        // Проверка безопасности серверной конфигурации
        if !self.is_safe_server_config()? {
            return Err(ConfigError::InvalidConfig("Unsafe server configuration".to_string()));
        }

        // Validate RAID configuration
        if self.raid.raid_level > 1 {
            return Err(ConfigError::InvalidConfig("Only RAID levels 0 and 1 are supported".to_string()));
        }

        if self.raid.min_disks < 2 {
            return Err(ConfigError::InvalidConfig("Minimum 2 disks required for RAID".to_string()));
        }

        if self.raid.stripe_size == 0 {
            return Err(ConfigError::InvalidConfig("Stripe size must be greater than 0".to_string()));
        }

        // Проверка безопасности RAID конфигурации
        if !self.is_safe_raid_config()? {
            return Err(ConfigError::InvalidConfig("Unsafe RAID configuration".to_string()));
        }

        // Validate bridge configuration
        if self.bridge.fee_percentage <= 0.0 || self.bridge.fee_percentage >= 1.0 {
            return Err(ConfigError::InvalidConfig("Fee percentage must be between 0 and 1".to_string()));
        }

        if self.bridge.min_amount >= self.bridge.max_amount {
            return Err(ConfigError::InvalidConfig("Minimum amount must be less than maximum amount".to_string()));
        }

        // Проверка безопасности bridge конфигурации
        if !self.is_safe_bridge_config()? {
            return Err(ConfigError::InvalidConfig("Unsafe bridge configuration".to_string()));
        }

        Ok(())
    }

    // Вспомогательные методы для проверки безопасности
    fn is_safe_server_config(&self) -> Result<bool, ConfigError> {
        // Проверка TLS версии
        if self.server.tls_version != "1.3" {
            return Err(ConfigError::InvalidConfig(
                "Only TLS 1.3 is supported for security reasons".to_string()
            ));
        }

        // Проверка cipher suites
        let unsafe_ciphers = ["RC4", "DES", "3DES", "MD5"];
        for cipher in &self.server.cipher_suites {
            if unsafe_ciphers.iter().any(|&c| cipher.contains(c)) {
                return Err(ConfigError::InvalidConfig(
                    format!("Unsafe cipher suite detected: {}", cipher)
                ));
            }
        }

        // Проверка максимального количества соединений
        if self.server.max_connections > 100000 {
            return Err(ConfigError::InvalidConfig(
                "Maximum connections limit too high".to_string()
            ));
        }

        // Проверка таймаутов
        if self.server.keep_alive > 300 || self.server.client_timeout > 60 {
            return Err(ConfigError::InvalidConfig(
                "Timeout values too high".to_string()
            ));
        }

        Ok(true)
    }

    fn is_safe_raid_config(&self) -> Result<bool, ConfigError> {
        // Проверка RAID уровня
        if self.raid.raid_level > 1 {
            return Err(ConfigError::InvalidConfig(
                "Only RAID levels 0 and 1 are supported".to_string()
            ));
        }

        // Проверка минимального количества дисков
        if self.raid.min_disks < 2 {
            return Err(ConfigError::InvalidConfig(
                "Minimum 2 disks required for RAID".to_string()
            ));
        }

        // Проверка размера страйпа
        if self.raid.stripe_size == 0 || self.raid.stripe_size > 1024 * 1024 * 1024 {
            return Err(ConfigError::InvalidConfig(
                "Invalid stripe size".to_string()
            ));
        }

        Ok(true)
    }

    fn is_safe_bridge_config(&self) -> Result<bool, ConfigError> {
        // Проверка комиссии
        if self.bridge.fee_percentage <= 0.0 || self.bridge.fee_percentage >= 1.0 {
            return Err(ConfigError::InvalidConfig(
                "Fee percentage must be between 0 and 1".to_string()
            ));
        }

        // Проверка минимальной и максимальной суммы
        if self.bridge.min_amount >= self.bridge.max_amount {
            return Err(ConfigError::InvalidConfig(
                "Minimum amount must be less than maximum amount".to_string()
            ));
        }

        // Проверка количества подтверждений
        if self.bridge.confirmation_blocks < 1 || self.bridge.confirmation_blocks > 100 {
            return Err(ConfigError::InvalidConfig(
                "Invalid confirmation blocks count".to_string()
            ));
        }

        Ok(true)
    }

    fn verify_config_signature(&self, contents: &str, signature: &str) -> Result<bool, ConfigError> {
        // Реализовать проверку подписи
        Ok(true) // Временная заглушка
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.http_port, 8080);
        assert_eq!(config.server.https_port, 8443);
        assert_eq!(config.raid.raid_level, 1);
        assert_eq!(config.bridge.source_chain, "ethereum");
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();
        
        // Test invalid port configuration
        config.server.http_port = config.server.https_port;
        assert!(config.validate().is_err());

        // Test invalid RAID configuration
        config = AppConfig::default();
        config.raid.raid_level = 2;
        assert!(config.validate().is_err());

        // Test invalid bridge configuration
        config = AppConfig::default();
        config.bridge.fee_percentage = 1.5;
        assert!(config.validate().is_err());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub values: HashMap<String, String>,
    pub last_modified: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStats {
    pub total_sections: u64,
    pub total_values: u64,
    pub last_load_time: Option<DateTime<Utc>>,
    pub last_save_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetrics {
    pub sections: HashMap<String, ConfigSection>,
    pub stats: ConfigStats,
}

pub struct ConfigSystem {
    config: Arc<Mutex<ConfigMetrics>>,
    file_path: String,
}

impl ConfigSystem {
    pub fn new(file_path: &str) -> Self {
        Self {
            config: Arc::new(Mutex::new(ConfigMetrics {
                sections: HashMap::new(),
                stats: ConfigStats {
                    total_sections: 0,
                    total_values: 0,
                    last_load_time: None,
                    last_save_time: None,
                    last_error: None,
                },
            })),
            file_path: file_path.to_string(),
        }
    }

    pub async fn load_config(&self) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Err("Config file does not exist".to_string());
        }

        let mut file = File::open(path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let metrics: ConfigMetrics = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        config.sections = metrics.sections;
        config.stats = metrics.stats;
        config.stats.last_load_time = Some(Utc::now());

        info!("Loaded configuration from: {}", self.file_path);
        Ok(())
    }

    pub async fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().await;
        
        let path = Path::new(&self.file_path);
        let parent = path.parent().ok_or("Invalid config file path")?;

        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;

        let contents = serde_json::to_string_pretty(&*config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        file.write_all(contents.as_bytes())
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        info!("Saved configuration to: {}", self.file_path);
        Ok(())
    }

    pub async fn add_section(&self, section: ConfigSection) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if config.sections.contains_key(&section.id) {
            return Err(format!("Section '{}' already exists", section.id));
        }

        config.sections.insert(section.id.clone(), section.clone());
        config.stats.total_sections += 1;
        config.stats.total_values += section.values.len() as u64;

        info!("Added new section: {}", section.id);
        Ok(())
    }

    pub async fn remove_section(&self, id: &str) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if let Some(section) = config.sections.remove(id) {
            config.stats.total_sections -= 1;
            config.stats.total_values -= section.values.len() as u64;
            info!("Removed section: {}", id);
            Ok(())
        } else {
            Err(format!("Section '{}' not found", id))
        }
    }

    pub async fn get_section(&self, id: &str) -> Result<ConfigSection, String> {
        let config = self.config.lock().await;
        
        config
            .sections
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Section '{}' not found", id))
    }

    pub async fn get_all_sections(&self) -> Vec<ConfigSection> {
        let config = self.config.lock().await;
        config.sections.values().cloned().collect()
    }

    pub async fn get_active_sections(&self) -> Vec<ConfigSection> {
        let config = self.config.lock().await;
        config
            .sections
            .values()
            .filter(|s| s.active)
            .cloned()
            .collect()
    }

    pub async fn set_value(
        &self,
        section_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        let section = config
            .sections
            .get_mut(section_id)
            .ok_or_else(|| format!("Section '{}' not found", section_id))?;

        if !section.active {
            return Err("Section is not active".to_string());
        }

        let old_value = section.values.insert(key.to_string(), value.to_string());
        section.last_modified = Some(Utc::now());

        if old_value.is_none() {
            config.stats.total_values += 1;
        }

        info!(
            "Set value: {} = {} in section: {}",
            key, value, section_id
        );
        Ok(())
    }

    pub async fn get_value(&self, section_id: &str, key: &str) -> Result<String, String> {
        let config = self.config.lock().await;
        
        let section = config
            .sections
            .get(section_id)
            .ok_or_else(|| format!("Section '{}' not found", section_id))?;

        if !section.active {
            return Err("Section is not active".to_string());
        }

        section
            .values
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Value '{}' not found in section '{}'", key, section_id))
    }

    pub async fn remove_value(&self, section_id: &str, key: &str) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        let section = config
            .sections
            .get_mut(section_id)
            .ok_or_else(|| format!("Section '{}' not found", section_id))?;

        if !section.active {
            return Err("Section is not active".to_string());
        }

        if section.values.remove(key).is_some() {
            config.stats.total_values -= 1;
            section.last_modified = Some(Utc::now());
            info!("Removed value: {} from section: {}", key, section_id);
            Ok(())
        } else {
            Err(format!("Value '{}' not found in section '{}'", key, section_id))
        }
    }

    pub async fn set_section_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        let section = config
            .sections
            .get_mut(id)
            .ok_or_else(|| format!("Section '{}' not found", id))?;

        section.active = active;
        info!(
            "Section '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_section(&self, id: &str, new_section: ConfigSection) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if !config.sections.contains_key(id) {
            return Err(format!("Section '{}' not found", id));
        }

        let old_section = config.sections.remove(id).unwrap();
        config.stats.total_values -= old_section.values.len() as u64;

        config.sections.insert(new_section.id.clone(), new_section.clone());
        config.stats.total_values += new_section.values.len() as u64;

        info!("Updated section: {}", id);
        Ok(())
    }
} 