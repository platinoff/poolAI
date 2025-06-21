//! Admin Panel - Веб-интерфейс для административного управления

use actix_web::{web, HttpResponse, Responder, get, post, delete};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::pool::pool_cok::{PoolNode, PoolMigrationManager, MigrationTask, PoolError};
use crate::core::state::AppState;
use crate::pool::pool::PoolManager;
use crate::monitoring::metrics::SystemMetrics;
use crate::network::api::ApiServer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub admin_token: String,
    pub allowed_ips: Vec<String>,
    pub rate_limit: u32,
}

pub struct AdminPanel {
    state: Arc<AppState>,
    pool_manager: Arc<PoolManager>,
    metrics: Arc<RwLock<SystemMetrics>>,
    api_server: Arc<ApiServer>,
    config: AdminConfig,
    sessions: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl AdminPanel {
    pub fn new(
        state: Arc<AppState>,
        pool_manager: Arc<PoolManager>,
        metrics: Arc<RwLock<SystemMetrics>>,
        api_server: Arc<ApiServer>,
        config: AdminConfig,
    ) -> Self {
        Self {
            state,
            pool_manager,
            metrics,
            api_server,
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, address: &str) -> std::io::Result<()> {
        let state = self.state.clone();
        let pool_manager = self.pool_manager.clone();
        let metrics = self.metrics.clone();
        let api_server = self.api_server.clone();
        let config = self.config.clone();
        let sessions = self.sessions.clone();

        actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .app_data(web::Data::new(state.clone()))
                .app_data(web::Data::new(pool_manager.clone()))
                .app_data(web::Data::new(metrics.clone()))
                .app_data(web::Data::new(api_server.clone()))
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(sessions.clone()))
                .service(get_system_stats)
                .service(get_pool_status)
                .service(restart_system)
                .service(enable_maintenance)
                .service(disable_maintenance)
                .service(get_logs)
                .service(login)
                .service(logout)
        })
        .bind(address)?
        .run()
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    token: String,
}

#[post("/login")]
async fn login(
    req: web::Json<LoginRequest>,
    config: web::Data<AdminConfig>,
    sessions: web::Data<Arc<RwLock<HashMap<String, DateTime<Utc>>>>>,
) -> impl Responder {
    if req.token != config.admin_token {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid token"
        }));
    }

    let session_id = Uuid::new_v4().to_string();
    let mut sessions = sessions.write();
    sessions.insert(session_id.clone(), Utc::now());

    HttpResponse::Ok().json(serde_json::json!({
        "session_id": session_id
    }))
}

#[post("/logout")]
async fn logout(
    session_id: web::Header<String>,
    sessions: web::Data<Arc<RwLock<HashMap<String, DateTime<Utc>>>>>,
) -> impl Responder {
    let mut sessions = sessions.write();
    sessions.remove(&session_id.to_string());
    HttpResponse::Ok().json(serde_json::json!({
        "status": "logged out"
    }))
}

#[get("/system/stats")]
async fn get_system_stats(
    state: web::Data<Arc<AppState>>,
    pool_manager: web::Data<Arc<PoolManager>>,
    metrics: web::Data<Arc<RwLock<SystemMetrics>>>,
) -> impl Responder {
    let metrics = metrics.read().await;
    
    let stats = serde_json::json!({
        "total_workers": pool_manager.get_worker_count(),
        "active_workers": pool_manager.get_active_worker_count(),
        "total_hashrate": pool_manager.get_total_hashrate(),
        "system_load": metrics.system_load,
        "memory_usage": metrics.memory_usage,
        "cpu_usage": metrics.cpu_usage,
        "uptime": metrics.uptime.as_secs(),
        "maintenance_mode": state.is_maintenance_mode().await,
    });
    
    HttpResponse::Ok().json(stats)
}

#[get("/pool/status")]
async fn get_pool_status(
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    let status = serde_json::json!({
        "is_running": pool_manager.is_running(),
        "worker_count": pool_manager.get_worker_count(),
        "active_tasks": pool_manager.get_active_task_count(),
        "queue_size": pool_manager.get_queue_size(),
        "last_block": pool_manager.get_last_block_hash(),
    });
    
    HttpResponse::Ok().json(status)
}

#[post("/system/restart")]
async fn restart_system(
    pool_manager: web::Data<Arc<PoolManager>>,
    api_server: web::Data<Arc<ApiServer>>,
) -> impl Responder {
    match restart_system_internal(pool_manager.as_ref(), api_server.as_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "system restarted"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[post("/maintenance/enable")]
async fn enable_maintenance(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    state.set_maintenance_mode(true).await;
    HttpResponse::Ok().json(serde_json::json!({
        "status": "maintenance mode enabled"
    }))
}

#[post("/maintenance/disable")]
async fn disable_maintenance(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    state.set_maintenance_mode(false).await;
    HttpResponse::Ok().json(serde_json::json!({
        "status": "maintenance mode disabled"
    }))
}

#[get("/logs")]
async fn get_logs() -> impl Responder {
    let logs = vec![
        serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "level": "INFO",
            "message": "System logs requested"
        })
    ];
    
    HttpResponse::Ok().json(logs)
}

async fn restart_system_internal(
    pool_manager: &PoolManager,
    api_server: &ApiServer,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Admin: Restarting system");
    
    // Остановка компонентов
    pool_manager.stop().await?;
    api_server.stop().await?;
    
    // Запуск компонентов
    pool_manager.start().await?;
    api_server.start().await?;
    
    log::info!("Admin: System restarted successfully");
    Ok(())
}

// API функции для main.rs
pub async fn get_pool_stats() -> impl Responder {
    serde_json::json!({
        "total_workers": 0,
        "active_workers": 0,
        "hashrate": "0 H/s",
        "difficulty": 1.0,
        "last_block": "0000000000000000000000000000000000000000000000000000000000000000"
    })
}

pub async fn get_worker_stats() -> impl Responder {
    serde_json::json!({
        "workers": []
    })
}

pub async fn update_pool_config() -> impl Responder {
    serde_json::json!({
        "status": "config updated"
    })
}

pub async fn add_worker() -> impl Responder {
    serde_json::json!({
        "status": "worker added"
    })
}

pub async fn remove_worker() -> impl Responder {
    serde_json::json!({
        "status": "worker removed"
    })
}

pub async fn get_reward_stats() -> impl Responder {
    serde_json::json!({
        "total_rewards": 0.0,
        "pending_rewards": 0.0,
        "paid_rewards": 0.0
    })
}

pub async fn toggle_maintenance_mode() -> impl Responder {
    serde_json::json!({
        "status": "maintenance mode toggled"
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_login() {
        let config = AdminConfig {
            admin_token: "test_token".to_string(),
            allowed_ips: vec![],
            rate_limit: 100,
        };
        
        let app = test::init_service(
            actix_web::App::new()
                .app_data(web::Data::new(config))
                .service(login)
        ).await;

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginRequest {
                token: "test_token".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
} 