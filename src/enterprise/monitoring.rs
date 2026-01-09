//! Advanced monitoring module
//!
//! Provides real-time dashboards, alerts, and advanced metrics.
//!
//! # Features
//!
//! - Real-time metrics aggregation
//! - Custom dashboards
//! - Alert rules and notifications
//! - Performance analytics
//! - Time-series data storage
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::enterprise::monitoring::{MonitoringManager, AlertRule, AlertSeverity};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = MonitoringManager::new();
//! manager.initialize().await?;
//!
//! // Create alert rule
//! let rule = AlertRule {
//!     name: "high-cpu".to_string(),
//!     metric: "cpu_usage".to_string(),
//!     threshold: 90.0,
//!     severity: AlertSeverity::Warning,
//!     enabled: true,
//! };
//!
//! manager.create_alert_rule(rule).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// Alert severity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    /// Informational alerts
    Info,
    /// Warning alerts
    Warning,
    /// Error alerts
    Error,
    /// Critical alerts
    Critical,
}

impl AlertSeverity {
    /// Returns the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertSeverity::Info => "INFO",
            AlertSeverity::Warning => "WARNING",
            AlertSeverity::Error => "ERROR",
            AlertSeverity::Critical => "CRITICAL",
        }
    }
}

/// Alert rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Metric name to monitor
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator (">", "<", ">=", "<=", "==")
    pub operator: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Whether rule is enabled
    pub enabled: bool,
    /// Optional tenant ID for tenant-specific alerts
    pub tenant_id: Option<Uuid>,
}

impl Default for AlertRule {
    fn default() -> Self {
        Self {
            name: String::new(),
            metric: String::new(),
            threshold: 0.0,
            operator: ">".to_string(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        }
    }
}

/// Alert instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: Uuid,
    /// Alert rule name
    pub rule_name: String,
    /// Metric name
    pub metric: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold: f64,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Timestamp when alert was triggered
    pub triggered_at: DateTime<Utc>,
    /// Whether alert is acknowledged
    pub acknowledged: bool,
    /// Optional tenant ID
    pub tenant_id: Option<Uuid>,
}

/// Metric data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Metric name
    pub metric: String,
    /// Metric value
    pub value: f64,
    /// Optional tags (key-value pairs)
    pub tags: HashMap<String, String>,
    /// Optional tenant ID
    pub tenant_id: Option<Uuid>,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    /// Dashboard ID
    pub id: Uuid,
    /// Dashboard name
    pub name: String,
    /// Dashboard description
    pub description: String,
    /// Metrics to display
    pub metrics: Vec<String>,
    /// Dashboard layout (JSON string for flexibility)
    pub layout: String,
    /// Whether dashboard is public
    pub is_public: bool,
    /// Optional tenant ID for tenant-specific dashboards
    pub tenant_id: Option<Uuid>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Monitoring manager
///
/// Manages advanced monitoring, dashboards, and alerts.
pub struct MonitoringManager {
    alert_rules: Arc<RwLock<HashMap<String, AlertRule>>>,
    active_alerts: Arc<RwLock<HashMap<Uuid, Alert>>>,
    metrics_history: Arc<RwLock<Vec<MetricDataPoint>>>,
    dashboards: Arc<RwLock<HashMap<Uuid, Dashboard>>>,
    initialized: Arc<RwLock<bool>>,
}

