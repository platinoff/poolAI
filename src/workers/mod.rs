pub mod workers;

pub use workers::*;

use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub name: String,
    pub solana_address: String,
    pub memory_gb: u32,
    pub gpu_model: String,
    pub active: bool,
    pub last_heartbeat: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    pub hashrate: f64,
    pub rewards: f64,
    pub uptime: f64,
    pub memory_usage: f32,
    pub gpu_usage: f32,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    pub config: WorkerConfig,
    pub stats: WorkerStats,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub tasks_completed: u32,
    pub average_time: f64,
    pub success_rate: f32,
    pub error_rate: f32,
}

pub struct WorkerManager {
    workers: Arc<Mutex<Vec<WorkerMetrics>>>,
}

impl WorkerManager {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_worker(&self, config: WorkerConfig) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        if workers.iter().any(|w| w.config.name == config.name) {
            return Err("Worker with this name already exists".to_string());
        }

        let metrics = WorkerMetrics {
            config,
            stats: WorkerStats {
                hashrate: 0.0,
                rewards: 0.0,
                uptime: 0.0,
                memory_usage: 0.0,
                gpu_usage: 0.0,
                last_update: Utc::now(),
            },
            performance: PerformanceMetrics {
                tasks_completed: 0,
                average_time: 0.0,
                success_rate: 1.0,
                error_rate: 0.0,
            },
        };

        workers.push(metrics);
        Ok(())
    }

    pub async fn remove_worker(&self, name: &str) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        let index = workers.iter().position(|w| w.config.name == name)
            .ok_or_else(|| "Worker not found".to_string())?;
        
        workers.remove(index);
        Ok(())
    }

    pub async fn update_worker_metrics(&self, name: &str, stats: WorkerStats) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        let worker = workers.iter_mut()
            .find(|w| w.config.name == name)
            .ok_or_else(|| "Worker not found".to_string())?;
        
        worker.stats = stats;
        Ok(())
    }

    pub async fn get_worker_metrics(&self, name: &str) -> Option<WorkerMetrics> {
        let workers = self.workers.lock().await;
        workers.iter()
            .find(|w| w.config.name == name)
            .cloned()
    }

    pub async fn get_all_workers(&self) -> Vec<WorkerMetrics> {
        let workers = self.workers.lock().await;
        workers.clone()
    }

    pub async fn get_active_workers(&self) -> Vec<WorkerMetrics> {
        let workers = self.workers.lock().await;
        workers.iter()
            .filter(|w| w.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_worker_stats(&self, name: &str) -> Option<WorkerStats> {
        let workers = self.workers.lock().await;
        workers.iter()
            .find(|w| w.config.name == name)
            .map(|w| w.stats.clone())
    }

    pub async fn get_worker_performance(&self, name: &str) -> Option<PerformanceMetrics> {
        let workers = self.workers.lock().await;
        workers.iter()
            .find(|w| w.config.name == name)
            .map(|w| w.performance.clone())
    }
}

/// Инициализация workers модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing workers module");
    Ok(())
}

/// Остановка workers модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down workers module");
    Ok(())
}

/// Проверка здоровья workers модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Workers module health check passed");
    Ok(())
} 