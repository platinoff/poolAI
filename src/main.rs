//! PoolAI - Система управления пулом майнинга с интеграцией генеративных моделей
//! Version: Beta_bolvanka_v1
//! 
//! Основные возможности:
//! - Управление AI майнинг пулами
//! - Интеграция с генеративными моделями
//! - Оптимизация GPU/ASIC/CPU ресурсов
//! - Telegram бот для управления
//! - Веб-интерфейс для мониторинга
//! - RAID система для отказоустойчивости

use actix_web::{web, App, HttpServer, middleware, Responder};
use std::sync::Arc;
use parking_lot::RwLock;
use log::{info, error, LevelFilter};
use env_logger::Builder;
use tokio::signal;
use std::process;
use actix_web::middleware::Logger;
use actix_web::http::header;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::env;

// Импорты из модулей PoolAI
use crate::core::state::AppState;
use crate::core::config::AppConfig;
use crate::core::error::CursorError;
use crate::pool::pool::PoolManager;
use crate::pool::pool_cok::PoolConfig;
use crate::pool::pool_cok::PoolStats;
use crate::pool::reward_system::{RewardSystem, ActivityType};
use crate::raid::burstraid::BurstRaidManager;
use crate::admin::admin_panel::AdminPanel;
use crate::admin::admin_panel::{
    get_pool_stats,
    get_worker_stats,
    update_pool_config,
    add_worker,
    remove_worker,
    get_reward_stats,
    toggle_maintenance_mode,
};
use crate::monitoring::metrics::SystemMetrics;
use crate::network::api::ApiServer;

const VERSION: &str = "Beta_bolvanka_v1";
const BUILD_DATE: &str = env!("VERGEN_BUILD_TIMESTAMP");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логирования
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    info!("Starting PoolAI v{} (Build: {})", VERSION, BUILD_DATE);
    info!("PoolAI - AI Mining Pool Management System");
    info!("Features: GPU/ASIC/CPU optimization, Model integration, Telegram bot, Web UI");

    // Инициализация основных систем
    let app_state = Arc::new(AppState::new());
    let pool_manager = Arc::new(PoolManager::new(PoolConfig::default()));
    let raid_manager = Arc::new(BurstRaidManager::new());
    let metrics = Arc::new(RwLock::new(SystemMetrics::default()));
    let api_server = Arc::new(ApiServer::new());
    
    // Инициализация административной панели
    let admin_config = crate::admin::admin_panel::AdminConfig {
        admin_token: "admin_token_123".to_string(),
        allowed_ips: vec!["127.0.0.1".to_string(), "::1".to_string()],
        rate_limit: 100,
    };
    
    let admin_panel = Arc::new(AdminPanel::new(
        app_state.clone(),
        pool_manager.clone(),
        metrics.clone(),
        api_server.clone(),
        admin_config,
    ));
    
    info!("All subsystems initialized successfully");

    // Запуск HTTP сервера
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(pool_manager.clone()))
            .app_data(web::Data::new(raid_manager.clone()))
            .app_data(web::Data::new(metrics.clone()))
            .app_data(web::Data::new(api_server.clone()))
            .app_data(web::Data::new(admin_panel.clone()))
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-PoolAI-Version", VERSION)))
            .service(
                web::scope("/api/v1")
                    .route("/status", web::get().to(get_status))
                    .route("/pool/stats", web::get().to(get_pool_stats))
                    .route("/workers/stats", web::get().to(get_worker_stats))
                    .route("/pool/config", web::put().to(update_pool_config))
                    .route("/workers/add", web::post().to(add_worker))
                    .route("/workers/remove", web::delete().to(remove_worker))
                    .route("/rewards/stats", web::get().to(get_reward_stats))
                    .route("/maintenance/toggle", web::post().to(toggle_maintenance_mode))
            )
            .service(
                web::scope("/admin")
                    .route("/system/stats", web::get().to(get_admin_system_stats))
                    .route("/pool/status", web::get().to(get_admin_pool_status))
                    .route("/system/restart", web::post().to(restart_system))
                    .route("/maintenance/enable", web::post().to(enable_maintenance))
                    .route("/maintenance/disable", web::post().to(disable_maintenance))
                    .route("/logs", web::get().to(get_admin_logs))
            )
    })
    .bind("127.0.0.1:8080")?;

    info!("HTTP server started on http://127.0.0.1:8080");
    info!("API available at http://127.0.0.1:8080/api/v1/status");
    info!("Admin panel available at http://127.0.0.1:8080/admin");

    // Запуск сервера
    server.run().await?;

    Ok(())
}

async fn get_status() -> impl Responder {
    serde_json::json!({
        "status": "running",
        "version": VERSION,
        "build_date": BUILD_DATE,
        "features": [
            "GPU/ASIC/CPU optimization",
            "Model integration", 
            "Telegram bot",
            "Web UI",
            "RAID system",
            "Monitoring",
            "Reward system",
            "Admin panel"
        ],
        "timestamp": chrono::Utc::now()
    })
}

// Административные функции
async fn get_admin_system_stats(
    app_state: web::Data<Arc<AppState>>,
    pool_manager: web::Data<Arc<PoolManager>>,
    metrics: web::Data<Arc<RwLock<SystemMetrics>>>,
) -> impl Responder {
    let metrics = metrics.read().await;
    
    serde_json::json!({
        "total_workers": pool_manager.get_worker_count(),
        "active_workers": pool_manager.get_active_worker_count(),
        "total_hashrate": pool_manager.get_total_hashrate(),
        "system_load": metrics.system_load,
        "memory_usage": metrics.memory_usage,
        "cpu_usage": metrics.cpu_usage,
        "uptime": metrics.uptime.as_secs(),
        "maintenance_mode": app_state.is_maintenance_mode().await,
    })
}

async fn get_admin_pool_status(
    pool_manager: web::Data<Arc<PoolManager>>,
) -> impl Responder {
    serde_json::json!({
        "is_running": pool_manager.is_running(),
        "worker_count": pool_manager.get_worker_count(),
        "active_tasks": pool_manager.get_active_task_count(),
        "queue_size": pool_manager.get_queue_size(),
        "last_block": pool_manager.get_last_block_hash(),
    })
}

async fn restart_system(
    pool_manager: web::Data<Arc<PoolManager>>,
    api_server: web::Data<Arc<ApiServer>>,
) -> impl Responder {
    match restart_system_internal(pool_manager.as_ref(), api_server.as_ref()).await {
        Ok(_) => serde_json::json!({
            "status": "system restarted"
        }),
        Err(e) => serde_json::json!({
            "error": e.to_string()
        }),
    }
}

async fn enable_maintenance(
    app_state: web::Data<Arc<AppState>>,
) -> impl Responder {
    app_state.set_maintenance_mode(true).await;
    serde_json::json!({
        "status": "maintenance mode enabled"
    })
}

async fn disable_maintenance(
    app_state: web::Data<Arc<AppState>>,
) -> impl Responder {
    app_state.set_maintenance_mode(false).await;
    serde_json::json!({
        "status": "maintenance mode disabled"
    })
}

async fn get_admin_logs() -> impl Responder {
    let logs = vec![
        serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "level": "INFO",
            "message": "System logs requested"
        })
    ];
    
    serde_json::json!(logs)
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