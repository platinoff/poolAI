//! Virtual Machines (VM) module
//!
//! Provides VM instance management, resource limits, isolation, auto-recovery,
//! and resource monitoring capabilities.
//!
//! # Examples
//!
//! ## Creating a VM instance
//!
//! ```no_run
//! use poolai::vm::{VmManager, VmResources, VmIsolation};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = VmManager::new();
//!
//! let instance = manager.create_instance(
//!     "my-vm".to_string(),
//!     VmResources::default(),
//!     VmIsolation::ProcessSandbox,
//! ).await?;
//!
//! println!("Created VM instance: {:?}", instance.id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuring auto-recovery
//!
//! ```no_run
//! use poolai::vm::{VmManager, AutoRecoveryConfig};
//! use uuid::Uuid;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = VmManager::new();
//! let instance_id = Uuid::new_v4();
//!
//! let auto_recovery = AutoRecoveryConfig {
//!     max_restart_attempts: 5,
//!     initial_restart_delay_secs: 1,
//!     max_restart_delay_secs: 60,
//!     use_exponential_backoff: true,
//! };
//!
//! manager.update_instance(instance_id, None, None, Some(auto_recovery)).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Monitoring resource usage
//!
//! ```no_run
//! use poolai::vm::VmManager;
//! use uuid::Uuid;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = VmManager::new();
//! let instance_id = Uuid::new_v4();
//!
//! let stats = manager.get_resource_usage_stats(instance_id).await?;
//! println!("CPU avg: {:.2}%", stats.cpu_percent_avg);
//! # Ok(())
//! # }
//! ```
//!
//! Concept alignment:
//! - VM instance management (planned in `docs/concept/poolAI_concept.txt`)
//! - Isolation/security hooks (stubbed)
//! - Resource optimization primitives (basic)

mod resources;
pub use resources::{PlatformResourceLimiter, ResourceLimiter, ResourceLimits, ResourceUsage};

mod isolation;
pub use isolation::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkIsolationConfig, NetworkIsolator,
    PlatformFilesystemIsolator, PlatformNetworkIsolator,
};

use crate::core::error::AppError;
use crate::runtime::health::{HealthMonitor, HealthStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

/// VM lifecycle status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmStatus {
    Creating,
    Running,
    Stopped,
    Failed(String),
}

/// Resource limits / requests for a VM instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmResources {
    pub cpu_cores: u16,
    pub memory_mb: u32,
    pub gpu_required: bool,
}

impl Default for VmResources {
    fn default() -> Self {
        Self {
            cpu_cores: 2,
            memory_mb: 2048,
            gpu_required: false,
        }
    }
}

/// Isolation/security policy placeholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmIsolation {
    /// Best-effort isolation using OS-level process sandboxing (planned).
    ProcessSandbox,
    /// Hardware virtualization (planned).
    HardwareVm,
}

/// Auto-recovery configuration for VM instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRecoveryConfig {
    /// Maximum number of restart attempts before giving up
    pub max_restart_attempts: u32,
    /// Initial delay before first restart (in seconds)
    pub initial_restart_delay_secs: u64,
    /// Maximum delay between restarts (in seconds)
    pub max_restart_delay_secs: u64,
    /// Whether to use exponential backoff
    pub use_exponential_backoff: bool,
}

impl Default for AutoRecoveryConfig {
    fn default() -> Self {
        Self {
            max_restart_attempts: 5,
            initial_restart_delay_secs: 1,
            max_restart_delay_secs: 60,
            use_exponential_backoff: true,
        }
    }
}

/// A VM instance representation (in-memory for now).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInstance {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub status: VmStatus,
    pub resources: VmResources,
    pub isolation: VmIsolation,
    /// Auto-recovery configuration
    pub auto_recovery: AutoRecoveryConfig,
    /// Number of restart attempts (internal tracking)
    #[serde(skip)]
    pub restart_attempts: u32,
    /// Process ID of the running process (if spawned)
    #[serde(skip)]
    pub process_id: Option<u32>,
}

/// Resource usage history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageHistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub usage: ResourceUsage,
}

/// Resource usage statistics (aggregated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageStats {
    pub cpu_percent_min: f32,
    pub cpu_percent_max: f32,
    pub cpu_percent_avg: f32,
    pub memory_mb_min: u32,
    pub memory_mb_max: u32,
    pub memory_mb_avg: f32,
    pub gpu_utilization_min: Option<f32>,
    pub gpu_utilization_max: Option<f32>,
    pub gpu_utilization_avg: Option<f32>,
    pub sample_count: usize,
}

