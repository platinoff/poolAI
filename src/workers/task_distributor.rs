//! Task Distributor - Распределение задач между воркерами

use super::worker_manager::{Worker, WorkerStatus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};

/// Распределитель задач
pub struct TaskDistributor {
    distribution_strategy: DistributionStrategy,
}

impl TaskDistributor {
    /// Создает новый распределитель задач
    pub fn new(strategy: DistributionStrategy) -> Self {
        Self {
            distribution_strategy: strategy,
        }
    }

    /// Распределяет задачу между воркерами
    pub async fn distribute_task(
        &self,
        task: Task,
        workers: &Arc<RwLock<HashMap<String, Worker>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let workers = workers.read().await;
        
        // Фильтруем подходящих воркеров
        let suitable_workers: Vec<&Worker> = workers.values()
            .filter(|w| w.status == WorkerStatus::Active)
            .filter(|w| self.worker_satisfies_requirements(w, &task.requirements))
            .collect();
        
        if suitable_workers.is_empty() {
            return Err("No suitable worker found for task".into());
        }
        
        // Выбираем воркера согласно стратегии
        let selected_worker = match self.distribution_strategy {
            DistributionStrategy::RoundRobin => self.round_robin_select(&suitable_workers),
            DistributionStrategy::LeastLoaded => self.least_loaded_select(&suitable_workers),
            DistributionStrategy::HashrateBased => self.hashrate_based_select(&suitable_workers),
            DistributionStrategy::CapabilityBased => self.capability_based_select(&suitable_workers, &task),
        };
        
        info!("Task {} assigned to worker {} using {:?} strategy", 
              task.id, selected_worker.id, self.distribution_strategy);
        
        Ok(selected_worker.id.clone())
    }

    /// Проверяет, удовлетворяет ли воркер требованиям задачи
    fn worker_satisfies_requirements(&self, worker: &Worker, requirements: &TaskRequirements) -> bool {
        // Проверяем ресурсы
        let has_cpu = worker.cpu_usage + requirements.min_cpu <= 100.0;
        let has_memory = worker.memory_usage + requirements.min_memory <= 100.0;
        let has_gpu = worker.gpu_usage + requirements.min_gpu <= 100.0;
        
        // Проверяем возможности
        let has_capabilities = requirements.capabilities.iter()
            .all(|cap| worker.capabilities.contains(cap));
        
        has_cpu && has_memory && has_gpu && has_capabilities
    }

    /// Выбор воркера по принципу Round Robin
    fn round_robin_select<'a>(&self, workers: &[&'a Worker]) -> &'a Worker {
        // Простая реализация - выбираем первого подходящего
        workers[0]
    }

    /// Выбор наименее загруженного воркера
    fn least_loaded_select<'a>(&self, workers: &[&'a Worker]) -> &'a Worker {
        workers.iter()
            .min_by(|a, b| {
                let load_a = (a.cpu_usage + a.memory_usage + a.gpu_usage) / 3.0;
                let load_b = (b.cpu_usage + b.memory_usage + b.gpu_usage) / 3.0;
                load_a.partial_cmp(&load_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }

    /// Выбор воркера на основе хешрейта
    fn hashrate_based_select<'a>(&self, workers: &[&'a Worker]) -> &'a Worker {
        workers.iter()
            .max_by(|a, b| a.hashrate.partial_cmp(&b.hashrate).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    }

    /// Выбор воркера на основе возможностей
    fn capability_based_select<'a>(&self, workers: &[&'a Worker], task: &Task) -> &'a Worker {
        workers.iter()
            .max_by(|a, b| {
                let score_a = self.calculate_capability_score(a, task);
                let score_b = self.calculate_capability_score(b, task);
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }

    /// Вычисляет оценку соответствия воркера задаче
    fn calculate_capability_score(&self, worker: &Worker, task: &Task) -> f64 {
        let mut score = 0.0;
        
        // Базовый балл за каждое совпадение возможностей
        for capability in &task.requirements.capabilities {
            if worker.capabilities.contains(capability) {
                score += 1.0;
            }
        }
        
        // Бонус за низкую загрузку
        let load = (worker.cpu_usage + worker.memory_usage + worker.gpu_usage) / 3.0;
        score += (100.0 - load) / 100.0;
        
        // Бонус за высокий хешрейт
        score += worker.hashrate / 1000.0; // Нормализуем хешрейт
        
        score
    }

    /// Получает статистику распределения
    pub async fn get_distribution_stats(&self, workers: &Arc<RwLock<HashMap<String, Worker>>>) -> DistributionStats {
        let workers = workers.read().await;
        let total_workers = workers.len();
        let active_workers = workers.values().filter(|w| w.status == WorkerStatus::Active).count();
        
        let total_hashrate: f64 = workers.values().map(|w| w.hashrate).sum();
        let average_load: f64 = if !workers.is_empty() {
            workers.values()
                .map(|w| (w.cpu_usage + w.memory_usage + w.gpu_usage) / 3.0)
                .sum::<f64>() / workers.len() as f64
        } else {
            0.0
        };
        
        DistributionStats {
            total_workers,
            active_workers,
            total_hashrate,
            average_load,
            strategy: self.distribution_strategy.clone(),
        }
    }
}

/// Стратегия распределения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionStrategy {
    RoundRobin,
    LeastLoaded,
    HashrateBased,
    CapabilityBased,
}

/// Задача
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub priority: TaskPriority,
    pub requirements: TaskRequirements,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
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
    pub timeout: Option<std::time::Duration>,
}

/// Статистика распределения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionStats {
    pub total_workers: usize,
    pub active_workers: usize,
    pub total_hashrate: f64,
    pub average_load: f64,
    pub strategy: DistributionStrategy,
} 