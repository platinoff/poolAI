use std::collections::HashMap;
use tokio::sync::RwLock;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Worker already exists: {0}")]
    WorkerExists(String),
    #[error("Worker not found: {0}")]
    WorkerNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub id: String,
    pub address: String,
    pub status: WorkerStatus,
    pub storage_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Online,
    Offline,
    Busy,
}

pub struct WorkerManager {
    workers: HashMap<String, WorkerInfo>,
}

impl WorkerManager {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        // Инициализация менеджера воркеров
        Ok(())
    }

    pub async fn add_worker(&mut self, worker: WorkerInfo) -> Result<(), Error> {
        if self.workers.contains_key(&worker.id) {
            return Err(Error::WorkerExists(worker.id));
        }

        self.workers.insert(worker.id.clone(), worker);
        Ok(())
    }

    pub async fn remove_worker(&mut self, worker_id: &str) -> Result<(), Error> {
        if !self.workers.contains_key(worker_id) {
            return Err(Error::WorkerNotFound(worker_id.to_string()));
        }

        self.workers.remove(worker_id);
        Ok(())
    }

    pub async fn update_worker_status(&mut self, worker_id: &str, status: WorkerStatus) -> Result<(), Error> {
        if let Some(worker) = self.workers.get_mut(worker_id) {
            worker.status = status;
            Ok(())
        } else {
            Err(Error::WorkerNotFound(worker_id.to_string()))
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Остановка всех воркеров
        self.workers.clear();
        Ok(())
    }
} 