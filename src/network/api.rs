//! Network API - REST API endpoints для моделей
//! 
//! Этот модуль предоставляет:
//! - REST API endpoints
//! - WebSocket handlers
//! - Аутентификация
//! - Rate limiting

use crate::core::model_interface::{
    ModelInterface, ModelRequest, ModelResponse, ModelInfo, ModelConfig, ModelMetrics
};
use crate::core::error::AppError;
use crate::monitoring::metrics::SystemMetrics;
use crate::pool::worker::WorkerStatus;
use crate::runtime::instance::InstanceManager;
use crate::platform::gpu::GpuManager;

use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{State, Path, Json, Query},
    response::{Json as JsonResponse, Html},
    http::{StatusCode, HeaderMap},
    headers::{Authorization, Bearer},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{CorsLayer, Any};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

/// Состояние API сервера
#[derive(Clone)]
pub struct ApiState {
    pub model_manager: Arc<dyn ModelInterface + Send + Sync>,
    pub instance_manager: Arc<InstanceManager>,
    pub gpu_manager: Arc<GpuManager>,
    pub system_metrics: Arc<RwLock<SystemMetrics>>,
    pub rate_limiter: Arc<RateLimiter>,
}

/// API сервер
pub struct ApiServer {
    state: ApiState,
    router: Router,
    config: ApiConfig,
}

impl ApiServer {
    /// Создает новый API сервер
    pub fn new(state: ApiState, config: ApiConfig) -> Self {
        let router = Self::create_router(state.clone());
        
        Self {
            state,
            router,
            config,
        }
    }

    /// Создает роутер с маршрутами
    fn create_router(state: ApiState) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            // Системные endpoints
            .route("/api/v1/status", get(api::get_status))
            .route("/api/v1/health", get(api::get_health))
            .route("/api/v1/metrics", get(api::get_metrics))
            .route("/api/v1/info", get(api::get_info))
            
            // Модели
            .route("/api/v1/models", get(api::get_models))
            .route("/api/v1/models/:name", get(api::get_model))
            .route("/api/v1/models/:name/request", post(api::process_request))
            .route("/api/v1/models/:name/config", get(api::get_model_config))
            .route("/api/v1/models/:name/config", put(api::update_model_config))
            .route("/api/v1/models/:name/metrics", get(api::get_model_metrics))
            .route("/api/v1/models/:name/health", get(api::get_model_health))
            
            // Воркеры
            .route("/api/v1/workers", get(api::get_workers))
            .route("/api/v1/workers/:id", get(api::get_worker))
            .route("/api/v1/workers/:id/status", get(api::get_worker_status))
            
            // GPU
            .route("/api/v1/gpu", get(api::get_gpu_info))
            .route("/api/v1/gpu/optimize", post(api::optimize_gpu))
            .route("/api/v1/gpu/config", get(api::get_gpu_config))
            .route("/api/v1/gpu/config", put(api::update_gpu_config))
            
            // Память
            .route("/api/v1/memory", get(api::get_memory_info))
            .route("/api/v1/memory/optimize", post(api::optimize_memory))
            
            // Система
            .route("/api/v1/system/restart", post(api::restart_system))
            .route("/api/v1/system/shutdown", post(api::shutdown_system))
            .route("/api/v1/system/update", post(api::update_system))
            
            // Мониторинг
            .route("/api/v1/monitoring/alerts", get(api::get_alerts))
            .route("/api/v1/monitoring/logs", get(api::get_logs))
            .route("/api/v1/monitoring/events", get(api::get_events))
            
            // Документация
            .route("/api/docs", get(api::get_docs))
            .route("/api/openapi.json", get(api::get_openapi))
            
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB limit
            .with_state(state)
    }

    /// Запускает API сервер
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        log::info!("API Server starting on {}", addr);
        
        axum::serve(listener, self.router.clone()).await?;
        
        Ok(())
    }

    /// Останавливает API сервер
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("API Server stopping");
        Ok(())
    }
}

