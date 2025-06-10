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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub alert_type: String,
    pub severity: String,
    pub condition: String,
    pub threshold: f64,
    pub cooldown: Duration,
    pub channels: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStats {
    pub total_alerts: u64,
    pub triggered_alerts: u64,
    pub resolved_alerts: u64,
    pub last_trigger_time: Option<DateTime<Utc>>,
    pub last_resolve_time: Option<DateTime<Utc>>,
    pub current_state: String,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMetrics {
    pub config: AlertConfig,
    pub stats: AlertStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub alert_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub value: f64,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

pub struct AlertSystem {
    alerts: Arc<Mutex<HashMap<String, AlertMetrics>>>,
    events: Arc<Mutex<HashMap<String, AlertEvent>>>,
}

impl AlertSystem {
    pub fn new() -> Self {
        Self {
            alerts: Arc::new(Mutex::new(HashMap::new())),
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_alert(&self, config: AlertConfig) -> Result<(), String> {
        let mut alerts = self.alerts.lock().await;
        
        if alerts.contains_key(&config.id) {
            return Err(format!("Alert '{}' already exists", config.id));
        }

        let metrics = AlertMetrics {
            config,
            stats: AlertStats {
                total_alerts: 0,
                triggered_alerts: 0,
                resolved_alerts: 0,
                last_trigger_time: None,
                last_resolve_time: None,
                current_state: "ok".to_string(),
                last_error: None,
            },
        };

        alerts.insert(metrics.config.id.clone(), metrics);
        info!("Added new alert: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_alert(&self, id: &str) -> Result<(), String> {
        let mut alerts = self.alerts.lock().await;
        let mut events = self.events.lock().await;
        
        if !alerts.contains_key(id) {
            return Err(format!("Alert '{}' not found", id));
        }

        // Remove associated events
        events.retain(|_, e| e.alert_id != id);
        
        alerts.remove(id);
        info!("Removed alert: {}", id);
        Ok(())
    }

    pub async fn check_alert(
        &self,
        alert_id: &str,
        value: f64,
        metadata: HashMap<String, String>,
    ) -> Result<Option<String>, String> {
        let mut alerts = self.alerts.lock().await;
        let mut events = self.events.lock().await;
        
        let alert = alerts
            .get_mut(alert_id)
            .ok_or_else(|| format!("Alert '{}' not found", alert_id))?;

        if !alert.config.active {
            return Err("Alert is not active".to_string());
        }

        let now = Utc::now();
        let should_trigger = self.evaluate_condition(
            value,
            &alert.config.condition,
            alert.config.threshold,
        )?;

        if should_trigger {
            if self.can_trigger_alert(alert, now) {
                let event = AlertEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    alert_id: alert_id.to_string(),
                    timestamp: now,
                    event_type: "trigger".to_string(),
                    value,
                    message: format!(
                        "Alert '{}' triggered: value {} {} {}",
                        alert.config.name, value, alert.config.condition, alert.config.threshold
                    ),
                    metadata,
                };

                events.insert(event.id.clone(), event.clone());
                alert.stats.triggered_alerts += 1;
                alert.stats.last_trigger_time = Some(now);
                alert.stats.current_state = "triggered".to_string();

                self.send_alert_notification(alert, &event).await?;

                info!(
                    "Triggered alert: {} with value: {}",
                    alert_id, value
                );
                Ok(Some(event.id))
            } else {
                Ok(None)
            }
        } else if alert.stats.current_state == "triggered" {
            let event = AlertEvent {
                id: uuid::Uuid::new_v4().to_string(),
                alert_id: alert_id.to_string(),
                timestamp: now,
                event_type: "resolve".to_string(),
                value,
                message: format!(
                    "Alert '{}' resolved: value {} {} {}",
                    alert.config.name, value, alert.config.condition, alert.config.threshold
                ),
                metadata,
            };

            events.insert(event.id.clone(), event.clone());
            alert.stats.resolved_alerts += 1;
            alert.stats.last_resolve_time = Some(now);
            alert.stats.current_state = "ok".to_string();

            self.send_alert_notification(alert, &event).await?;

            info!(
                "Resolved alert: {} with value: {}",
                alert_id, value
            );
            Ok(Some(event.id))
        } else {
            Ok(None)
        }
    }

    fn evaluate_condition(
        &self,
        value: f64,
        condition: &str,
        threshold: f64,
    ) -> Result<bool, String> {
        match condition {
            ">" => Ok(value > threshold),
            ">=" => Ok(value >= threshold),
            "<" => Ok(value < threshold),
            "<=" => Ok(value <= threshold),
            "==" => Ok(value == threshold),
            "!=" => Ok(value != threshold),
            _ => Err(format!("Invalid condition: {}", condition)),
        }
    }

    fn can_trigger_alert(&self, alert: &AlertMetrics, now: DateTime<Utc>) -> bool {
        if let Some(last_trigger) = alert.stats.last_trigger_time {
            let cooldown = chrono::Duration::from_std(alert.config.cooldown)
                .unwrap_or(chrono::Duration::seconds(0));
            now - last_trigger > cooldown
        } else {
            true
        }
    }

    async fn send_alert_notification(
        &self,
        alert: &AlertMetrics,
        event: &AlertEvent,
    ) -> Result<(), String> {
        for channel in &alert.config.channels {
            info!(
                "Sending alert notification to {}: {} (severity: {})",
                channel, event.message, alert.config.severity
            );
        }

        Ok(())
    }

    pub async fn get_alert(&self, id: &str) -> Result<AlertMetrics, String> {
        let alerts = self.alerts.lock().await;
        
        alerts
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Alert '{}' not found", id))
    }

    pub async fn get_all_alerts(&self) -> Vec<AlertMetrics> {
        let alerts = self.alerts.lock().await;
        alerts.values().cloned().collect()
    }

    pub async fn get_active_alerts(&self) -> Vec<AlertMetrics> {
        let alerts = self.alerts.lock().await;
        alerts
            .values()
            .filter(|a| a.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_events(
        &self,
        alert_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Vec<AlertEvent> {
        let events = self.events.lock().await;
        events
            .values()
            .filter(|e| {
                e.alert_id == alert_id
                    && start_time.map_or(true, |t| e.timestamp >= t)
                    && end_time.map_or(true, |t| e.timestamp <= t)
            })
            .cloned()
            .collect()
    }

    pub async fn set_alert_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut alerts = self.alerts.lock().await;
        
        let alert = alerts
            .get_mut(id)
            .ok_or_else(|| format!("Alert '{}' not found", id))?;

        alert.config.active = active;
        info!(
            "Alert '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_alert_config(&self, id: &str, new_config: AlertConfig) -> Result<(), String> {
        let mut alerts = self.alerts.lock().await;
        
        let alert = alerts
            .get_mut(id)
            .ok_or_else(|| format!("Alert '{}' not found", id))?;

        alert.config = new_config;
        info!("Updated alert configuration: {}", id);
        Ok(())
    }
} 