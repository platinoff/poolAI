//! Integration tests for VM Write Operations
//!
//! Tests:
//! - Create VM instance
//! - Update VM instance
//! - Delete VM instance
//! - Start/Stop/Restart VM instance
//! - RBAC permission checks (stub - would require full API server)

use poolai::vm::{VmIsolation, VmManager, VmResources, VmStatus};

#[tokio::test]
async fn test_create_vm_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources {
        cpu_cores: 2,
        memory_mb: 1024,
        gpu_required: false,
        gpu_scheduling_policy: None,
    };

    let instance = manager
        .create_instance(
            "test-vm-create".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    assert_eq!(instance.name, "test-vm-create");
    assert_eq!(instance.resources.cpu_cores, 2);
    assert_eq!(instance.resources.memory_mb, 1024);
    assert!(!instance.resources.gpu_required);
    assert!(matches!(
        instance.status,
        VmStatus::Creating | VmStatus::Stopped
    ));

    // Verify instance exists in list
    let instances = manager.list_instances().await;
    assert!(instances.iter().any(|i| i.id == instance.id));
}

#[tokio::test]
async fn test_update_vm_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm-update".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Update name and resources
    let new_resources = VmResources {
        cpu_cores: 4,
        memory_mb: 2048,
        gpu_required: true,
        gpu_scheduling_policy: None,
    };

    let updated = manager
        .update_instance(
            instance.id,
            Some("test-vm-updated".to_string()),
            Some(new_resources),
            None,
            None, // auto_recovery
        )
        .await
        .unwrap();

    assert_eq!(updated.name, "test-vm-updated");
    assert_eq!(updated.resources.cpu_cores, 4);
    assert_eq!(updated.resources.memory_mb, 2048);
    assert!(updated.resources.gpu_required);
}

#[tokio::test]
async fn test_delete_vm_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm-delete".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Verify instance exists
    let instances_before = manager.list_instances().await;
    assert!(instances_before.iter().any(|i| i.id == instance.id));

    // Delete instance
    manager.delete_instance(instance.id).await.unwrap();

    // Verify instance is removed
    let instances_after = manager.list_instances().await;
    assert!(!instances_after.iter().any(|i| i.id == instance.id));

    // Verify get_instance returns None
    let deleted = manager.get_instance(instance.id).await;
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_start_stop_vm_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm-start-stop".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Initially creating or stopped
    let inst_before = manager.get_instance(instance.id).await.unwrap();
    assert!(matches!(
        inst_before.status,
        VmStatus::Creating | VmStatus::Stopped
    ));

    // Start instance
    manager.start_instance(instance.id).await.unwrap();

    let inst_running = manager.get_instance(instance.id).await.unwrap();
    assert!(matches!(inst_running.status, VmStatus::Running));

    // Stop instance
    manager.stop_instance(instance.id).await.unwrap();

    let inst_stopped = manager.get_instance(instance.id).await.unwrap();
    assert!(matches!(inst_stopped.status, VmStatus::Stopped));
}

#[tokio::test]
async fn test_restart_vm_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let resources = VmResources::default();
    let instance = manager
        .create_instance(
            "test-vm-restart".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start instance
    manager.start_instance(instance.id).await.unwrap();
    let inst_before = manager.get_instance(instance.id).await.unwrap();
    assert!(matches!(inst_before.status, VmStatus::Running));

    // Restart instance
    manager.restart_instance(instance.id).await.unwrap();

    // Verify instance is still running after restart
    let inst_after = manager.get_instance(instance.id).await.unwrap();
    assert!(matches!(inst_after.status, VmStatus::Running));
}

#[tokio::test]
async fn test_update_nonexistent_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let nonexistent_id = uuid::Uuid::new_v4();

    let result = manager
        .update_instance(
            nonexistent_id,
            Some("new-name".to_string()),
            None,
            None,
            None, // auto_recovery
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let nonexistent_id = uuid::Uuid::new_v4();

    let result = manager.delete_instance(nonexistent_id).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_start_stop_nonexistent_instance() {
    let manager = VmManager::new();
    manager.initialize().await.unwrap();

    let nonexistent_id = uuid::Uuid::new_v4();

    let start_result = manager.start_instance(nonexistent_id).await;
    assert!(start_result.is_err());

    let stop_result = manager.stop_instance(nonexistent_id).await;
    assert!(stop_result.is_err());
}
