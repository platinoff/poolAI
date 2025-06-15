use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use solana_sdk::pubkey::Pubkey;
use tokio::time::{Duration, sleep};
use thiserror::Error;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid;
use reqwest;
use reqwest::ClientBuilder;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use std::sync::Mutex as StdMutex;
use ring::hmac;
use hex;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex as TokioMutex;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use crate::runtime::worker::WorkerManager;
use crate::runtime::scheduler::SchedulerSystem;
use crate::runtime::queue::QueueSystem;
use crate::runtime::cache::CacheSystem;
use crate::runtime::storage::StorageSystem;

#[derive(Error, Debug)]
pub enum PoolError {
    #[error("Pool connection error: {0}")]
    ConnectionError(String),
    #[error("Task migration error: {0}")]
    MigrationError(String),
    #[error("Pool synchronization error: {0}")]
    SyncError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Invalid task data: {0}")]
    InvalidTaskData(String),
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Error, Debug)]
pub enum PoolMigrationError {
    #[error("Pool connection error: {0}")]
    ConnectionError(String),
    #[error("Task migration error: {0}")]
    MigrationError(String),
    #[error("Pool synchronization error: {0}")]
    SyncError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Invalid task data: {0}")]
    InvalidTaskData(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolNode {
    pub id: String,
    pub url: String,
    pub auth_token: String,
    pub capacity: u32,
    pub current_load: u32,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub tls_cert: Option<Vec<u8>>,
    pub pubkey: Pubkey,
    pub load_factor: f64,
    pub available_slots: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationTask {
    pub id: uuid::Uuid,
    pub source_node: String,
    pub target_node: String,
    pub task_data: serde_json::Value,
    pub priority: u32,
    pub timestamp: i64,
    pub signature: String,
}

impl MigrationTask {
    pub fn new(
        source_node: String,
        target_node: String,
        task_data: serde_json::Value,
        priority: u32,
        secret_key: &[u8],
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let mut task = Self {
            id,
            source_node,
            target_node,
            task_data,
            priority,
            timestamp,
            signature: String::new(),
        };
        
        task.sign(secret_key);
        task
    }

    pub fn sign(&mut self, secret_key: &[u8]) {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key);
        let message = format!(
            "{}{}{}{}{}",
            self.id, self.source_node, self.target_node, self.timestamp, self.priority
        );
        let signature = hmac::sign(&key, message.as_bytes());
        self.signature = hex::encode(signature.as_ref());
    }

    pub fn verify(&self, secret_key: &[u8]) -> bool {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key);
        let message = format!(
            "{}{}{}{}{}",
            self.id, self.source_node, self.target_node, self.timestamp, self.priority
        );
        let signature = hex::decode(&self.signature).unwrap_or_default();
        hmac::verify(&key, message.as_bytes(), &signature).is_ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMirrorTask {
    pub id: uuid::Uuid,
    pub source_node: String,
    pub target_node: String,
    pub file_path: String,
    pub file_size: u64,
    pub priority: u32,
    pub timestamp: i64,
    pub signature: String,
    pub checksum: String,
}

impl FileMirrorTask {
    pub fn new(
        source_node: String,
        target_node: String,
        file_path: String,
        file_size: u64,
        priority: u32,
        secret_key: &[u8],
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let mut task = Self {
            id,
            source_node,
            target_node,
            file_path,
            file_size,
            priority,
            timestamp,
            signature: String::new(),
            checksum: String::new(),
        };
        
        task.sign(secret_key);
        task
    }

    pub fn sign(&mut self, secret_key: &[u8]) {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key);
        let message = format!(
            "{}{}{}{}{}{}",
            self.id, self.source_node, self.target_node, self.file_path, self.timestamp, self.priority
        );
        let signature = hmac::sign(&key, message.as_bytes());
        self.signature = hex::encode(signature.as_ref());
    }

    pub fn verify(&self, secret_key: &[u8]) -> bool {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key);
        let message = format!(
            "{}{}{}{}{}{}",
            self.id, self.source_node, self.target_node, self.file_path, self.timestamp, self.priority
        );
        let signature = hex::decode(&self.signature).unwrap_or_default();
        hmac::verify(&key, message.as_bytes(), &signature).is_ok()
    }
}

pub struct PoolMigrationManager {
    nodes: Arc<RwLock<HashMap<Pubkey, PoolNode>>>,
    migration_queue: Arc<RwLock<Vec<MigrationTask>>>,
    local_pubkey: Pubkey,
    local_address: String,
    auth_key: StdMutex<Vec<u8>>,
    rng: StdMutex<SystemRandom>,
    client: ClientBuilder,
}

impl PoolMigrationManager {
    pub fn new(local_pubkey: Pubkey, local_address: String) -> Self {
        let mut rng = SystemRandom::new();
        let mut auth_key = vec![0u8; 32];
        rng.fill(&mut auth_key).unwrap();

        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            migration_queue: Arc::new(RwLock::new(Vec::new())),
            local_pubkey,
            local_address,
            auth_key: StdMutex::new(auth_key),
            rng: StdMutex::new(rng),
            client: ClientBuilder::new(),
        }
    }

    pub async fn register_node(&self, node: PoolNode) -> Result<(), PoolMigrationError> {
        let mut nodes = self.nodes.write();
        nodes.insert(node.pubkey, node.clone());
        info!("Registered new pool node: {:?}", node);
        Ok(())
    }