impl MonitoringManager {
    /// Creates a new monitoring manager
    pub fn new() -> Self {
        Self {
            alert_rules: Arc::new(RwLock::new(HashMap::new())),
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            dashboards: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the monitoring manager
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // TODO: Initialize metrics aggregation
        // TODO: Initialize dashboard storage
        // TODO: Initialize alert rules engine

        *initialized = true;
        info!("Monitoring manager initialized");
        Ok(())
    }

    /// Records a metric data point
    ///
    /// Stores the metric and checks alert rules.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if recording fails.
    pub async fn record_metric(&self, data_point: MetricDataPoint) -> Result<(), AppError> {
        // Store metric in history (keep last 10000 points)
        let mut history = self.metrics_history.write().await;
        history.push(data_point.clone());
        if history.len() > 10000 {
            history.remove(0);
        }
        drop(history);

        // Check alert rules
        self.check_alert_rules(&data_point).await?;

        Ok(())
    }

    /// Checks alert rules against a metric data point
    async fn check_alert_rules(&self, data_point: &MetricDataPoint) -> Result<(), AppError> {
        let rules = self.alert_rules.read().await;

        for (rule_name, rule) in rules.iter() {
            if !rule.enabled {
                continue;
            }

            // Check if metric matches
            if rule.metric != data_point.metric {
                continue;
            }

            // Check tenant match (if specified)
            if let Some(rule_tenant) = rule.tenant_id {
                if data_point.tenant_id != Some(rule_tenant) {
                    continue;
                }
            }

            // Check threshold
            let triggered = match rule.operator.as_str() {
                ">" => data_point.value > rule.threshold,
                "<" => data_point.value < rule.threshold,
                ">=" => data_point.value >= rule.threshold,
                "<=" => data_point.value <= rule.threshold,
                "==" => (data_point.value - rule.threshold).abs() < 0.001,
                _ => {
                    warn!("Unknown alert operator: {}", rule.operator);
                    continue;
                }
            };

            if triggered {
                // Create alert
                let alert = Alert {
                    id: Uuid::new_v4(),
                    rule_name: rule_name.clone(),
                    metric: data_point.metric.clone(),
                    current_value: data_point.value,
                    threshold: rule.threshold,
                    severity: rule.severity,
                    triggered_at: Utc::now(),
                    acknowledged: false,
                    tenant_id: data_point.tenant_id,
                };

                let mut alerts = self.active_alerts.write().await;
                alerts.insert(alert.id, alert.clone());

                info!(
                    "Alert triggered: {} (metric={}, value={}, threshold={}, severity={})",
                    rule_name,
                    data_point.metric,
                    data_point.value,
                    rule.threshold,
                    rule.severity.as_str()
                );
            }
        }

        Ok(())
    }

    /// Creates an alert rule
    ///
    /// # Errors
    ///
    /// Returns `AppError` if rule creation fails.
    pub async fn create_alert_rule(&self, rule: AlertRule) -> Result<(), AppError> {
        if rule.name.is_empty() {
            return Err(AppError::ValidationError(
                "Alert rule name cannot be empty".to_string(),
            ));
        }

        if rule.metric.is_empty() {
            return Err(AppError::ValidationError(
                "Alert rule metric cannot be empty".to_string(),
            ));
        }

        let valid_operators = [">", "<", ">=", "<=", "=="];
        if !valid_operators.contains(&rule.operator.as_str()) {
            return Err(AppError::ValidationError(format!(
                "Invalid alert operator: {}. Valid operators are: {:?}. \
                Context: Alert rule operator must be one of the supported comparison operators. \
                Suggestion: Use '>' for greater than, '<' for less than, '>=' for greater or equal, '<=' for less or equal, '==' for equal.",
                rule.operator, valid_operators
            )));
        }

        let mut rules = self.alert_rules.write().await;
        rules.insert(rule.name.clone(), rule.clone());

        info!("Created alert rule: {}", rule.name);
        Ok(())
    }

    /// Gets an alert rule
    pub async fn get_alert_rule(&self, name: &str) -> Result<Option<AlertRule>, AppError> {
        let rules = self.alert_rules.read().await;
        Ok(rules.get(name).cloned())
    }

    /// Lists all alert rules
    pub async fn list_alert_rules(&self) -> Result<Vec<AlertRule>, AppError> {
        let rules = self.alert_rules.read().await;
        Ok(rules.values().cloned().collect())
    }

    /// Gets active alerts
    ///
    /// # Arguments
    ///
    /// * `severity` - Optional severity filter
    /// * `tenant_id` - Optional tenant ID filter
    /// * `acknowledged` - Optional acknowledged filter
    pub async fn get_active_alerts(
        &self,
        severity: Option<AlertSeverity>,
        tenant_id: Option<Uuid>,
        acknowledged: Option<bool>,
    ) -> Result<Vec<Alert>, AppError> {
        let alerts = self.active_alerts.read().await;
        let mut filtered: Vec<Alert> = alerts
            .values()
            .filter(|alert| {
                if let Some(sev) = severity {
                    if alert.severity != sev {
                        return false;
                    }
                }
                if let Some(tid) = tenant_id {
                    if alert.tenant_id != Some(tid) {
                        return false;
                    }
                }
                if let Some(ack) = acknowledged {
                    if alert.acknowledged != ack {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by triggered_at (newest first)
        filtered.sort_by(|a, b| b.triggered_at.cmp(&a.triggered_at));
        Ok(filtered)
    }

    /// Acknowledges an alert
    ///
    /// # Errors
    ///
    /// Returns `AppError` if alert is not found.
    pub async fn acknowledge_alert(&self, alert_id: Uuid) -> Result<(), AppError> {
        let mut alerts = self.active_alerts.write().await;
        let alert = alerts.get_mut(&alert_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Alert not found: {}. \
                Context: Cannot acknowledge non-existent alert. \
                Suggestion: Check alert ID.",
                alert_id
            ))
        })?;

        alert.acknowledged = true;
        info!("Acknowledged alert: {}", alert_id);
        Ok(())
    }

    /// Creates a dashboard
    ///
    /// # Errors
    ///
    /// Returns `AppError` if dashboard creation fails.
    pub async fn create_dashboard(&self, dashboard: Dashboard) -> Result<(), AppError> {
        if dashboard.name.is_empty() {
            return Err(AppError::ValidationError(
                "Dashboard name cannot be empty".to_string(),
            ));
        }

        let mut dashboards = self.dashboards.write().await;
        dashboards.insert(dashboard.id, dashboard.clone());

        info!("Created dashboard: {} ({})", dashboard.name, dashboard.id);
        Ok(())
    }

    /// Gets a dashboard
    pub async fn get_dashboard(&self, id: Uuid) -> Result<Option<Dashboard>, AppError> {
        let dashboards = self.dashboards.read().await;
        Ok(dashboards.get(&id).cloned())
    }

    /// Lists all dashboards
    ///
    /// # Arguments
    ///
    /// * `tenant_id` - Optional tenant ID filter
    pub async fn list_dashboards(
        &self,
        tenant_id: Option<Uuid>,
    ) -> Result<Vec<Dashboard>, AppError> {
        let dashboards = self.dashboards.read().await;
        let filtered: Vec<Dashboard> = dashboards
            .values()
            .filter(|dashboard| {
                if let Some(tid) = tenant_id {
                    if dashboard.tenant_id != Some(tid) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        Ok(filtered)
    }

    /// Gets metric history
    ///
    /// # Arguments
    ///
    /// * `metric` - Metric name filter
    /// * `start_time` - Optional start time filter
    /// * `end_time` - Optional end time filter
    /// * `tenant_id` - Optional tenant ID filter
    /// * `limit` - Maximum number of results
    pub async fn get_metric_history(
        &self,
        metric: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        tenant_id: Option<Uuid>,
        limit: Option<usize>,
    ) -> Result<Vec<MetricDataPoint>, AppError> {
        let history = self.metrics_history.read().await;
        let mut filtered: Vec<MetricDataPoint> = history
            .iter()
            .filter(|point| {
                if let Some(m) = metric {
                    if point.metric != m {
                        return false;
                    }
                }
                if let Some(start) = start_time {
                    if point.timestamp < start {
                        return false;
                    }
                }
                if let Some(end) = end_time {
                    if point.timestamp > end {
                        return false;
                    }
                }
                if let Some(tid) = tenant_id {
                    if point.tenant_id != Some(tid) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by timestamp (newest first)
        filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = limit {
            filtered.truncate(limit);
        }

        Ok(filtered)
    }

    /// Shuts down the monitoring manager
    pub async fn shutdown(&self) -> Result<(), AppError> {
        *self.initialized.write().await = false;
        info!("Monitoring manager shut down");
        Ok(())
    }
}

impl Default for MonitoringManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_manager_initialization() {
        let manager = MonitoringManager::new();
        assert!(manager.initialize().await.is_ok());
        assert!(manager.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_alert_rule() {
        let manager = MonitoringManager::new();
        manager.initialize().await.unwrap();

        let rule = AlertRule {
            name: "high-cpu".to_string(),
            metric: "cpu_usage".to_string(),
            threshold: 90.0,
            operator: ">".to_string(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        };

        assert!(manager.create_alert_rule(rule).await.is_ok());
    }

    #[tokio::test]
    async fn test_record_metric_and_trigger_alert() {
        let manager = MonitoringManager::new();
        manager.initialize().await.unwrap();

        // Create alert rule
        let rule = AlertRule {
            name: "high-cpu".to_string(),
            metric: "cpu_usage".to_string(),
            threshold: 90.0,
            operator: ">".to_string(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        };

        manager.create_alert_rule(rule).await.unwrap();

        // Record metric that triggers alert
        let data_point = MetricDataPoint {
            timestamp: Utc::now(),
            metric: "cpu_usage".to_string(),
            value: 95.0,
            tags: HashMap::new(),
            tenant_id: None,
        };

        manager.record_metric(data_point).await.unwrap();

        // Check for active alerts
        let alerts = manager.get_active_alerts(None, None, None).await.unwrap();
        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].metric, "cpu_usage");
        assert_eq!(alerts[0].current_value, 95.0);
    }

    #[tokio::test]
    async fn test_create_dashboard() {
        let manager = MonitoringManager::new();
        manager.initialize().await.unwrap();

        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name: "test-dashboard".to_string(),
            description: "Test dashboard".to_string(),
            metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
            layout: "{}".to_string(),
            is_public: false,
            tenant_id: None,
            created_at: Utc::now(),
        };

        assert!(manager.create_dashboard(dashboard).await.is_ok());
    }
}

/// Global monitoring manager instance
static MONITORING_MANAGER: OnceLock<Arc<MonitoringManager>> = OnceLock::new();

/// Get global monitoring manager instance.
///
/// This function returns a singleton instance of `MonitoringManager` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
///
/// # Examples
///
/// ```no_run
/// use poolai::enterprise::monitoring::get_global_monitoring_manager;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let manager = get_global_monitoring_manager();
/// manager.initialize().await?;
///
/// // Get active alerts
/// let alerts = manager.get_active_alerts(None, None, None).await?;
/// for alert in alerts {
///     println!("Alert: {} - {}", alert.metric, alert.current_value);
/// }
/// # Ok(())
/// # }
/// ```
pub fn get_global_monitoring_manager() -> Arc<MonitoringManager> {
    MONITORING_MANAGER
        .get_or_init(|| Arc::new(MonitoringManager::new()))
        .clone()
}
