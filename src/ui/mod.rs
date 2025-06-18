//! UI Module - Интерфейс управления и визуализации
//! 
//! Этот модуль предоставляет веб-интерфейс для:
//! - Визуализации метрик модели
//! - Управления параметрами
//! - Мониторинга состояния
//! - Настройки масштабирования
//! - Отображения результатов
//! - Управления ресурсами

pub mod dashboard;
pub mod components;
pub mod styles;
pub mod utils;

use crate::core::model_interface::ModelInterface;
use crate::monitoring::metrics::ModelMetrics;
use crate::pool::worker::WorkerStatus;
use crate::runtime::instance::InstanceManager;
use crate::network::api::ApiServer;
use crate::platform::gpu::GpuManager;

use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Состояние UI приложения
#[derive(Clone)]
pub struct UiState {
    pub model_interface: Arc<dyn ModelInterface + Send + Sync>,
    pub instance_manager: Arc<InstanceManager>,
    pub api_server: Arc<ApiServer>,
    pub gpu_manager: Arc<GpuManager>,
    pub metrics: Arc<RwLock<ModelMetrics>>,
}

/// Конфигурация UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub host: String,
    pub port: u16,
    pub enable_ssl: bool,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
    pub static_files_path: String,
    pub api_prefix: String,
    pub websocket_path: String,
    pub cors_origins: Vec<String>,
    pub rate_limit: u32,
    pub session_timeout: u64,
    pub theme: UiTheme,
    pub language: String,
}

/// Тема UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiTheme {
    Light,
    Dark,
    Auto,
}

/// Основной UI сервер
pub struct UiServer {
    config: UiConfig,
    state: UiState,
    router: Router,
}

impl UiServer {
    /// Создает новый UI сервер
    pub fn new(config: UiConfig, state: UiState) -> Self {
        let router = Self::create_router(state.clone());
        
        Self {
            config,
            state,
            router,
        }
    }

    /// Создает роутер с маршрутами
    fn create_router(state: UiState) -> Router {
        Router::new()
            // Основные страницы
            .route("/", get(dashboard::index))
            .route("/dashboard", get(dashboard::dashboard))
            .route("/models", get(dashboard::models))
            .route("/workers", get(dashboard::workers))
            .route("/monitoring", get(dashboard::monitoring))
            .route("/settings", get(dashboard::settings))
            
            // API endpoints
            .route("/api/status", get(api::get_status))
            .route("/api/metrics", get(api::get_metrics))
            .route("/api/models", get(api::get_models))
            .route("/api/workers", get(api::get_workers))
            .route("/api/gpu", get(api::get_gpu_info))
            .route("/api/memory", get(api::get_memory_info))
            
            // WebSocket для real-time обновлений
            .route("/ws/metrics", get(websocket::metrics_stream))
            .route("/ws/events", get(websocket::events_stream))
            
            // Статические файлы
            .nest_service("/static", get(static_files::serve))
            
            .with_state(state)
    }

    /// Запускает UI сервер
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        log::info!("UI Server starting on {}", addr);
        
        axum::serve(listener, self.router.clone()).await?;
        
        Ok(())
    }

    /// Останавливает UI сервер
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("UI Server stopping");
        Ok(())
    }

    /// Получает статус UI сервера
    pub fn get_status(&self) -> UiStatus {
        UiStatus {
            running: true,
            host: self.config.host.clone(),
            port: self.config.port,
            uptime: std::time::Duration::from_secs(0), // TODO: реализовать
            connections: 0, // TODO: реализовать
        }
    }
}

/// Статус UI сервера
#[derive(Debug, Clone, Serialize)]
pub struct UiStatus {
    pub running: bool,
    pub host: String,
    pub port: u16,
    pub uptime: std::time::Duration,
    pub connections: u32,
}

/// Инициализация UI модуля
pub async fn init_ui(config: UiConfig, state: UiState) -> Result<UiServer, Box<dyn std::error::Error>> {
    log::info!("Initializing UI module");
    
    let server = UiServer::new(config, state);
    
    log::info!("UI module initialized successfully");
    Ok(server)
}

/// Запуск UI модуля
pub async fn start_ui(server: UiServer) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting UI server");
    server.start().await
}

/// Остановка UI модуля
pub async fn stop_ui(server: UiServer) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Stopping UI server");
    server.stop().await
}

// Подмодули
mod api;
mod websocket;
mod static_files; 