use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::env;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilsConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub utility_type: String,
    pub max_retries: u32,
    pub retry_delay: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilsStats {
    pub total_operations: u64,
    pub total_retries: u64,
    pub total_success: u64,
    pub total_failure: u64,
    pub last_operation_time: Option<DateTime<Utc>>,
    pub last_success_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilsMetrics {
    pub config: UtilsConfig,
    pub stats: UtilsStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub utility_type: String,
    pub operation_type: String,
    pub parameters: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub error: Option<String>,
    pub retry_count: u32,
}

pub struct UtilsSystem {
    utils: Arc<Mutex<HashMap<String, UtilsMetrics>>>,
    operations: Arc<Mutex<HashMap<String, Vec<Operation>>>>,
}

impl UtilsSystem {
    pub fn new() -> Self {
        Self {
            utils: Arc::new(Mutex::new(HashMap::new())),
            operations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_utility(&self, config: UtilsConfig) -> Result<(), String> {
        let mut utils = self.utils.lock().await;
        
        if utils.contains_key(&config.id) {
            return Err(format!("Utility '{}' already exists", config.id));
        }

        utils.insert(
            config.id.clone(),
            UtilsMetrics {
                config: config.clone(),
                stats: UtilsStats {
                    total_operations: 0,
                    total_retries: 0,
                    total_success: 0,
                    total_failure: 0,
                    last_operation_time: None,
                    last_success_time: None,
                    last_error: None,
                },
            },
        );

        info!("Added new utility: {}", config.id);
        Ok(())
    }

    pub async fn remove_utility(&self, id: &str) -> Result<(), String> {
        let mut utils = self.utils.lock().await;
        let mut operations = self.operations.lock().await;
        
        if utils.remove(id).is_some() {
            operations.remove(id);
            info!("Removed utility: {}", id);
            Ok(())
        } else {
            Err(format!("Utility '{}' not found", id))
        }
    }

    pub async fn execute_operation(
        &self,
        utility_id: &str,
        operation_type: &str,
        parameters: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut utils = self.utils.lock().await;
        let mut operations = self.operations.lock().await;
        
        let metrics = utils
            .get_mut(utility_id)
            .ok_or_else(|| format!("Utility '{}' not found", utility_id))?;

        if !metrics.config.active {
            return Err("Utility is not active".to_string());
        }

        let operation = Operation {
            id: format!("{}-{}", utility_id, Utc::now().timestamp()),
            utility_type: utility_id.to_string(),
            operation_type: operation_type.to_string(),
            parameters: parameters.clone(),
            timestamp: Utc::now(),
            success: false,
            error: None,
            retry_count: 0,
        };

        metrics.stats.total_operations += 1;
        metrics.stats.last_operation_time = Some(operation.timestamp);

        let result = match operation_type {
            "file_copy" => self.copy_file(&parameters).await,
            "file_move" => self.move_file(&parameters).await,
            "file_delete" => self.delete_file(&parameters).await,
            "directory_create" => self.create_directory(&parameters).await,
            "directory_delete" => self.delete_directory(&parameters).await,
            "command_execute" => self.execute_command(&parameters).await,
            "environment_set" => self.set_environment(&parameters).await,
            "environment_get" => self.get_environment(&parameters).await,
            _ => Err(format!("Unknown operation type: {}", operation_type)),
        };

        match result {
            Ok(_) => {
                metrics.stats.total_success += 1;
                metrics.stats.last_success_time = Some(Utc::now());
                info!(
                    "Executed operation: {} - {}",
                    utility_id, operation_type
                );
            }
            Err(e) => {
                metrics.stats.total_failure += 1;
                metrics.stats.last_error = Some(e.clone());
                error!(
                    "Failed to execute operation: {} - {}: {}",
                    utility_id, operation_type, e
                );
            }
        }

        operations
            .entry(utility_id.to_string())
            .or_insert_with(Vec::new)
            .push(operation);

        result
    }

    async fn copy_file(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let source = parameters
            .get("source")
            .ok_or_else(|| "Missing source parameter".to_string())?;
        let destination = parameters
            .get("destination")
            .ok_or_else(|| "Missing destination parameter".to_string())?;

        if !self.is_safe_path(source) || !self.is_safe_path(destination) {
            return Err("Unsafe path detected".to_string());
        }

        if !Path::new(source).exists() {
            return Err("Source file does not exist".to_string());
        }

        if !self.has_permission(source, "read")? {
            return Err("No read permission for source file".to_string());
        }

        if !self.has_enough_space(destination)? {
            return Err("Not enough disk space".to_string());
        }

        let temp_dest = format!("{}.tmp", destination);
        std::fs::copy(source, &temp_dest)
            .map_err(|e| format!("Failed to copy file: {}", e))?;

        std::fs::rename(&temp_dest, destination)
            .map_err(|e| format!("Failed to finalize copy: {}", e))?;

        Ok(())
    }

    async fn move_file(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let source = parameters
            .get("source")
            .ok_or_else(|| "Missing source parameter".to_string())?;
        let destination = parameters
            .get("destination")
            .ok_or_else(|| "Missing destination parameter".to_string())?;

        std::fs::rename(source, destination)
            .map_err(|e| format!("Failed to move file: {}", e))?;

        Ok(())
    }

    async fn delete_file(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let path = parameters
            .get("path")
            .ok_or_else(|| "Missing path parameter".to_string())?;

        std::fs::remove_file(path)
            .map_err(|e| format!("Failed to delete file: {}", e))?;

        Ok(())
    }

    async fn create_directory(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let path = parameters
            .get("path")
            .ok_or_else(|| "Missing path parameter".to_string())?;

        std::fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        Ok(())
    }

    async fn delete_directory(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let path = parameters
            .get("path")
            .ok_or_else(|| "Missing path parameter".to_string())?;

        std::fs::remove_dir_all(path)
            .map_err(|e| format!("Failed to delete directory: {}", e))?;

        Ok(())
    }

    async fn execute_command(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let command = parameters
            .get("command")
            .ok_or_else(|| "Missing command parameter".to_string())?;

        if !self.is_safe_command(command)? {
            return Err("Unsafe command detected".to_string());
        }

        let output = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .await
                    .map_err(|e| format!("Failed to execute command: {}", e))?
            }
        ).await;

        match output {
            Ok(Ok(output)) => {
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Command failed: {}", error));
                }
                Ok(())
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Command execution timed out".to_string())
        }
    }

    async fn set_environment(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let key = parameters
            .get("key")
            .ok_or_else(|| "Missing key parameter".to_string())?;
        let value = parameters
            .get("value")
            .ok_or_else(|| "Missing value parameter".to_string())?;

        if !self.is_safe_env_key(key)? {
            return Err("Unsafe environment key".to_string());
        }

        if !self.is_safe_env_value(value)? {
            return Err("Unsafe environment value".to_string());
        }

        std::env::set_var(key, value);
        Ok(())
    }

    async fn get_environment(&self, parameters: &HashMap<String, String>) -> Result<(), String> {
        let key = parameters
            .get("key")
            .ok_or_else(|| "Missing key parameter".to_string())?;

        env::var(key).map_err(|e| format!("Failed to get environment variable: {}", e))?;
        Ok(())
    }

    pub async fn get_utility(&self, id: &str) -> Result<UtilsMetrics, String> {
        let utils = self.utils.lock().await;
        
        utils
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Utility '{}' not found", id))
    }

    pub async fn get_all_utilities(&self) -> Vec<UtilsMetrics> {
        let utils = self.utils.lock().await;
        utils.values().cloned().collect()
    }

    pub async fn get_active_utilities(&self) -> Vec<UtilsMetrics> {
        let utils = self.utils.lock().await;
        utils
            .values()
            .filter(|u| u.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_operations(&self, utility_id: &str) -> Result<Vec<Operation>, String> {
        let operations = self.operations.lock().await;
        
        operations
            .get(utility_id)
            .cloned()
            .ok_or_else(|| format!("No operations found for utility '{}'", utility_id))
    }

    pub async fn set_utility_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut utils = self.utils.lock().await;
        
        let metrics = utils
            .get_mut(id)
            .ok_or_else(|| format!("Utility '{}' not found", id))?;

        metrics.config.active = active;
        info!(
            "Utility '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_utility_config(&self, id: &str, new_config: UtilsConfig) -> Result<(), String> {
        let mut utils = self.utils.lock().await;
        
        if !utils.contains_key(id) {
            return Err(format!("Utility '{}' not found", id));
        }

        let metrics = utils.get_mut(id).unwrap();
        metrics.config = new_config;

        info!("Updated utility: {}", id);
        Ok(())
    }

    fn is_safe_path(&self, path: &str) -> bool {
        let path = Path::new(path);
        let base = Path::new("/safe/base/path");
        path.starts_with(base)
    }

    fn has_permission(&self, path: &str, operation: &str) -> Result<bool, String> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;
        
        let permissions = metadata.permissions();
        match operation {
            "read" => Ok(permissions.readonly()),
            "write" => Ok(!permissions.readonly()),
            _ => Err("Invalid operation".to_string())
        }
    }

    fn has_enough_space(&self, path: &str) -> Result<bool, String> {
        let path = Path::new(path);
        let available = fs2::available_space(path)
            .map_err(|e| format!("Failed to get available space: {}", e))?;
        
        Ok(available > 1024 * 1024 * 100)
    }

    fn is_safe_command(&self, command: &str) -> Result<bool, String> {
        let dangerous = [
            "rm -rf", "mkfs", "dd", "format", "chmod 777",
            "chown", "passwd", "useradd", "userdel"
        ];

        if dangerous.iter().any(|&cmd| command.contains(cmd)) {
            return Err("Dangerous command detected".to_string());
        }

        if command.contains("|") || command.contains(";") || command.contains("&&") {
            return Err("Command contains unsafe operators".to_string());
        }

        Ok(true)
    }

    fn is_safe_env_key(&self, key: &str) -> Result<bool, String> {
        let system_vars = ["PATH", "HOME", "USER", "SHELL", "TERM"];
        if system_vars.contains(&key) {
            return Err("Cannot modify system environment variables".to_string());
        }

        if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Invalid environment key format".to_string());
        }

        Ok(true)
    }

    fn is_safe_env_value(&self, value: &str) -> Result<bool, String> {
        if value.len() > 1024 {
            return Err("Environment value too long".to_string());
        }

        if value.contains(|c: char| !c.is_ascii()) {
            return Err("Environment value contains non-ASCII characters".to_string());
        }

        Ok(true)
    }
} 