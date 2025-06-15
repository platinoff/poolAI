use actix_web::{web, App, HttpServer, middleware, Responder};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::CursorCore;
use crate::raid::BurstRaidManager;
use crate::pool::{PoolManager, PoolConfig, PoolStats};
use log::{info, error, LevelFilter};
use env_logger::Builder;
use tokio::signal;
use std::process;
use actix_web::middleware::Logger;
use actix_web::http::header;
use crate::pool::reward_system::{RewardSystem, ActivityType};
use crate::core::{
    error::CursorError,
    lib_manager::{LibraryManager, LibraryStatus},
};
use crate::admin::{
    AdminPanel,
    get_pool_stats,
    get_worker_stats,
    update_pool_config,
    add_worker,
    remove_worker,
    get_reward_stats,
    toggle_maintenance_mode,
};
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

// Импорты из новых модулей
use crate::core::state::AppState;
use crate::core::config::AppConfig;
use crate::network::tls::TlsManager;
use crate::platform::model::ModelSystem;
use crate::network::network::NetworkSystem;
use crate::runtime::storage::StorageSystem;
use crate::runtime::cache::CacheSystem;
use crate::runtime::queue::QueueSystem;
use crate::runtime::scheduler::SchedulerSystem;
use crate::monitoring::monitor::MonitorSystem;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::metrics::MetricsSystem;
use crate::monitoring::alert::AlertSystem;
use crate::core::error::ErrorSystem;
use crate::core::config::ConfigSystem;
use crate::core::utils::UtilsSystem;

mod state;
mod workers;
mod burstraid;
mod tgbot;
mod model;
mod smallworld;
mod tokenizer;
mod pool_web;
mod bridges;
mod lmrouter;
mod loadbalancer;
mod pool_cok;
mod vobe_dancing;
mod vibe;
mod ssh_server;
mod config;
mod tls;

use state::AppState;
use vobe_dancing::VobeDancer;
use vibe::{VibeManager, Mood};
use ssh_server::SshServer;
use config::AppConfig;
use tls::TlsManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub start_time: DateTime<Utc>,
    pub uptime: Duration,
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_warnings: u64,
    pub total_info: u64,
    pub last_error: Option<String>,
    pub last_warning: Option<String>,
    pub last_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub config: SystemConfig,
    pub stats: SystemStats,
}

pub struct System {
    config: Arc<Mutex<SystemMetrics>>,
    model_system: Arc<ModelSystem>,
    network_system: Arc<NetworkSystem>,
    storage_system: Arc<StorageSystem>,
    cache_system: Arc<CacheSystem>,
    queue_system: Arc<QueueSystem>,
    scheduler_system: Arc<SchedulerSystem>,
    monitor_system: Arc<MonitorSystem>,
    logger_system: Arc<LoggerSystem>,
    metrics_system: Arc<MetricsSystem>,
    alert_system: Arc<AlertSystem>,
    error_system: Arc<ErrorSystem>,
    config_system: Arc<ConfigSystem>,
    utils_system: Arc<UtilsSystem>,
}

