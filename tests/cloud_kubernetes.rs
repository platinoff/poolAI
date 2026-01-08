//! Unit tests for KubernetesManager operations

#[cfg(feature = "cloud")]
use poolai::cloud::kubernetes::KubernetesManager;
#[cfg(feature = "cloud")]
use poolai::AppError;

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_kubernetes_manager_creation() {
    let manager = KubernetesManager::new("test-namespace".to_string());
    // Just verify it can be created
    let _ = manager;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_kubernetes_manager_initialization() -> Result<(), AppError> {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_get_pod_status_empty_name() {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await.unwrap();

    let result = manager.get_pod_status("").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Pod name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_get_pod_status_success() -> Result<(), AppError> {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await?;

    let status = manager.get_pod_status("my-pod").await?;
    assert_eq!(status.name, "my-pod");
    assert_eq!(status.phase, "Running");
    assert!(status.ready);

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_deployment_empty_name() {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await.unwrap();

    let result = manager.scale_deployment("", 3).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Deployment name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_deployment_negative_replicas() {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await.unwrap();

    let result = manager.scale_deployment("my-deployment", -1).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Replicas must be non-negative"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_deployment_success() -> Result<(), AppError> {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await?;

    manager.scale_deployment("my-deployment", 5).await?;
    manager.scale_deployment("my-deployment", 0).await?; // Scale to 0 is valid

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_is_cluster_available() {
    let manager = KubernetesManager::new("test-namespace".to_string());
    // Placeholder always returns false
    assert!(!manager.is_cluster_available().await);
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_list_pods() -> Result<(), AppError> {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await?;

    let pods = manager.list_pods().await?;
    // Placeholder returns empty list
    assert_eq!(pods.len(), 0);

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_list_deployments() -> Result<(), AppError> {
    let manager = KubernetesManager::new("test-namespace".to_string());
    manager.initialize().await?;

    let deployments = manager.list_deployments().await?;
    // Placeholder returns empty list
    assert_eq!(deployments.len(), 0);

    Ok(())
}
