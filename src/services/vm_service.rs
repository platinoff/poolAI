//! VM-facing operations for the HTTP API.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::vm::{ResourceUsage, VmInstance, VmManager, VmNetwork, VmTemplate};
use std::sync::Arc;
use uuid::Uuid;

pub const VM_MANAGER_UNAVAILABLE_MESSAGE: &str =
    "VM manager not initialized. Suggestion: complete application startup (vm::initialize).";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmServiceError {
    ManagerUnavailable,
}

#[derive(Debug)]
pub enum VmResourceUsageError {
    ManagerUnavailable,
    Query(AppError),
}

fn require_vm_manager(ctx: &ApiContext) -> Result<Arc<VmManager>, VmServiceError> {
    ctx.vm_manager
        .get()
        .cloned()
        .ok_or(VmServiceError::ManagerUnavailable)
}

pub struct VmService;

impl VmService {
    pub async fn list_instances(ctx: &ApiContext) -> Result<Vec<VmInstance>, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.list_instances().await)
    }

    pub fn is_resource_limits_supported(ctx: &ApiContext) -> Result<bool, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.is_resource_limits_supported())
    }

    pub async fn get_instance_resource_usage(
        ctx: &ApiContext,
        instance_id: Uuid,
    ) -> Result<ResourceUsage, VmResourceUsageError> {
        let manager =
            require_vm_manager(ctx).map_err(|_| VmResourceUsageError::ManagerUnavailable)?;
        manager
            .get_instance_resource_usage(instance_id)
            .await
            .map_err(VmResourceUsageError::Query)
    }

    pub async fn list_templates(ctx: &ApiContext) -> Result<Vec<VmTemplate>, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.list_templates().await)
    }

    pub async fn get_template(
        ctx: &ApiContext,
        id: Uuid,
    ) -> Result<Option<VmTemplate>, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.get_template(id).await)
    }

    pub async fn list_networks(ctx: &ApiContext) -> Result<Vec<VmNetwork>, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.list_networks().await)
    }

    pub async fn get_network(
        ctx: &ApiContext,
        id: Uuid,
    ) -> Result<Option<VmNetwork>, VmServiceError> {
        let manager = require_vm_manager(ctx)?;
        Ok(manager.get_network(id).await)
    }
}