impl System {
    pub fn new(config: SystemConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(SystemMetrics {
                config: config.clone(),
                stats: SystemStats {
                    start_time: Utc::now(),
                    uptime: Duration::from_secs(0),
                    total_requests: 0,
                    total_errors: 0,
                    total_warnings: 0,
                    total_info: 0,
                    last_error: None,
                    last_warning: None,
                    last_info: None,
                },
            })),
            model_system: Arc::new(ModelSystem::new()),
            network_system: Arc::new(NetworkSystem::new()),
            storage_system: Arc::new(StorageSystem::new()),
            cache_system: Arc::new(CacheSystem::new()),
            queue_system: Arc::new(QueueSystem::new()),
            scheduler_system: Arc::new(SchedulerSystem::new()),
            monitor_system: Arc::new(MonitorSystem::new()),
            logger_system: Arc::new(LoggerSystem::new()),
            metrics_system: Arc::new(MetricsSystem::new()),
            alert_system: Arc::new(AlertSystem::new()),
            error_system: Arc::new(ErrorSystem::new()),
            config_system: Arc::new(ConfigSystem::new("config.json")),
            utils_system: Arc::new(UtilsSystem::new()),
        }
    }

    pub async fn initialize(&self) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if !config.config.active {
            return Err("System is not active".to_string());
        }

        if config.config.debug {
            info!("Initializing system in debug mode");
        }

        // Добавить таймаут для инициализации
        let init_result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                // Инициализация подсистем с обработкой ошибок
                let mut errors = Vec::new();
                
                // Инициализация в правильном порядке
                let init_order = vec![
                    ("config", self.config_system.initialize().await),
                    ("logger", self.logger_system.initialize().await),
                    ("error", self.error_system.initialize().await),
                    ("metrics", self.metrics_system.initialize().await),
                    ("monitor", self.monitor_system.initialize().await),
                    ("network", self.network_system.initialize().await),
                    ("storage", self.storage_system.initialize().await),
                    ("cache", self.cache_system.initialize().await),
                    ("queue", self.queue_system.initialize().await),
                    ("scheduler", self.scheduler_system.initialize().await),
                    ("model", self.model_system.initialize().await),
                    ("alert", self.alert_system.initialize().await),
                    ("utils", self.utils_system.initialize().await),
                ];

                for (name, result) in init_order {
                    if let Err(e) = result {
                        errors.push(format!("{} system: {}", name, e));
                    }
                }
                
                if !errors.is_empty() {
                    Err(errors.join(", "))
                } else {
                    Ok(())
                }
            }
        ).await;

        match init_result {
            Ok(Ok(_)) => {
                info!("System initialized successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("Failed to initialize system: {}", e);
                Err(e)
            }
            Err(_) => {
                error!("System initialization timed out");
                Err("Initialization timeout".to_string())
            }
        }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if !config.config.active {
            return Err("System is not active".to_string());
        }

        if config.config.debug {
            info!("Shutting down system in debug mode");
        }

        // Добавить таймаут для graceful shutdown
        let shutdown_result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                // Graceful shutdown подсистем в обратном порядке
                let mut errors = Vec::new();
                
                // Сначала останавливаем обработку новых запросов
                if let Err(e) = self.queue_system.stop_accepting().await {
                    errors.push(format!("Queue system: {}", e));
                }
                
                // Затем останавливаем подсистемы в правильном порядке
                let shutdown_order = vec![
                    ("utils", self.utils_system.shutdown().await),
                    ("alert", self.alert_system.shutdown().await),
                    ("model", self.model_system.shutdown().await),
                    ("scheduler", self.scheduler_system.shutdown().await),
                    ("queue", self.queue_system.shutdown().await),
                    ("cache", self.cache_system.shutdown().await),
                    ("storage", self.storage_system.shutdown().await),
                    ("network", self.network_system.shutdown().await),
                    ("monitor", self.monitor_system.shutdown().await),
                    ("metrics", self.metrics_system.shutdown().await),
                    ("error", self.error_system.shutdown().await),
                    ("logger", self.logger_system.shutdown().await),
                    ("config", self.config_system.shutdown().await),
                ];

                for (name, result) in shutdown_order {
                    if let Err(e) = result {
                        errors.push(format!("{} system: {}", name, e));
                    }
                }
                
                if !errors.is_empty() {
                    Err(errors.join(", "))
                } else {
                    Ok(())
                }
            }
        ).await;

        match shutdown_result {
            Ok(Ok(_)) => {
                config.config.active = false;
                info!("System shut down successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("Failed to shutdown system: {}", e);
                Err(e)
            }
            Err(_) => {
                error!("System shutdown timed out");
                Err("Shutdown timeout".to_string())
            }
        }
    }

    pub async fn get_config(&self) -> Result<SystemConfig, String> {
        let config = self.config.lock().await;
        Ok(config.config.clone())
    }

    pub async fn get_stats(&self) -> Result<SystemStats, String> {
        let config = self.config.lock().await;
        Ok(config.stats.clone())
    }

    pub async fn update_config(&self, new_config: SystemConfig) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if !config.config.active {
            return Err("System is not active".to_string());
        }

        config.config = new_config;
        info!("System configuration updated");
        Ok(())
    }

    pub async fn set_active(&self, active: bool) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if config.config.active == active {
            return Ok(());
        }

        config.config.active = active;
        info!(
            "System {}",
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_stats(&self) -> Result<(), String> {
        let mut config = self.config.lock().await;
        
        if !config.config.active {
            return Err("System is not active".to_string());
        }

        let now = Utc::now();
        config.stats.uptime = now.signed_duration_since(config.stats.start_time).to_std().unwrap();

        info!("System statistics updated");
        Ok(())
    }
}

