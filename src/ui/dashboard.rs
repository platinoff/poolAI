//! Dashboard Module - Дашборд моделей и визуализация метрик
//! 
//! Этот модуль предоставляет:
//! - Дашборд моделей
//! - Графики метрик
//! - Статус системы
//! - Управление

use crate::core::model_interface::ModelInterface;
use crate::monitoring::metrics::ModelMetrics;
use crate::pool::worker::WorkerStatus;
use crate::runtime::instance::InstanceManager;
use crate::platform::gpu::GpuManager;

use axum::{
    extract::State,
    response::{Html, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::UiState;

/// Главная страница дашборда
pub async fn index(State(state): State<UiState>) -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI Dashboard</title>
        <link rel="stylesheet" href="/static/css/dashboard.css">
    </head>
    <body>
        <div id="app">
            <header class="dashboard-header">
                <h1>PoolAI Dashboard</h1>
                <nav class="dashboard-nav">
                    <a href="/dashboard" class="active">Dashboard</a>
                    <a href="/models">Models</a>
                    <a href="/workers">Workers</a>
                    <a href="/monitoring">Monitoring</a>
                    <a href="/settings">Settings</a>
                </nav>
            </header>
            
            <main class="dashboard-main">
                <div class="dashboard-grid">
                    <div class="card">
                        <h3>System Status</h3>
                        <div id="system-status">Loading...</div>
                    </div>
                    
                    <div class="card">
                        <h3>GPU Usage</h3>
                        <div id="gpu-usage">Loading...</div>
                    </div>
                    
                    <div class="card">
                        <h3>Memory Usage</h3>
                        <div id="memory-usage">Loading...</div>
                    </div>
                    
                    <div class="card">
                        <h3>Active Models</h3>
                        <div id="active-models">Loading...</div>
                    </div>
                </div>
                
                <div class="dashboard-charts">
                    <div class="chart-container">
                        <h3>Performance Metrics</h3>
                        <canvas id="performance-chart"></canvas>
                    </div>
                    
                    <div class="chart-container">
                        <h3>Resource Usage</h3>
                        <canvas id="resource-chart"></canvas>
                    </div>
                </div>
            </main>
        </div>
        
        <script src="/static/js/dashboard.js"></script>
    </body>
    </html>
    "#;
    
    Html(html.to_string())
}

/// Дашборд с метриками
pub async fn dashboard(State(state): State<UiState>) -> Html<String> {
    let metrics = state.metrics.read().await;
    let gpu_info = state.gpu_manager.get_gpu_info().await.unwrap_or_default();
    
    let html = format!(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI - Dashboard</title>
        <link rel="stylesheet" href="/static/css/dashboard.css">
    </head>
    <body>
        <div id="app">
            <header class="dashboard-header">
                <h1>PoolAI Dashboard</h1>
                <div class="status-indicator">
                    <span class="status-dot active"></span>
                    <span>System Online</span>
                </div>
            </header>
            
            <main class="dashboard-main">
                <div class="metrics-overview">
                    <div class="metric-card">
                        <h3>GPU Usage</h3>
                        <div class="metric-value">{:.1}%</div>
                        <div class="metric-label">Current GPU utilization</div>
                    </div>
                    
                    <div class="metric-card">
                        <h3>Memory Usage</h3>
                        <div class="metric-value">{:.1}%</div>
                        <div class="metric-label">Current memory usage</div>
                    </div>
                    
                    <div class="metric-card">
                        <h3>Active Models</h3>
                        <div class="metric-value">{}</div>
                        <div class="metric-label">Currently running models</div>
                    </div>
                    
                    <div class="metric-card">
                        <h3>Requests/sec</h3>
                        <div class="metric-value">{:.1}</div>
                        <div class="metric-label">Requests per second</div>
                    </div>
                </div>
                
                <div class="charts-section">
                    <div class="chart-card">
                        <h3>Performance Over Time</h3>
                        <canvas id="performance-chart"></canvas>
                    </div>
                    
                    <div class="chart-card">
                        <h3>Resource Usage</h3>
                        <canvas id="resource-chart"></canvas>
                    </div>
                </div>
            </main>
        </div>
        
        <script>
            // Передаем данные в JavaScript
            window.dashboardData = {{
                gpuUsage: {:.1},
                memoryUsage: {:.1},
                activeModels: {},
                requestsPerSec: {:.1},
                metrics: {:?}
            }};
        </script>
        <script src="/static/js/dashboard.js"></script>
    </body>
    </html>
    "#, 
    gpu_info.usage.unwrap_or(0.0),
    metrics.memory_usage.unwrap_or(0.0),
    metrics.active_models,
    metrics.requests_per_sec.unwrap_or(0.0),
    gpu_info.usage.unwrap_or(0.0),
    metrics.memory_usage.unwrap_or(0.0),
    metrics.active_models,
    metrics.requests_per_sec.unwrap_or(0.0),
    metrics
    );
    
    Html(html)
}

/// Страница моделей
pub async fn models(State(state): State<UiState>) -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI - Models</title>
        <link rel="stylesheet" href="/static/css/models.css">
    </head>
    <body>
        <div id="app">
            <header class="models-header">
                <h1>Model Management</h1>
                <button class="btn-primary" onclick="loadModel()">Load Model</button>
            </header>
            
            <main class="models-main">
                <div class="models-grid" id="models-grid">
                    <!-- Models will be loaded here -->
                </div>
            </main>
        </div>
        
        <script src="/static/js/models.js"></script>
    </body>
    </html>
    "#;
    
    Html(html.to_string())
}

