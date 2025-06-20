//! System Manager - Управление системными процессами

use crate::core::state::AppState;
use crate::pool::pool::PoolManager;
use crate::monitoring::metrics::SystemMetrics;
use crate::network::api::ApiServer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};

/// Менеджер системы
pub struct SystemManager {
    state: Arc<AppState>,
    pool_manager: Arc<PoolManager>,
    metrics: Arc<RwLock<SystemMetrics>>,
    api_server: Arc<ApiServer>,
}

impl SystemManager {
    /// Создает новый менеджер системы
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

    /// Запускает все компоненты системы
    pub async fn start_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("SystemManager: Starting system components");
        
        // Запуск пула
        self.pool_manager.start().await?;
        
        // Запуск API сервера
        self.api_server.start().await?;
        
        log::info!("SystemManager: All components started successfully");
        Ok(())
    }

    /// Останавливает все компоненты системы
    pub async fn stop_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("SystemManager: Stopping system components");
        
        // Остановка API сервера
        self.api_server.stop().await?;
        
        // Остановка пула
        self.pool_manager.stop().await?;
        
        log::info!("SystemManager: All components stopped successfully");
        Ok(())
    }

    /// Перезапускает систему
    pub async fn restart_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("SystemManager: Restarting system");
        
        self.stop_system().await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        self.start_system().await?;
        
        log::info!("SystemManager: System restarted successfully");
        Ok(())
    }

    /// Получает статус системы
    pub async fn get_system_status(&self) -> SystemStatus {
        let metrics = self.metrics.read().await;
        
        SystemStatus {
            is_running: self.pool_manager.is_running(),
            uptime: metrics.uptime,
            cpu_usage: metrics.cpu_usage,
            memory_usage: metrics.memory_usage,
            disk_usage: metrics.disk_usage,
            network_usage: metrics.network_usage,
            worker_count: self.pool_manager.get_worker_count(),
            active_tasks: self.pool_manager.get_active_task_count(),
            maintenance_mode: self.state.is_maintenance_mode().await,
        }
    }

    /// Получает информацию о системе
    pub async fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_date: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
            rust_version: env!("VERGEN_RUSTC_SEMVER").to_string(),
            target_arch: env!("VERGEN_CARGO_TARGET_ARCH").to_string(),
            target_os: env!("VERGEN_CARGO_TARGET_OS").to_string(),
        }
    }

    /// Проверяет здоровье системы
    pub async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus {
            overall: "healthy".to_string(),
            components: Vec::new(),
            timestamp: chrono::Utc::now(),
        };

        // Проверка пула
        let pool_health = if self.pool_manager.is_running() {
            "healthy"
        } else {
            "unhealthy"
        };
        status.components.push(ComponentHealth {
            name: "pool".to_string(),
            status: pool_health.to_string(),
            message: "Pool manager status".to_string(),
        });

        // Проверка API сервера
        let api_health = if self.api_server.is_running() {
            "healthy"
        } else {
            "unhealthy"
        };
        status.components.push(ComponentHealth {
            name: "api".to_string(),
            status: api_health.to_string(),
            message: "API server status".to_string(),
        });

        // Проверка метрик
        let metrics = self.metrics.read().await;
        let metrics_health = if metrics.is_valid() {
            "healthy"
        } else {
            "unhealthy"
        };
        status.components.push(ComponentHealth {
            name: "metrics".to_string(),
            status: metrics_health.to_string(),
            message: "System metrics status".to_string(),
        });

        // Обновляем общий статус
        if status.components.iter().any(|c| c.status == "unhealthy") {
            status.overall = "unhealthy".to_string();
        }

        status
    }

    /// Получает статистику производительности
    pub async fn get_performance_stats(&self) -> PerformanceStats {
        let metrics = self.metrics.read().await;
        
        PerformanceStats {
            cpu_usage: metrics.cpu_usage,
            memory_usage: metrics.memory_usage,
            disk_usage: metrics.disk_usage,
            network_usage: metrics.network_usage,
            gpu_usage: metrics.gpu_usage,
            task_throughput: self.pool_manager.get_task_throughput(),
            average_response_time: self.pool_manager.get_average_response_time(),
            error_rate: self.pool_manager.get_error_rate(),
        }
    }
}

/// Статус системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub is_running: bool,
    pub uptime: std::time::Duration,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub worker_count: usize,
    pub active_tasks: usize,
    pub maintenance_mode: bool,
}

/// Информация о системе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub build_date: String,
    pub rust_version: String,
    pub target_arch: String,
    pub target_os: String,
}

/// Статус здоровья
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall: String,
    pub components: Vec<ComponentHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Здоровье компонента
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: String,
    pub message: String,
}

/// Статистика производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub gpu_usage: f64,
    pub task_throughput: f64,
    pub average_response_time: std::time::Duration,
    pub error_rate: f64,
} 