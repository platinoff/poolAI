use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub storage_type: String,
    pub max_size: u64,
    pub max_files: u32,
    pub max_file_size: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_size: u64,
    pub total_files: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub current_size: u64,
    pub current_files: u32,
    pub last_operation_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub config: StorageConfig,
    pub stats: StorageStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub storage_id: String,
    pub name: String,
    pub size: u64,
    pub content_type: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

pub struct StorageSystem {
    storages: Arc<Mutex<HashMap<String, StorageMetrics>>>,
    files: Arc<Mutex<HashMap<String, File>>>,
}

impl StorageSystem {
    pub fn new() -> Self {
        Self {
            storages: Arc::new(Mutex::new(HashMap::new())),
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_storage(&self, config: StorageConfig) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        
        if storages.contains_key(&config.id) {
            return Err(format!("Storage '{}' already exists", config.id));
        }

        let metrics = StorageMetrics {
            config,
            stats: StorageStats {
                total_size: 0,
                total_files: 0,
                successful_operations: 0,
                failed_operations: 0,
                current_size: 0,
                current_files: 0,
                last_operation_time: None,
                last_error: None,
            },
        };

        storages.insert(metrics.config.id.clone(), metrics);
        info!("Added new storage: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_storage(&self, id: &str) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        let mut files = self.files.lock().await;
        
        if !storages.contains_key(id) {
            return Err(format!("Storage '{}' not found", id));
        }

        // Remove associated files
        files.retain(|_, f| f.storage_id != id);
        
        storages.remove(id);
        info!("Removed storage: {}", id);
        Ok(())
    }

    pub async fn store_file(
        &self,
        storage_id: &str,
        name: &str,
        size: u64,
        content_type: &str,
    ) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        let mut files = self.files.lock().await;
        
        let storage = storages
            .get_mut(storage_id)
            .ok_or_else(|| format!("Storage '{}' not found", storage_id))?;

        if !storage.config.active {
            return Err("Storage is not active".to_string());
        }

        if storage.stats.current_files >= storage.config.max_files {
            return Err("Storage has reached maximum files".to_string());
        }

        if storage.stats.current_size + size > storage.config.max_size {
            return Err("Storage has reached maximum size".to_string());
        }

        if size > storage.config.max_file_size {
            return Err("File size exceeds maximum".to_string());
        }

        let file = File {
            id: uuid::Uuid::new_v4().to_string(),
            storage_id: storage_id.to_string(),
            name: name.to_string(),
            size,
            content_type: content_type.to_string(),
            timestamp: Utc::now(),
            status: "pending".to_string(),
        };

        files.insert(file.id.clone(), file.clone());
        storage.stats.current_files += 1;
        storage.stats.current_size += size;
        storage.stats.total_files += 1;
        storage.stats.total_size += size;

        info!(
            "Stored file: {} in storage: {} (size: {}, type: {})",
            file.id, storage_id, size, content_type
        );
        Ok(())
    }

    pub async fn process_file(&self, file_id: &str) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        let mut files = self.files.lock().await;
        
        let file = files
            .get_mut(file_id)
            .ok_or_else(|| format!("File '{}' not found", file_id))?;

        let storage = storages
            .get_mut(&file.storage_id)
            .ok_or_else(|| format!("Storage '{}' not found", file.storage_id))?;

        if !storage.config.active {
            return Err("Storage is not active".to_string());
        }

        let start_time = Utc::now();

        match self.validate_file(file, &storage.config).await {
            Ok(_) => {
                file.status = "stored".to_string();
                storage.stats.successful_operations += 1;
            }
            Err(e) => {
                file.status = "failed".to_string();
                storage.stats.failed_operations += 1;
                storage.stats.current_files -= 1;
                storage.stats.current_size -= file.size;
                storage.stats.last_error = Some(e);
            }
        }

        storage.stats.last_operation_time = Some(start_time);
        info!("Processed file: {}", file_id);
        Ok(())
    }

    async fn validate_file(
        &self,
        file: &File,
        config: &StorageConfig,
    ) -> Result<(), String> {
        // Simulate file validation
        let is_valid = file.size <= config.max_file_size;
        
        // Simulate network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        if !is_valid {
            return Err("File size exceeds maximum".to_string());
        }

        info!(
            "Validated file: {} in storage: {} (size: {}, type: {})",
            file.id, file.storage_id, file.size, file.content_type
        );
        Ok(())
    }

    pub async fn get_storage(&self, id: &str) -> Result<StorageMetrics, String> {
        let storages = self.storages.lock().await;
        
        storages
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Storage '{}' not found", id))
    }

    pub async fn get_all_storages(&self) -> Vec<StorageMetrics> {
        let storages = self.storages.lock().await;
        storages.values().cloned().collect()
    }

    pub async fn get_active_storages(&self) -> Vec<StorageMetrics> {
        let storages = self.storages.lock().await;
        storages
            .values()
            .filter(|s| s.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_files(&self, storage_id: &str) -> Vec<File> {
        let files = self.files.lock().await;
        files
            .values()
            .filter(|f| f.storage_id == storage_id)
            .cloned()
            .collect()
    }

    pub async fn set_storage_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        
        let storage = storages
            .get_mut(id)
            .ok_or_else(|| format!("Storage '{}' not found", id))?;

        storage.config.active = active;
        info!(
            "Storage '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_storage_config(&self, id: &str, new_config: StorageConfig) -> Result<(), String> {
        let mut storages = self.storages.lock().await;
        
        let storage = storages
            .get_mut(id)
            .ok_or_else(|| format!("Storage '{}' not found", id))?;

        storage.config = new_config;
        info!("Updated storage configuration: {}", id);
        Ok(())
    }
} 