fn init_logging() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl+c");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    info!("Starting Cursor Core...");

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    // Initialize TLS manager
    let tls_manager = match TlsManager::new(
        &config.server.cert_path,
        &config.server.key_path,
        config.server.cert_chain_path.as_deref(),
        config.server.enable_http2,
        config.server.enable_ocsp_stapling,
    ) {
        Ok(manager) => manager,
        Err(e) => {
            error!("Failed to initialize TLS manager: {}", e);
            process::exit(1);
        }
    };

    // Initialize vibe manager with component statuses
    let vibe_manager = Arc::new(RwLock::new(VibeManager::new()));
    {
        let mut vibe = vibe_manager.write();
        vibe.update_status("System starting up...");
        vibe.set_mood(Mood::Productive);
        vibe.update_component_status("RAID", "Initializing", Mood::Focused);
        vibe.update_component_status("LoadBalancer", "Starting", Mood::Creative);
        vibe.update_component_status("Bridge", "Connecting", Mood::Flowing);
    }

    // Initialize SSH server
    let ssh_server = SshServer::new(vibe_manager.clone(), 2222);
    tokio::spawn(async move {
        if let Err(e) = ssh_server.start().await {
            error!("SSH server error: {}", e);
        }
    });

    // Initialize Vobe dancer
    let vobe_dancer = Arc::new(RwLock::new(VobeDancer::new()));

    // Initialize RAID manager
    let raid_manager = match BurstRaidManager::new(config.raid) {
        Ok(manager) => {
            vibe_manager.write().update_component_status("RAID", "Ready", Mood::Dancing);
            manager
        },
        Err(e) => {
            error!("Failed to initialize RAID manager: {}", e);
            vibe_manager.write().update_component_status("RAID", "Error", Mood::Focused);
            process::exit(1);
        }
    };

    // Start RAID health monitoring
    let raid_manager_clone = Arc::new(raid_manager);
    let raid_monitor = raid_manager_clone.clone();
    tokio::spawn(async move {
        raid_monitor.monitor_health().await;
    });

    let core = match CursorCore::new(&config.solana_rpc_url) {
        Ok(core) => core,
        Err(e) => {
            error!("Failed to initialize CursorCore: {}", e);
            process::exit(1);
        }
    };

    // Initialize bridge
    match core.initialize_bridge(
        &config.bridge.source_chain,
        &config.bridge.target_chain,
        config.bridge.min_amount,
        config.bridge.fee_percentage,
        config.bridge.max_amount,
    ).await {
        Ok(bridge_id) => info!("Bridge initialized with ID: {}", bridge_id),
        Err(e) => {
            error!("Failed to initialize bridge: {}", e);
            process::exit(1);
        }
    }

    // Create application state
    let app_state = web::Data::new(AppState {
        core: Arc::new(core),
        raid_manager: raid_manager_clone,
        vobe_dancer: vobe_dancer.clone(),
        vibe_manager: vibe_manager.clone(),
        reward_system: Arc::new(RewardSystem::new(1.0)),
        lib_manager: Arc::new(LibraryManager::new(
            std::env::current_dir()?.join("libs")
        )),
        pool_manager: Arc::new(PoolManager::new()),
    });

    let admin_panel = Arc::new(AdminPanel::new(app_state.clone()));

    // Configure CORS
    let cors = middleware::Cors::default()
        .allowed_origin("https://localhost:8443")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(3600);

    // Start HTTP and HTTPS servers
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors.clone())
            .app_data(app_state.clone())
            .app_data(web::Data::new(admin_panel.clone()))
            .service(web::resource("/health").to(|| async { "OK" }))
            .service(web::resource("/dance").to(|state: web::Data<AppState>| async move {
                if let Ok(mut dancer) = state.vobe_dancer.try_write() {
                    if let Err(e) = dancer.start_dance() {
                        error!("Failed to start dance: {}", e);
                        return "Error starting dance";
                    }
                }
                "Dance started! Press 'q' to stop."
            }))
            .service(web::resource("/vibe").to(|state: web::Data<AppState>| async move {
                if let Ok(mut vibe) = state.vibe_manager.try_write() {
                    vibe.update_status("Vibe session active");
                    vibe.set_mood(Mood::Happy);
                    if let Err(e) = vibe.start_vibe_session() {
                        error!("Failed to start vibe session: {}", e);
                        return "Error starting vibe session";
                    }
                }
                "Vibe session started! Press 'q' to stop."
            }))
            .service(web::resource("/vibe/status").to(|state: web::Data<AppState>| async move {
                if let Ok(vibe) = state.vibe_manager.try_read() {
                    format!("Current vibe: {:?}\nStatus: {}", vibe.mood, vibe.status)
                } else {
                    "Error reading vibe status".to_string()
                }
            }))
            .service(
                web::scope("/admin")
                    .route("/stats", web::get().to(get_pool_stats))
                    .route("/worker/{id}", web::get().to(get_worker_stats))
                    .route("/config", web::put().to(update_pool_config))
                    .route("/worker", web::post().to(add_worker))
                    .route("/worker/{id}", web::delete().to(remove_worker))
                    .route("/rewards", web::get().to(get_reward_stats))
                    .route("/maintenance", web::post().to(toggle_maintenance_mode))
            )
            .service(
                web::scope("/api/pools")
                    .route("/dashboard", web::get().to(get_dashboard))
                    .route("/summaries", web::get().to(get_pool_summaries))
                    .route("", web::post().to(create_pool))
                    .route("/{name}", web::put().to(update_pool))
                    .route("/{name}", web::delete().to(delete_pool))
                    .route("/{name}/scale", web::post().to(scale_pool))
            )
            .route("/api/libs/libtorch/check", web::get().to(check_libtorch))
            .route("/api/libs/libtorch/download", web::post().to(download_libtorch))
            .route("/api/libs/libtorch/verify", web::get().to(verify_libtorch))
            .route("/api/libs/environment/setup", web::post().to(setup_environment))
            .route("/api/libs/{name}", web::get().to(get_library_info))
            .route("/api/libs/{name}/update", web::post().to(update_library))
    })
    .bind(format!("0.0.0.0:{}", config.server.http_port))?;

    let https_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors.clone())
            .app_data(app_state.clone())
            .app_data(web::Data::new(admin_panel.clone()))
            .service(web::resource("/health").to(|| async { "OK" }))
    })
    .bind_rustls(format!("0.0.0.0:{}", config.server.https_port), tls_manager.get_config())?;

    info!("Starting HTTP server on port {}", config.server.http_port);
    info!("Starting HTTPS server on port {}", config.server.https_port);

    // Run both servers
    let http_future = http_server.run();
    let https_future = https_server.run();

    // Wait for shutdown
    tokio::select! {
        _ = http_future => {},
        _ = https_future => {},
        _ = shutdown_signal() => {
            info!("Shutdown signal received");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_flow() {
        init_logging();
        let core = CursorCore::new("https://api.mainnet-beta.solana.com").unwrap();

        // Test bridge initialization
        assert!(core.initialize_bridge("ethereum", "solana", 0.1, 0.01, 1000.0).await.is_ok());

        // Test model registration
        let model_config = cursor_codes::lmrouter::ModelConfig {
            name: "test-model".to_string(),
            version: "1.0".to_string(),
            endpoint: "http://test.com".to_string(),
            max_tokens: 1000,
            max_requests_per_minute: 60,
            priority: 1,
        };
        assert!(core.register_language_model("test-model".to_string(), model_config).await.is_ok());

        // Test wallet creation
        assert!(core.create_solana_wallet("test_wallet".to_string()).await.is_ok());
    }
}

async fn process_mining_result(
    app_state: &Arc<AppState>,
    worker_id: &str,
    performance: f64,
) -> Result<(), String> {
    // Добавить валидацию входных данных
    if performance <= 0.0 {
        return Err("Invalid performance value".to_string());
    }

    // Добавить таймаут для обработки
    let process_result = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        async {
            // Обработка результата майнинга
            let mut state = app_state.state.lock().await;
            
            // Проверка существования воркера
            if !state.workers.contains_key(worker_id) {
                return Err("Worker not found".to_string());
            }

            // Обновление статистики
            if let Some(worker) = state.workers.get_mut(worker_id) {
                worker.performance = performance;
                worker.last_update = Utc::now();
            }

            Ok(())
        }
    ).await;

    match process_result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Processing timeout".to_string())
    }
}

