use std::fs;
use std::path::Path;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidConfig {
    pub raid_level: u8,
    pub min_disks: usize,
    pub stripe_size: usize,
    pub redundancy: usize,
    pub health_check_interval: u64,
    pub node_timeout: u64,
    pub data_dir: String,
    pub mount_dir: String,
    pub worker_timeout: u64,
    pub max_retries: u32,
    pub retry_delay: u64,
}

impl Default for RaidConfig {
    fn default() -> Self {
        Self {
            raid_level: 0,
            min_disks: 2,
            stripe_size: 1024 * 1024, // 1MB
            redundancy: 1,
            health_check_interval: 10,
            node_timeout: 30,
            data_dir: "data".to_string(),
            mount_dir: "mnt".to_string(),
            worker_timeout: 60,
            max_retries: 3,
            retry_delay: 5,
        }
    }
}

impl RaidConfig {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let config: RaidConfig = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.raid_level > 1 {
            return Err(Error::ConfigError("Unsupported RAID level".to_string()));
        }

        if self.min_disks < 2 {
            return Err(Error::ConfigError("Minimum 2 disks required".to_string()));
        }

        if self.stripe_size == 0 {
            return Err(Error::ConfigError("Stripe size must be greater than 0".to_string()));
        }

        if self.redundancy == 0 {
            return Err(Error::ConfigError("Redundancy must be greater than 0".to_string()));
        }

        Ok(())
    }
} 