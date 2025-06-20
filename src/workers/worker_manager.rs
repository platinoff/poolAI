//! Worker Manager - Управление воркерами

use crate::core::state::AppState;
use crate::pool::pool::PoolManager;
use crate::monitoring::metrics::WorkerMetrics;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};

/// Менеджер воркеров
pub struct WorkerManager {
    workers: Arc<RwLock<HashMap<String, Worker>>>,
    app_state: Arc<AppState>,
    pool_manager: Arc<PoolManager>,
}

impl WorkerManager {
    /// Создает новый менеджер воркеров
    pub fn new(
        app_state: Arc<AppState>,
        pool_manager: Arc<PoolManager>,
    ) -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
            app_state,
            pool_manager,
        }
    }

    /// Добавляет нового воркера
    pub async fn add_worker(&self, worker: Worker) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        
        if workers.contains_key(&worker.id) {
            return Err("Worker with this ID already exists".into());
        }
        
        workers.insert(worker.id.clone(), worker.clone());
        info!("Worker {} added successfully", worker.id);
        
        // Уведомляем пул о новом воркере
        self.pool_manager.add_worker(&worker.id).await?;
        
        Ok(())
    }

    /// Удаляет воркера
    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        
        if workers.remove(worker_id).is_some() {
            info!("Worker {} removed successfully", worker_id);
            
            // Уведомляем пул об удалении воркера
            self.pool_manager.remove_worker(worker_id).await?;
        } else {
            warn!("Worker {} not found", worker_id);
        }
        
        Ok(())
    }

    /// Получает список всех воркеров
    pub async fn get_workers(&self) -> Vec<Worker> {
        let workers = self.workers.read().await;
        workers.values().cloned().collect()
    }

    /// Получает воркера по ID
    pub async fn get_worker(&self, worker_id: &str) -> Option<Worker> {
        let workers = self.workers.read().await;
        workers.get(worker_id).cloned()
    }

    /// Обновляет статус воркера
    pub async fn update_worker_status(&self, worker_id: &str, status: WorkerStatus) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        
        if let Some(worker) = workers.get_mut(worker_id) {
            worker.status = status;
            worker.last_seen = chrono::Utc::now();
            info!("Worker {} status updated to {:?}", worker_id, status);
        } else {
            return Err("Worker not found".into());
        }
        
        Ok(())
    }

    /// Обновляет метрики воркера
    pub async fn update_worker_metrics(&self, worker_id: &str, metrics: WorkerMetrics) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        
        if let Some(worker) = workers.get_mut(worker_id) {
            worker.cpu_usage = metrics.cpu_usage;
            worker.memory_usage = metrics.memory_usage;
            worker.gpu_usage = metrics.gpu_usage;
            worker.hashrate = metrics.hashrate;
            worker.last_seen = chrono::Utc::now();
        } else {
            return Err("Worker not found".into());
        }
        
        Ok(())
    }

    /// Получает активных воркеров
    pub async fn get_active_workers(&self) -> Vec<Worker> {
        let workers = self.workers.read().await;
        workers.values()
            .filter(|w| w.status == WorkerStatus::Active)
            .cloned()
            .collect()
    }

    /// Получает статистику воркеров
    pub async fn get_worker_stats(&self) -> WorkerStats {
        let workers = self.workers.read().await;
        let total_workers = workers.len();
        let active_workers = workers.values().filter(|w| w.status == WorkerStatus::Active).count();
        let total_hashrate: f64 = workers.values().map(|w| w.hashrate).sum();
        
        let average_load = if !workers.is_empty() {
            let total_load: f64 = workers.values()
                .map(|w| (w.cpu_usage + w.memory_usage + w.gpu_usage) / 3.0)
                .sum();
            total_load / workers.len() as f64
        } else {
            0.0
        };
        
        WorkerStats {
            total_workers,
            active_workers,
            total_hashrate,
            average_load,
        }
    }

    /// Проверяет здоровье воркеров
    pub async fn health_check(&self) -> Result<WorkerHealthStatus, Box<dyn std::error::Error>> {
        let workers = self.workers.read().await;
        let mut status = WorkerHealthStatus {
            overall: "healthy".to_string(),
            workers: Vec::new(),
            timestamp: chrono::Utc::now(),
        };
        
        for (id, worker) in workers.iter() {
            let worker_health = if worker.status == WorkerStatus::Active {
                "healthy"
            } else if worker.status == WorkerStatus::Error {
                "unhealthy"
            } else {
                "warning"
            };
            
            status.workers.push(WorkerHealth {
                id: id.clone(),
                status: worker_health.to_string(),
                last_seen: worker.last_seen,
                uptime: worker.uptime,
            });
        }
        
        // Обновляем общий статус
        if status.workers.iter().any(|w| w.status == "unhealthy") {
            status.overall = "unhealthy".to_string();
        } else if status.workers.iter().any(|w| w.status == "warning") {
            status.overall = "warning".to_string();
        }
        
        Ok(status)
    }

    /// Очищает неактивных воркеров
    pub async fn cleanup_inactive_workers(&self, timeout: std::time::Duration) -> Result<usize, Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        let now = chrono::Utc::now();
        let mut removed_count = 0;
        
        let inactive_workers: Vec<String> = workers.iter()
            .filter(|(_, worker)| {
                let time_since_last_seen = now.signed_duration_since(worker.last_seen);
                time_since_last_seen > chrono::Duration::from_std(timeout).unwrap_or_default()
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for worker_id in inactive_workers {
            if workers.remove(&worker_id).is_some() {
                removed_count += 1;
                info!("Removed inactive worker {}", worker_id);
            }
        }
        
        Ok(removed_count)
    }
}

/// Воркер
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub status: WorkerStatus,
    pub hashrate: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: f64,
    pub uptime: std::time::Duration,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub capabilities: Vec<String>,
}

/// Статус воркера
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    Active,
    Inactive,
    Busy,
    Error,
    Maintenance,
}

/// Статистика воркеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    pub total_workers: usize,
    pub active_workers: usize,
    pub total_hashrate: f64,
    pub average_load: f64,
}

/// Статус здоровья воркеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHealthStatus {
    pub overall: String,
    pub workers: Vec<WorkerHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Здоровье воркера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHealth {
    pub id: String,
    pub status: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub uptime: std::time::Duration,
} 