async fn get_worker_rewards(
    app_state: &Arc<AppState>,
    worker_id: &str,
) -> Result<String, String> {
    // Добавить таймаут для получения наград
    let rewards_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            let state = app_state.state.lock().await;
            
            // Проверка существования воркера
            if !state.workers.contains_key(worker_id) {
                return Err("Worker not found".to_string());
            }

            // Получение наград
            if let Some(worker) = state.workers.get(worker_id) {
                Ok(format!("{:.2}", worker.rewards))
            } else {
                Err("Failed to get worker rewards".to_string())
            }
        }
    ).await;

    match rewards_result {
        Ok(Ok(rewards)) => Ok(rewards),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Rewards retrieval timeout".to_string())
    }
}

async fn check_libtorch(data: web::Data<AppState>) -> impl Responder {
    match data.lib_manager.check_libtorch().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn download_libtorch(data: web::Data<AppState>) -> impl Responder {
    match data.lib_manager.download_libtorch().await {
        Ok(_) => HttpResponse::Ok().json("LibTorch downloaded successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn verify_libtorch(data: web::Data<AppState>) -> impl Responder {
    match data.lib_manager.verify_libtorch().await {
        Ok(valid) => HttpResponse::Ok().json(valid),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn setup_environment(data: web::Data<AppState>) -> impl Responder {
    match data.lib_manager.setup_environment().await {
        Ok(_) => HttpResponse::Ok().json("Environment setup completed"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn get_library_info(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    match data.lib_manager.get_library_info(&name) {
        Some(info) => HttpResponse::Ok().json(info),
        None => HttpResponse::NotFound().json("Library not found"),
    }
}

async fn update_library(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    match data.lib_manager.update_library(&name).await {
        Ok(_) => HttpResponse::Ok().json("Library updated successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

async fn get_dashboard(data: web::Data<AppState>) -> impl Responder {
    let pool_manager = data.pool_manager.read();
    let stats = pool_manager.get_dashboard_stats();
    web::Json(stats)
}

async fn get_pool_summaries(data: web::Data<AppState>) -> impl Responder {
    let pool_manager = data.pool_manager.read();
    let summaries = pool_manager.get_pool_summaries();
    web::Json(summaries)
}

async fn create_pool(
    data: web::Data<AppState>,
    config: web::Json<PoolConfig>,
) -> impl Responder {
    let mut pool_manager = data.pool_manager.write();
    match pool_manager.create_pool(config.into_inner()) {
        Ok(_) => web::Json(json!({ "status": "success" })),
        Err(e) => web::Json(json!({ "status": "error", "message": e.to_string() }))
    }
}

async fn update_pool(
    data: web::Data<AppState>,
    name: web::Path<String>,
    config: web::Json<PoolConfig>,
) -> impl Responder {
    let mut pool_manager = data.pool_manager.write();
    match pool_manager.update_pool(&name, config.into_inner()) {
        Ok(_) => web::Json(json!({ "status": "success" })),
        Err(e) => web::Json(json!({ "status": "error", "message": e.to_string() }))
    }
}

async fn delete_pool(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    let mut pool_manager = data.pool_manager.write();
    match pool_manager.delete_pool(&name) {
        Ok(_) => web::Json(json!({ "status": "success" })),
        Err(e) => web::Json(json!({ "status": "error", "message": e.to_string() }))
    }
}

async fn scale_pool(
    data: web::Data<AppState>,
    name: web::Path<String>,
    scale: web::Json<i32>,
) -> impl Responder {
    let mut pool_manager = data.pool_manager.write();
    match pool_manager.scale_pool(&name, scale.into_inner()) {
        Ok(_) => web::Json(json!({ "status": "success" })),
        Err(e) => web::Json(json!({ "status": "error", "message": e.to_string() }))
    }
} 