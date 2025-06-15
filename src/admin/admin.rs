use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::state::AppState;
use crate::workers::{Worker, Task};
use crate::reward_system::{RewardSystem, ActivityType, GenerationMetrics};
use solana_sdk::pubkey::Pubkey;
use log::{info, warn, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolStats {
    total_workers: usize,
    active_workers: usize,
    total_power: f64,
    average_performance: f64,
    total_rewards: f64,
    tasks_in_queue: usize,
    gpu_utilization: f64,
    cpu_utilization: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerStats {
    id: String,
    solana_address: String,
    mining_power: f64,
    gpu_memory: u64,
    cpu_cores: u32,
    available_models: Vec<String>,
    current_task: Option<TaskInfo>,
    performance: f64,
    total_rewards: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInfo {
    task_type: String,
    status: String,
    progress: f64,
    start_time: String,
    estimated_completion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolConfig {
    max_workers: usize,
    min_power_required: f64,
    reward_multiplier: f64,
    task_timeout: u64,
    maintenance_mode: bool,
}

pub struct AdminPanel {
    app_state: Arc<AppState>,
    config: PoolConfig,
}

impl AdminPanel {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            config: PoolConfig {
                max_workers: 1000,
                min_power_required: 1.0,
                reward_multiplier: 1.0,
                task_timeout: 3600,
                maintenance_mode: false,
            },
        }
    }

    pub async fn get_pool_stats(&self) -> PoolStats {
        let workers = self.app_state.workers.read();
        let total_workers = workers.len();
        let active_workers = workers.values()
            .filter(|w| w.mining_power > 0.0)
            .count();
        
        let total_power: f64 = workers.values()
            .map(|w| w.mining_power)
            .sum();
        
        let average_performance = if total_workers > 0 {
            total_power / total_workers as f64
        } else {
            0.0
        };

        PoolStats {
            total_workers,
            active_workers,
            total_power,
            average_performance,
            total_rewards: 0.0, // TODO: Implement
            tasks_in_queue: 0, // TODO: Implement
            gpu_utilization: 0.0, // TODO: Implement
            cpu_utilization: 0.0, // TODO: Implement
        }
    }

    pub async fn get_worker_stats(&self, worker_id: &str) -> Option<WorkerStats> {
        let workers = self.app_state.workers.read();
        workers.get(worker_id).map(|worker| {
            WorkerStats {
                id: worker.id.clone(),
                solana_address: worker.solana_address.to_string(),
                mining_power: worker.mining_power,
                gpu_memory: worker.gpu_memory,
                cpu_cores: worker.cpu_cores,
                available_models: worker.available_models.clone(),
                current_task: None, // TODO: Implement
                performance: 0.0, // TODO: Implement
                total_rewards: 0.0, // TODO: Implement
            }
        })
    }

    pub async fn update_pool_config(&mut self, config: PoolConfig) -> Result<(), String> {
        // Validate config
        if config.max_workers == 0 {
            return Err("Max workers cannot be zero".to_string());
        }
        if config.min_power_required < 0.0 {
            return Err("Minimum power cannot be negative".to_string());
        }
        if config.reward_multiplier <= 0.0 {
            return Err("Reward multiplier must be positive".to_string());
        }
        if config.task_timeout == 0 {
            return Err("Task timeout cannot be zero".to_string());
        }

        self.config = config;
        info!("Pool configuration updated: {:?}", self.config);
        Ok(())
    }

    pub async fn add_worker(&self, worker: Worker) -> Result<(), String> {
        if self.config.maintenance_mode {
            return Err("Cannot add workers in maintenance mode".to_string());
        }

        let workers = self.app_state.workers.read();
        if workers.len() >= self.config.max_workers {
            return Err("Maximum number of workers reached".to_string());
        }

        if worker.mining_power < self.config.min_power_required {
            return Err("Worker power below minimum required".to_string());
        }

        let mut workers = self.app_state.workers.write();
        workers.insert(worker.id.clone(), worker);
        info!("New worker added: {}", worker.id);
        Ok(())
    }

    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), String> {
        let mut workers = self.app_state.workers.write();
        if workers.remove(worker_id).is_some() {
            info!("Worker removed: {}", worker_id);
            Ok(())
        } else {
            Err("Worker not found".to_string())
        }
    }

    pub async fn distribute_task(&self, task: Task) -> Result<(), String> {
        if self.config.maintenance_mode {
            return Err("Cannot distribute tasks in maintenance mode".to_string());
        }

        // Find suitable worker
        let workers = self.app_state.workers.read();
        let worker = workers.values()
            .filter(|w| w.mining_power >= self.config.min_power_required)
            .min_by(|a, b| a.mining_power.partial_cmp(&b.mining_power).unwrap())
            .ok_or_else(|| "No suitable worker found".to_string())?;

        // Send task to worker
        if let Err(e) = worker.task_sender.send(task) {
            return Err(format!("Failed to send task to worker: {}", e));
        }

        info!("Task distributed to worker: {}", worker.id);
        Ok(())
    }

    pub async fn get_reward_stats(&self) -> Result<Vec<(String, f64)>, String> {
        let workers = self.app_state.workers.read();
        let mut stats = Vec::new();

        for worker in workers.values() {
            let metrics = self.app_state.reward_system.get_user_metrics(&worker.id)
                .map_err(|e| e.to_string())?;
            stats.push((worker.id.clone(), metrics.total_rewards));
        }

        stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(stats)
    }

    pub async fn toggle_maintenance_mode(&mut self) {
        self.config.maintenance_mode = !self.config.maintenance_mode;
        info!(
            "Maintenance mode {}",
            if self.config.maintenance_mode { "enabled" } else { "disabled" }
        );
    }
}

// API Endpoints
pub async fn get_pool_stats(admin: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let stats = admin.get_pool_stats().await;
    HttpResponse::Ok().json(stats)
}

pub async fn get_worker_stats(
    admin: web::Data<Arc<AdminPanel>>,
    worker_id: web::Path<String>,
) -> impl Responder {
    match admin.get_worker_stats(&worker_id).await {
        Some(stats) => HttpResponse::Ok().json(stats),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_pool_config(
    admin: web::Data<Arc<AdminPanel>>,
    config: web::Json<PoolConfig>,
) -> impl Responder {
    match admin.update_pool_config(config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn add_worker(
    admin: web::Data<Arc<AdminPanel>>,
    worker: web::Json<Worker>,
) -> impl Responder {
    match admin.add_worker(worker.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn remove_worker(
    admin: web::Data<Arc<AdminPanel>>,
    worker_id: web::Path<String>,
) -> impl Responder {
    match admin.remove_worker(&worker_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn get_reward_stats(admin: web::Data<Arc<AdminPanel>>) -> impl Responder {
    match admin.get_reward_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn toggle_maintenance_mode(admin: web::Data<Arc<AdminPanel>>) -> impl Responder {
    admin.toggle_maintenance_mode().await;
    HttpResponse::Ok().finish()
} 