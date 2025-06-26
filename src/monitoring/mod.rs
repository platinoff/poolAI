pub mod metrics;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: Instant,
    pub source: String,
    pub resolved: bool,
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub overall_health: f32,
    pub active_workers: usize,
    pub total_requests: u64,
    pub error_rate: f32,
    pub average_response_time_ms: f64,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
    pub disk_usage_percent: f32,
}

#[derive(Debug, Clone)]
pub struct HistoricalData {
    pub timestamp: Instant,
    pub metrics: HashMap<String, f64>,
    pub alerts: Vec<Alert>,
}

pub struct Monitoring {
    alerts: Arc<RwLock<Vec<Alert>>>,
    metrics_collector: Arc<metrics::MetricsCollector>,
    status: Arc<RwLock<SystemStatus>>,
    historical_data: Arc<RwLock<Vec<HistoricalData>>>,
}

impl Monitoring {
    pub fn new() -> Self {
        Self {
            alerts: Arc::new(RwLock::new(Vec::new())),
            metrics_collector: Arc::new(metrics::MetricsCollector::new()),
            status: Arc::new(RwLock::new(SystemStatus {
                overall_health: 100.0,
                active_workers: 0,
                total_requests: 0,
                error_rate: 0.0,
                average_response_time_ms: 0.0,
                gpu_utilization: 0.0,
                memory_usage_mb: 0.0,
                disk_usage_percent: 0.0,
            })),
            historical_data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn collect_metrics(&self) -> Result<metrics::Metrics, AppError> {
        self.metrics_collector.collect().await
    }

    pub async fn process_alert(&self, alert: Alert) -> Result<(), AppError> {
        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
        
        // Check alert limits
        if alerts.len() > 1000 {
            alerts.retain(|a| !a.resolved);
        }
        
        Ok(())
    }

    pub async fn update_status(&self, status: SystemStatus) -> Result<(), AppError> {
        let mut current_status = self.status.write().await;
        *current_status = status;
        
        // Save historical data
        let historical_entry = HistoricalData {
            timestamp: Instant::now(),
            metrics: HashMap::new(), // Will be filled from metrics
            alerts: self.alerts.read().await.clone(),
        };
        
        let mut historical_data = self.historical_data.write().await;
        historical_data.push(historical_entry);
        
        // Limit historical data size
        if historical_data.len() > 10000 {
            historical_data.drain(0..1000);
        }
        
        Ok(())
    }

    pub async fn get_historical_data(&self, duration: Duration) -> Result<Vec<HistoricalData>, AppError> {
        let historical_data = self.historical_data.read().await;
        let cutoff_time = Instant::now() - duration;
        
        let filtered_data: Vec<HistoricalData> = historical_data
            .iter()
            .filter(|data| data.timestamp >= cutoff_time)
            .cloned()
            .collect();
        
        Ok(filtered_data)
    }

    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().await;
        alerts.iter()
            .filter(|alert| !alert.resolved)
            .cloned()
            .collect()
    }

    pub async fn resolve_alert(&self, alert_id: &str) -> Result<(), AppError> {
        let mut alerts = self.alerts.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            Ok(())
        } else {
            Err(AppError::Model(format!("Alert '{}' not found", alert_id)))
        }
    }

    pub async fn get_system_status(&self) -> SystemStatus {
        self.status.read().await.clone()
    }

    pub async fn start_monitoring(&self) -> Result<(), AppError> {
        // Start background monitoring
        let metrics_collector = self.metrics_collector.clone();
        let status = self.status.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Collect metrics
                if let Ok(metrics) = metrics_collector.collect().await {
                    // Update status based on metrics
                    let mut current_status = status.write().await;
                    current_status.gpu_utilization = metrics.gpu_utilization;
                    current_status.memory_usage_mb = metrics.memory_usage_mb;
                    current_status.average_response_time_ms = metrics.average_response_time_ms;
                    
                    // Calculate overall system health
                    let health_score = Self::calculate_health_score(&metrics);
                    current_status.overall_health = health_score;
                }
            }
        });
        
        Ok(())
    }

    fn calculate_health_score(metrics: &metrics::Metrics) -> f32 {
        let mut score: f32 = 100.0;
        
        // Penalties for various issues
        if metrics.gpu_utilization > 90.0 {
            score -= 20.0;
        }
        
        if metrics.memory_usage_mb > 8000.0 {
            score -= 15.0;
        }
        
        if metrics.average_response_time_ms > 5000.0 {
            score -= 25.0;
        }
        
        if metrics.error_rate > 0.05 {
            score -= 30.0;
        }
        
        score.max(0.0)
    }
}

// Global monitoring instance
static mut GLOBAL_MONITORING: Option<Monitoring> = None;

/// Initialize monitoring module
pub async fn initialize() -> Result<(), AppError> {
    tracing::info!("Initializing monitoring module");
    
    let monitoring = Monitoring::new();
    
    // Store global instance
    unsafe {
        GLOBAL_MONITORING = Some(monitoring);
    }
    
    // Start background monitoring
    unsafe {
        if let Some(monitoring) = &GLOBAL_MONITORING {
            monitoring.start_monitoring().await?;
        }
    }
    
    tracing::info!("Monitoring module initialized successfully");
    Ok(())
}

/// Shutdown monitoring module
pub async fn shutdown() -> Result<(), AppError> {
    tracing::info!("Shutting down monitoring module");
    
    // Cleanup global monitoring
    unsafe {
        GLOBAL_MONITORING = None;
    }
    
    tracing::info!("Monitoring module shut down successfully");
    Ok(())
}

/// Health check for monitoring module
pub async fn health_check() -> Result<(), AppError> {
    tracing::info!("Monitoring module health check");
    
    // Check if global monitoring exists
    unsafe {
        if GLOBAL_MONITORING.is_none() {
            return Err(AppError::Resource("Global monitoring not initialized".to_string()));
        }
    }
    
    tracing::info!("Monitoring module health check passed");
    Ok(())
}
