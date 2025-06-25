pub mod worker;

use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse, ModelMetrics};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_workers: usize,
    pub max_queue_size: usize,
    pub load_balancing_strategy: LoadBalancingStrategy,
    pub auto_scaling: bool,
    pub scaling_threshold: f32,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Weighted,
    Random,
}

#[derive(Debug, Clone)]
pub struct PoolMetrics {
    pub active_workers: usize,
    pub queue_size: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
}

pub struct Pool {
    config: PoolConfig,
    workers: Arc<RwLock<HashMap<String, worker::Worker>>>,
    metrics: Arc<RwLock<PoolMetrics>>,
}

impl Pool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            workers: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PoolMetrics {
                active_workers: 0,
                queue_size: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                gpu_utilization: 0.0,
                memory_usage_mb: 0.0,
            })),
        }
    }

    pub async fn add_worker(&self, worker_id: String, worker: worker::Worker) -> Result<(), AppError> {
        let mut workers = self.workers.write().await;
        workers.insert(worker_id, worker);
        
        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.active_workers = workers.len();
        
        Ok(())
    }

    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), AppError> {
        let mut workers = self.workers.write().await;
        if workers.remove(worker_id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write().await;
            metrics.active_workers = workers.len();
            Ok(())
        } else {
            Err(AppError::Model(format!("Worker '{}' not found", worker_id)))
        }
    }

    pub async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Select worker according to load balancing strategy
        let worker = self.select_worker().await?;
        
        // Process request
        let response = worker.process_request(request).await?;
        
        // Update metrics
        self.update_metrics(&response).await;
        
        Ok(response)
    }

    async fn select_worker(&self) -> Result<worker::Worker, AppError> {
        let workers = self.workers.read().await;
        
        if workers.is_empty() {
            return Err(AppError::Resource("No workers available".to_string()));
        }
        
        match self.config.load_balancing_strategy {
            LoadBalancingStrategy::RoundRobin => {
                // Simple round-robin implementation
                if let Some((_, worker)) = workers.iter().next() {
                    Ok(worker.clone())
                } else {
                    Err(AppError::Resource("No workers available".to_string()))
                }
            }
            LoadBalancingStrategy::LeastConnections => {
                // Select worker with least connections
                workers.iter()
                    .min_by_key(|(_, worker)| worker.get_active_connections())
                    .map(|(_, worker)| worker.clone())
                    .ok_or_else(|| AppError::Resource("No workers available".to_string()))
            }
            LoadBalancingStrategy::Weighted => {
                // Weighted selection based on metrics
                workers.iter()
                    .max_by_key(|(_, worker)| worker.get_health_score())
                    .map(|(_, worker)| worker.clone())
                    .ok_or_else(|| AppError::Resource("No workers available".to_string()))
            }
            LoadBalancingStrategy::Random => {
                // Random selection
                use rand::seq::SliceRandom;
                let worker_list: Vec<_> = workers.values().collect();
                worker_list.choose(&mut rand::thread_rng())
                    .map(|worker| (*worker).clone())
                    .ok_or_else(|| AppError::Resource("No workers available".to_string()))
            }
        }
    }

    async fn update_metrics(&self, response: &ModelResponse) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        metrics.successful_requests += 1;
        metrics.average_response_time_ms = 
            (metrics.average_response_time_ms * (metrics.total_requests - 1) as f64 + 
             response.metrics.processing_time_ms as f64) / metrics.total_requests as f64;
    }

    pub async fn get_metrics(&self) -> PoolMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn scale_up(&self) -> Result<(), AppError> {
        if self.config.auto_scaling {
            let workers = self.workers.read().await;
            if workers.len() < self.config.max_workers {
                // Add new worker logic
                log::info!("Scaling up pool - adding new worker");
                // TODO: Implement actual worker creation
            }
        }
        Ok(())
    }

    pub async fn scale_down(&self) -> Result<(), AppError> {
        if self.config.auto_scaling {
            let workers = self.workers.read().await;
            if workers.len() > 1 {
                // Remove worker logic
                log::info!("Scaling down pool - removing worker");
                // TODO: Implement actual worker removal
            }
        }
        Ok(())
    }

    pub async fn distribute_resources(&self) -> Result<(), AppError> {
        let workers = self.workers.read().await;
        for worker in workers.values() {
            worker.optimize_resources().await?;
        }
        Ok(())
    }
} 