/// Resource alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAlertThresholds {
    /// CPU usage threshold (0.0-100.0)
    pub cpu_percent_threshold: Option<f32>,
    /// Memory usage threshold in MB
    pub memory_mb_threshold: Option<u32>,
    /// GPU utilization threshold (0.0-100.0)
    pub gpu_utilization_threshold: Option<f32>,
}

impl Default for ResourceAlertThresholds {
    fn default() -> Self {
        Self {
            cpu_percent_threshold: Some(90.0),
            memory_mb_threshold: None,
            gpu_utilization_threshold: Some(95.0),
        }
    }
}

/// VM Manager - central orchestrator for VM instances.
pub struct VmManager {
    instances: Arc<RwLock<HashMap<Uuid, VmInstance>>>,
    health_monitor: Arc<RwLock<HealthMonitor>>,
    #[allow(dead_code)] // Used for periodic health checks
    health_check_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    resource_limiter: Arc<dyn ResourceLimiter>,
    network_isolator: Arc<dyn NetworkIsolator>,
    filesystem_isolator: Arc<dyn FilesystemIsolator>,
    /// Resource usage history per instance (max 1000 entries per instance)
    resource_history: Arc<RwLock<HashMap<Uuid, Vec<ResourceUsageHistoryEntry>>>>,
    /// Resource alert thresholds per instance
    resource_alert_thresholds: Arc<RwLock<HashMap<Uuid, ResourceAlertThresholds>>>,
}

