//! Workers Module - Управление воркерами
//! 
//! Этот модуль предоставляет:
//! - Управление воркерами
//! - Распределение задач
//! - Мониторинг воркеров
//! - Балансировку нагрузки

pub mod worker_manager;
pub mod task_distributor;
pub mod worker_monitor;

use crate::core::state::AppState;
use crate::pool::pool::PoolManager;
use crate::monitoring::metrics::WorkerMetrics;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Менеджер воркеров
pub struct WorkerManager {
    workers: Arc<RwLock<HashMap<String, Worker>>>,
    task_distributor: Arc<TaskDistributor>,
    monitor: Arc<WorkerMonitor>,
}

impl WorkerManager {
    /// Создает новый менеджер воркеров
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
            task_distributor: Arc::new(TaskDistributor::new()),
            monitor: Arc::new(WorkerMonitor::new()),
        }
    }

    /// Добавляет нового воркера
    pub async fn add_worker(&self, worker: Worker) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        workers.insert(worker.id.clone(), worker);
        log::info!("Worker {} added", worker.id);
        Ok(())
    }

    /// Удаляет воркера
    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        if workers.remove(worker_id).is_some() {
            log::info!("Worker {} removed", worker_id);
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

    /// Распределяет задачу между воркерами
    pub async fn distribute_task(&self, task: Task) -> Result<String, Box<dyn std::error::Error>> {
        self.task_distributor.distribute_task(task, &self.workers).await
    }

    /// Получает метрики воркеров
    pub async fn get_worker_metrics(&self) -> HashMap<String, WorkerMetrics> {
        self.monitor.get_metrics(&self.workers).await
    }

    /// Получает статистику воркеров
    pub async fn get_worker_stats(&self) -> WorkerStats {
        let workers = self.workers.read().await;
        let total_workers = workers.len();
        let active_workers = workers.values().filter(|w| w.status == WorkerStatus::Active).count();
        let total_hashrate: f64 = workers.values().map(|w| w.hashrate).sum();
        
        WorkerStats {
            total_workers,
            active_workers,
            total_hashrate,
            average_load: self.monitor.get_average_load(&workers).await,
        }
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

/// Задача
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub priority: TaskPriority,
    pub requirements: TaskRequirements,
    pub data: serde_json::Value,
}

/// Приоритет задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Требования к задаче
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub min_cpu: f64,
    pub min_memory: f64,
    pub min_gpu: f64,
    pub capabilities: Vec<String>,
}

/// Статистика воркеров
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    pub total_workers: usize,
    pub active_workers: usize,
    pub total_hashrate: f64,
    pub average_load: f64,
}

/// Распределитель задач
pub struct TaskDistributor;

impl TaskDistributor {
    pub fn new() -> Self {
        Self
    }

    pub async fn distribute_task(
        &self,
        task: Task,
        workers: &Arc<RwLock<HashMap<String, Worker>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let workers = workers.read().await;
        
        // Находим подходящего воркера
        let suitable_worker = workers.values()
            .filter(|w| w.status == WorkerStatus::Active)
            .filter(|w| self.worker_satisfies_requirements(w, &task.requirements))
            .min_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        
        match suitable_worker {
            Some(worker) => {
                log::info!("Task {} assigned to worker {}", task.id, worker.id);
                Ok(worker.id.clone())
            }
            None => Err("No suitable worker found".into()),
        }
    }

    fn worker_satisfies_requirements(&self, worker: &Worker, requirements: &TaskRequirements) -> bool {
        worker.cpu_usage + requirements.min_cpu <= 100.0 &&
        worker.memory_usage + requirements.min_memory <= 100.0 &&
        worker.gpu_usage + requirements.min_gpu <= 100.0 &&
        requirements.capabilities.iter().all(|cap| worker.capabilities.contains(cap))
    }
}

/// Монитор воркеров
pub struct WorkerMonitor;

impl WorkerMonitor {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_metrics(
        &self,
        workers: &Arc<RwLock<HashMap<String, Worker>>>,
    ) -> HashMap<String, WorkerMetrics> {
        let workers = workers.read().await;
        let mut metrics = HashMap::new();
        
        for (id, worker) in workers.iter() {
            metrics.insert(id.clone(), WorkerMetrics {
                cpu_usage: worker.cpu_usage,
                memory_usage: worker.memory_usage,
                gpu_usage: worker.gpu_usage,
                hashrate: worker.hashrate,
                uptime: worker.uptime,
                status: worker.status.clone(),
            });
        }
        
        metrics
    }

    pub async fn get_average_load(&self, workers: &HashMap<String, Worker>) -> f64 {
        if workers.is_empty() {
            return 0.0;
        }
        
        let total_load: f64 = workers.values()
            .map(|w| (w.cpu_usage + w.memory_usage + w.gpu_usage) / 3.0)
            .sum();
        
        total_load / workers.len() as f64
    }
}

/// Инициализация workers модуля
pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing workers module");
    Ok(())
}

/// Остановка workers модуля
pub async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Shutting down workers module");
    Ok(())
}

/// Проверка здоровья workers модуля
pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Workers module health check passed");
    Ok(())
}

pub use worker_manager::*;
pub use task_distributor::*;
pub use worker_monitor::*; 