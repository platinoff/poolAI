//! PoolAI - Система управления пулом майнинга с интеграцией генеративных моделей
//! Version: Beta_bolvanka_v1

pub mod core;
pub mod version;

use serde::{Deserialize, Serialize};

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
                "Core system".to_string(),
                "State management".to_string(),
                "Configuration".to_string(),
                "Error handling".to_string(),
                "Utilities".to_string(),
            ],
            modules: vec![
                "core".to_string(),
                "version".to_string(),
            ],
            build_date: "2024-01-01T00:00:00Z".to_string(),
        }
    }
}

/// Получение информации о системе
pub fn get_system_info() -> SystemInfo {
    SystemInfo::default()
}

/// Инициализация системы
pub async fn initialize_system() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing PoolAI v{}", VERSION);
    core::initialize().await?;
    log::info!("PoolAI v{} initialized successfully", VERSION);
    Ok(())
}

/// Остановка системы
pub async fn shutdown_system() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Shutting down PoolAI v{}", VERSION);
    core::shutdown().await?;
    log::info!("PoolAI v{} shut down successfully", VERSION);
    Ok(())
}

/// Проверка здоровья системы
pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    core::health_check().await
} 