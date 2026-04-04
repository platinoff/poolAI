//! VM-facing operations for the HTTP API.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::runtime::health::HealthStatus;
use crate::vm::{
    ResourceUsage, VmInstance, VmIsolation, VmManager, VmNetwork, VmResources, VmTemplate,
};
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

/// Mutations return domain [`AppError`] on failure; HTTP layer maps to status codes.
#[derive(Debug)]
pub enum VmMutationError {
    ManagerUnavailable,
    Operation(AppError),
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

    pub async fn create_instance(
        ctx: &ApiContext,
        name: String,
        resources: VmResources,
        isolation: VmIsolation,
    ) -> Result<VmInstance, VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .create_instance(name, resources, isolation)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn update_instance(
        ctx: &ApiContext,
        id: Uuid,
        name: Option<String>,
        resources: Option<VmResources>,
        isolation: Option<VmIsolation>,
    ) -> Result<VmInstance, VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .update_instance(id, name, resources, isolation, None)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn delete_instance(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .delete_instance(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn start_instance(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .start_instance(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn stop_instance(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .stop_instance(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn restart_instance(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .restart_instance(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn get_instance_health(
        ctx: &ApiContext,
        id: Uuid,
    ) -> Result<Option<HealthStatus>, VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .get_instance_health(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn create_template(
        ctx: &ApiContext,
        template: VmTemplate,
    ) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .create_template(template)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn update_template(
        ctx: &ApiContext,
        template: VmTemplate,
    ) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .update_template(template)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn delete_template(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .delete_template(id)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn create_network(
        ctx: &ApiContext,
        network: VmNetwork,
    ) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .create_network(network)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn update_network(
        ctx: &ApiContext,
        network: VmNetwork,
    ) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .update_network(network)
            .await
            .map_err(VmMutationError::Operation)
    }

    pub async fn delete_network(ctx: &ApiContext, id: Uuid) -> Result<(), VmMutationError> {
        let manager = require_vm_manager(ctx).map_err(|_| VmMutationError::ManagerUnavailable)?;
        manager
            .delete_network(id)
            .await
            .map_err(VmMutationError::Operation)
    }
}
