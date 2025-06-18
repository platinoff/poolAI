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
use crate::admin::admin::AdminPanel;
use crate::admin::admin_panel::{
    get_pool_stats,
    get_worker_stats,
    update_pool_config,
    add_worker,
    remove_worker,
    get_reward_stats,
    toggle_maintenance_mode,
};

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
    
    info!("All subsystems initialized successfully");

    // Запуск HTTP сервера
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(pool_manager.clone()))
            .app_data(web::Data::new(raid_manager.clone()))
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
    })
    .bind("127.0.0.1:8080")?;

    info!("HTTP server started on http://127.0.0.1:8080");
    info!("API available at http://127.0.0.1:8080/api/v1/status");

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
            "Reward system"
        ],
        "timestamp": chrono::Utc::now()
    })
} 