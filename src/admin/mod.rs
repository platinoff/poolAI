use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::error::Error;

use crate::{
    workers::WorkerManager,
    reward_system::RewardSystem,
    vm::VMManager,
    tuning::TuningSystem,
};

pub mod admin;
pub mod admin_ui;
pub mod admin_panel;

pub use admin::*;
pub use admin_ui::*;
pub use admin_panel::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub maintenance_mode: bool,
    pub max_workers: u32,
    pub min_memory_gb: u32,
    pub max_memory_gb: u32,
    pub allowed_gpu_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_workers: u32,
    pub active_workers: u32,
    pub total_rewards: f64,
    pub average_performance: f32,
    pub system_load: f32,
}

pub struct AdminPanel {
    config: Arc<Mutex<AdminConfig>>,
    worker_manager: Arc<WorkerManager>,
    reward_system: Arc<RewardSystem>,
    vm_manager: Arc<Mutex<VMManager>>,
    tuning_system: Arc<TuningSystem>,
}

impl AdminPanel {
    pub fn new(
        config: AdminConfig,
        worker_manager: Arc<WorkerManager>,
        reward_system: Arc<RewardSystem>,
        vm_manager: Arc<Mutex<VMManager>>,
        tuning_system: Arc<TuningSystem>,
    ) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            worker_manager,
            reward_system,
            vm_manager,
            tuning_system,
        }
    }

    pub async fn get_pool_stats(&self) -> PoolStats {
        let workers = self.worker_manager.get_all_workers().await;
        let active_workers = self.worker_manager.get_active_workers().await;
        
        PoolStats {
            total_workers: workers.len() as u32,
            active_workers: active_workers.len() as u32,
            total_rewards: self.reward_system.get_total_rewards(),
            average_performance: self.calculate_average_performance(&workers),
            system_load: self.calculate_system_load().await,
        }
    }

    pub async fn toggle_maintenance_mode(&self) -> Result<(), String> {
        let mut config = self.config.lock().await;
        config.maintenance_mode = !config.maintenance_mode;
        
        if config.maintenance_mode {
            // Gracefully stop all workers
            let workers = self.worker_manager.get_active_workers().await;
            for worker in workers {
                if let Err(e) = self.worker_manager.remove_worker(&worker.config.name).await {
                    error!("Failed to stop worker {}: {}", worker.config.name, e);
                }
            }
        }
        
        Ok(())
    }

    pub async fn update_config(&self, new_config: AdminConfig) -> Result<(), String> {
        let mut config = self.config.lock().await;
        *config = new_config;
        Ok(())
    }

    pub async fn get_worker_metrics(&self, name: &str) -> Option<crate::workers::WorkerMetrics> {
        self.worker_manager.get_worker_metrics(name).await
    }

    pub async fn get_all_workers(&self) -> Vec<crate::workers::WorkerMetrics> {
        self.worker_manager.get_all_workers().await
    }

    pub async fn get_reward_stats(&self) -> crate::reward_system::RewardMetrics {
        self.reward_system.get_metrics()
    }

    pub async fn get_tuning_metrics(&self) -> crate::tuning::TuningMetrics {
        self.tuning_system.get_metrics()
    }

    pub async fn get_vm_status(&self) -> crate::vm::VMStatus {
        let vm_manager = self.vm_manager.lock().await;
        vm_manager.get_status().clone()
    }

    fn calculate_average_performance(&self, workers: &[crate::workers::WorkerMetrics]) -> f32 {
        if workers.is_empty() {
            return 0.0;
        }

        let total_performance: f32 = workers.iter()
            .map(|w| w.performance.success_rate)
            .sum();

        total_performance / workers.len() as f32
    }

    async fn calculate_system_load(&self) -> f32 {
        let vm_manager = self.vm_manager.lock().await;
        let status = vm_manager.get_status();
        
        // Combine CPU and memory usage
        (status.cpu_usage + (status.memory_usage as f32 / 100.0)) / 2.0
    }
}

// API Handlers
pub async fn get_pool_stats(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let stats = data.get_pool_stats().await;
    HttpResponse::Ok().json(stats)
}

pub async fn toggle_maintenance(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    match data.toggle_maintenance_mode().await {
        Ok(_) => HttpResponse::Ok().json("Maintenance mode toggled"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn update_config(
    data: web::Data<Arc<AdminPanel>>,
    config: web::Json<AdminConfig>,
) -> impl Responder {
    match data.update_config(config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Config updated"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn get_worker_metrics(
    data: web::Data<Arc<AdminPanel>>,
    name: web::Path<String>,
) -> impl Responder {
    match data.get_worker_metrics(&name).await {
        Some(metrics) => HttpResponse::Ok().json(metrics),
        None => HttpResponse::NotFound().json("Worker not found"),
    }
}

pub async fn get_all_workers(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let workers = data.get_all_workers().await;
    HttpResponse::Ok().json(workers)
}

pub async fn get_reward_stats(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let stats = data.get_reward_stats().await;
    HttpResponse::Ok().json(stats)
}

pub async fn get_tuning_metrics(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let metrics = data.get_tuning_metrics().await;
    HttpResponse::Ok().json(metrics)
}

pub async fn get_vm_status(data: web::Data<Arc<AdminPanel>>) -> impl Responder {
    let status = data.get_vm_status().await;
    HttpResponse::Ok().json(status)
}

/// Инициализация admin модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing admin module");
    Ok(())
}

/// Остановка admin модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down admin module");
    Ok(())
}

/// Проверка здоровья admin модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Admin module health check passed");
    Ok(())
} 