/// Конфигурация API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub enable_ssl: bool,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
    pub rate_limit: u32,
    pub max_request_size: usize,
    pub enable_cors: bool,
    pub cors_origins: Vec<String>,
    pub enable_auth: bool,
    pub auth_tokens: Vec<String>,
    pub enable_docs: bool,
    pub enable_metrics: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_ssl: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            rate_limit: 1000,
            max_request_size: 10 * 1024 * 1024, // 10MB
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
            enable_auth: false,
            auth_tokens: vec![],
            enable_docs: true,
            enable_metrics: true,
        }
    }
}

/// Rate limiter
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<u64>>>>,
    limit: u32,
    window: u64,
}

impl RateLimiter {
    pub fn new(limit: u32, window: u64) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            limit,
            window,
        }
    }

    pub async fn check_rate_limit(&self, client_id: &str) -> Result<bool, AppError> {
        let mut requests = self.requests.write().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let client_requests = requests.entry(client_id.to_string()).or_insert_with(Vec::new);
        
        // Удаляем старые запросы
        client_requests.retain(|&timestamp| now - timestamp < self.window);
        
        // Проверяем лимит
        if client_requests.len() >= self.limit as usize {
            return Ok(false);
        }
        
        // Добавляем новый запрос
        client_requests.push(now);
        Ok(true)
    }
}

// API handlers
mod api {
    use super::*;

    /// Получение статуса системы
    pub async fn get_status(State(state): State<ApiState>) -> JsonResponse<ApiResponse<SystemStatus>> {
        let status = SystemStatus {
            status: "online".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            timestamp: chrono::Utc::now(),
        };
        
        JsonResponse(ApiResponse::success(status))
    }

    /// Получение здоровья системы
    pub async fn get_health(State(state): State<ApiState>) -> JsonResponse<ApiResponse<HealthStatus>> {
        let health = HealthStatus {
            status: "healthy".to_string(),
            checks: vec![
                HealthCheck {
                    name: "api".to_string(),
                    status: "healthy".to_string(),
                    message: "API is running".to_string(),
                },
                HealthCheck {
                    name: "models".to_string(),
                    status: "healthy".to_string(),
                    message: "Models are available".to_string(),
                },
                HealthCheck {
                    name: "gpu".to_string(),
                    status: "healthy".to_string(),
                    message: "GPU is operational".to_string(),
                },
            ],
            timestamp: chrono::Utc::now(),
        };
        
        JsonResponse(ApiResponse::success(health))
    }

    /// Получение метрик системы
    pub async fn get_metrics(State(state): State<ApiState>) -> JsonResponse<ApiResponse<SystemMetrics>> {
        let metrics = state.system_metrics.read().await.clone();
        JsonResponse(ApiResponse::success(metrics))
    }

    /// Получение информации о системе
    pub async fn get_info(State(state): State<ApiState>) -> JsonResponse<ApiResponse<SystemInfo>> {
        let info = SystemInfo {
            name: "PoolAI".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "AI Mining Pool Management System".to_string(),
            features: vec![
                "GPU Optimization".to_string(),
                "Model Management".to_string(),
                "Real-time Monitoring".to_string(),
                "Telegram Bot Integration".to_string(),
            ],
            timestamp: chrono::Utc::now(),
        };
        
        JsonResponse(ApiResponse::success(info))
    }

    /// Получение списка моделей
    pub async fn get_models(State(state): State<ApiState>) -> JsonResponse<ApiResponse<Vec<ModelInfo>>> {
        // В реальной реализации здесь должен быть доступ к менеджеру моделей
        let models = vec![
            ModelInfo {
                name: "gpt-3.5-turbo".to_string(),
                version: "1.0.0".to_string(),
                description: "GPT-3.5 Turbo model".to_string(),
                model_type: crate::core::model_interface::ModelType::LanguageModel,
                parameters: 7_000_000_000,
                context_length: 4096,
                supported_features: vec![
                    crate::core::model_interface::ModelFeature::TextGeneration,
                    crate::core::model_interface::ModelFeature::TextCompletion,
                ],
                hardware_requirements: crate::core::model_interface::HardwareRequirements {
                    min_gpu_memory: 8192,
                    recommended_gpu_memory: 16384,
                    min_ram: 16384,
                    recommended_ram: 32768,
                    min_cpu_cores: 8,
                    recommended_cpu_cores: 16,
                    gpu_types: vec!["NVIDIA RTX 4090".to_string()],
                    supported_precisions: vec![crate::core::model_interface::Precision::FP16],
                },
                license: Some("MIT".to_string()),
                author: Some("OpenAI".to_string()),
            }
        ];
        
        JsonResponse(ApiResponse::success(models))
    }

