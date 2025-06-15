use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use chrono::{DateTime, Utc};
use crate::core::state::AppState;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_pools: u32,
    pub active_pools: u32,
    pub total_workers: u32,
    pub active_workers: u32,
    pub total_memory_gb: u32,
    pub total_cpu_cores: u32,
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub average_load: f32,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSummary {
    pub name: String,
    pub description: String,
    pub active_workers: u32,
    pub total_workers: u32,
    pub memory_usage_gb: u32,
    pub cpu_usage_cores: u32,
    pub status: String,
    pub last_scale_time: Option<DateTime<Utc>>,
}

pub struct HomeController {
    pool_manager: Arc<RwLock<PoolManager>>,
}

impl HomeController {
    pub fn new(pool_manager: Arc<RwLock<PoolManager>>) -> Self {
        Self { pool_manager }
    }

    pub async fn get_dashboard(&self) -> impl Responder {
        let pools = self.pool_manager.read().await.list_pools().await;
        
        let stats = DashboardStats {
            total_pools: pools.len() as u32,
            active_pools: pools.iter()
                .filter(|p| p.stats.active_workers > 0)
                .count() as u32,
            total_workers: pools.iter()
                .map(|p| p.stats.total_workers)
                .sum(),
            active_workers: pools.iter()
                .map(|p| p.stats.active_workers)
                .sum(),
            total_memory_gb: pools.iter()
                .map(|p| p.stats.total_memory_gb)
                .sum(),
            total_cpu_cores: pools.iter()
                .map(|p| p.stats.total_cpu_cores)
                .sum(),
            total_tasks: pools.iter()
                .map(|p| p.stats.total_tasks)
                .sum(),
            completed_tasks: pools.iter()
                .map(|p| p.stats.completed_tasks)
                .sum(),
            failed_tasks: pools.iter()
                .map(|p| p.stats.failed_tasks)
                .sum(),
            average_load: pools.iter()
                .map(|p| p.stats.average_load)
                .sum::<f32>() / pools.len() as f32,
            last_update: Utc::now(),
        };

        HttpResponse::Ok().json(stats)
    }

    pub async fn get_pool_summaries(&self) -> impl Responder {
        let pools = self.pool_manager.read().await.list_pools().await;
        
        let summaries: Vec<PoolSummary> = pools.into_iter()
            .map(|p| PoolSummary {
                name: p.config.name,
                description: p.config.description,
                active_workers: p.stats.active_workers,
                total_workers: p.stats.total_workers,
                memory_usage_gb: p.stats.total_memory_gb,
                cpu_usage_cores: p.stats.total_cpu_cores,
                status: if p.stats.active_workers > 0 { "active".to_string() } else { "inactive".to_string() },
                last_scale_time: p.stats.last_scale_time,
            })
            .collect();

        HttpResponse::Ok().json(summaries)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/home")
            .route("/dashboard", web::get().to(get_dashboard))
            .route("/pools", web::get().to(get_pool_summaries))
    );
}

async fn get_dashboard(
    controller: web::Data<HomeController>,
) -> impl Responder {
    controller.get_dashboard().await
}

async fn get_pool_summaries(
    controller: web::Data<HomeController>,
) -> impl Responder {
    controller.get_pool_summaries().await
} 