//! PoolAI - Система управления пулом майнинга с интеграцией генеративных моделей
//! Version: MVP_v1

// MVP Modules - PRIORITY 1
pub mod core;
pub mod pool;
pub mod monitoring;

// Stage 2 Modules - PRIORITY 2 (future)
#[cfg(feature = "stage2")]
pub mod network;
#[cfg(feature = "stage2")]
pub mod platform;
#[cfg(feature = "stage2")]
pub mod tgbot;

// Stage 3 Modules - PRIORITY 3 (future)
#[cfg(feature = "stage3")]
pub mod runtime;
#[cfg(feature = "stage3")]
pub mod libs;
#[cfg(feature = "stage3")]
pub mod vm;
#[cfg(feature = "stage3")]
pub mod raid;
#[cfg(feature = "stage3")]
pub mod ui;

use serde::{Deserialize, Serialize};

/// Версия PoolAI
pub const VERSION: &str = "MVP_v1";

/// Информация о системе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub name: String,
    pub description: String,
    pub mvp_modules: Vec<String>,
    pub stage2_modules: Vec<String>,
    pub stage3_modules: Vec<String>,
    pub build_date: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            name: "PoolAI".to_string(),
            description: "Система управления пулом майнинга с интеграцией генеративных моделей".to_string(),
            mvp_modules: vec![
                "core".to_string(),
                "pool".to_string(),
                "monitoring".to_string(),
            ],
            stage2_modules: vec![
                "network".to_string(),
                "platform".to_string(),
                "tgbot".to_string(),
            ],
            stage3_modules: vec![
                "runtime".to_string(),
                "libs".to_string(),
                "vm".to_string(),
                "raid".to_string(),
                "ui".to_string(),
            ],
            build_date: "2024-01-01T00:00:00Z".to_string(),
        }
    }
}

/// Получение информации о системе
pub fn get_system_info() -> SystemInfo {
    SystemInfo::default()
}

/// Инициализация системы MVP
pub async fn initialize_system() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing PoolAI MVP v{}", VERSION);
    
    // Initialize MVP modules
    core::initialize().await?;
    pool::initialize().await?;
    monitoring::initialize().await?;
    
    tracing::info!("PoolAI MVP v{} initialized successfully", VERSION);
    Ok(())
}

/// Остановка системы MVP
pub async fn shutdown_system() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Shutting down PoolAI MVP v{}", VERSION);
    
    // Shutdown MVP modules in reverse order
    monitoring::shutdown().await?;
    pool::shutdown().await?;
    core::shutdown().await?;
    
    tracing::info!("PoolAI MVP v{} shut down successfully", VERSION);
    Ok(())
}

/// Проверка здоровья системы MVP
pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    // Check MVP modules health
    core::health_check().await?;
    pool::health_check().await?;
    monitoring::health_check().await?;
    
    Ok(())
}
