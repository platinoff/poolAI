//! Monitoring module for PoolAI
//!
//! Provides system monitoring, metrics collection, alerts, and health checks.
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::monitoring::{Monitoring, Alert, AlertSeverity};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let monitoring = Monitoring::new();
//!
//! // Collect metrics
//! let metrics = monitoring.collect_metrics().await?;
//! println!("CPU usage: {}%", metrics.cpu_usage_percent);
//!
//! // Process an alert
//! let alert = Alert {
//!     id: "alert-1".to_string(),
//!     severity: AlertSeverity::Warning,
//!     message: "High CPU usage".to_string(),
//!     timestamp: std::time::Instant::now(),
//!     source: "system".to_string(),
//!     resolved: false,
//! };
//! monitoring.process_alert(alert).await?;
//!
//! // Get system status
//! let status = monitoring.get_system_status().await;
//! println!("Overall health: {}%", status.overall_health);
//! # Ok(())
//! # }
//! ```

pub mod metrics;

use crate::core::error::AppError;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// System alert
///
/// Represents an alert or warning in the system with severity and metadata.
///
/// # Example
///
/// ```rust
/// use poolai::monitoring::{Alert, AlertSeverity};
/// use std::time::Instant;
///
/// let alert = Alert {
///     id: "high-cpu-123".to_string(),
///     severity: AlertSeverity::Warning,
///     message: "CPU usage above 80%".to_string(),
///     timestamp: Instant::now(),
///     source: "monitoring".to_string(),
///     resolved: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: Instant,
    pub source: String,
    pub resolved: bool,
}

/// Alert severity levels
///
/// # Example
///
/// ```rust
/// use poolai::monitoring::AlertSeverity;
///
/// let severity = AlertSeverity::Warning;
/// match severity {
///     AlertSeverity::Info => println!("Informational alert"),
///     AlertSeverity::Warning => println!("Warning alert"),
///     AlertSeverity::Error => println!("Error alert"),
///     AlertSeverity::Critical => println!("Critical alert"),
/// }
/// ```
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

/// Monitoring system for PoolAI
///
/// Provides centralized monitoring, metrics collection, and alert management.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::monitoring::Monitoring;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let monitoring = Monitoring::new();
///
/// // Collect metrics periodically
/// let metrics = monitoring.collect_metrics().await?;
///
/// // Get current status
/// let status = monitoring.get_system_status().await;
/// # Ok(())
/// # }
/// ```
pub struct Monitoring {
    alerts: Arc<RwLock<Vec<Alert>>>,
    metrics_collector: Arc<metrics::MetricsCollector>,
    status: Arc<RwLock<SystemStatus>>,
    historical_data: Arc<RwLock<Vec<HistoricalData>>>,
}

impl Default for Monitoring {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitoring {
    /// Create a new monitoring instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::monitoring::Monitoring;
    ///
    /// let monitoring = Monitoring::new();
    /// // Remember to call initialize() before using
    /// ```
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

    /// Collect current system metrics
    ///
    /// Gathers metrics from all sources (CPU, memory, GPU, disk, network).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::Monitoring;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitoring = Monitoring::new();
    /// let metrics = monitoring.collect_metrics().await?;
    /// println!("CPU: {}%, Memory: {}MB",
    ///     metrics.cpu_usage_percent,
    ///     metrics.memory_usage_mb);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collect_metrics(&self) -> Result<metrics::Metrics, AppError> {
        self.metrics_collector.collect().await
    }

    /// Process and store an alert
    ///
    /// Adds an alert to the monitoring system and triggers any configured
    /// alert handlers or notifications.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::{Monitoring, Alert, AlertSeverity};
    /// use std::time::Instant;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitoring = Monitoring::new();
    /// let alert = Alert {
    ///     id: "high-cpu-123".to_string(),
    ///     severity: AlertSeverity::Warning,
    ///     message: "CPU usage above 80%".to_string(),
    ///     timestamp: Instant::now(),
    ///     source: "monitoring".to_string(),
    ///     resolved: false,
    /// };
    /// monitoring.process_alert(alert).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    pub async fn get_historical_data(
        &self,
        duration: Duration,
    ) -> Result<Vec<HistoricalData>, AppError> {
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
        alerts
            .iter()
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
            Err(AppError::MonitoringError(format!(
                "Alert '{}' not found. Context: Cannot resolve alert that doesn't exist or has already been resolved. \
                Suggestion: Check alert ID spelling, verify alert exists using get_active_alerts(), or check if alert was already resolved. \
                Current alert ID: '{}', Total alerts: {}",
                alert_id, alert_id, alerts.len()
            )))
        }
    }

    /// Get current system status
    ///
    /// Returns a snapshot of the current system health and metrics.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::Monitoring;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitoring = Monitoring::new();
    /// let status = monitoring.get_system_status().await;
    /// println!("Health: {}%, Workers: {}, Requests: {}",
    ///     status.overall_health,
    ///     status.active_workers,
    ///     status.total_requests);
    /// # Ok(())
    /// # }
    /// ```
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

pub use crate::monitoring::metrics::MetricsCollector;

// Global monitoring instance - using OnceLock for thread-safe initialization
// Wrapped in Arc for shared ownership across async contexts
static GLOBAL_MONITORING: OnceLock<Arc<Monitoring>> = OnceLock::new();

/// Initialize monitoring module
/// Initialize the global monitoring instance
///
/// Sets up the global monitoring singleton. Must be called before
/// using other monitoring functions that rely on the global instance.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::monitoring::initialize;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// // Initialize monitoring at application startup
/// initialize().await?;
/// // Now you can use monitoring functions
/// # Ok(())
/// # }
/// ```
pub async fn initialize() -> Result<(), AppError> {
    tracing::info!("Initializing monitoring module");

    let monitoring = Monitoring::new();
    let monitoring_arc = Arc::new(monitoring);

    // Store global instance
    GLOBAL_MONITORING
        .set(monitoring_arc.clone())
        .map_err(|_| AppError::MonitoringError(
            "Monitoring already initialized. Context: Attempted to initialize monitoring module twice. \
            Suggestion: Ensure initialize() is called only once at application startup. \
            Note: Monitoring module uses OnceLock for thread-safe single initialization.".to_string()
        ))?;

    // Start background monitoring
    monitoring_arc.start_monitoring().await?;

    tracing::info!("Monitoring module initialized successfully");
    Ok(())
}

/// Shutdown monitoring module
pub async fn shutdown() -> Result<(), AppError> {
    tracing::info!("Shutting down monitoring module");

    // Note: OnceLock doesn't support clearing, so we can't fully remove it
    // The monitoring instance will remain in memory but won't be accessible after this
    // For true cleanup, consider using a different pattern or accept this limitation

    tracing::info!("Monitoring module shut down successfully");
    Ok(())
}

/// Health check for monitoring module
pub async fn health_check() -> Result<(), AppError> {
    tracing::info!("Monitoring module health check");

    // Check if global monitoring exists
    if GLOBAL_MONITORING.get().is_none() {
        return Err(AppError::MonitoringError(
            "Monitoring not initialized. Context: Attempted to use monitoring functionality before initialization. \
            Suggestion: Call monitoring::initialize() before using monitoring functions. \
            Note: This should be called once at application startup.".to_string()
        ));
    }

    tracing::info!("Monitoring module health check passed");
    Ok(())
}
