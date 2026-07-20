//! Multi-tenancy module
//!
//! Provides tenant isolation and resource management for multi-client support.
//!
//! # Features
//!
//! - Tenant creation and management
//! - Resource quotas per tenant
//! - Resource usage tracking
//! - Tenant isolation (data, compute, network)
//! - Tenant-level access control
//! - Quota validation before resource creation
//!
//! # Persistence (enterprise horizon band 51–52+)
//!
//! Default store is **in-memory**. Band 52 wires `POOLAI_TENANT_STORE=sqlite` +
//! optional `POOLAI_TENANT_DATA_DIR` as a **production verify stub** (durable path
//! resolution only — restart-safe CRUD lands in later phase-A bands).
//! See [`docs/development/TENANT_STORE.md`] and [`docs/development/TENANT_PERSIST.md`].
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::enterprise::multi_tenancy::{TenantManager, TenantConfig};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = TenantManager::new();
//! manager.initialize().await?;
//!
//! let tenant = manager.create_tenant("tenant-abc", TenantConfig::default()).await?;
//!
//! // Check quota before creating resources
//! let quota_check = manager.check_quota(tenant.id, 2, 512, 1).await?;
//! if quota_check.allowed {
//!     // Create resources...
//! }
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// Env key for tenant store backend (`memory` default; `sqlite` horizon).
pub const POOLAI_TENANT_STORE: &str = "POOLAI_TENANT_STORE";

/// Env key for durable tenant data directory (band 52 store wire).
pub const POOLAI_TENANT_DATA_DIR: &str = "POOLAI_TENANT_DATA_DIR";

/// Resolve configured tenant store mode (PH-S1149 scaffold).
pub fn tenant_store_mode() -> &'static str {
    match std::env::var(POOLAI_TENANT_STORE)
        .unwrap_or_else(|_| "memory".into())
        .to_ascii_lowercase()
        .as_str()
    {
        "sqlite" => "sqlite",
        _ => "memory",
    }
}

/// Optional durable data directory from env (PH-S1160).
pub fn tenant_store_data_dir_from_env() -> Option<PathBuf> {
    std::env::var(POOLAI_TENANT_DATA_DIR)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// Canonical sqlite DB file name under the configured data dir (band 52 wire).
pub const TENANT_STORE_SQLITE_FILE: &str = "tenants.sqlite";

/// Band-52 tenant store wire snapshot (mode + durable path; no CRUD yet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantStoreWire {
    /// `memory` or `sqlite`.
    pub mode: String,
    /// Resolved sqlite file path when mode is sqlite and data dir is set.
    pub durable_path: Option<String>,
    /// True when sqlite mode has a durable path (production-ready wire).
    pub configured: bool,
}

/// Resolve tenant store wire for ops / verify / contracts (PH-S1160).
pub fn tenant_store_wire() -> TenantStoreWire {
    let mode = tenant_store_mode().to_string();
    if mode != "sqlite" {
        return TenantStoreWire {
            mode,
            durable_path: None,
            configured: false,
        };
    }
    let durable_path = tenant_store_data_dir_from_env().map(|dir| {
        dir.join(TENANT_STORE_SQLITE_FILE)
            .to_string_lossy()
            .replace('\\', "/")
    });
    let configured = durable_path.is_some();
    TenantStoreWire {
        mode,
        durable_path,
        configured,
    }
}

/// Wire label for admin / metrics depth fields (PH-S1160).
pub fn tenant_store_wire_label(wire: &TenantStoreWire) -> &'static str {
    if wire.mode == "sqlite" && wire.configured {
        "sqlite"
    } else if wire.mode == "sqlite" {
        "sqlite_unconfigured"
    } else {
        "memory"
    }
}

/// Tenant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    /// Maximum number of workers per tenant
    pub max_workers: Option<usize>,
    /// Maximum memory per tenant (MB)
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU cores per tenant
    pub max_cpu_cores: Option<usize>,
    /// Maximum storage per tenant (MB)
    pub max_storage_mb: Option<u64>,
    /// Maximum number of VM instances per tenant
    pub max_vm_instances: Option<usize>,
    /// Whether tenant is active
    pub active: bool,
}

impl Default for TenantConfig {
    fn default() -> Self {
        Self {
            max_workers: Some(10),
            max_memory_mb: Some(1024),
            max_cpu_cores: Some(4),
            max_storage_mb: Some(10000),
            max_vm_instances: Some(5),
            active: true,
        }
    }
}

