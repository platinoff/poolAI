//! Integration tests for Enterprise Tenant Management
//!
//! Tests:
//! - Tenant creation
//! - Tenant update (configuration, active status)
//! - Tenant deletion
//! - Resource usage tracking
//! - Quota checking

#[cfg(feature = "enterprise")]
use poolai::enterprise::multi_tenancy::{
    get_global_tenant_manager, Tenant, TenantConfig, TenantManager,
};
#[cfg(feature = "enterprise")]
use poolai::core::error::AppError;
#[cfg(feature = "enterprise")]
use uuid::Uuid;

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_tenant_manager_initialization() {
    let manager = get_global_tenant_manager();
    assert!(manager.initialize().await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_create_tenant() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    let tenant = manager
        .create_tenant("test-tenant".to_string(), TenantConfig::default())
        .await
        .unwrap();

    assert_eq!(tenant.name, "test-tenant");
    assert!(tenant.config.active);
    assert_eq!(tenant.usage.workers, 0);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_update_tenant_config() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    // Create tenant
    let tenant = manager
        .create_tenant("test-tenant-update".to_string(), TenantConfig::default())
        .await
        .unwrap();

    // Update tenant configuration
    let new_config = TenantConfig {
        max_workers: Some(20),
        max_memory_mb: Some(2048),
        max_cpu_cores: Some(8),
        max_storage_mb: Some(20000),
        max_vm_instances: Some(10),
        active: true,
    };

    let updated = manager
        .update_tenant(tenant.id, Some(new_config.clone()), None)
        .await
        .unwrap();

    assert_eq!(updated.config.max_workers, Some(20));
    assert_eq!(updated.config.max_memory_mb, Some(2048));
    assert_eq!(updated.config.max_cpu_cores, Some(8));
    assert_eq!(updated.config.max_storage_mb, Some(20000));
    assert_eq!(updated.config.max_vm_instances, Some(10));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_update_tenant_active_status() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    // Create tenant
    let tenant = manager
        .create_tenant("test-tenant-active".to_string(), TenantConfig::default())
        .await
        .unwrap();

    assert!(tenant.config.active);

    // Deactivate tenant
    let updated = manager
        .update_tenant(tenant.id, None, Some(false))
        .await
        .unwrap();

    assert!(!updated.config.active);

    // Reactivate tenant
    let updated = manager
        .update_tenant(tenant.id, None, Some(true))
        .await
        .unwrap();

    assert!(updated.config.active);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_update_tenant_combined() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    // Create tenant
    let tenant = manager
        .create_tenant("test-tenant-combined".to_string(), TenantConfig::default())
        .await
        .unwrap();

    // Update both config and active status
    let new_config = TenantConfig {
        max_workers: Some(15),
        max_memory_mb: Some(1536),
        max_cpu_cores: Some(6),
        max_storage_mb: Some(15000),
        max_vm_instances: Some(8),
        active: false,
    };

    let updated = manager
        .update_tenant(tenant.id, Some(new_config.clone()), Some(false))
        .await
        .unwrap();

    assert_eq!(updated.config.max_workers, Some(15));
    assert!(!updated.config.active);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_update_nonexistent_tenant() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    let nonexistent_id = Uuid::new_v4();
    let result = manager
        .update_tenant(nonexistent_id, None, Some(false))
        .await;

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_delete_tenant() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    // Create tenant
    let tenant = manager
        .create_tenant("test-tenant-delete".to_string(), TenantConfig::default())
        .await
        .unwrap();

    // Delete tenant (should succeed if no resources)
    assert!(manager.delete_tenant(tenant.id).await.is_ok());

    // Verify tenant is deleted
    let retrieved = manager.get_tenant(tenant.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_delete_tenant_with_resources() {
    let manager = get_global_tenant_manager();
    manager.initialize().await.unwrap();

    // Create tenant
    let tenant = manager
        .create_tenant("test-tenant-resources".to_string(), TenantConfig::default())
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
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("active resources"));
    }

    // Remove resources
    manager
        .decrement_usage(tenant.id, 1, 100, 1, None, None)
        .await
        .unwrap();

    // Now delete should succeed
    assert!(manager.delete_tenant(tenant.id).await.is_ok());
}
