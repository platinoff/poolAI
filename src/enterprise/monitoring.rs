//! Advanced monitoring module
//!
//! Provides real-time dashboards, alerts, and advanced metrics.
//!
//! Monitoring depth (phase E band 91+): `POOLAI_MONITORING_DATA_DIR` enables SQLite
//! (`monitoring.db`). See [`docs/development/MONITORING_DEPTH.md`].
//! Store wire (band 92): `POOLAI_MONITORING_STORE` + `monitoring_store_wire()` —
//! [`docs/development/MONITORING_STORE.md`].
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
//!     operator: ">".to_string(),
//!     severity: AlertSeverity::Warning,
//!     enabled: true,
//!     tenant_id: None,
//! };
//!
//! manager.create_alert_rule(rule).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// SQLite file under `POOLAI_MONITORING_DATA_DIR` (FM-030).
pub const MONITORING_DB_FILE: &str = "monitoring.db";

/// Env key for durable monitoring SQLite directory (band 91 scaffold / band 92 wire).
pub const POOLAI_MONITORING_DATA_DIR: &str = "POOLAI_MONITORING_DATA_DIR";

/// Env key for monitoring store backend mode (band 92 wire).
pub const POOLAI_MONITORING_STORE: &str = "POOLAI_MONITORING_STORE";

/// Canonical monitoring store modes (band 91 scaffold).
pub const MONITORING_STORE_MODES: &[&str] = &["memory", "sqlite"];

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

/// Resolve configured monitoring store mode (PH-S1550; durable wire PH-S1560).
///
/// Prefer explicit `POOLAI_MONITORING_STORE`; else a set data dir implies `sqlite`
/// (band 91+ compatibility).
pub fn monitoring_store_mode() -> &'static str {
    match std::env::var(POOLAI_MONITORING_STORE)
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "sqlite" => "sqlite",
        "memory" => "memory",
        _ => {
            if data_dir_from_env().is_some() {
                "sqlite"
            } else {
                "memory"
            }
        }
    }
}

/// Optional durable data directory as [`PathBuf`] (PH-S1560).
pub fn monitoring_store_data_dir_from_env() -> Option<PathBuf> {
    data_dir_from_env().map(PathBuf::from)
}

/// Monitoring store wire snapshot (mode + durable path; CRUD via DATA_DIR already).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitoringStoreWire {
    /// `memory` or `sqlite`.
    pub mode: String,
    /// Resolved sqlite file path when mode is sqlite and data dir is set.
    pub durable_path: Option<String>,
    /// True when sqlite mode has a durable path.
    pub configured: bool,
}

/// Resolve monitoring store wire for ops / verify / contracts (PH-S1560).
pub fn monitoring_store_wire() -> MonitoringStoreWire {
    let mode = monitoring_store_mode().to_string();
    if mode != "sqlite" {
        return MonitoringStoreWire {
            mode,
            durable_path: None,
            configured: false,
        };
    }
    let durable_path = monitoring_store_data_dir_from_env().map(|dir| {
        dir.join(MONITORING_DB_FILE)
            .to_string_lossy()
            .replace('\\', "/")
    });
    let configured = durable_path.is_some();
    MonitoringStoreWire {
        mode,
        durable_path,
        configured,
    }
}

/// Wire label for admin / metrics depth fields (PH-S1560).
pub fn monitoring_store_wire_label(wire: &MonitoringStoreWire) -> &'static str {
    if wire.mode == "sqlite" && wire.configured {
        "sqlite"
    } else if wire.mode == "sqlite" {
        "sqlite_unconfigured"
    } else {
        "memory"
    }
}

