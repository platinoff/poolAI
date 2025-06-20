//! PoolAI - Система управления пулом майнинга с интеграцией генеративных моделей
//! Version: Beta_bolvanka_v1
//! 
//! Эта библиотека предоставляет:
//! - Управление AI майнинг пулами
//! - Интеграцию с генеративными моделями
//! - Оптимизацию GPU/ASIC/CPU ресурсов
//! - Telegram бот для управления
//! - Веб-интерфейс для мониторинга
//! - RAID систему для отказоустойчивости

pub mod core;
pub mod libs;
pub mod pool;
pub mod monitoring;
pub mod runtime;
pub mod network;
pub mod platform;
pub mod vm;
pub mod tgbot;
pub mod raid;
pub mod ui;
pub mod admin;
pub mod workers;
pub mod version;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Версия PoolAI
pub const VERSION: &str = "Beta_bolvanka_v1";

/// Информация о системе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
    pub modules: Vec<String>,
    pub build_date: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            name: "PoolAI".to_string(),
            description: "Система управления пулом майнинга с интеграцией генеративных моделей".to_string(),
            features: vec![
                "GPU/ASIC/CPU optimization".to_string(),
                "Model integration".to_string(),
                "Telegram bot".to_string(),
                "Web UI".to_string(),
                "RAID system".to_string(),
                "Monitoring".to_string(),
                "Reward system".to_string(),
            ],
            modules: vec![
                "core".to_string(),
                "libs".to_string(),
                "pool".to_string(),
                "monitoring".to_string(),
                "runtime".to_string(),
                "network".to_string(),
                "platform".to_string(),
                "vm".to_string(),
                "tgbot".to_string(),
                "raid".to_string(),
                "ui".to_string(),
                "admin".to_string(),
                "workers".to_string(),
                "version".to_string(),
            ],
            build_date: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        }
    }
}

/// Получение информации о системе
pub fn get_system_info() -> SystemInfo {
    SystemInfo::default()
}

/// Статус системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub modules_loaded: usize,
    pub features_enabled: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Инициализация системы
pub async fn initialize_system() -> Result<SystemStatus, Box<dyn std::error::Error>> {
    log::info!("Initializing PoolAI v{}", VERSION);
    
    // Инициализация модулей
    core::initialize().await?;
    libs::initialize().await?;
    pool::initialize().await?;
    monitoring::initialize().await?;
    runtime::initialize().await?;
    network::initialize().await?;
    platform::initialize().await?;
    vm::initialize().await?;
    tgbot::initialize().await?;
    raid::initialize().await?;
    ui::initialize().await?;
    admin::initialize().await?;
    workers::initialize().await?;
    
    log::info!("PoolAI v{} initialized successfully", VERSION);
    
    Ok(SystemStatus {
        status: "initialized".to_string(),
        version: VERSION.to_string(),
        uptime: 0,
        modules_loaded: 14,
        features_enabled: 7,
        timestamp: chrono::Utc::now(),
    })
}

/// Остановка системы
pub async fn shutdown_system() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Shutting down PoolAI v{}", VERSION);
    
    // Остановка модулей
    workers::shutdown().await?;
    admin::shutdown().await?;
    ui::shutdown().await?;
    raid::shutdown().await?;
    tgbot::shutdown().await?;
    vm::shutdown().await?;
    platform::shutdown().await?;
    network::shutdown().await?;
    runtime::shutdown().await?;
    monitoring::shutdown().await?;
    pool::shutdown().await?;
    libs::shutdown().await?;
    core::shutdown().await?;
    
    log::info!("PoolAI v{} shut down successfully", VERSION);
    Ok(())
}

