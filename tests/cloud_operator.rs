//! Integration tests for Kubernetes Operator module

#[cfg(feature = "cloud")]
use poolai::cloud::operator::{
    PoolAIOperator, PoolAITenant, PoolAIVM, PoolAIWorker, TenantQuotas, VmResources, VmStorage,
    WorkerResources,
};

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_operator_creation() {
    let operator = PoolAIOperator::new("test-namespace".to_string());
    assert!(!operator.is_running().await);
}

#[cfg(feature = "cloud")]
#[tokio::test]
#[ignore] // Requires actual Kubernetes cluster
async fn test_operator_start_stop() {
    let operator = PoolAIOperator::new("test-namespace".to_string());

    // Initially not running
    assert!(!operator.is_running().await);

    // Start operator
    let result = operator.start().await;
    assert!(result.is_ok());
    assert!(operator.is_running().await);

    // Start again (should be idempotent)
    let result = operator.start().await;
    assert!(result.is_ok());
    assert!(operator.is_running().await);

    // Stop operator
    let result = operator.stop().await;
    assert!(result.is_ok());
    assert!(!operator.is_running().await);

    // Stop again (should be idempotent)
    let result = operator.stop().await;
    assert!(result.is_ok());
    assert!(!operator.is_running().await);
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_operator_start_with_empty_namespace() {
    let operator = PoolAIOperator::new("".to_string());

    let result = operator.start().await;
    assert!(result.is_err());
    assert!(!operator.is_running().await);
}

#[cfg(feature = "cloud")]
#[test]
fn test_poolai_worker_creation() {
    let worker = PoolAIWorker {
        name: "test-worker".to_string(),
        image: "poolai/worker:v1.0.0".to_string(),
        replicas: 3,
        resources: WorkerResources {
            cpu: "500m".to_string(),
            memory: "512Mi".to_string(),
            gpu: Some(1),
        },
        env: None,
    };

    assert_eq!(worker.name, "test-worker");
    assert_eq!(worker.image, "poolai/worker:v1.0.0");
    assert_eq!(worker.replicas, 3);
    assert_eq!(worker.resources.cpu, "500m");
    assert_eq!(worker.resources.memory, "512Mi");
    assert_eq!(worker.resources.gpu, Some(1));
}

#[cfg(feature = "cloud")]
#[test]
fn test_poolai_vm_creation() {
    let vm = PoolAIVM {
        name: "test-vm".to_string(),
        image: "poolai/vm:v1.0.0".to_string(),
        resources: VmResources {
            cpu: "1".to_string(),
            memory: "2Gi".to_string(),
            gpu: None,
        },
        storage: VmStorage {
            size: "20Gi".to_string(),
            storage_class: "ssd".to_string(),
        },
        ports: None,
    };

    assert_eq!(vm.name, "test-vm");
    assert_eq!(vm.image, "poolai/vm:v1.0.0");
    assert_eq!(vm.resources.cpu, "1");
    assert_eq!(vm.resources.memory, "2Gi");
    assert_eq!(vm.storage.size, "20Gi");
    assert_eq!(vm.storage.storage_class, "ssd");
}

#[cfg(feature = "cloud")]
#[test]
fn test_poolai_tenant_creation() {
    let tenant = PoolAITenant {
        name: "tenant-abc".to_string(),
        quotas: TenantQuotas {
            max_workers: Some(10),
            max_memory_mb: Some(1024),
            max_cpu_cores: Some(4),
            max_storage_mb: Some(10000),
        },
        active: true,
    };

    assert_eq!(tenant.name, "tenant-abc");
    assert_eq!(tenant.quotas.max_workers, Some(10));
    assert_eq!(tenant.quotas.max_memory_mb, Some(1024));
    assert_eq!(tenant.quotas.max_cpu_cores, Some(4));
    assert_eq!(tenant.quotas.max_storage_mb, Some(10000));
    assert!(tenant.active);
}

#[cfg(feature = "cloud")]
#[test]
fn test_worker_resources_without_gpu() {
    let resources = WorkerResources {
        cpu: "100m".to_string(),
        memory: "128Mi".to_string(),
        gpu: None,
    };

    assert_eq!(resources.cpu, "100m");
    assert_eq!(resources.memory, "128Mi");
    assert_eq!(resources.gpu, None);
}

#[cfg(feature = "cloud")]
#[test]
fn test_tenant_quotas_partial() {
    let quotas = TenantQuotas {
        max_workers: Some(5),
        max_memory_mb: None,
        max_cpu_cores: Some(2),
        max_storage_mb: None,
    };

    assert_eq!(quotas.max_workers, Some(5));
    assert_eq!(quotas.max_memory_mb, None);
    assert_eq!(quotas.max_cpu_cores, Some(2));
    assert_eq!(quotas.max_storage_mb, None);
}