/// Production verify stub for alert rule fields (PH-S1550).
pub fn validate_monitoring_alert_fields(rule: &AlertRule) -> Result<(), AppError> {
    if rule.name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Alert rule name cannot be empty".to_string(),
        ));
    }
    if rule.metric.trim().is_empty() {
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
    Ok(())
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
/// Supports persistent storage for metrics history (SQLite, optional).
pub struct MonitoringManager {
    alert_rules: Arc<RwLock<HashMap<String, AlertRule>>>,
    active_alerts: Arc<RwLock<HashMap<Uuid, Alert>>>,
    metrics_history: Arc<RwLock<Vec<MetricDataPoint>>>, // In-memory cache (last 1000 points)
    dashboards: Arc<RwLock<HashMap<Uuid, Dashboard>>>,
    initialized: Arc<RwLock<bool>>,
    /// SQLite database path for metrics persistence (None = in-memory only)
    db_path: Option<String>,
}

impl MonitoringManager {
    /// Creates a new monitoring manager
    ///
    /// Metrics are stored in-memory by default. For persistent storage,
    /// use `new_with_persistence()` or configure persistence in `initialize()`.
    pub fn new() -> Self {
        Self::new_with_persistence(None)
    }

    /// Coordinator persistence when `POOLAI_MONITORING_DATA_DIR` is set (e.g. `data/monitoring`).
    pub fn new_from_env() -> Self {
        Self::new_with_persistence(db_path_from_env())
    }

    /// Creates a new monitoring manager with SQLite persistence
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to SQLite database file (None = in-memory)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::enterprise::monitoring::MonitoringManager;
    ///
    /// // Create with file-based persistence
    /// let manager = MonitoringManager::new_with_persistence(Some("./data/metrics.db".to_string()));
    /// ```
    pub fn new_with_persistence(db_path: Option<String>) -> Self {
        Self {
            alert_rules: Arc::new(RwLock::new(HashMap::new())),
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            dashboards: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
            db_path,
        }
    }

    /// Initialize SQLite database schema
    fn init_database_schema(conn: &Connection) -> SqliteResult<()> {
        // Create metrics_history table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS metrics_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                metric TEXT NOT NULL,
                value REAL NOT NULL,
                tags TEXT,
                tenant_id TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Create indexes for efficient queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics_history(timestamp)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_metric ON metrics_history(metric)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_metrics_tenant ON metrics_history(tenant_id)",
            [],
        )?;

        // Create dashboards table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dashboards (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                metrics TEXT NOT NULL,
                layout TEXT NOT NULL,
                is_public INTEGER NOT NULL DEFAULT 0,
                tenant_id TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // Create alert_rules table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS alert_rules (
                name TEXT PRIMARY KEY,
                metric TEXT NOT NULL,
                threshold REAL NOT NULL,
                operator TEXT NOT NULL,
                severity TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                tenant_id TEXT
            )",
            [],
        )?;

        Ok(())
    }

    fn load_persisted_config(
        conn: &Connection,
    ) -> Result<(HashMap<String, AlertRule>, HashMap<Uuid, Dashboard>), String> {
        let mut rules = HashMap::new();
        let mut rule_stmt = conn
            .prepare(
                "SELECT name, metric, threshold, operator, severity, enabled, tenant_id FROM alert_rules",
            )
            .map_err(|e| format!("prepare alert_rules: {e}"))?;
        let rule_rows = rule_stmt
            .query_map([], |row| {
                let name: String = row.get(0)?;
                let metric: String = row.get(1)?;
                let threshold: f64 = row.get(2)?;
                let operator: String = row.get(3)?;
                let severity_str: String = row.get(4)?;
                let enabled: i64 = row.get(5)?;
                let tenant_id_str: Option<String> = row.get(6)?;
                Ok(AlertRule {
                    name,
                    metric,
                    threshold,
                    operator,
                    severity: severity_from_str(&severity_str),
                    enabled: enabled != 0,
                    tenant_id: tenant_id_str.and_then(|s| Uuid::parse_str(&s).ok()),
                })
            })
            .map_err(|e| format!("query alert_rules: {e}"))?;
        for row in rule_rows {
            let rule = row.map_err(|e| format!("read alert_rule row: {e}"))?;
            rules.insert(rule.name.clone(), rule);
        }

        let mut dashboards = HashMap::new();
        let mut dash_stmt = conn
            .prepare(
                "SELECT id, name, description, metrics, layout, is_public, tenant_id, created_at FROM dashboards",
            )
            .map_err(|e| format!("prepare dashboards: {e}"))?;
        let dash_rows = dash_stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let name: String = row.get(1)?;
                let description: String = row.get(2)?;
                let metrics_json: String = row.get(3)?;
                let layout: String = row.get(4)?;
                let is_public: i64 = row.get(5)?;
                let tenant_id_str: Option<String> = row.get(6)?;
                let created_at_str: String = row.get(7)?;
                let id = Uuid::parse_str(&id_str).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
                })?;
                let metrics: Vec<String> = serde_json::from_str(&metrics_json).unwrap_or_default();
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            7,
                            "created_at".into(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .with_timezone(&Utc);
                Ok(Dashboard {
                    id,
                    name,
                    description,
                    metrics,
                    layout,
                    is_public: is_public != 0,
                    tenant_id: tenant_id_str.and_then(|s| Uuid::parse_str(&s).ok()),
                    created_at,
                })
            })
            .map_err(|e| format!("query dashboards: {e}"))?;
        for row in dash_rows {
            let dashboard = row.map_err(|e| format!("read dashboard row: {e}"))?;
            dashboards.insert(dashboard.id, dashboard);
        }

        Ok((rules, dashboards))
    }

    fn upsert_alert_rule_conn(conn: &Connection, rule: &AlertRule) -> Result<(), String> {
        let tenant_id_str = rule.tenant_id.map(|id| id.to_string());
        conn.execute(
            "INSERT OR REPLACE INTO alert_rules (name, metric, threshold, operator, severity, enabled, tenant_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &rule.name,
                &rule.metric,
                rule.threshold,
                &rule.operator,
                rule.severity.as_str(),
                i64::from(rule.enabled),
                tenant_id_str,
            ],
        )
        .map_err(|e| format!("upsert alert_rule {}: {e}", rule.name))?;
        Ok(())
    }

    fn upsert_dashboard_conn(conn: &Connection, dashboard: &Dashboard) -> Result<(), String> {
        let metrics_json = serde_json::to_string(&dashboard.metrics)
            .map_err(|e| format!("serialize metrics: {e}"))?;
        let tenant_id_str = dashboard.tenant_id.map(|id| id.to_string());
        conn.execute(
            "INSERT OR REPLACE INTO dashboards (id, name, description, metrics, layout, is_public, tenant_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                dashboard.id.to_string(),
                &dashboard.name,
                &dashboard.description,
                metrics_json,
                &dashboard.layout,
                i64::from(dashboard.is_public),
                tenant_id_str,
                dashboard.created_at.to_rfc3339(),
            ],
        )
        .map_err(|e| format!("upsert dashboard {}: {e}", dashboard.id))?;
        Ok(())
    }

    fn delete_dashboard_conn(conn: &Connection, id: Uuid) -> Result<(), String> {
        conn.execute(
            "DELETE FROM dashboards WHERE id = ?1",
            params![id.to_string()],
        )
        .map_err(|e| format!("delete dashboard {id}: {e}"))?;
        Ok(())
    }

    /// Initializes the monitoring manager
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Initialize SQLite database if db_path is configured
        if let Some(ref db_path) = self.db_path {
            // Ensure parent directory exists
            if let Some(parent) = Path::new(db_path).parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    AppError::ConfigError(format!("Failed to create database directory: {}", e))
                })?;
            }

            let db_path_clone = db_path.clone();
            let loaded = tokio::task::spawn_blocking(move || -> Result<
                (HashMap<String, AlertRule>, HashMap<Uuid, Dashboard>),
                AppError,
            > {
                let conn = Connection::open(&db_path_clone).map_err(|e| {
                    AppError::ConfigError(format!("Failed to open SQLite database: {}", e))
                })?;
                Self::init_database_schema(&conn).map_err(|e| {
                    AppError::ConfigError(format!("Failed to initialize database schema: {}", e))
                })?;
                Self::load_persisted_config(&conn).map_err(|e| {
                    AppError::ConfigError(format!("Failed to load monitoring config: {}", e))
                })
            })
            .await
            .map_err(|e| {
                AppError::ConfigError(format!("Database initialization task failed: {}", e))
            })??;

            *self.alert_rules.write().await = loaded.0;
            *self.dashboards.write().await = loaded.1;

            info!("SQLite database initialized at: {}", db_path);
        }

        *initialized = true;
        info!(
            "Monitoring manager initialized (persistence: {})",
            if self.db_path.is_some() {
                "enabled"
            } else {
                "in-memory only"
            }
        );
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
        // Store metric in in-memory history (keep last 1000 points for fast access)
        let mut history = self.metrics_history.write().await;
        history.push(data_point.clone());
        if history.len() > 1000 {
            history.remove(0);
        }
        drop(history);

        // Persist to SQLite database if db_path is configured
        if let Some(ref db_path) = self.db_path {
            let data_point_clone = data_point.clone();
            let history_len = self.metrics_history.read().await.len();
            let db_path_clone = db_path.clone();

            // Use spawn_blocking for database operations — must await so callers/tests
            // see a consistent SQLite view before the next read/query.
            if let Err(join_err) = tokio::task::spawn_blocking(move || {
                let conn = match Connection::open(&db_path_clone) {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("Failed to open database connection: {}", e);
                        return;
                    }
                };

                // Serialize tags to JSON
                let tags_json = serde_json::to_string(&data_point_clone.tags)
                    .unwrap_or_else(|_| "{}".to_string());

                // Insert metric into database
                let tenant_id_str = data_point_clone.tenant_id.map(|id| id.to_string());
                let timestamp_str = data_point_clone.timestamp.to_rfc3339();

                if let Err(e) = conn.execute(
                    "INSERT INTO metrics_history (timestamp, metric, value, tags, tenant_id) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        timestamp_str,
                        &data_point_clone.metric,
                        data_point_clone.value,
                        tags_json,
                        tenant_id_str
                    ],
                ) {
                    warn!("Failed to persist metric to database: {}", e);
                } else {
                    // Cleanup old metrics (older than 30 days) periodically
                    // Only cleanup every 1000th insert to avoid performance impact
                    if history_len % 1000 == 0 {
                        let cutoff = (Utc::now() - chrono::Duration::days(30)).to_rfc3339();
                        if let Err(e) = conn.execute(
                            "DELETE FROM metrics_history WHERE timestamp < ?1",
                            params![cutoff],
                        ) {
                            warn!("Failed to cleanup old metrics: {}", e);
                        }
                    }
                }
            })
            .await
            {
                warn!("SQLite persistence task join error: {}", join_err);
            }
        }

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
        validate_monitoring_alert_fields(&rule)?;

        let mut rules = self.alert_rules.write().await;
        rules.insert(rule.name.clone(), rule.clone());
        drop(rules);

        if let Some(ref db_path) = self.db_path {
            let db_path = db_path.clone();
            let rule = rule.clone();
            match tokio::task::spawn_blocking(move || {
                let conn = Connection::open(&db_path).map_err(|e| format!("open db: {e}"))?;
                Self::upsert_alert_rule_conn(&conn, &rule)
            })
            .await
            {
                Ok(Ok(())) => {}
                Ok(Err(e)) => warn!("Failed to persist alert rule: {}", e),
                Err(join_err) => warn!("alert_rule persistence join error: {}", join_err),
            }
        }

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
        drop(dashboards);

        if let Some(ref db_path) = self.db_path {
            let db_path = db_path.clone();
            let dashboard = dashboard.clone();
            match tokio::task::spawn_blocking(move || {
                let conn = Connection::open(&db_path).map_err(|e| format!("open db: {e}"))?;
                Self::upsert_dashboard_conn(&conn, &dashboard)
            })
            .await
            {
                Ok(Ok(())) => {}
                Ok(Err(e)) => warn!("Failed to persist dashboard: {}", e),
                Err(join_err) => warn!("dashboard persistence join error: {}", join_err),
            }
        }

        info!("Created dashboard: {} ({})", dashboard.name, dashboard.id);
        Ok(())
    }

    /// Gets a dashboard
    pub async fn get_dashboard(&self, id: Uuid) -> Result<Option<Dashboard>, AppError> {
        let dashboards = self.dashboards.read().await;
        Ok(dashboards.get(&id).cloned())
    }

    /// Deletes a dashboard by id.
    ///
    /// Returns `Ok(true)` if a dashboard was removed, `Ok(false)` if not found.
    pub async fn delete_dashboard(&self, id: Uuid) -> Result<bool, AppError> {
        let mut dashboards = self.dashboards.write().await;
        let removed = dashboards.remove(&id).is_some();
        drop(dashboards);

        if removed {
            if let Some(ref db_path) = self.db_path {
                let db_path = db_path.clone();
                match tokio::task::spawn_blocking(move || {
                    let conn = Connection::open(&db_path).map_err(|e| format!("open db: {e}"))?;
                    Self::delete_dashboard_conn(&conn, id)
                })
                .await
                {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => warn!("Failed to delete dashboard from db: {}", e),
                    Err(join_err) => warn!("dashboard delete join error: {}", join_err),
                }
            }
            info!("Deleted dashboard: {}", id);
            Ok(true)
        } else {
            Ok(false)
        }
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
        // Query from SQLite database if db_path is configured
        if let Some(ref db_path) = self.db_path {
            let metric_clone = metric.map(|s| s.to_string());
            let start_time_clone = start_time.map(|t| t.to_rfc3339());
            let end_time_clone = end_time.map(|t| t.to_rfc3339());
            let tenant_id_clone = tenant_id.map(|id| id.to_string());
            let limit_clone = limit;
            let db_path_clone = db_path.clone();

            // Use spawn_blocking for database operations
            let results = tokio::task::spawn_blocking(move || {
                let conn = match Connection::open(&db_path_clone) {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("Failed to open database connection: {}", e);
                        return Ok::<Vec<MetricDataPoint>, AppError>(Vec::new());
                    }
                };

                // Build SQL query with filters
                let mut query = "SELECT timestamp, metric, value, tags, tenant_id FROM metrics_history WHERE 1=1".to_string();
                let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

                if let Some(ref m) = metric_clone {
                    query.push_str(" AND metric = ?");
                    params_vec.push(Box::new(m.clone()));
                }

                if let Some(ref start) = start_time_clone {
                    query.push_str(" AND timestamp >= ?");
                    params_vec.push(Box::new(start.clone()));
                }

                if let Some(ref end) = end_time_clone {
                    query.push_str(" AND timestamp <= ?");
                    params_vec.push(Box::new(end.clone()));
                }

                if let Some(ref tid) = tenant_id_clone {
                    query.push_str(" AND tenant_id = ?");
                    params_vec.push(Box::new(tid.clone()));
                }

                query.push_str(" ORDER BY timestamp DESC");

                if let Some(l) = limit_clone {
                    query.push_str(&format!(" LIMIT {}", l));
                }

                // Execute query
                let mut stmt = match conn.prepare(&query) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(AppError::ConfigError(format!("Failed to prepare query: {}", e)));
                    }
                };

                let rows = match stmt.query_map(
                    rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
                    |row| {
                        let timestamp_str: String = row.get(0)?;
                        let metric: String = row.get(1)?;
                        let value: f64 = row.get(2)?;
                        let tags_json: String = row.get(3)?;
                        let tenant_id_str: Option<String> = row.get(4)?;

                        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                            .map_err(|_| rusqlite::Error::InvalidColumnType(0, "timestamp".to_string(), rusqlite::types::Type::Text))?
                            .with_timezone(&Utc);

                        let tags: HashMap<String, String> = serde_json::from_str(&tags_json)
                            .unwrap_or_else(|_| HashMap::new());

                        let tenant_id = tenant_id_str.and_then(|s| Uuid::parse_str(&s).ok());

                        Ok(MetricDataPoint {
                            timestamp,
                            metric,
                            value,
                            tags,
                            tenant_id,
                        })
                    },
                ) {
                    Ok(r) => r,
                    Err(e) => {
                        return Err(AppError::ConfigError(format!("Failed to execute query: {}", e)));
                    }
                };

                let mut results = Vec::new();
                for row_result in rows {
                    match row_result {
                        Ok(point) => results.push(point),
                        Err(e) => {
                            warn!("Failed to parse metric row: {}", e);
                        }
                    }
                }

                Ok(results)
            })
            .await
            .map_err(|e| AppError::ConfigError(format!("Database query task failed: {}", e)))??;

            return Ok(results);
        }

        // Fallback to in-memory history if database is not available
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

