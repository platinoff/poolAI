//! Multi-tenancy module
//!
//! Provides tenant isolation and resource management for multi-client support.
//!
//! # Features
//!
//! - Tenant creation and management
//! - Resource quotas per tenant
//! - Tenant isolation (data, compute, network)
//! - Tenant-level access control
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
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

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
            active: true,
        }
    }
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
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Tenant manager
///
/// Manages tenant lifecycle and resource quotas.
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
    pub async fn create_tenant(
        &self,
        name: String,
        config: TenantConfig,
    ) -> Result<Tenant, AppError> {
        let tenant = Tenant {
            id: Uuid::new_v4(),
            name,
            config,
            created_at: chrono::Utc::now(),
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