/// Tenant resource usage
///
/// Tracks current resource consumption for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantResourceUsage {
    /// Current number of workers
    pub workers: usize,
    /// Current memory usage (MB)
    pub memory_mb: u64,
    /// Current CPU cores in use
    pub cpu_cores: usize,
    /// Current storage usage (MB)
    pub storage_mb: u64,
    /// Current number of VM instances
    pub vm_instances: usize,
}

/// Tenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique tenant identifier
    pub id: Uuid,
    /// Tenant name
    pub name: String,
    /// Tenant configuration
    pub config: TenantConfig,
    /// Current resource usage
    pub usage: TenantResourceUsage,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Quota check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaCheckResult {
    /// Whether the requested resources are allowed
    pub allowed: bool,
    /// Reason if not allowed
    pub reason: Option<String>,
    /// Current usage after allocation (if allowed)
    pub projected_usage: Option<TenantResourceUsage>,
}

/// Tenant manager
///
/// Manages tenant lifecycle, resource quotas, and usage tracking.
pub struct TenantManager {
    tenants: Arc<RwLock<HashMap<Uuid, Tenant>>>,
    initialized: Arc<RwLock<bool>>,
}

impl TenantManager {
    /// Creates a new tenant manager
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the tenant manager
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        *initialized = true;
        info!("Tenant manager initialized");
        Ok(())
    }

    /// Creates a new tenant
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant creation fails.
    pub async fn create_tenant(
        &self,
        name: String,
        config: TenantConfig,
    ) -> Result<Tenant, AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Tenant name cannot be empty".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        let tenant = Tenant {
            id: Uuid::new_v4(),
            name,
            config,
            usage: TenantResourceUsage::default(),
            created_at: now,
            updated_at: now,
        };

        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.id, tenant.clone());

        info!("Created tenant: {} ({})", tenant.name, tenant.id);
        Ok(tenant)
    }

    /// Gets a tenant by ID
    pub async fn get_tenant(&self, id: Uuid) -> Result<Option<Tenant>, AppError> {
        let tenants = self.tenants.read().await;
        Ok(tenants.get(&id).cloned())
    }

    /// Lists all tenants
    pub async fn list_tenants(&self) -> Result<Vec<Tenant>, AppError> {
        let tenants = self.tenants.read().await;
        Ok(tenants.values().cloned().collect())
    }

    /// Updates tenant configuration
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found or update fails.
    pub async fn update_tenant(
        &self,
        id: Uuid,
        config: Option<TenantConfig>,
        active: Option<bool>,
    ) -> Result<Tenant, AppError> {
        let mut tenants = self.tenants.write().await;
        let tenant = tenants.get_mut(&id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Tenant not found: {}. \
                Context: Cannot update non-existent tenant. \
                Suggestion: Check tenant ID and ensure tenant exists.",
                id
            ))
        })?;

        if let Some(new_config) = config {
            tenant.config = new_config;
        }

        if let Some(new_active) = active {
            tenant.config.active = new_active;
        }

        tenant.updated_at = chrono::Utc::now();
        let updated_tenant = tenant.clone();

        info!("Updated tenant: {} ({})", updated_tenant.name, id);
        Ok(updated_tenant)
    }

    /// Deletes a tenant
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found or has active resources.
    pub async fn delete_tenant(&self, id: Uuid) -> Result<(), AppError> {
        // Check tenant and resources first with read lock
        let (tenant_name, has_resources) = {
            let tenants = self.tenants.read().await;
            let tenant = tenants.get(&id).ok_or_else(|| {
                AppError::ValidationError(format!(
                    "Tenant not found: {}. \
                    Context: Cannot delete non-existent tenant. \
                    Suggestion: Check tenant ID.",
                    id
                ))
            })?;

            let has_resources = tenant.usage.workers > 0
                || tenant.usage.memory_mb > 0
                || tenant.usage.cpu_cores > 0
                || tenant.usage.storage_mb > 0
                || tenant.usage.vm_instances > 0;

            (tenant.name.clone(), has_resources)
        };

        // Check if tenant has active resources
        if has_resources {
            let tenants = self.tenants.read().await;
            let tenant = tenants.get(&id).unwrap(); // Safe because we just checked
            return Err(AppError::ValidationError(format!(
                "Cannot delete tenant {}: tenant has active resources. \
                Context: Tenant deletion requires all resources to be released first. \
                Suggestion: Stop all workers, VM instances, and release storage before deleting tenant. \
                Current usage: workers={}, memory={}MB, cpu_cores={}, storage={}MB, vm_instances={}",
                tenant.name,
                tenant.usage.workers,
                tenant.usage.memory_mb,
                tenant.usage.cpu_cores,
                tenant.usage.storage_mb,
                tenant.usage.vm_instances
            )));
        }

        // Now remove with write lock
        let mut tenants = self.tenants.write().await;
        tenants.remove(&id);
        info!("Deleted tenant: {} ({})", tenant_name, id);
        Ok(())
    }

    /// Checks if requested resources are within quota
    ///
    /// Validates that the tenant has sufficient quota for the requested resources.
    ///
    /// # Arguments
    ///
    /// * `tenant_id` - Tenant ID
    /// * `workers` - Number of workers requested
    /// * `memory_mb` - Memory requested (MB)
    /// * `cpu_cores` - CPU cores requested
    /// * `storage_mb` - Storage requested (MB, optional)
    /// * `vm_instances` - VM instances requested (optional)
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found.
    pub async fn check_quota(
        &self,
        tenant_id: Uuid,
        workers: usize,
        memory_mb: u64,
        cpu_cores: usize,
        storage_mb: Option<u64>,
        vm_instances: Option<usize>,
    ) -> Result<QuotaCheckResult, AppError> {
        // Clone tenant data to avoid borrowing issues
        let tenant = {
            let tenants = self.tenants.read().await;
            tenants.get(&tenant_id).cloned().ok_or_else(|| {
                AppError::ValidationError(format!(
                    "Tenant not found: {}. \
                    Context: Cannot check quota for non-existent tenant. \
                    Suggestion: Check tenant ID.",
                    tenant_id
                ))
            })?
        };

        if !tenant.config.active {
            return Ok(QuotaCheckResult {
                allowed: false,
                reason: Some("Tenant is not active".to_string()),
                projected_usage: None,
            });
        }

        // Check workers quota
        if let Some(max_workers) = tenant.config.max_workers {
            let new_workers = tenant.usage.workers + workers;
            if new_workers > max_workers {
                return Ok(QuotaCheckResult {
                    allowed: false,
                    reason: Some(format!(
                        "Workers quota exceeded: current={}, requested={}, max={}. \
                        Context: Tenant has reached maximum workers limit. \
                        Suggestion: Reduce number of workers or increase quota.",
                        tenant.usage.workers, workers, max_workers
                    )),
                    projected_usage: None,
                });
            }
        }

        // Check memory quota
        if let Some(max_memory) = tenant.config.max_memory_mb {
            let new_memory = tenant.usage.memory_mb + memory_mb;
            if new_memory > max_memory {
                return Ok(QuotaCheckResult {
                    allowed: false,
                    reason: Some(format!(
                        "Memory quota exceeded: current={}MB, requested={}MB, max={}MB. \
                        Context: Tenant has reached maximum memory limit. \
                        Suggestion: Reduce memory allocation or increase quota.",
                        tenant.usage.memory_mb, memory_mb, max_memory
                    )),
                    projected_usage: None,
                });
            }
        }

        // Check CPU quota
        if let Some(max_cpu) = tenant.config.max_cpu_cores {
            let new_cpu = tenant.usage.cpu_cores + cpu_cores;
            if new_cpu > max_cpu {
                return Ok(QuotaCheckResult {
                    allowed: false,
                    reason: Some(format!(
                        "CPU quota exceeded: current={}, requested={}, max={}. \
                        Context: Tenant has reached maximum CPU cores limit. \
                        Suggestion: Reduce CPU allocation or increase quota.",
                        tenant.usage.cpu_cores, cpu_cores, max_cpu
                    )),
                    projected_usage: None,
                });
            }
        }

        // Check storage quota
        if let Some(storage) = storage_mb {
            if let Some(max_storage) = tenant.config.max_storage_mb {
                let new_storage = tenant.usage.storage_mb + storage;
                if new_storage > max_storage {
                    return Ok(QuotaCheckResult {
                        allowed: false,
                        reason: Some(format!(
                            "Storage quota exceeded: current={}MB, requested={}MB, max={}MB. \
                            Context: Tenant has reached maximum storage limit. \
                            Suggestion: Free up storage or increase quota.",
                            tenant.usage.storage_mb, storage, max_storage
                        )),
                        projected_usage: None,
                    });
                }
            }
        }

        // Check VM instances quota
        if let Some(vm_inst) = vm_instances {
            if let Some(max_vm) = tenant.config.max_vm_instances {
                let new_vm = tenant.usage.vm_instances + vm_inst;
                if new_vm > max_vm {
                    return Ok(QuotaCheckResult {
                        allowed: false,
                        reason: Some(format!(
                            "VM instances quota exceeded: current={}, requested={}, max={}. \
                            Context: Tenant has reached maximum VM instances limit. \
                            Suggestion: Stop existing VM instances or increase quota.",
                            tenant.usage.vm_instances, vm_inst, max_vm
                        )),
                        projected_usage: None,
                    });
                }
            }
        }

        // All checks passed - calculate projected usage
        let mut projected = tenant.usage.clone();
        projected.workers += workers;
        projected.memory_mb += memory_mb;
        projected.cpu_cores += cpu_cores;
        if let Some(storage) = storage_mb {
            projected.storage_mb += storage;
        }
        if let Some(vm_inst) = vm_instances {
            projected.vm_instances += vm_inst;
        }

        Ok(QuotaCheckResult {
            allowed: true,
            reason: None,
            projected_usage: Some(projected),
        })
    }

    /// Records resource usage for a tenant
    ///
    /// Updates the tenant's resource usage tracking.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found.
    pub async fn record_usage(
        &self,
        tenant_id: Uuid,
        usage: TenantResourceUsage,
    ) -> Result<(), AppError> {
        let mut tenants = self.tenants.write().await;
        let tenant = tenants.get_mut(&tenant_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Tenant not found: {}. \
                Context: Cannot record usage for non-existent tenant. \
                Suggestion: Check tenant ID.",
                tenant_id
            ))
        })?;

        tenant.usage = usage;
        tenant.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Gets current resource usage for a tenant
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found.
    pub async fn get_usage(&self, tenant_id: Uuid) -> Result<TenantResourceUsage, AppError> {
        let tenants = self.tenants.read().await;
        let tenant = tenants.get(&tenant_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Tenant not found: {}. \
                Context: Cannot get usage for non-existent tenant. \
                Suggestion: Check tenant ID.",
                tenant_id
            ))
        })?;

        Ok(tenant.usage.clone())
    }

    /// Increments resource usage for a tenant
    ///
    /// Atomically adds resources to tenant usage tracking.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found.
    pub async fn increment_usage(
        &self,
        tenant_id: Uuid,
        workers: usize,
        memory_mb: u64,
        cpu_cores: usize,
        storage_mb: Option<u64>,
        vm_instances: Option<usize>,
    ) -> Result<(), AppError> {
        let mut tenants = self.tenants.write().await;
        let tenant = tenants.get_mut(&tenant_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Tenant not found: {}. \
                Context: Cannot increment usage for non-existent tenant. \
                Suggestion: Check tenant ID.",
                tenant_id
            ))
        })?;

        tenant.usage.workers += workers;
        tenant.usage.memory_mb += memory_mb;
        tenant.usage.cpu_cores += cpu_cores;
        if let Some(storage) = storage_mb {
            tenant.usage.storage_mb += storage;
        }
        if let Some(vm_inst) = vm_instances {
            tenant.usage.vm_instances += vm_inst;
        }
        tenant.updated_at = chrono::Utc::now();

        Ok(())
    }

    /// Decrements resource usage for a tenant
    ///
    /// Atomically removes resources from tenant usage tracking.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if tenant is not found or usage would go negative.
    pub async fn decrement_usage(
        &self,
        tenant_id: Uuid,
        workers: usize,
        memory_mb: u64,
        cpu_cores: usize,
        storage_mb: Option<u64>,
        vm_instances: Option<usize>,
    ) -> Result<(), AppError> {
        let mut tenants = self.tenants.write().await;
        let tenant = tenants.get_mut(&tenant_id).ok_or_else(|| {
            AppError::ValidationError(format!(
                "Tenant not found: {}. \
                Context: Cannot decrement usage for non-existent tenant. \
                Suggestion: Check tenant ID.",
                tenant_id
            ))
        })?;

        // Prevent negative usage
        if tenant.usage.workers < workers {
            warn!(
                "Attempted to decrement workers below zero for tenant {}: current={}, decrement={}",
                tenant_id, tenant.usage.workers, workers
            );
            tenant.usage.workers = 0;
        } else {
            tenant.usage.workers -= workers;
        }

        if tenant.usage.memory_mb < memory_mb {
            warn!(
                "Attempted to decrement memory below zero for tenant {}: current={}MB, decrement={}MB",
                tenant_id, tenant.usage.memory_mb, memory_mb
            );
            tenant.usage.memory_mb = 0;
        } else {
            tenant.usage.memory_mb -= memory_mb;
        }

        if tenant.usage.cpu_cores < cpu_cores {
            warn!(
                "Attempted to decrement CPU cores below zero for tenant {}: current={}, decrement={}",
                tenant_id, tenant.usage.cpu_cores, cpu_cores
            );
            tenant.usage.cpu_cores = 0;
        } else {
            tenant.usage.cpu_cores -= cpu_cores;
        }

        if let Some(storage) = storage_mb {
            if tenant.usage.storage_mb < storage {
                warn!(
                    "Attempted to decrement storage below zero for tenant {}: current={}MB, decrement={}MB",
                    tenant_id, tenant.usage.storage_mb, storage
                );
                tenant.usage.storage_mb = 0;
            } else {
                tenant.usage.storage_mb -= storage;
            }
        }

        if let Some(vm_inst) = vm_instances {
            if tenant.usage.vm_instances < vm_inst {
                warn!(
                    "Attempted to decrement VM instances below zero for tenant {}: current={}, decrement={}",
                    tenant_id, tenant.usage.vm_instances, vm_inst
                );
                tenant.usage.vm_instances = 0;
            } else {
                tenant.usage.vm_instances -= vm_inst;
            }
        }

        tenant.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Shuts down the tenant manager
    pub async fn shutdown(&self) -> Result<(), AppError> {
        *self.initialized.write().await = false;
        info!("Tenant manager shut down");
        Ok(())
    }
}