/// Проверка здоровья системы
pub async fn health_check() -> Result<SystemHealth, Box<dyn std::error::Error>> {
    let mut health = SystemHealth {
        status: "healthy".to_string(),
        checks: Vec::new(),
        timestamp: chrono::Utc::now(),
    };
    
    // Проверка модулей
    let module_checks = vec![
        ("core", core::health_check().await),
        ("libs", libs::health_check().await),
        ("pool", pool::health_check().await),
        ("monitoring", monitoring::health_check().await),
        ("runtime", runtime::health_check().await),
        ("network", network::health_check().await),
        ("platform", platform::health_check().await),
        ("vm", vm::health_check().await),
        ("tgbot", tgbot::health_check().await),
        ("raid", raid::health_check().await),
        ("ui", ui::health_check().await),
        ("admin", admin::health_check().await),
        ("workers", workers::health_check().await),
    ];
    
    for (module, check_result) in module_checks {
        health.checks.push(ModuleHealth {
            module: module.to_string(),
            status: if check_result.is_ok() { "healthy".to_string() } else { "unhealthy".to_string() },
            message: check_result.map(|_| "OK".to_string()).unwrap_or_else(|e| e.to_string()),
        });
    }
    
    // Обновляем общий статус
    if health.checks.iter().any(|check| check.status == "unhealthy") {
        health.status = "warning".to_string();
    }
    
    Ok(health)
}

/// Здоровье системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: String,
    pub checks: Vec<ModuleHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Здоровье модуля
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleHealth {
    pub module: String,
    pub status: String,
    pub message: String,
}

/// Конфигурация системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub version: String,
    pub debug: bool,
    pub log_level: String,
    pub modules: HashMap<String, bool>,
    pub features: HashMap<String, bool>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        let mut modules = HashMap::new();
        modules.insert("core".to_string(), true);
        modules.insert("libs".to_string(), true);
        modules.insert("pool".to_string(), true);
        modules.insert("monitoring".to_string(), true);
        modules.insert("runtime".to_string(), true);
        modules.insert("network".to_string(), true);
        modules.insert("platform".to_string(), true);
        modules.insert("vm".to_string(), true);
        modules.insert("tgbot".to_string(), true);
        modules.insert("raid".to_string(), true);
        modules.insert("ui".to_string(), true);
        modules.insert("admin".to_string(), true);
        
        let mut features = HashMap::new();
        features.insert("gpu_optimization".to_string(), true);
        features.insert("model_integration".to_string(), true);
        features.insert("telegram_bot".to_string(), true);
        features.insert("web_ui".to_string(), true);
        features.insert("raid_system".to_string(), true);
        features.insert("monitoring".to_string(), true);
        features.insert("reward_system".to_string(), true);
        
        Self {
            version: VERSION.to_string(),
            debug: false,
            log_level: "info".to_string(),
            modules,
            features,
        }
    }
}

/// Получение конфигурации системы
pub fn get_system_config() -> SystemConfig {
    SystemConfig::default()
}

/// Обновление конфигурации системы
pub async fn update_system_config(config: SystemConfig) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Updating system configuration");
    
    // Здесь должна быть логика обновления конфигурации
    // Пока что просто логируем
    
    log::info!("System configuration updated successfully");
    Ok(())
}

/// Получение статистики системы
pub async fn get_system_stats() -> SystemStats {
    SystemStats {
        version: VERSION.to_string(),
        uptime: std::time::Duration::from_secs(0), // TODO: реализовать
        modules_loaded: 13,
        features_enabled: 7,
        memory_usage: 0.0, // TODO: реализовать
        cpu_usage: 0.0, // TODO: реализовать
        disk_usage: 0.0, // TODO: реализовать
        network_usage: 0.0, // TODO: реализовать
        timestamp: chrono::Utc::now(),
    }
}

/// Статистика системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub version: String,
    pub uptime: std::time::Duration,
    pub modules_loaded: usize,
    pub features_enabled: usize,
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// Re-exports для удобства использования
pub use core::*;
pub use pool::*;
pub use monitoring::*;
pub use runtime::*;
pub use network::*;
pub use platform::*;
pub use vm::*;
pub use tgbot::*;
pub use raid::*;
pub use ui::*;
pub use admin::*;
pub use libs::*; 