/// Страница воркеров
pub async fn workers(State(state): State<UiState>) -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI - Workers</title>
        <link rel="stylesheet" href="/static/css/workers.css">
    </head>
    <body>
        <div id="app">
            <header class="workers-header">
                <h1>Worker Management</h1>
                <button class="btn-primary" onclick="addWorker()">Add Worker</button>
            </header>
            
            <main class="workers-main">
                <div class="workers-grid" id="workers-grid">
                    <!-- Workers will be loaded here -->
                </div>
            </main>
        </div>
        
        <script src="/static/js/workers.js"></script>
    </body>
    </html>
    "#;
    
    Html(html.to_string())
}

/// Страница мониторинга
pub async fn monitoring(State(state): State<UiState>) -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI - Monitoring</title>
        <link rel="stylesheet" href="/static/css/monitoring.css">
    </head>
    <body>
        <div id="app">
            <header class="monitoring-header">
                <h1>System Monitoring</h1>
                <div class="monitoring-controls">
                    <button class="btn-secondary" onclick="refreshMetrics()">Refresh</button>
                    <button class="btn-secondary" onclick="exportMetrics()">Export</button>
                </div>
            </header>
            
            <main class="monitoring-main">
                <div class="monitoring-grid">
                    <div class="monitoring-card">
                        <h3>System Metrics</h3>
                        <div id="system-metrics">Loading...</div>
                    </div>
                    
                    <div class="monitoring-card">
                        <h3>Performance Metrics</h3>
                        <div id="performance-metrics">Loading...</div>
                    </div>
                    
                    <div class="monitoring-card">
                        <h3>Error Logs</h3>
                        <div id="error-logs">Loading...</div>
                    </div>
                </div>
            </main>
        </div>
        
        <script src="/static/js/monitoring.js"></script>
    </body>
    </html>
    "#;
    
    Html(html.to_string())
}

/// Страница настроек
pub async fn settings(State(state): State<UiState>) -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PoolAI - Settings</title>
        <link rel="stylesheet" href="/static/css/settings.css">
    </head>
    <body>
        <div id="app">
            <header class="settings-header">
                <h1>System Settings</h1>
                <button class="btn-primary" onclick="saveSettings()">Save Settings</button>
            </header>
            
            <main class="settings-main">
                <div class="settings-grid">
                    <div class="settings-card">
                        <h3>General Settings</h3>
                        <form id="general-settings">
                            <div class="form-group">
                                <label>System Name</label>
                                <input type="text" name="system_name" value="PoolAI">
                            </div>
                            <div class="form-group">
                                <label>Log Level</label>
                                <select name="log_level">
                                    <option value="debug">Debug</option>
                                    <option value="info" selected>Info</option>
                                    <option value="warn">Warning</option>
                                    <option value="error">Error</option>
                                </select>
                            </div>
                        </form>
                    </div>
                    
                    <div class="settings-card">
                        <h3>GPU Settings</h3>
                        <form id="gpu-settings">
                            <div class="form-group">
                                <label>Memory Limit (MB)</label>
                                <input type="number" name="gpu_memory_limit" value="8192">
                            </div>
                            <div class="form-group">
                                <label>Temperature Limit (°C)</label>
                                <input type="number" name="gpu_temp_limit" value="85">
                            </div>
                        </form>
                    </div>
                    
                    <div class="settings-card">
                        <h3>Model Settings</h3>
                        <form id="model-settings">
                            <div class="form-group">
                                <label>Default Batch Size</label>
                                <input type="number" name="default_batch_size" value="16">
                            </div>
                            <div class="form-group">
                                <label>Max Concurrent Models</label>
                                <input type="number" name="max_concurrent_models" value="4">
                            </div>
                        </form>
                    </div>
                </div>
            </main>
        </div>
        
        <script src="/static/js/settings.js"></script>
    </body>
    </html>
    "#;
    
    Html(html.to_string())
}

/// Данные для дашборда
#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub system_status: SystemStatus,
    pub gpu_metrics: GpuMetrics,
    pub memory_metrics: MemoryMetrics,
    pub model_metrics: ModelMetrics,
    pub worker_metrics: WorkerMetrics,
}

/// Статус системы
#[derive(Debug, Clone, Serialize)]
pub struct SystemStatus {
    pub online: bool,
    pub uptime: u64,
    pub version: String,
    pub last_update: u64,
}

/// GPU метрики
#[derive(Debug, Clone, Serialize)]
pub struct GpuMetrics {
    pub usage: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: f64,
    pub power_usage: f64,
}

/// Метрики памяти
#[derive(Debug, Clone, Serialize)]
pub struct MemoryMetrics {
    pub used: u64,
    pub total: u64,
    pub available: u64,
    pub usage_percent: f64,
}

/// Метрики воркеров
#[derive(Debug, Clone, Serialize)]
pub struct WorkerMetrics {
    pub total_workers: u32,
    pub active_workers: u32,
    pub idle_workers: u32,
    pub failed_workers: u32,
} 