    pub async fn update_node_status(&self, pubkey: Pubkey, load_factor: f64, available_slots: u32) {
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(&pubkey) {
            node.load_factor = load_factor;
            node.available_slots = available_slots;
            node.last_heartbeat = chrono::Utc::now();
        }
    }

    pub async fn find_optimal_target(&self, task_size: u32) -> Option<PoolNode> {
        let nodes = self.nodes.read();
        nodes.values()
            .filter(|node| node.available_slots >= task_size)
            .min_by(|a, b| a.load_factor.partial_cmp(&b.load_factor).unwrap())
            .cloned()
    }

    pub async fn queue_migration(&self, task: MigrationTask) -> Result<(), PoolMigrationError> {
        let mut queue = self.migration_queue.write();
        queue.push(task);
        info!("Queued migration task: {:?}", task);
        Ok(())
    }

    pub async fn process_migration_queue(&self) {
        loop {
            let task = {
                let mut queue = self.migration_queue.write();
                if queue.is_empty() {
                    None
                } else {
                    Some(queue.remove(0))
                }
            };

            if let Some(task) = task {
                if let Err(e) = self.execute_migration(&task).await {
                    error!("Failed to execute migration task: {:?}", e);
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn execute_migration(&self, task: &MigrationTask) -> Result<(), PoolMigrationError> {
        if let Some(file_task) = task.task_data.as_object()
            .and_then(|obj| serde_json::from_value::<FileMirrorTask>(serde_json::Value::Object(obj.clone())).ok()) {
            return self.execute_file_migration(&file_task).await;
        }
        
        // ... existing migration logic ...
        Ok(())
    }

    async fn execute_file_migration(&self, task: &FileMirrorTask) -> Result<(), PoolMigrationError> {
        let nodes = self.nodes.read();
        let target_node = nodes.get(&task.target_node)
            .ok_or_else(|| PoolMigrationError::NodeNotFound(task.target_node.clone()))?;

        let client = self.client.build()
            .map_err(|e| PoolMigrationError::ConnectionError(e.to_string()))?;

        let file = std::fs::File::open(&task.file_path)
            .map_err(|e| PoolMigrationError::InvalidTaskData(format!("Failed to open file: {}", e)))?;

        let response = client
            .post(&format!("{}/mirror", target_node.url))
            .header("Authorization", format!("Bearer {}", target_node.auth_token))
            .body(file)
            .send()
            .await
            .map_err(|e| PoolMigrationError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(PoolMigrationError::ConnectionError(
                format!("Failed to mirror file: {}", response.status())
            ));
        }

        Ok(())
    }

    pub async fn monitor_pool_health(&self) {
        loop {
            let nodes = self.nodes.read();
            for node in nodes.values() {
                // Check node health
                if node.last_heartbeat.timestamp() < chrono::Utc::now().timestamp() - 300 {
                    warn!("Node {} is not responding", node.id);
                }
            }
            sleep(Duration::from_secs(60)).await;
        }
    }

    pub async fn balance_load(&self) {
        loop {
            let nodes = self.nodes.read();
            let total_load: f64 = nodes.values().map(|n| n.load_factor).sum();
            let avg_load = total_load / nodes.len() as f64;

            for node in nodes.values() {
                if node.load_factor > avg_load * 1.2 {
                    // Trigger load balancing
                    info!("Node {} is overloaded, triggering load balancing", node.id);
                }
            }
            sleep(Duration::from_secs(300)).await;
        }
    }

    pub async fn mirror_file(&self, source_path: String, target_node: String) -> Result<(), PoolMigrationError> {
        let file_size = std::fs::metadata(&source_path)
            .map_err(|e| PoolMigrationError::InvalidTaskData(format!("Failed to get file metadata: {}", e)))?
            .len();

        let task = FileMirrorTask::new(
            self.local_address.clone(),
            target_node,
            source_path,
            file_size,
            1, // High priority for file transfers
            &self.auth_key.lock().unwrap(),
        );

        self.queue_migration(task.into()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_registration() {
        let manager = PoolMigrationManager::new(Pubkey::new_unique(), "localhost:8080".to_string());
        let node = PoolNode {
            id: "test".to_string(),
            url: "http://localhost:8080".to_string(),
            auth_token: "token".to_string(),
            capacity: 100,
            current_load: 0,
            last_heartbeat: chrono::Utc::now(),
            tls_cert: None,
            pubkey: Pubkey::new_unique(),
            load_factor: 0.0,
            available_slots: 100,
        };
        assert!(manager.register_node(node).await.is_ok());
    }

    #[tokio::test]
    async fn test_optimal_target_selection() {
        let manager = PoolMigrationManager::new(Pubkey::new_unique(), "localhost:8080".to_string());
        let node = PoolNode {
            id: "test".to_string(),
            url: "http://localhost:8080".to_string(),
            auth_token: "token".to_string(),
            capacity: 100,
            current_load: 0,
            last_heartbeat: chrono::Utc::now(),
            tls_cert: None,
            pubkey: Pubkey::new_unique(),
            load_factor: 0.0,
            available_slots: 100,
        };
        manager.register_node(node).await.unwrap();
        assert!(manager.find_optimal_target(50).await.is_some());
    }
} 