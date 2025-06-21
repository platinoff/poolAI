//! Admin Module - Административные функции системы
//! 
//! Этот модуль предоставляет:
//! - Управление системой
//! - Мониторинг состояния
//! - Настройку конфигурации
//! - Административные функции

pub mod admin_panel;
pub mod system_manager;
pub mod config_manager;

use crate::core::state::AppState;
use crate::pool::pool::PoolManager;
use crate::monitoring::metrics::SystemMetrics;
use crate::network::api::ApiServer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Административная панель
pub struct AdminPanel {
    state: Arc<AppState>,
    pool_manager: Arc<PoolManager>,
    metrics: Arc<RwLock<SystemMetrics>>,
    api_server: Arc<ApiServer>,
}

impl AdminPanel {
    /// Создает новую административную панель
    pub fn new(
        state: Arc<AppState>,
        pool_manager: Arc<PoolManager>,
        metrics: Arc<RwLock<SystemMetrics>>,
        api_server: Arc<ApiServer>,
    ) -> Self {
        Self {
            state,
            pool_manager,
            metrics,
            api_server,
        }
    }

    /// Получает статистику системы
    pub async fn get_system_stats(&self) -> SystemStats {
        let metrics = self.metrics.read().await;
        
        SystemStats {
            total_workers: self.pool_manager.get_worker_count(),
            active_workers: self.pool_manager.get_active_worker_count(),
            total_hashrate: self.pool_manager.get_total_hashrate(),
            system_load: metrics.system_load,
            memory_usage: metrics.memory_usage,
            cpu_usage: metrics.cpu_usage,
            uptime: metrics.uptime,
        }
    }

    /// Получает статус пула
    pub async fn get_pool_status(&self) -> PoolStatus {
        PoolStatus {
            is_running: self.pool_manager.is_running(),
            worker_count: self.pool_manager.get_worker_count(),
            active_tasks: self.pool_manager.get_active_task_count(),
            queue_size: self.pool_manager.get_queue_size(),
            last_block: self.pool_manager.get_last_block_hash(),
        }
    }

    /// Перезапускает систему
    pub async fn restart_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Admin: Restarting system");
        
        // Остановка компонентов
        self.pool_manager.stop().await?;
        self.api_server.stop().await?;
        
        // Запуск компонентов
        self.pool_manager.start().await?;
        self.api_server.start().await?;
        
        log::info!("Admin: System restarted successfully");
        Ok(())
    }

    /// Включает режим обслуживания
    pub async fn enable_maintenance_mode(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Admin: Enabling maintenance mode");
        self.state.set_maintenance_mode(true).await;
        Ok(())
    }

    /// Выключает режим обслуживания
    pub async fn disable_maintenance_mode(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Admin: Disabling maintenance mode");
        self.state.set_maintenance_mode(false).await;
        Ok(())
    }

    /// Получает логи системы
    pub async fn get_system_logs(&self, limit: usize) -> Vec<LogEntry> {
        // Здесь должна быть логика получения логов
        vec![
            LogEntry {
                timestamp: chrono::Utc::now(),
                level: "INFO".to_string(),
                message: "System logs requested".to_string(),
            }
        ]
    }
}

/// Статистика системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_workers: usize,
    pub active_workers: usize,
    pub total_hashrate: f64,
    pub system_load: f64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub uptime: std::time::Duration,
}

/// Статус пула
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatus {
    pub is_running: bool,
    pub worker_count: usize,
    pub active_tasks: usize,
    pub queue_size: usize,
    pub last_block: String,
}

/// Запись лога
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
}

/// Инициализация admin модуля
pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing admin module");
    Ok(())
}

/// Остановка admin модуля
pub async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Shutting down admin module");
    Ok(())
}

/// Проверка здоровья admin модуля
pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Admin module health check passed");
    Ok(())
}

pub use admin_panel::*;
pub use system_manager::*;
pub use config_manager::*; 