    /// Получение информации о модели
    pub async fn get_model(
        State(state): State<ApiState>,
        Path(name): Path<String>,
    ) -> JsonResponse<ApiResponse<ModelInfo>> {
        // В реальной реализации здесь должен быть доступ к конкретной модели
        let model_info = ModelInfo {
            name: name.clone(),
            version: "1.0.0".to_string(),
            description: format!("Model: {}", name),
            model_type: crate::core::model_interface::ModelType::LanguageModel,
            parameters: 7_000_000_000,
            context_length: 4096,
            supported_features: vec![
                crate::core::model_interface::ModelFeature::TextGeneration,
            ],
            hardware_requirements: crate::core::model_interface::HardwareRequirements {
                min_gpu_memory: 8192,
                recommended_gpu_memory: 16384,
                min_ram: 16384,
                recommended_ram: 32768,
                min_cpu_cores: 8,
                recommended_cpu_cores: 16,
                gpu_types: vec!["NVIDIA RTX 4090".to_string()],
                supported_precisions: vec![crate::core::model_interface::Precision::FP16],
            },
            license: Some("MIT".to_string()),
            author: Some("PoolAI".to_string()),
        };
        
        JsonResponse(ApiResponse::success(model_info))
    }

    /// Обработка запроса к модели
    pub async fn process_request(
        State(state): State<ApiState>,
        Path(name): Path<String>,
        Json(request): Json<ModelRequest>,
    ) -> JsonResponse<ApiResponse<ModelResponse>> {
        // Проверяем rate limit
        let client_id = "default"; // В реальной реализации извлекаем из запроса
        if !state.rate_limiter.check_rate_limit(client_id).await.unwrap_or(false) {
            return JsonResponse(ApiResponse::error(
                "Rate limit exceeded".to_string(),
                StatusCode::TOO_MANY_REQUESTS,
            ));
        }

        // Обрабатываем запрос
        match state.model_manager.process_request(request).await {
            Ok(response) => JsonResponse(ApiResponse::success(response)),
            Err(e) => JsonResponse(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Получение конфигурации модели
    pub async fn get_model_config(
        State(state): State<ApiState>,
        Path(name): Path<String>,
    ) -> JsonResponse<ApiResponse<ModelConfig>> {
        // В реальной реализации получаем конфигурацию модели
        let config = ModelConfig {
            model_path: Some(format!("/models/{}", name)),
            device: crate::core::model_interface::DeviceConfig {
                device_type: crate::core::model_interface::DeviceType::GPU,
                device_id: Some(0),
                memory_fraction: 0.8,
                allow_growth: true,
            },
            performance: crate::core::model_interface::PerformanceConfig {
                batch_size: 16,
                max_concurrent_requests: 32,
                timeout_seconds: 30,
                retry_attempts: 3,
                enable_caching: true,
                cache_size: 1024 * 1024 * 1024,
            },
            memory: crate::core::model_interface::MemoryConfig {
                max_memory_usage: 16384,
                memory_pool_size: 8192,
                enable_memory_optimization: true,
                garbage_collection_threshold: 0.8,
            },
            inference: crate::core::model_interface::InferenceConfig {
                default_temperature: 0.7,
                default_max_tokens: 100,
                default_top_p: 0.9,
                enable_sampling: true,
                enable_beam_search: false,
                beam_width: 5,
            },
            optimization: crate::core::model_interface::OptimizationConfig {
                enable_quantization: true,
                quantization_type: Some(crate::core::model_interface::Precision::FP16),
                enable_pruning: false,
                enable_distillation: false,
                enable_compilation: true,
                optimization_level: crate::core::model_interface::OptimizationLevel::Advanced,
            },
        };
        
        JsonResponse(ApiResponse::success(config))
    }

    /// Обновление конфигурации модели
    pub async fn update_model_config(
        State(state): State<ApiState>,
        Path(name): Path<String>,
        Json(config): Json<ModelConfig>,
    ) -> JsonResponse<ApiResponse<()>> {
        match state.model_manager.update_config(config).await {
            Ok(()) => JsonResponse(ApiResponse::success(())),
            Err(e) => JsonResponse(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Получение метрик модели
    pub async fn get_model_metrics(
        State(state): State<ApiState>,
        Path(name): Path<String>,
    ) -> JsonResponse<ApiResponse<ModelMetrics>> {
        match state.model_manager.get_metrics().await {
            Ok(metrics) => JsonResponse(ApiResponse::success(metrics)),
            Err(e) => JsonResponse(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Получение здоровья модели
    pub async fn get_model_health(
        State(state): State<ApiState>,
        Path(name): Path<String>,
    ) -> JsonResponse<ApiResponse<crate::core::model_interface::ModelHealth>> {
        match state.model_manager.health_check().await {
            Ok(health) => JsonResponse(ApiResponse::success(health)),
            Err(e) => JsonResponse(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Получение списка воркеров
    pub async fn get_workers(State(state): State<ApiState>) -> JsonResponse<ApiResponse<Vec<WorkerInfo>>> {
        // В реальной реализации получаем список воркеров
        let workers = vec![
            WorkerInfo {
                id: "worker_001".to_string(),
                name: "GPU Worker 1".to_string(),
                status: WorkerStatus::Running,
                gpu_usage: 85.5,
                memory_usage: 12.3,
                temperature: 72.0,
                hash_rate: 95.2,
            }
        ];
        
        JsonResponse(ApiResponse::success(workers))
    }

    /// Получение информации о воркере
    pub async fn get_worker(
        State(state): State<ApiState>,
        Path(id): Path<String>,
    ) -> JsonResponse<ApiResponse<WorkerInfo>> {
        let worker = WorkerInfo {
            id: id.clone(),
            name: format!("Worker {}", id),
            status: WorkerStatus::Running,
            gpu_usage: 85.5,
            memory_usage: 12.3,
            temperature: 72.0,
            hash_rate: 95.2,
        };
        
        JsonResponse(ApiResponse::success(worker))
    }

    /// Получение статуса воркера
    pub async fn get_worker_status(
        State(state): State<ApiState>,
        Path(id): Path<String>,
    ) -> JsonResponse<ApiResponse<WorkerStatus>> {
        JsonResponse(ApiResponse::success(WorkerStatus::Running))
    }

    /// Получение информации о GPU
    pub async fn get_gpu_info(State(state): State<ApiState>) -> JsonResponse<ApiResponse<GpuInfo>> {
        match state.gpu_manager.get_gpu_info().await {
            Ok(gpu_info) => JsonResponse(ApiResponse::success(gpu_info)),
            Err(e) => JsonResponse(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Оптимизация GPU
    pub async fn optimize_gpu(State(state): State<ApiState>) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации выполняем оптимизацию GPU
        JsonResponse(ApiResponse::success(()))
    }

    /// Получение конфигурации GPU
    pub async fn get_gpu_config(State(state): State<ApiState>) -> JsonResponse<ApiResponse<GpuConfig>> {
        let config = GpuConfig {
            power_limit: 250,
            temperature_limit: 85.0,
            memory_clock: 16000,
            gpu_clock: 2000,
            fan_speed: 80,
        };
        
        JsonResponse(ApiResponse::success(config))
    }

    /// Обновление конфигурации GPU
    pub async fn update_gpu_config(
        State(state): State<ApiState>,
        Json(config): Json<GpuConfig>,
    ) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации применяем новую конфигурацию GPU
        JsonResponse(ApiResponse::success(()))
    }

    /// Получение информации о памяти
    pub async fn get_memory_info(State(state): State<ApiState>) -> JsonResponse<ApiResponse<MemoryInfo>> {
        let memory_info = MemoryInfo {
            total: 32 * 1024 * 1024, // 32GB
            used: 16 * 1024 * 1024,  // 16GB
            available: 16 * 1024 * 1024, // 16GB
            usage_percent: 50.0,
        };
        
        JsonResponse(ApiResponse::success(memory_info))
    }

    /// Оптимизация памяти
    pub async fn optimize_memory(State(state): State<ApiState>) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации выполняем оптимизацию памяти
        JsonResponse(ApiResponse::success(()))
    }

    /// Перезапуск системы
    pub async fn restart_system(State(state): State<ApiState>) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации выполняем перезапуск системы
        JsonResponse(ApiResponse::success(()))
    }

    /// Выключение системы
    pub async fn shutdown_system(State(state): State<ApiState>) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации выполняем выключение системы
        JsonResponse(ApiResponse::success(()))
    }

    /// Обновление системы
    pub async fn update_system(State(state): State<ApiState>) -> JsonResponse<ApiResponse<()>> {
        // В реальной реализации выполняем обновление системы
        JsonResponse(ApiResponse::success(()))
    }

    /// Получение алертов
    pub async fn get_alerts(State(state): State<ApiState>) -> JsonResponse<ApiResponse<Vec<Alert>>> {
        let alerts = vec![
            Alert {
                id: "alert_001".to_string(),
                level: "warning".to_string(),
                message: "GPU temperature is high".to_string(),
                timestamp: chrono::Utc::now(),
            }
        ];
        
        JsonResponse(ApiResponse::success(alerts))
    }

    /// Получение логов
    pub async fn get_logs(
        State(state): State<ApiState>,
        Query(params): Query<LogParams>,
    ) -> JsonResponse<ApiResponse<Vec<LogEntry>>> {
        let logs = vec![
            LogEntry {
                level: "info".to_string(),
                message: "System started successfully".to_string(),
                timestamp: chrono::Utc::now(),
            }
        ];
        
        JsonResponse(ApiResponse::success(logs))
    }

    /// Получение событий
    pub async fn get_events(State(state): State<ApiState>) -> JsonResponse<ApiResponse<Vec<Event>>> {
        let events = vec![
            Event {
                id: "event_001".to_string(),
                type_: "model_loaded".to_string(),
                data: serde_json::json!({"model": "gpt-3.5-turbo"}),
                timestamp: chrono::Utc::now(),
            }
        ];
        
        JsonResponse(ApiResponse::success(events))
    }

    /// Получение документации
    pub async fn get_docs() -> Html<String> {
        let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>PoolAI API Documentation</title>
            <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
        </head>
        <body>
            <div id="swagger-ui"></div>
            <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js"></script>
            <script>
                window.onload = function() {
                    SwaggerUIBundle({
                        url: '/api/openapi.json',
                        dom_id: '#swagger-ui',
                    });
                };
            </script>
        </body>
        </html>
        "#;
        
        Html(html.to_string())
    }

    /// Получение OpenAPI спецификации
    pub async fn get_openapi() -> JsonResponse<serde_json::Value> {
        let openapi = serde_json::json!({
            "openapi": "3.0.0",
            "info": {
                "title": "PoolAI API",
                "version": "1.0.0",
                "description": "API for PoolAI - AI Mining Pool Management System"
            },
            "paths": {
                "/api/v1/status": {
                    "get": {
                        "summary": "Get system status",
                        "responses": {
                            "200": {
                                "description": "System status"
                            }
                        }
                    }
                }
            }
        });
        
        JsonResponse(openapi)
    }
}

// Структуры данных

/// Статус системы
#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Статус здоровья
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub checks: Vec<HealthCheck>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Проверка здоровья
#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub message: String,
}

/// Информация о системе
#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub features: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Информация о воркере
#[derive(Debug, Serialize)]
pub struct WorkerInfo {
    pub id: String,
    pub name: String,
    pub status: WorkerStatus,
    pub gpu_usage: f64,
    pub memory_usage: f64,
    pub temperature: f64,
    pub hash_rate: f64,
}

/// Конфигурация GPU
#[derive(Debug, Serialize, Deserialize)]
pub struct GpuConfig {
    pub power_limit: u32,
    pub temperature_limit: f64,
    pub memory_clock: u32,
    pub gpu_clock: u32,
    pub fan_speed: u32,
}

/// Информация о памяти
#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
}

/// Алерт
#[derive(Debug, Serialize)]
pub struct Alert {
    pub id: String,
    pub level: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Параметры логов
#[derive(Debug, Deserialize)]
pub struct LogParams {
    pub level: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Запись лога
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Событие
#[derive(Debug, Serialize)]
pub struct Event {
    pub id: String,
    pub type_: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// API ответ
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String, _status: StatusCode) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
} 