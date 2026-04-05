//! Enterprise-facing operations for the HTTP API (multi-tenancy, audit, security, …).
//!
//! Handlers in `network::enterprise_api` stay thin: parse input, call `EnterpriseService`, map to HTTP.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::enterprise::multi_tenancy::{
    QuotaCheckResult, Tenant, TenantConfig, TenantResourceUsage,
};
use uuid::Uuid;

/// `create_tenant` runs `TenantManager::initialize` first; distinguish init vs create failures for HTTP mapping.
#[derive(Debug)]
pub enum TenantCreateError {
    Init(AppError),
    Create(AppError),
}

pub struct EnterpriseService;

impl EnterpriseService {
    pub async fn list_tenants(ctx: &ApiContext) -> Result<Vec<Tenant>, AppError> {
        ctx.tenant_manager.list_tenants().await
    }

    pub async fn create_tenant(
        ctx: &ApiContext,
        name: String,
        config: TenantConfig,
    ) -> Result<Tenant, TenantCreateError> {
        if let Err(e) = ctx.tenant_manager.initialize().await {
            return Err(TenantCreateError::Init(e));
        }
        ctx.tenant_manager
            .create_tenant(name, config)
            .await
            .map_err(TenantCreateError::Create)
    }

    pub async fn get_tenant(ctx: &ApiContext, id: Uuid) -> Result<Option<Tenant>, AppError> {
        ctx.tenant_manager.get_tenant(id).await
    }

    pub async fn update_tenant(
        ctx: &ApiContext,
        id: Uuid,
        config: Option<TenantConfig>,
        active: Option<bool>,
    ) -> Result<Tenant, AppError> {
        ctx.tenant_manager.update_tenant(id, config, active).await
    }

    pub async fn delete_tenant(ctx: &ApiContext, id: Uuid) -> Result<(), AppError> {
        ctx.tenant_manager.delete_tenant(id).await
    }

    pub async fn get_tenant_usage(
        ctx: &ApiContext,
        tenant_id: Uuid,
    ) -> Result<TenantResourceUsage, AppError> {
        ctx.tenant_manager.get_usage(tenant_id).await
    }

    pub async fn check_tenant_quota(
        ctx: &ApiContext,
        tenant_id: Uuid,
        workers: usize,
        memory_mb: u64,
        cpu_cores: usize,
        storage_mb: Option<u64>,
        vm_instances: Option<usize>,
    ) -> Result<QuotaCheckResult, AppError> {
        ctx.tenant_manager
            .check_quota(
                tenant_id,
                workers,
                memory_mb,
                cpu_cores,
                storage_mb,
                vm_instances,
            )
            .await
    }
}
