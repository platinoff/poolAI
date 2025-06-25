use crate::core::error::AppError;
use crate::monitoring::SystemStatus;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DashboardData {
    pub system_status: Option<SystemStatus>,
    pub metrics: DashboardMetrics,
    pub alerts: Vec<DashboardAlert>,
    pub recent_activity: Vec<DashboardActivity>,
}

#[derive(Debug, Clone)]
pub struct DashboardMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub active_workers: usize,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
    pub cpu_usage_percent: f32,
}

#[derive(Debug, Clone)]
pub struct DashboardAlert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: std::time::Instant,
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
pub struct DashboardActivity {
    pub id: String,
    pub activity_type: ActivityType,
    pub description: String,
    pub timestamp: std::time::Instant,
    pub user: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ActivityType {
    ModelRequest,
    WorkerStarted,
    WorkerStopped,
    AlertTriggered,
    SystemUpdate,
    UserAction,
}

pub struct Dashboard {
    state: Arc<RwLock<UiState>>,
    metrics: Arc<RwLock<DashboardMetrics>>,
    alerts: Arc<RwLock<Vec<DashboardAlert>>>,
    activities: Arc<RwLock<Vec<DashboardActivity>>>,
}

#[derive(Debug, Clone)]
struct UiState {
    current_page: String,
    user_preferences: std::collections::HashMap<String, String>,
    notifications: Vec<crate::ui::UiNotification>,
    system_status: Option<SystemStatus>,
}

impl Dashboard {
    pub async fn new(state: Arc<RwLock<UiState>>) -> Result<Self, AppError> {
        let metrics = DashboardMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            active_workers: 0,
            gpu_utilization: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        };
        