/// Global monitoring manager instance
static MONITORING_MANAGER: OnceLock<Arc<MonitoringManager>> = OnceLock::new();

/// Align `get_global_monitoring_manager` with [`crate::core::state::AppState::enterprise_monitoring_manager`].
pub fn try_install_global_monitoring_manager(mgr: Arc<MonitoringManager>) {
    let _ = MONITORING_MANAGER.set(mgr);
}

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
        .get_or_init(|| Arc::new(MonitoringManager::new_from_env()))
        .clone()
}

/// Directory for SQLite persistence (`POOLAI_MONITORING_DATA_DIR`).
pub fn data_dir_from_env() -> Option<String> {
    std::env::var(POOLAI_MONITORING_DATA_DIR)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

/// Full path to `monitoring.db` when env is set.
pub fn db_path_from_env() -> Option<String> {
    data_dir_from_env().map(|dir| {
        Path::new(&dir)
            .join(MONITORING_DB_FILE)
            .to_string_lossy()
            .into_owned()
    })
}

fn severity_from_str(s: &str) -> AlertSeverity {
    match s {
        "INFO" => AlertSeverity::Info,
        "WARNING" => AlertSeverity::Warning,
        "ERROR" => AlertSeverity::Error,
        "CRITICAL" => AlertSeverity::Critical,
        _ => AlertSeverity::Warning,
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

    #[tokio::test]
    async fn test_sqlite_persists_dashboards_and_rules_across_init() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let db_path = tmp
            .path()
            .join(MONITORING_DB_FILE)
            .to_string_lossy()
            .into_owned();

        let rule = AlertRule {
            name: "high-mem".to_string(),
            metric: "memory_usage".to_string(),
            threshold: 90.0,
            operator: ">".to_string(),
            severity: AlertSeverity::Critical,
            enabled: true,
            tenant_id: None,
        };
        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name: "ops".to_string(),
            description: "ops board".to_string(),
            metrics: vec!["cpu_usage".to_string()],
            layout: "{}".to_string(),
            is_public: true,
            tenant_id: None,
            created_at: Utc::now(),
        };

        {
            let manager = MonitoringManager::new_with_persistence(Some(db_path.clone()));
            manager.initialize().await.expect("init");
            manager.create_alert_rule(rule.clone()).await.expect("rule");
            manager
                .create_dashboard(dashboard.clone())
                .await
                .expect("dashboard");
        }

        let reloaded = MonitoringManager::new_with_persistence(Some(db_path));
        reloaded.initialize().await.expect("reinit");
        let rules = reloaded.list_alert_rules().await.expect("rules");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "high-mem");
        let dashboards = reloaded.list_dashboards(None).await.expect("dashboards");
        assert_eq!(dashboards.len(), 1);
        assert_eq!(dashboards[0].id, dashboard.id);
    }

    #[test]
    fn monitoring_store_mode_defaults_memory_ph_s1550() {
        std::env::remove_var(POOLAI_MONITORING_STORE);
        std::env::remove_var(POOLAI_MONITORING_DATA_DIR);
        assert_eq!(monitoring_store_mode(), "memory");
        std::env::set_var(POOLAI_MONITORING_DATA_DIR, "/tmp/poolai-monitoring-wire");
        assert_eq!(monitoring_store_mode(), "sqlite");
        std::env::remove_var(POOLAI_MONITORING_DATA_DIR);
        assert!(MONITORING_STORE_MODES.contains(&"memory"));
        assert_eq!(POOLAI_MONITORING_DATA_DIR, "POOLAI_MONITORING_DATA_DIR");
        assert_eq!(POOLAI_MONITORING_STORE, "POOLAI_MONITORING_STORE");
    }

    #[test]
    fn monitoring_store_wire_memory_default_ph_s1560() {
        std::env::remove_var(POOLAI_MONITORING_STORE);
        std::env::remove_var(POOLAI_MONITORING_DATA_DIR);
        let wire = monitoring_store_wire();
        assert_eq!(wire.mode, "memory");
        assert!(!wire.configured);
        assert!(wire.durable_path.is_none());
        assert_eq!(monitoring_store_wire_label(&wire), "memory");
        assert_eq!(MONITORING_DB_FILE, "monitoring.db");
    }

    #[test]
    fn monitoring_store_wire_sqlite_unconfigured_ph_s1560() {
        std::env::set_var(POOLAI_MONITORING_STORE, "sqlite");
        std::env::remove_var(POOLAI_MONITORING_DATA_DIR);
        let wire = monitoring_store_wire();
        assert_eq!(wire.mode, "sqlite");
        assert!(!wire.configured);
        assert_eq!(monitoring_store_wire_label(&wire), "sqlite_unconfigured");
        std::env::remove_var(POOLAI_MONITORING_STORE);
    }

    #[test]
    fn monitoring_store_wire_sqlite_configured_ph_s1560() {
        std::env::set_var(POOLAI_MONITORING_STORE, "sqlite");
        std::env::set_var(POOLAI_MONITORING_DATA_DIR, "/tmp/poolai-monitoring-wire");
        let wire = monitoring_store_wire();
        assert!(wire.configured);
        assert_eq!(monitoring_store_wire_label(&wire), "sqlite");
        let path = wire.durable_path.as_deref().expect("path");
        assert!(path.ends_with(MONITORING_DB_FILE) || path.contains(MONITORING_DB_FILE));
        std::env::remove_var(POOLAI_MONITORING_STORE);
        std::env::remove_var(POOLAI_MONITORING_DATA_DIR);
    }

    #[test]
    fn validate_monitoring_alert_fields_ok_ph_s1550() {
        let rule = AlertRule {
            name: "high-cpu".into(),
            metric: "cpu_usage".into(),
            threshold: 90.0,
            operator: ">".into(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        };
        assert!(validate_monitoring_alert_fields(&rule).is_ok());
    }

    #[test]
    fn validate_monitoring_alert_fields_rejects_empty_name_ph_s1550() {
        let rule = AlertRule {
            name: "  ".into(),
            metric: "cpu_usage".into(),
            threshold: 90.0,
            operator: ">".into(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        };
        let err = validate_monitoring_alert_fields(&rule).unwrap_err();
        assert!(err.to_string().contains("name"));
    }

    #[test]
    fn validate_monitoring_alert_fields_rejects_bad_operator_ph_s1550() {
        let rule = AlertRule {
            name: "bad-op".into(),
            metric: "cpu_usage".into(),
            threshold: 90.0,
            operator: "!=".into(),
            severity: AlertSeverity::Warning,
            enabled: true,
            tenant_id: None,
        };
        let err = validate_monitoring_alert_fields(&rule).unwrap_err();
        assert!(err.to_string().contains("operator"));
    }
}