impl VmManager {
    pub fn new() -> Self {
        let health_monitor = Arc::new(RwLock::new(HealthMonitor::new(30))); // 30 second interval
        let resource_limiter: Arc<dyn ResourceLimiter> = Arc::new(PlatformResourceLimiter::new());
        let network_isolator: Arc<dyn NetworkIsolator> = Arc::new(PlatformNetworkIsolator::new());
        let filesystem_isolator: Arc<dyn FilesystemIsolator> =
            Arc::new(PlatformFilesystemIsolator::new());

        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            health_monitor,
            health_check_task: Arc::new(RwLock::new(None)),
            resource_limiter,
            network_isolator,
            filesystem_isolator,
            resource_history: Arc::new(RwLock::new(HashMap::new())),
            resource_alert_thresholds: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing VM manager");

        // Initialize health monitor
        {
            let mut hm = self.health_monitor.write().await;
            hm.initialize().await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to initialize health monitor: {}. \
                    Context: Health monitor initialization failed during VM manager startup. \
                    Suggestion: Check system permissions and ensure health monitor dependencies are available. \
                    Error details: {}",
                    e, e
                ))
            })?;
            hm.start().await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to start health monitor: {}. \
                    Context: Health monitor failed to start after initialization. \
                    Suggestion: Check system resources and ensure health monitor background tasks can be spawned. \
                    Error details: {}",
                    e, e
                ))
            })?;
        }

        // Start periodic health checks for running VM instances
        self.start_periodic_health_checks().await;

        Ok(())
    }

    /// Start periodic health checks for running VM instances
    async fn start_periodic_health_checks(&self) {
        let instances = Arc::clone(&self.instances);
        let health_monitor = Arc::clone(&self.health_monitor);

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                // Get all running instances
                let running_instances: Vec<(Uuid, String)> = {
                    let insts = instances.read().await;
                    insts
                        .values()
                        .filter(|inst| matches!(inst.status, VmStatus::Running))
                        .map(|inst| (inst.id, inst.name.clone()))
                        .collect()
                };

                // Perform health check for each running instance
                for (id, name) in running_instances {
                    let instances_clone = Arc::clone(&instances);
                    let health_status = {
                        let hm = health_monitor.read().await;
                        hm.check_process_health(id, move || {
                            let instances = Arc::clone(&instances_clone);
                            Box::pin(async move {
                                // Check if instance is still running
                                let insts = instances.read().await;
                                match insts.get(&id) {
                                    Some(inst) if matches!(inst.status, VmStatus::Running) => {
                                        Ok(())
                                    }
                                    Some(inst) => {
                                        let status = inst.status.clone();
                                        Err(AppError::ValidationError(format!(
                                            "VM instance is not running: {}. \
                                            Context: Health checks can only be performed on running VM instances. \
                                            Suggestion: Start the VM instance first using start_instance() before checking its health. \
                                            Current status: {:?}",
                                            id, status
                                        )))
                                    }
                                    None => {
                                        Err(AppError::ValidationError(format!(
                                            "VM instance not found: {}. \
                                            Context: The VM instance was removed or does not exist. \
                                            Suggestion: Verify the instance ID is correct.",
                                            id
                                        )))
                                    }
                                }
                            })
                        })
                        .await
                    };

                    // Handle unhealthy status
                    if matches!(health_status, HealthStatus::Unhealthy(_)) {
                        warn!("VM instance {} ({}) health check failed", id, name);

                        // Check failure count and config from health monitor
                        let (failure_count, max_failures, auto_restart) = {
                            let hm = health_monitor.read().await;
                            let failure_count = hm.get_failure_count(id).await.unwrap_or(0);
                            let config = hm.get_config();
                            (failure_count, config.max_failures, config.auto_restart)
                        };

                        // Auto-restart if configured and failure count reached threshold
                        if auto_restart && failure_count >= max_failures {
                            // Check restart attempts and auto-recovery config
                            let (should_restart, restart_delay, restart_attempts) = {
                                let mut insts = instances.write().await;
                                if let Some(inst) = insts.get_mut(&id) {
                                    let restart_attempts = inst.restart_attempts;
                                    let config = &inst.auto_recovery;

                                    // Check if we've exceeded max restart attempts
                                    if restart_attempts >= config.max_restart_attempts {
                                        warn!(
                                            "VM instance {} ({}) exceeded max restart attempts ({}), marking as failed",
                                            id, name, config.max_restart_attempts
                                        );
                                        inst.status = VmStatus::Failed(format!(
                                            "Exceeded max restart attempts ({})",
                                            config.max_restart_attempts
                                        ));
                                        (false, 0, restart_attempts)
                                    } else {
                                        // Calculate restart delay with exponential backoff
                                        let delay = if config.use_exponential_backoff {
                                            let delay = config.initial_restart_delay_secs
                                                * (1u64 << restart_attempts.min(10)); // Cap at 2^10 = 1024
                                            delay.min(config.max_restart_delay_secs)
                                        } else {
                                            config.initial_restart_delay_secs
                                        };

                                        inst.restart_attempts += 1;
                                        (true, delay, inst.restart_attempts)
                                    }
                                } else {
                                    (false, 0, 0)
                                }
                            };

                            if should_restart {
                                warn!(
                                    "Auto-restarting VM instance {} ({}) after {} failures (attempt {}/{})",
                                    id, name, failure_count, restart_attempts,
                                    {
                                        let insts = instances.read().await;
                                        insts.get(&id)
                                            .map(|inst| inst.auto_recovery.max_restart_attempts)
                                            .unwrap_or(5)
                                    }
                                );

                                // Restart the instance
                                let instances_clone = Arc::clone(&instances);
                                let health_monitor_clone = Arc::clone(&health_monitor);

                                // Stop the instance first
                                {
                                    let mut insts = instances_clone.write().await;
                                    if let Some(inst) = insts.get_mut(&id) {
                                        inst.status = VmStatus::Stopped;
                                    }
                                }

                                // Unregister health check
                                {
                                    let hm = health_monitor_clone.write().await;
                                    hm.unregister_check(id).await;
                                }

                                // Wait with exponential backoff before restarting
                                info!(
                                    "Waiting {} seconds before restarting VM instance {} ({})",
                                    restart_delay, id, name
                                );
                                tokio::time::sleep(Duration::from_secs(restart_delay)).await;

                                // Restart the instance
                                {
                                    let mut insts = instances_clone.write().await;
                                    if let Some(inst) = insts.get_mut(&id) {
                                        inst.status = VmStatus::Running;
                                    }
                                }

                                // Re-register health check with reset failure count
                                {
                                    let hm = health_monitor_clone.write().await;
                                    hm.register_check(id, name.clone()).await;
                                }

                                info!(
                                    "VM instance {} ({}) restarted after health check failure (attempt {})",
                                    id, name, restart_attempts
                                );
                            }
                        } else if failure_count >= max_failures {
                            // Mark as failed if auto-restart is disabled
                            let mut insts = instances.write().await;
                            if let Some(inst) = insts.get_mut(&id) {
                                inst.status = VmStatus::Failed(format!(
                                    "Health check failed {} times",
                                    failure_count
                                ));
                            }
                        }
                    }
                }
            }
        });

        *self.health_check_task.write().await = Some(task);
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down VM manager");

        // Stop health check task
        if let Some(task) = self.health_check_task.write().await.take() {
            task.abort();
        }

        // Shutdown health monitor
        {
            let mut hm = self.health_monitor.write().await;
            hm.shutdown().await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to shutdown health monitor: {}. \
                    Context: Health monitor failed to shutdown gracefully during VM manager shutdown. \
                    Suggestion: Health monitor may have already stopped. Check logs for details. \
                    Error details: {}",
                    e, e
                ))
            })?;
        }

        Ok(())
    }

    pub async fn create_instance(
        &self,
        name: String,
        resources: VmResources,
        isolation: VmIsolation,
    ) -> Result<VmInstance, AppError> {
        let id = Uuid::new_v4();
        let instance = VmInstance {
            id,
            name,
            created_at: Utc::now(),
            status: VmStatus::Creating,
            resources,
            isolation,
            auto_recovery: AutoRecoveryConfig::default(),
            restart_attempts: 0,
            process_id: None,
        };

        self.instances.write().await.insert(id, instance.clone());
        info!("VM instance {} ({}) created", id, instance.name);
        Ok(instance)
    }

    pub async fn update_instance(
        &self,
        id: Uuid,
        name: Option<String>,
        resources: Option<VmResources>,
        isolation: Option<VmIsolation>,
        auto_recovery: Option<AutoRecoveryConfig>,
    ) -> Result<VmInstance, AppError> {
        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;

        if let Some(new_name) = name {
            inst.name = new_name;
        }
        if let Some(new_resources) = resources {
            inst.resources = new_resources;
        }
        if let Some(new_isolation) = isolation {
            inst.isolation = new_isolation;
        }
        if let Some(new_auto_recovery) = auto_recovery {
            inst.auto_recovery = new_auto_recovery;
            // Reset restart attempts when auto-recovery config changes
            inst.restart_attempts = 0;
        }

        let updated = inst.clone();
        info!("VM instance {} updated", id);
        Ok(updated)
    }

    pub async fn delete_instance(&self, id: Uuid) -> Result<(), AppError> {
        // Stop instance if running
        if let Some(inst) = self.get_instance(id).await {
            if matches!(inst.status, VmStatus::Running) {
                self.stop_instance(id).await?;
            }
        }

        // Unregister health check
        {
            let hm = self.health_monitor.write().await;
            hm.unregister_check(id).await;
        }

        // Remove from HashMap
        let mut instances = self.instances.write().await;
        instances
            .remove(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;

        // Clean up resource history and alert thresholds
        {
            let mut history = self.resource_history.write().await;
            history.remove(&id);
        }
        {
            let mut alert_thresholds = self.resource_alert_thresholds.write().await;
            alert_thresholds.remove(&id);
        }

        info!("VM instance {} deleted", id);
        Ok(())
    }

    pub async fn list_instances(&self) -> Vec<VmInstance> {
        self.instances.read().await.values().cloned().collect()
    }

    pub async fn get_instance(&self, id: Uuid) -> Option<VmInstance> {
        self.instances.read().await.get(&id).cloned()
    }

    pub async fn start_instance(&self, id: Uuid) -> Result<(), AppError> {
        let name = {
            let mut instances = self.instances.write().await;
            let inst = instances.get_mut(&id).ok_or_else(|| {
                AppError::ValidationError(format!(
                    "VM instance {} not found. Context: Cannot start a VM instance that doesn't exist. \
                    Suggestion: Verify instance ID using list_instances() or create_instance() first. \
                    Instance ID: {}",
                    id, id
                ))
            })?;

            inst.status = VmStatus::Running;
            // Reset restart attempts on successful start
            inst.restart_attempts = 0;
            inst.name.clone()
        };

        // Register health check for this instance
        {
            let hm = self.health_monitor.write().await;
            hm.register_check(id, name).await;
        }

        info!(
            "VM instance {} started and registered for health checks",
            id
        );
        Ok(())
    }

    pub async fn stop_instance(&self, id: Uuid) -> Result<(), AppError> {
        // Unregister health check
        {
            let hm = self.health_monitor.write().await;
            hm.unregister_check(id).await;
        }

        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;

        inst.status = VmStatus::Stopped;
        info!(
            "VM instance {} stopped and unregistered from health checks",
            id
        );
        Ok(())
    }

    /// Get health status for a VM instance
    pub async fn get_instance_health(&self, id: Uuid) -> Result<Option<HealthStatus>, AppError> {
        let health_monitor = self.health_monitor.read().await;
        Ok(health_monitor.get_health_status(id).await)
    }

    /// Perform manual health check for a VM instance
    pub async fn check_instance_health(&self, id: Uuid) -> Result<HealthStatus, AppError> {
        let instances = Arc::clone(&self.instances);
        let health_monitor = Arc::clone(&self.health_monitor);

        let status = {
            let hm = health_monitor.read().await;
            let instances_clone = Arc::clone(&instances);
            hm.check_process_health(id, move || {
                let instances = Arc::clone(&instances_clone);
                Box::pin(async move {
                    // Check if instance is still running
                    let insts = instances.read().await;
                    match insts.get(&id) {
                        Some(inst) if matches!(inst.status, VmStatus::Running) => Ok(()),
                        Some(inst) => {
                            let status = inst.status.clone();
                            Err(AppError::ValidationError(format!(
                                "VM instance {} is not running. Context: Health checks can only be performed on running VM instances. \
                                Suggestion: Start the VM instance first using start_instance() before checking its health. \
                                Instance ID: {}, Current status: {:?}",
                                id, id, status
                            )))
                        },
                        None => Err(AppError::ValidationError(format!(
                            "VM instance {} not found. Context: The VM instance was removed or does not exist during health check. \
                            Suggestion: Verify the instance ID is correct and ensure the instance exists using list_instances(). \
                            Instance ID: {}",
                            id, id
                        ))),
                    }
                })
            })
            .await
        };

        Ok(status)
    }

    /// Apply resource limits to a command (for future process spawning)
    pub async fn apply_resource_limits(
        &self,
        command: &mut tokio::process::Command,
        instance_id: Uuid,
    ) -> Result<(), AppError> {
        let instances = self.instances.read().await;
        let instance = instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "VM instance {} not found. Context: Cannot apply resource limits to a non-existent VM instance. \
                Suggestion: Verify instance ID is correct and ensure instance exists using list_instances(). \
                Instance ID: {}",
                instance_id, instance_id
            ))
        })?;

        let limits = ResourceLimits::from(instance.resources.clone());
        self.resource_limiter.apply_limits(command, &limits).await
    }

    /// Get resource usage for a VM instance
    ///
    /// This method attempts to get current resource usage and automatically
    /// adds it to the history. If process_id is not available, returns the
    /// most recent history entry if available.
    pub async fn get_instance_resource_usage(
        &self,
        instance_id: Uuid,
    ) -> Result<ResourceUsage, AppError> {
        let instances = self.instances.read().await;
        let instance = instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "VM instance {} not found. Context: Cannot get resource usage for a non-existent VM instance. \
                Suggestion: Verify instance ID is correct and ensure instance exists using list_instances(). \
                Instance ID: {}",
                instance_id, instance_id
            ))
        })?;
        
        // If process_id is available, try to get current resource usage
        if let Some(process_id) = instance.process_id {
            match self.resource_limiter.get_usage(process_id).await {
                Ok(usage) => {
                    // Record usage in history
                    self.record_resource_usage(instance_id, usage.clone()).await?;
                    return Ok(usage);
                }
                Err(e) => {
                    warn!("Failed to get resource usage for process {}: {}", process_id, e);
                    // Fall through to history lookup
                }
            }
        }
        
        // Fallback: return the most recent history entry if available
        let history = self.resource_history.read().await;
        if let Some(entries) = history.get(&instance_id) {
            if let Some(latest) = entries.last() {
                return Ok(latest.usage.clone());
            }
        }

        Err(AppError::ConfigError(
            "Process ID not available and no history found - process spawning not yet implemented".to_string(),
        ))
    }

    /// Record resource usage in history
    ///
    /// This method adds a resource usage entry to the history for an instance.
    /// History is limited to 1000 entries per instance (FIFO).
    pub async fn record_resource_usage(
        &self,
        instance_id: Uuid,
        usage: ResourceUsage,
    ) -> Result<(), AppError> {
        let instances = self.instances.read().await;
        instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!("VM instance {} not found", instance_id))
        })?;

        let entry = ResourceUsageHistoryEntry {
            timestamp: Utc::now(),
            usage,
        };

        let mut history = self.resource_history.write().await;
        let entries = history.entry(instance_id).or_insert_with(Vec::new);

        // Limit history to 1000 entries (FIFO)
        if entries.len() >= 1000 {
            entries.remove(0);
        }

        entries.push(entry);

        // Check alerts
        self.check_resource_alerts(instance_id, &entries.last().unwrap().usage)
            .await;

        Ok(())
    }

    /// Get resource usage history for a VM instance
    ///
    /// Returns the last N entries from the history, or all entries if limit is None.
    pub async fn get_resource_usage_history(
        &self,
        instance_id: Uuid,
        limit: Option<usize>,
    ) -> Result<Vec<ResourceUsageHistoryEntry>, AppError> {
        let instances = self.instances.read().await;
        instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!("VM instance {} not found", instance_id))
        })?;

        let history = self.resource_history.read().await;
        if let Some(entries) = history.get(&instance_id) {
            if let Some(limit) = limit {
                Ok(entries.iter().rev().take(limit).cloned().rev().collect())
            } else {
                Ok(entries.clone())
            }
        } else {
            Ok(vec![])
        }
    }

    /// Get resource usage statistics (aggregated)
    ///
    /// Calculates min, max, and average for CPU, memory, and GPU usage
    /// from the history entries.
    pub async fn get_resource_usage_stats(
        &self,
        instance_id: Uuid,
        limit: Option<usize>,
    ) -> Result<ResourceUsageStats, AppError> {
        let instances = self.instances.read().await;
        instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!("VM instance {} not found", instance_id))
        })?;

        let history = self.resource_history.read().await;
        let entries = if let Some(entries) = history.get(&instance_id) {
            if let Some(limit) = limit {
                entries
                    .iter()
                    .rev()
                    .take(limit)
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                entries.clone()
            }
        } else {
            return Err(AppError::ValidationError(
                "No resource usage history available".to_string(),
            ));
        };

        if entries.is_empty() {
            return Err(AppError::ValidationError(
                "No resource usage history available".to_string(),
            ));
        }

        let mut cpu_values = Vec::new();
        let mut memory_values = Vec::new();
        let mut gpu_values = Vec::new();

        for entry in &entries {
            cpu_values.push(entry.usage.cpu_percent);
            memory_values.push(entry.usage.memory_mb);
            if let Some(gpu) = entry.usage.gpu_utilization {
                gpu_values.push(gpu);
            }
        }

        let cpu_min = cpu_values.iter().fold(f32::MAX, |a, &b| a.min(b));
        let cpu_max = cpu_values.iter().fold(0.0f32, |a, &b| a.max(b));
        let cpu_avg = cpu_values.iter().sum::<f32>() / cpu_values.len() as f32;

        let memory_min = *memory_values.iter().min().unwrap_or(&0);
        let memory_max = *memory_values.iter().max().unwrap_or(&0);
        let memory_avg = memory_values.iter().sum::<u32>() as f32 / memory_values.len() as f32;

        let (gpu_min, gpu_max, gpu_avg) = if gpu_values.is_empty() {
            (None, None, None)
        } else {
            let min = gpu_values.iter().fold(f32::MAX, |a, &b| a.min(b));
            let max = gpu_values.iter().fold(0.0f32, |a, &b| a.max(b));
            let avg = gpu_values.iter().sum::<f32>() / gpu_values.len() as f32;
            (Some(min), Some(max), Some(avg))
        };

        Ok(ResourceUsageStats {
            cpu_percent_min: cpu_min,
            cpu_percent_max: cpu_max,
            cpu_percent_avg: cpu_avg,
            memory_mb_min: memory_min,
            memory_mb_max: memory_max,
            memory_mb_avg: memory_avg,
            gpu_utilization_min: gpu_min,
            gpu_utilization_max: gpu_max,
            gpu_utilization_avg: gpu_avg,
            sample_count: entries.len(),
        })
    }

    /// Set resource alert thresholds for a VM instance
    pub async fn set_resource_alert_thresholds(
        &self,
        instance_id: Uuid,
        thresholds: ResourceAlertThresholds,
    ) -> Result<(), AppError> {
        let instances = self.instances.read().await;
        instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!("VM instance {} not found", instance_id))
        })?;

        let mut alert_thresholds = self.resource_alert_thresholds.write().await;
        alert_thresholds.insert(instance_id, thresholds);
        Ok(())
    }

    /// Get resource alert thresholds for a VM instance
    pub async fn get_resource_alert_thresholds(
        &self,
        instance_id: Uuid,
    ) -> Result<ResourceAlertThresholds, AppError> {
        let instances = self.instances.read().await;
        instances.get(&instance_id).ok_or_else(|| {
            AppError::ValidationError(format!("VM instance {} not found", instance_id))
        })?;

        let alert_thresholds = self.resource_alert_thresholds.read().await;
        Ok(alert_thresholds
            .get(&instance_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Check resource usage against alert thresholds
    ///
    /// This is called automatically when resource usage is recorded.
    async fn check_resource_alerts(&self, instance_id: Uuid, usage: &ResourceUsage) {
        let alert_thresholds = self.resource_alert_thresholds.read().await;
        if let Some(thresholds) = alert_thresholds.get(&instance_id) {
            let mut alerts = Vec::new();

            if let Some(cpu_threshold) = thresholds.cpu_percent_threshold {
                if usage.cpu_percent > cpu_threshold {
                    alerts.push(format!(
                        "CPU usage {}% exceeds threshold {}%",
                        usage.cpu_percent, cpu_threshold
                    ));
                }
            }

            if let Some(memory_threshold) = thresholds.memory_mb_threshold {
                if usage.memory_mb > memory_threshold {
                    alerts.push(format!(
                        "Memory usage {}MB exceeds threshold {}MB",
                        usage.memory_mb, memory_threshold
                    ));
                }
            }

            if let Some(gpu_threshold) = thresholds.gpu_utilization_threshold {
                if let Some(gpu_util) = usage.gpu_utilization {
                    if gpu_util > gpu_threshold {
                        alerts.push(format!(
                            "GPU utilization {}% exceeds threshold {}%",
                            gpu_util, gpu_threshold
                        ));
                    }
                }
            }

            if !alerts.is_empty() {
                warn!(
                    "Resource alerts for VM instance {}: {}",
                    instance_id,
                    alerts.join(", ")
                );
            }
        }
    }

    /// Check if resource limits are supported on this platform
    pub fn is_resource_limits_supported(&self) -> bool {
        self.resource_limiter.is_supported()
    }

    /// Apply network and filesystem isolation to a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    /// * `instance_id` - VM instance ID
    ///
    /// # Returns
    /// `Ok(())` if isolation was applied successfully
    ///
    /// # Note
    /// This method applies isolation based on the instance's isolation policy.
    /// Currently, isolation is applied based on VmIsolation enum, but full
    /// implementation requires process_id which is not yet available.
    pub async fn apply_isolation(
        &self,
        process_id: u32,
        instance_id: Uuid,
    ) -> Result<(), AppError> {
        let instance = {
            let instances = self.instances.read().await;
            instances
                .get(&instance_id)
                .ok_or_else(|| {
                    AppError::ValidationError(format!("VM instance {} not found", instance_id))
                })?
                .clone()
        };

        // Apply isolation based on instance's isolation policy
        match instance.isolation {
            VmIsolation::ProcessSandbox => {
                // Apply network isolation with graceful error handling
                let network_config = NetworkIsolationConfig {
                    enabled: true,
                    allowed_interfaces: vec![],
                    allowed_ports: vec![],
                    allow_loopback: true,
                    strict: false, // Graceful degradation by default
                };

                let network_result = self
                    .network_isolator
                    .apply_network_isolation(process_id, &network_config);

                // If network isolation fails, log but continue (unless strict mode)
                if let Err(ref e) = network_result {
                    warn!(
                        "Network isolation failed for process {} (instance {}): {}. Continuing with filesystem isolation.",
                        process_id, instance_id, e
                    );
                }

                // Apply filesystem isolation with graceful error handling
                let fs_config = FilesystemIsolationConfig {
                    enabled: true,
                    root_dir: None,
                    allowed_paths: vec![],
                    read_only_paths: vec![],
                    use_chroot: false,
                    strict: false, // Graceful degradation by default
                };

                let fs_result = self
                    .filesystem_isolator
                    .apply_filesystem_isolation(process_id, &fs_config);

                // If both isolations fail, return error
                match (network_result, fs_result) {
                    (Ok(_), Ok(_)) => {
                        info!(
                            "Successfully applied isolation to process {} for VM instance {}",
                            process_id, instance_id
                        );
                    }
                    (Err(net_err), Err(fs_err)) => {
                        return Err(AppError::ConfigError(format!(
                            "Both network and filesystem isolation failed for process {} (instance {}): network={}, filesystem={}",
                            process_id, instance_id, net_err, fs_err
                        )));
                    }
                    (Ok(_), Err(fs_err)) => {
                        warn!(
                            "Filesystem isolation failed for process {} (instance {}): {}. Network isolation applied.",
                            process_id, instance_id, fs_err
                        );
                    }
                    (Err(net_err), Ok(_)) => {
                        warn!(
                            "Network isolation failed for process {} (instance {}): {}. Filesystem isolation applied.",
                            process_id, instance_id, net_err
                        );
                    }
                }
            }
            VmIsolation::HardwareVm => {
                // Hardware VM isolation would be handled by the hypervisor
                // For now, just log
                info!(
                    "Hardware VM isolation requested for VM instance {} (not yet implemented)",
                    instance_id
                );
            }
        }

        Ok(())
    }

    /// Remove network and filesystem isolation from a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    ///
    /// # Returns
    /// `Ok(())` if isolation was removed successfully
    pub async fn remove_isolation(&self, process_id: u32) -> Result<(), AppError> {
        self.network_isolator.remove_network_isolation(process_id)?;
        self.filesystem_isolator
            .remove_filesystem_isolation(process_id)?;
        info!("Removed isolation from process {}", process_id);
        Ok(())
    }

    /// Check if network isolation is supported on this platform
    pub fn is_network_isolation_supported(&self) -> bool {
        self.network_isolator.is_supported()
    }

    /// Check if filesystem isolation is supported on this platform
    pub fn is_filesystem_isolation_supported(&self) -> bool {
        self.filesystem_isolator.is_supported()
    }

    /// Restart a VM instance (stop and start)
    ///
    /// This is a manual restart, which resets the restart attempts counter.
    pub async fn restart_instance(&self, id: Uuid) -> Result<(), AppError> {
        let name = {
            let mut instances = self.instances.write().await;
            let inst = instances.get_mut(&id).ok_or_else(|| {
                AppError::ValidationError(format!("VM instance {} not found", id))
            })?;

            // Reset restart attempts on manual restart
            inst.restart_attempts = 0;
            inst.name.clone()
        };

        info!("Restarting VM instance {} ({})", id, name);

        // Stop the instance
        self.stop_instance(id).await?;

        // Wait a bit before restarting
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Start the instance
        self.start_instance(id).await?;

        info!("VM instance {} ({}) restarted successfully", id, name);
        Ok(())
    }

    /// Get auto-recovery configuration for a VM instance
    pub async fn get_auto_recovery_config(&self, id: Uuid) -> Result<AutoRecoveryConfig, AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;
        Ok(inst.auto_recovery.clone())
    }

    /// Get restart attempts count for a VM instance
    pub async fn get_restart_attempts(&self, id: Uuid) -> Result<u32, AppError> {
        let instances = self.instances.read().await;
        let inst = instances
            .get(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;
        Ok(inst.restart_attempts)
    }

    /// Reset restart attempts for a VM instance
    pub async fn reset_restart_attempts(&self, id: Uuid) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;
        let inst = instances
            .get_mut(&id)
            .ok_or_else(|| AppError::ValidationError(format!(
                "VM instance not found: {}. \
                Context: The specified VM instance ID does not exist in the system. \
                Suggestion: Verify the instance ID is correct. Use list_instances() to see all available instances. \
                Instance ID: {}",
                id, id
            )))?;
        inst.restart_attempts = 0;
        info!("Reset restart attempts for VM instance {}", id);
        Ok(())
    }
}

static VM_MANAGER: OnceLock<Arc<VmManager>> = OnceLock::new();

/// Get global VM manager instance.
///
/// This function returns a singleton instance of `VmManager` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
///
/// # Examples
///
/// ```no_run
/// use poolai::vm::get_global_manager;
/// use uuid::Uuid;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let manager = get_global_manager();
///
/// // List all VM instances
/// let instances = manager.list_instances().await;
/// for instance in instances {
///     println!("VM: {} ({:?})", instance.name, instance.status);
/// }
/// # Ok(())
/// # }
/// ```
pub fn get_global_manager() -> Arc<VmManager> {
    VM_MANAGER
        .get_or_init(|| Arc::new(VmManager::new()))
        .clone()
}

/// Initialize the VM module.
///
/// This function initializes the global VM manager instance, including:
/// - Health monitor setup
/// - Periodic health check tasks
/// - Resource limiter initialization
///
/// # Examples
///
/// ```no_run
/// use poolai::vm::initialize;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// // Initialize VM module at application startup
/// initialize().await?;
/// println!("VM module initialized");
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns `AppError::ConfigError` if health monitor initialization fails.
pub async fn initialize() -> Result<(), AppError> {
    get_global_manager().initialize().await
}

/// Shutdown the VM module.
///
/// This function gracefully shuts down the global VM manager instance, including:
/// - Stopping all running VM instances
/// - Cleaning up health check tasks
/// - Releasing resources
///
/// # Examples
///
/// ```no_run
/// use poolai::vm::shutdown;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// // Shutdown VM module at application exit
/// shutdown().await?;
/// println!("VM module shut down");
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns `AppError::ConfigError` if shutdown fails.
pub async fn shutdown() -> Result<(), AppError> {
    get_global_manager().shutdown().await
}
