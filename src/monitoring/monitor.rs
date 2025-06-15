use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;
use cursor_codes::monitoring::alert::AlertSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub monitor_type: String,
    pub check_interval: Duration,
    pub alert_threshold: f64,
    pub alert_channels: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorStats {
    pub total_checks: u64,
    pub successful_checks: u64,
    pub failed_checks: u64,
    pub current_value: f64,
    pub last_check_time: Option<DateTime<Utc>>,
    pub last_alert_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorMetrics {
    pub config: MonitorConfig,
    pub stats: MonitorStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub id: String,
    pub monitor_id: String,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub status: String,
    pub error: Option<String>,
}

pub struct MonitorSystem {
    monitors: Arc<Mutex<HashMap<String, MonitorMetrics>>>,
    checks: Arc<Mutex<HashMap<String, Check>>>,
}

impl MonitorSystem {
    pub fn new() -> Self {
        Self {
            monitors: Arc::new(Mutex::new(HashMap::new())),
            checks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_monitor(&self, config: MonitorConfig) -> Result<(), String> {
        let mut monitors = self.monitors.lock().await;
        
        if monitors.contains_key(&config.id) {
            return Err(format!("Monitor '{}' already exists", config.id));
        }

        let metrics = MonitorMetrics {
            config,
            stats: MonitorStats {
                total_checks: 0,
                successful_checks: 0,
                failed_checks: 0,
                current_value: 0.0,
                last_check_time: None,
                last_alert_time: None,
                last_error: None,
            },
        };

        monitors.insert(metrics.config.id.clone(), metrics);
        info!("Added new monitor: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_monitor(&self, id: &str) -> Result<(), String> {
        let mut monitors = self.monitors.lock().await;
        let mut checks = self.checks.lock().await;
        
        if !monitors.contains_key(id) {
            return Err(format!("Monitor '{}' not found", id));
        }

        // Remove associated checks
        checks.retain(|_, c| c.monitor_id != id);
        
        monitors.remove(id);
        info!("Removed monitor: {}", id);
        Ok(())
    }

    pub async fn perform_check(&self, monitor_id: &str) -> Result<String, String> {
        let mut monitors = self.monitors.lock().await;
        let mut checks = self.checks.lock().await;
        
        let monitor = monitors
            .get_mut(monitor_id)
            .ok_or_else(|| format!("Monitor '{}' not found", monitor_id))?;

        if !monitor.config.active {
            return Err("Monitor is not active".to_string());
        }

        let check = Check {
            id: uuid::Uuid::new_v4().to_string(),
            monitor_id: monitor_id.to_string(),
            timestamp: Utc::now(),
            value: 0.0,
            status: "pending".to_string(),
            error: None,
        };

        checks.insert(check.id.clone(), check.clone());
        monitor.stats.total_checks += 1;
        monitor.stats.last_check_time = Some(check.timestamp);

        info!("Started check: {} for monitor: {}", check.id, monitor_id);
        Ok(check.id)
    }

    pub async fn complete_check(&self, check_id: &str, value: f64, success: bool, error: Option<String>) -> Result<(), String> {
        let mut monitors = self.monitors.lock().await;
        let mut checks = self.checks.lock().await;
        
        let check = checks
            .get_mut(check_id)
            .ok_or_else(|| format!("Check '{}' not found", check_id))?;

        let monitor = monitors
            .get_mut(&check.monitor_id)
            .ok_or_else(|| format!("Monitor '{}' not found", check.monitor_id))?;

        check.value = value;
        monitor.stats.current_value = value;

        if success {
            check.status = "success".to_string();
            monitor.stats.successful_checks += 1;
        } else {
            check.status = "failed".to_string();
            check.error = error.clone();
            monitor.stats.failed_checks += 1;
            monitor.stats.last_error = error;
        }

        if value > monitor.config.alert_threshold {
            self.send_alert(monitor, value).await?;
        }

        info!("Completed check: {} with status: {}", check_id, check.status);
        Ok(())
    }

    async fn send_alert(&self, monitor: &mut MonitorMetrics, value: f64) -> Result<(), String> {
        let now = Utc::now();
        monitor.stats.last_alert_time = Some(now);

        for channel in &monitor.config.alert_channels {
            info!(
                "Sending alert to {} for monitor: {} (value: {}, threshold: {})",
                channel, monitor.config.id, value, monitor.config.alert_threshold
            );
        }

        Ok(())
    }

    pub async fn get_monitor(&self, id: &str) -> Result<MonitorMetrics, String> {
        let monitors = self.monitors.lock().await;
        
        monitors
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Monitor '{}' not found", id))
    }

    pub async fn get_all_monitors(&self) -> Vec<MonitorMetrics> {
        let monitors = self.monitors.lock().await;
        monitors.values().cloned().collect()
    }

    pub async fn get_active_monitors(&self) -> Vec<MonitorMetrics> {
        let monitors = self.monitors.lock().await;
        monitors
            .values()
            .filter(|m| m.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_checks(&self, monitor_id: &str) -> Vec<Check> {
        let checks = self.checks.lock().await;
        checks
            .values()
            .filter(|c| c.monitor_id == monitor_id)
            .cloned()
            .collect()
    }

    pub async fn set_monitor_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut monitors = self.monitors.lock().await;
        
        let monitor = monitors
            .get_mut(id)
            .ok_or_else(|| format!("Monitor '{}' not found", id))?;

        monitor.config.active = active;
        info!(
            "Monitor '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_monitor_config(&self, id: &str, new_config: MonitorConfig) -> Result<(), String> {
        let mut monitors = self.monitors.lock().await;
        
        let monitor = monitors
            .get_mut(id)
            .ok_or_else(|| format!("Monitor '{}' not found", id))?;

        monitor.config = new_config;
        info!("Updated monitor configuration: {}", id);
        Ok(())
    }
} 