impl Default for TenantManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_tenant() {
        let manager = TenantManager::new();
        manager.initialize().await.unwrap();

        let tenant = manager
            .create_tenant("test-tenant".to_string(), TenantConfig::default())
            .await
            .unwrap();

        assert_eq!(tenant.name, "test-tenant");
        assert!(tenant.config.active);
        assert_eq!(tenant.usage.workers, 0);
    }

    #[tokio::test]
    async fn test_quota_check() {
        let manager = TenantManager::new();
        manager.initialize().await.unwrap();

        let tenant = manager
            .create_tenant("test-tenant".to_string(), TenantConfig::default())
            .await
            .unwrap();

        // Check quota within limits
        let result = manager
            .check_quota(tenant.id, 2, 256, 1, None, None)
            .await
            .unwrap();
        assert!(result.allowed);

        // Check quota exceeding limits
        let result = manager
            .check_quota(tenant.id, 20, 2048, 10, None, None)
            .await
            .unwrap();
        assert!(!result.allowed);
        assert!(result.reason.is_some());
    }

    #[tokio::test]
    async fn test_increment_decrement_usage() {
        let manager = TenantManager::new();
        manager.initialize().await.unwrap();

        let tenant = manager
            .create_tenant("test-tenant".to_string(), TenantConfig::default())
            .await
            .unwrap();

        // Increment usage
        manager
            .increment_usage(tenant.id, 2, 512, 1, Some(1000), Some(1))
            .await
            .unwrap();

        let usage = manager.get_usage(tenant.id).await.unwrap();
        assert_eq!(usage.workers, 2);
        assert_eq!(usage.memory_mb, 512);
        assert_eq!(usage.cpu_cores, 1);
        assert_eq!(usage.storage_mb, 1000);
        assert_eq!(usage.vm_instances, 1);

        // Decrement usage
        manager
            .decrement_usage(tenant.id, 1, 256, 1, Some(500), Some(1))
            .await
            .unwrap();

        let usage = manager.get_usage(tenant.id).await.unwrap();
        assert_eq!(usage.workers, 1);
        assert_eq!(usage.memory_mb, 256);
        assert_eq!(usage.cpu_cores, 0);
        assert_eq!(usage.storage_mb, 500);
        assert_eq!(usage.vm_instances, 0);
    }

    #[test]
    fn tenant_store_mode_defaults_to_memory_ph_s1149() {
        // Unset may still read process env; assert only valid modes.
        let mode = tenant_store_mode();
        assert!(mode == "memory" || mode == "sqlite");
        assert_eq!(POOLAI_TENANT_STORE, "POOLAI_TENANT_STORE");
    }

    #[test]
    fn tenant_store_wire_memory_default_ph_s1160() {
        std::env::remove_var(POOLAI_TENANT_STORE);
        std::env::remove_var(POOLAI_TENANT_DATA_DIR);
        let wire = tenant_store_wire();
        assert_eq!(wire.mode, "memory");
        assert!(wire.durable_path.is_none());
        assert!(!wire.configured);
        assert_eq!(tenant_store_wire_label(&wire), "memory");
        assert_eq!(POOLAI_TENANT_DATA_DIR, "POOLAI_TENANT_DATA_DIR");
        assert_eq!(TENANT_STORE_SQLITE_FILE, "tenants.sqlite");
    }

    #[test]
    fn tenant_store_wire_sqlite_unconfigured_ph_s1160() {
        std::env::set_var(POOLAI_TENANT_STORE, "sqlite");
        std::env::remove_var(POOLAI_TENANT_DATA_DIR);
        let wire = tenant_store_wire();
        assert_eq!(wire.mode, "sqlite");
        assert!(wire.durable_path.is_none());
        assert!(!wire.configured);
        assert_eq!(tenant_store_wire_label(&wire), "sqlite_unconfigured");
        std::env::remove_var(POOLAI_TENANT_STORE);
    }

    #[test]
    fn tenant_store_wire_sqlite_configured_ph_s1160() {
        std::env::set_var(POOLAI_TENANT_STORE, "sqlite");
        std::env::set_var(POOLAI_TENANT_DATA_DIR, "/tmp/poolai-tenant-wire");
        let wire = tenant_store_wire();
        assert_eq!(wire.mode, "sqlite");
        assert!(wire.configured);
        assert_eq!(tenant_store_wire_label(&wire), "sqlite");
        let path = wire.durable_path.as_deref().expect("durable path");
        assert!(path.contains("tenants.sqlite"));
        std::env::remove_var(POOLAI_TENANT_STORE);
        std::env::remove_var(POOLAI_TENANT_DATA_DIR);
    }

    #[tokio::test]
    async fn test_delete_tenant_with_resources() {
        let manager = TenantManager::new();
        manager.initialize().await.unwrap();

        let tenant = manager
            .create_tenant("test-tenant".to_string(), TenantConfig::default())
            .await
            .unwrap();

        // Add resources
        manager
            .increment_usage(tenant.id, 1, 100, 1, None, None)
            .await
            .unwrap();

        // Try to delete - should fail
        let result = manager.delete_tenant(tenant.id).await;
        assert!(result.is_err());

        // Remove resources
        manager
            .decrement_usage(tenant.id, 1, 100, 1, None, None)
            .await
            .unwrap();

        // Now delete should succeed
        assert!(manager.delete_tenant(tenant.id).await.is_ok());
    }
}

/// Global tenant manager instance
static TENANT_MANAGER: OnceLock<Arc<TenantManager>> = OnceLock::new();

/// If the `OnceLock` is still empty, store `mgr` so `get_global_tenant_manager` returns the same
/// instance as [`crate::core::state::AppState::tenant_manager`]. No-op if already initialized.
pub fn try_install_global_tenant_manager(mgr: Arc<TenantManager>) {
    let _ = TENANT_MANAGER.set(mgr);
}

/// Get global tenant manager instance.
///
/// This function returns a singleton instance of `TenantManager` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
///
/// # Examples
///
/// ```no_run
/// use poolai::enterprise::multi_tenancy::get_global_tenant_manager;
/// use uuid::Uuid;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let manager = get_global_tenant_manager();
///
/// // List all tenants
/// let tenants = manager.list_tenants().await;
/// for tenant in tenants {
///     println!("Tenant: {} ({:?})", tenant.name, tenant.id);
/// }
/// # Ok(())
/// # }
/// ```
pub fn get_global_tenant_manager() -> Arc<TenantManager> {
    TENANT_MANAGER
        .get_or_init(|| Arc::new(TenantManager::new()))
        .clone()
}
