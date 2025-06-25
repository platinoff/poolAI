use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::core::error::CursorError;
use crate::core::config::AppConfig;

/// Worker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: String,
    pub address: String,
    pub mining_power: f64,
    pub status: WorkerStatus,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Active,
    Inactive,
    Error,
}

/// Node status for distributed systems
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Degraded,
    Failed,
}

/// Main application state
pub struct AppState {
    pub workers: Arc<RwLock<HashMap<String, Worker>>>,
    pub config: Arc<RwLock<AppConfig>>,
    pub start_time: DateTime<Utc>,
    pub is_initialized: Arc<RwLock<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(AppConfig::default())),
            start_time: Utc::now(),
            is_initialized: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn initialize(&self) -> Result<(), CursorError> {
        let mut initialized = self.is_initialized.write();
        if *initialized {
            return Ok(());
        }

        // Initialize state
        log::info!("Initializing application state...");
        
        // Load configuration
        let config = AppConfig::load()?;
        *self.config.write() = config;
        
        *initialized = true;
        log::info!("Application state initialized successfully");
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<(), CursorError> {
        log::info!("Cleaning up application state...");
        
        // Clear workers
        self.workers.write().clear();
        
        // Mark as not initialized
        *self.is_initialized.write() = false;
        
        log::info!("Application state cleanup complete");
        Ok(())
    }

    pub fn add_worker(&self, worker: Worker) -> Result<(), CursorError> {
        let mut workers = self.workers.write();
        workers.insert(worker.id.clone(), worker);
        log::info!("Added worker: {}", worker.id);
        Ok(())
    }

    pub fn remove_worker(&self, worker_id: &str) -> Result<(), CursorError> {
        let mut workers = self.workers.write();
        if workers.remove(worker_id).is_some() {
            log::info!("Removed worker: {}", worker_id);
            Ok(())
        } else {
            Err(CursorError::NotFound(format!("Worker '{}' not found", worker_id)))
        }
    }

    pub fn get_worker(&self, worker_id: &str) -> Option<Worker> {
        self.workers.read().get(worker_id).cloned()
    }

    pub fn get_all_workers(&self) -> Vec<Worker> {
        self.workers.read().values().cloned().collect()
    }

    pub fn update_worker_status(&self, worker_id: &str, status: WorkerStatus) -> Result<(), CursorError> {
        let mut workers = self.workers.write();
        if let Some(worker) = workers.get_mut(worker_id) {
            worker.status = status;
            worker.last_seen = Utc::now();
            log::info!("Updated worker {} status to {:?}", worker_id, status);
            Ok(())
        } else {
            Err(CursorError::NotFound(format!("Worker '{}' not found", worker_id)))
        }
    }

    pub fn get_uptime(&self) -> std::time::Duration {
        let now = Utc::now();
        (now - self.start_time).to_std().unwrap_or_default()
    }

    pub fn is_ready(&self) -> bool {
        *self.is_initialized.read()
    }
} 