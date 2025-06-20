//! Worker Monitor - Мониторинг воркеров

use super::worker_manager::{Worker, WorkerStatus};
use crate::monitoring::metrics::WorkerMetrics;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};

/// Монитор воркеров
pub struct WorkerMonitor {
    metrics_history: Arc<RwLock<HashMap<String, Vec<WorkerMetrics>>>>,
    alert_thresholds: AlertThresholds,
}

impl WorkerMonitor {
    /// Создает новый монитор воркеров
    pub fn new(alert_thresholds: AlertThresholds) -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(HashMap::new())),
            alert_thresholds,
        }
    }

    /// Получает метрики всех воркеров
    pub async fn get_metrics(
        &self,
        workers: &Arc<RwLock<HashMap<String, Worker>>>,
    ) -> HashMap<String, WorkerMetrics> {
        let workers = workers.read().await;
        let mut metrics = HashMap::new();
        
        for (id, worker) in workers.iter() {
            let worker_metrics = WorkerMetrics {
                cpu_usage: worker.cpu_usage,
                memory_usage: worker.memory_usage,
                gpu_usage: worker.gpu_usage,
                hashrate: worker.hashrate,
                uptime: worker.uptime,
                status: worker.status.clone(),
            };
            
            metrics.insert(id.clone(), worker_metrics);
            
            // Сохраняем в историю
            self.save_metrics_history(id, &worker_metrics).await;
        }
        
        metrics
    }

    /// Сохраняет метрики в историю
    async fn save_metrics_history(&self, worker_id: &str, metrics: &WorkerMetrics) {
        let mut history = self.metrics_history.write().await;
        
        let worker_history = history.entry(worker_id.to_string()).or_insert_with(Vec::new);
        worker_history.push(metrics.clone());
        
        // Ограничиваем размер истории
        if worker_history.len() > 1000 {
            worker_history.remove(0);
        }
    }

    /// Получает среднюю нагрузку воркеров
    pub async fn get_average_load(&self, workers: &HashMap<String, Worker>) -> f64 {
        if workers.is_empty() {
            return 0.0;
        }
        
        let total_load: f64 = workers.values()
            .map(|w| (w.cpu_usage + w.memory_usage + w.gpu_usage) / 3.0)
            .sum();
        
        total_load / workers.len() as f64
    }

    /// Проверяет воркеров на наличие проблем
    pub async fn check_worker_health(&self, workers: &Arc<RwLock<HashMap<String, Worker>>>) -> Vec<WorkerAlert> {
        let workers = workers.read().await;
        let mut alerts = Vec::new();
        
        for (id, worker) in workers.iter() {
            // Проверяем CPU
            if worker.cpu_usage > self.alert_thresholds.max_cpu_usage {
                alerts.push(WorkerAlert {
                    worker_id: id.clone(),
                    alert_type: AlertType::HighCpuUsage,
                    message: format!("CPU usage is {}% (threshold: {}%)", 
                                   worker.cpu_usage, self.alert_thresholds.max_cpu_usage),
                    severity: AlertSeverity::Warning,
                    timestamp: chrono::Utc::now(),
                });
            }
            
            // Проверяем память
            if worker.memory_usage > self.alert_thresholds.max_memory_usage {
                alerts.push(WorkerAlert {
                    worker_id: id.clone(),
                    alert_type: AlertType::HighMemoryUsage,
                    message: format!("Memory usage is {}% (threshold: {}%)", 
                                   worker.memory_usage, self.alert_thresholds.max_memory_usage),
                    severity: AlertSeverity::Warning,
                    timestamp: chrono::Utc::now(),
                });
            }
            
            // Проверяем GPU
            if worker.gpu_usage > self.alert_thresholds.max_gpu_usage {
                alerts.push(WorkerAlert {
                    worker_id: id.clone(),
                    alert_type: AlertType::HighGpuUsage,
                    message: format!("GPU usage is {}% (threshold: {}%)", 
                                   worker.gpu_usage, self.alert_thresholds.max_gpu_usage),
                    severity: AlertSeverity::Warning,
                    timestamp: chrono::Utc::now(),
                });
            }
            
            // Проверяем статус
            if worker.status == WorkerStatus::Error {
                alerts.push(WorkerAlert {
                    worker_id: id.clone(),
                    alert_type: AlertType::WorkerError,
                    message: "Worker is in error state".to_string(),
                    severity: AlertSeverity::Critical,
                    timestamp: chrono::Utc::now(),
                });
            }
            
            // Проверяем время последнего обновления
            let time_since_last_seen = chrono::Utc::now().signed_duration_since(worker.last_seen);
            if time_since_last_seen > chrono::Duration::from_std(self.alert_thresholds.max_inactivity).unwrap_or_default() {
                alerts.push(WorkerAlert {
                    worker_id: id.clone(),
                    alert_type: AlertType::WorkerInactive,
                    message: format!("Worker inactive for {} seconds", time_since_last_seen.num_seconds()),
                    severity: AlertSeverity::Critical,
                    timestamp: chrono::Utc::now(),
                });
            }
        }
        
        alerts
    }

    /// Получает историю метрик воркера
    pub async fn get_worker_history(&self, worker_id: &str, limit: usize) -> Vec<WorkerMetrics> {
        let history = self.metrics_history.read().await;
        
        if let Some(worker_history) = history.get(worker_id) {
            let start = if worker_history.len() > limit {
                worker_history.len() - limit
            } else {
                0
            };
            
            worker_history[start..].to_vec()
        } else {
            Vec::new()
        }
    }

    /// Получает статистику производительности
    pub async fn get_performance_stats(&self, workers: &Arc<RwLock<HashMap<String, Worker>>>) -> PerformanceStats {
        let workers = workers.read().await;
        let mut stats = PerformanceStats {
            total_workers: workers.len(),
            active_workers: 0,
            total_hashrate: 0.0,
            average_cpu: 0.0,
            average_memory: 0.0,
            average_gpu: 0.0,
            alerts_count: 0,
        };
        
        if workers.is_empty() {
            return stats;
        }
        
        let mut total_cpu = 0.0;
        let mut total_memory = 0.0;
        let mut total_gpu = 0.0;
        
        for worker in workers.values() {
            if worker.status == WorkerStatus::Active {
                stats.active_workers += 1;
            }
            
            stats.total_hashrate += worker.hashrate;
            total_cpu += worker.cpu_usage;
            total_memory += worker.memory_usage;
            total_gpu += worker.gpu_usage;
        }
        
        stats.average_cpu = total_cpu / workers.len() as f64;
        stats.average_memory = total_memory / workers.len() as f64;
        stats.average_gpu = total_gpu / workers.len() as f64;
        
        // Получаем количество алертов
        let alerts = self.check_worker_health(workers).await;
        stats.alerts_count = alerts.len();
        
        stats
    }
}

/// Пороги для алертов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub max_cpu_usage: f64,
    pub max_memory_usage: f64,
    pub max_gpu_usage: f64,
    pub max_inactivity: std::time::Duration,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_cpu_usage: 90.0,
            max_memory_usage: 90.0,
            max_gpu_usage: 95.0,
            max_inactivity: std::time::Duration::from_secs(300), // 5 минут
        }
    }
}

/// Тип алерта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighCpuUsage,
    HighMemoryUsage,
    HighGpuUsage,
    WorkerError,
    WorkerInactive,
    LowHashrate,
}

/// Серьезность алерта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Алерт воркера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerAlert {
    pub worker_id: String,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Статистика производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub total_workers: usize,
    pub active_workers: usize,
    pub total_hashrate: f64,
    pub average_cpu: f64,
    pub average_memory: f64,
    pub average_gpu: f64,
    pub alerts_count: usize,
} 