        Ok(Self {
            state,
            metrics: Arc::new(RwLock::new(metrics)),
            alerts: Arc::new(RwLock::new(Vec::new())),
            activities: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn update_status(&self, status: SystemStatus) -> Result<(), AppError> {
        // Обновление метрик на основе системного статуса
        let mut metrics = self.metrics.write().await;
        metrics.gpu_utilization = status.gpu_utilization;
        metrics.memory_usage_mb = status.memory_usage_mb;
        metrics.cpu_usage_percent = status.error_rate * 100.0; // Упрощенная логика
        
        // Добавление активности
        self.add_activity(ActivityType::SystemUpdate, "System status updated".to_string(), None).await;
        
        Ok(())
    }

    pub async fn get_data(&self) -> Result<DashboardData, AppError> {
        let system_status = {
            let state = self.state.read().await;
            state.system_status.clone()
        };
        
        let metrics = self.metrics.read().await.clone();
        let alerts = self.alerts.read().await.clone();
        let activities = self.activities.read().await.clone();
        
        // Ограничение количества активностей
        let recent_activities = activities.into_iter()
            .take(50)
            .collect();
        
        Ok(DashboardData {
            system_status,
            metrics,
            alerts,
            recent_activity: recent_activities,
        })
    }

    pub async fn add_alert(&self, alert: DashboardAlert) -> Result<(), AppError> {
        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
        
        // Ограничение количества алертов
        if alerts.len() > 100 {
            alerts.drain(0..10);
        }
        
        // Добавление активности
        self.add_activity(
            ActivityType::AlertTriggered,
            format!("Alert triggered: {}", alert.message),
            None
        ).await;
        
        Ok(())
    }

    pub async fn resolve_alert(&self, alert_id: &str) -> Result<(), AppError> {
        let mut alerts = self.alerts.write().await;
        
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            
            // Добавление активности
            self.add_activity(
                ActivityType::AlertTriggered,
                format!("Alert resolved: {}", alert.message),
                None
            ).await;
        }
        
        Ok(())
    }

    pub async fn update_metrics(&self, new_metrics: DashboardMetrics) -> Result<(), AppError> {
        let mut metrics = self.metrics.write().await;
        *metrics = new_metrics;
        
        Ok(())
    }

    pub async fn add_activity(&self, activity_type: ActivityType, description: String, user: Option<String>) {
        let activity = DashboardActivity {
            id: self.generate_activity_id(),
            activity_type,
            description,
            timestamp: std::time::Instant::now(),
            user,
        };
        
        let mut activities = self.activities.write().await;
        activities.push(activity);
        
        // Ограничение количества активностей
        if activities.len() > 1000 {
            activities.drain(0..100);
        }
    }

    pub async fn get_metrics_summary(&self) -> Result<serde_json::Value, AppError> {
        let metrics = self.metrics.read().await;
        let system_status = {
            let state = self.state.read().await;
            state.system_status.clone()
        };
        
        let summary = serde_json::json!({
            "metrics": {
                "total_requests": metrics.total_requests,
                "successful_requests": metrics.successful_requests,
                "failed_requests": metrics.failed_requests,
                "success_rate": if metrics.total_requests > 0 {
                    metrics.successful_requests as f64 / metrics.total_requests as f64
                } else {
                    0.0
                },
                "average_response_time_ms": metrics.average_response_time_ms,
                "active_workers": metrics.active_workers,
            },
            "system": {
                "gpu_utilization": metrics.gpu_utilization,
                "memory_usage_mb": metrics.memory_usage_mb,
                "cpu_usage_percent": metrics.cpu_usage_percent,
                "overall_health": system_status.map(|s| s.overall_health).unwrap_or(0.0),
            },
            "alerts": {
                "total": self.alerts.read().await.len(),
                "unresolved": self.alerts.read().await.iter().filter(|a| !a.resolved).count(),
            },
            "activities": {
                "total": self.activities.read().await.len(),
                "recent": self.activities.read().await.iter()
                    .filter(|a| a.timestamp.elapsed().as_secs() < 3600)
                    .count(),
            }
        });
        
        Ok(summary)
    }

    pub async fn get_chart_data(&self, chart_type: &str) -> Result<serde_json::Value, AppError> {
        match chart_type {
            "requests_over_time" => {
                // Заглушка для данных запросов по времени
                let data = serde_json::json!({
                    "labels": ["00:00", "01:00", "02:00", "03:00", "04:00", "05:00"],
                    "datasets": [{
                        "label": "Requests",
                        "data": [120, 150, 180, 200, 160, 140],
                        "borderColor": "rgb(75, 192, 192)",
                        "backgroundColor": "rgba(75, 192, 192, 0.2)"
                    }]
                });
                Ok(data)
            }
            "gpu_utilization" => {
                // Заглушка для данных GPU
                let data = serde_json::json!({
                    "labels": ["GPU 0", "GPU 1", "GPU 2", "GPU 3"],
                    "datasets": [{
                        "label": "Utilization %",
                        "data": [75, 45, 60, 30],
                        "backgroundColor": [
                            "rgba(255, 99, 132, 0.8)",
                            "rgba(54, 162, 235, 0.8)",
                            "rgba(255, 205, 86, 0.8)",
                            "rgba(75, 192, 192, 0.8)"
                        ]
                    }]
                });
                Ok(data)
            }
            "memory_usage" => {
                // Заглушка для данных памяти
                let data = serde_json::json!({
                    "labels": ["Used", "Available"],
                    "datasets": [{
                        "label": "Memory (GB)",
                        "data": [8.5, 5.5],
                        "backgroundColor": [
                            "rgba(255, 99, 132, 0.8)",
                            "rgba(54, 162, 235, 0.8)"
                        ]
                    }]
                });
                Ok(data)
            }
            _ => Err(AppError::InvalidParameter),
        }
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Очистка данных дашборда
        self.metrics.write().await = DashboardMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            active_workers: 0,
            gpu_utilization: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        };
        
        self.alerts.write().await.clear();
        self.activities.write().await.clear();
        
        Ok(())
    }

    fn generate_activity_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        rand::random::<u64>().hash(&mut hasher);
        
        format!("activity_{:x}", hasher.finish())
    }
} 