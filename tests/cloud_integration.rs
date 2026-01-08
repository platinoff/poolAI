//! Integration tests for Cloud Integration module

#[cfg(feature = "cloud")]
use poolai::cloud::{CloudConfig, CloudManager};
#[cfg(feature = "cloud")]
use poolai::core::error::AppError;

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_manager_creation() {
    let config = CloudConfig::default();
    let manager = CloudManager::new(config);
    
    // Manager should be created successfully
    assert!(manager.kubernetes().is_none());
    assert!(manager.aws().is_none());
    assert!(manager.azure().is_none());
    assert!(manager.gcp().is_none());
    assert!(manager.autoscaler().is_none());
    assert!(manager.loadbalancer().is_none());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_manager_initialization() -> Result<(), AppError> {
    let config = CloudConfig::default();
    let manager = CloudManager::new(config);
    
    // Should initialize successfully even with all features disabled
    manager.initialize().await?;
    manager.shutdown().await?;
    
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_kubernetes_manager() {
    use poolai::cloud::kubernetes::KubernetesManager;
    
    let manager = KubernetesManager::new("default".to_string());
    
    // Should create successfully
    assert!(!manager.is_cluster_available().await);
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_autoscaler() {
    use poolai::cloud::autoscaling::AutoScaler;
    
    let autoscaler = AutoScaler::new();
    
    // Should get metrics (placeholder returns default)
    let metrics = autoscaler.get_metrics("test-resource").await.unwrap();
    assert_eq!(metrics.current_replicas, 1);
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.memory_usage, 0.0);
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_loadbalancer() {
    use poolai::cloud::loadbalancing::{LoadBalancer, Backend};
    
    let loadbalancer = LoadBalancer::new();
    
    // Should get health status (placeholder returns default)
    let health = loadbalancer.get_health_status().await.unwrap();
    assert_eq!(health.total_backends, 0);
    assert_eq!(health.healthy_backends, 0);
    assert_eq!(health.unhealthy_backends, 0);
    
    // Should add backend (placeholder)
    let backend = Backend {
        id: "test-backend".to_string(),
        address: "127.0.0.1".to_string(),
        port: 8080,
        weight: 100,
    };
    loadbalancer.add_backend(backend).await.unwrap();
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_manager() {
    use poolai::cloud::providers::aws::AwsManager;
    
    let manager = AwsManager::new(None);
    manager.initialize().await.unwrap();
    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_manager() {
    use poolai::cloud::providers::azure::AzureManager;
    
    let manager = AzureManager::new(None);
    manager.initialize().await.unwrap();
    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_manager() {
    use poolai::cloud::providers::gcp::GcpManager;
    
    let manager = GcpManager::new(None);
    manager.initialize().await.unwrap();
    manager.shutdown().await.unwrap();
}
