//! Health Monitor for Stage 4.1 Runtime
//!
//! Provides health checking for processes, workers, and VM instances

use crate::core::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

/// Health check result
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy(String), // Reason for unhealthy status
    Unknown,
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub max_failures: u32,
    pub auto_restart: bool,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            timeout_seconds: 5,
            max_failures: 3,
            auto_restart: true,
        }
    }
}

/// Health check entry for a process/VM instance
#[derive(Debug, Clone)]
struct HealthCheckEntry {
    id: Uuid,
    name: String,
    failure_count: u32,
    last_check: Option<chrono::DateTime<chrono::Utc>>,
    status: HealthStatus,
}

pub struct HealthMonitor {
    config: HealthCheckConfig,
    checks: Arc<RwLock<HashMap<Uuid, HealthCheckEntry>>>,
}

impl HealthMonitor {
    pub fn new(interval: u64) -> Self {
        Self {
            config: HealthCheckConfig {
                interval_seconds: interval,
                ..Default::default()
            },
            checks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Initializing Health Monitor (interval: {}s)",
            self.config.interval_seconds
        );
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting Health Monitor");
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Shutting down Health Monitor");
        Ok(())
    }

    pub fn get_health_score(&self) -> f32 {
        // Calculate health score based on registered checks
        // For now, return 1.0 (all healthy)
        1.0
    }

    /// Register a health check for a process/VM instance
    pub async fn register_check(&self, id: Uuid, name: String) {
        let mut checks = self.checks.write().await;
        checks.insert(
            id,
            HealthCheckEntry {
                id,
                name,
                failure_count: 0,
                last_check: None,
                status: HealthStatus::Unknown,
            },
        );
    }

    /// Unregister a health check
    pub async fn unregister_check(&self, id: Uuid) {
        let mut checks = self.checks.write().await;
        checks.remove(&id);
    }

    /// Perform health check for a process
    pub async fn check_process_health<F>(&self, id: Uuid, check_fn: F) -> HealthStatus
    where
        F: FnOnce() -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>,
        >,
    {
        let result =
            tokio::time::timeout(Duration::from_secs(self.config.timeout_seconds), check_fn())
                .await;

        let mut checks = self.checks.write().await;
        let entry = checks.get_mut(&id);

        match entry {
            Some(entry) => {
                entry.last_check = Some(chrono::Utc::now());

                match result {
                    Ok(Ok(())) => {
                        // Health check passed
                        entry.failure_count = 0;
                        entry.status = HealthStatus::Healthy;
                        HealthStatus::Healthy
                    }
                    Ok(Err(e)) => {
                        // Health check failed
                        entry.failure_count += 1;
                        let reason = format!("{}", e);
                        entry.status = HealthStatus::Unhealthy(reason.clone());

                        if entry.failure_count >= self.config.max_failures {
                            warn!(
                                "Process {} failed health check {} times (max: {})",
                                entry.name, entry.failure_count, self.config.max_failures
                            );
                        }

                        HealthStatus::Unhealthy(reason)
                    }
                    Err(_) => {
                        // Health check timed out
                        entry.failure_count += 1;
                        let reason = "Health check timeout".to_string();
                        entry.status = HealthStatus::Unhealthy(reason.clone());

                        if entry.failure_count >= self.config.max_failures {
                            warn!(
                                "Process {} failed health check {} times (max: {})",
                                entry.name, entry.failure_count, self.config.max_failures
                            );
                        }

                        HealthStatus::Unhealthy(reason)
                    }
                }
            }
            None => HealthStatus::Unknown,
        }
    }

    /// Get health status for a registered check
    pub async fn get_health_status(&self, id: Uuid) -> Option<HealthStatus> {
        let checks = self.checks.read().await;
        checks.get(&id).map(|e| e.status.clone())
    }

    /// Get all health check entries
    pub async fn list_checks(&self) -> Vec<(Uuid, String, HealthStatus)> {
        let checks = self.checks.read().await;
        checks
            .values()
            .map(|e| (e.id, e.name.clone(), e.status.clone()))
            .collect()
    }

    /// Get failure count for a registered check
    pub async fn get_failure_count(&self, id: Uuid) -> Option<u32> {
        let checks = self.checks.read().await;
        checks.get(&id).map(|e| e.failure_count)
    }

    /// Get health check configuration
    pub fn get_config(&self) -> &HealthCheckConfig {
        &self.config
    }
}
