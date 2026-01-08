//! Unit tests for CloudConfig validation

#[cfg(feature = "cloud")]
use poolai::cloud::CloudConfig;
#[cfg(feature = "cloud")]
use poolai::core::error::AppError;

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_default_valid() {
    let config = CloudConfig::default();
    // Default config should be valid (all features disabled)
    assert!(config.validate().is_ok());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_kubernetes_empty_namespace() {
    let mut config = CloudConfig::default();
    config.kubernetes_enabled = true;
    config.kubernetes_namespace = String::new();
    
    let result = config.validate();
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Kubernetes namespace cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_kubernetes_valid() {
    let mut config = CloudConfig::default();
    config.kubernetes_enabled = true;
    config.kubernetes_namespace = "poolai".to_string();
    
    assert!(config.validate().is_ok());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_aws_no_region() {
    let mut config = CloudConfig::default();
    config.aws_enabled = true;
    config.aws_region = None;
    
    // Clear environment variable for test
    std::env::remove_var("AWS_REGION");
    
    let result = config.validate();
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("AWS region must be set"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_aws_with_region() {
    let mut config = CloudConfig::default();
    config.aws_enabled = true;
    config.aws_region = Some("us-east-1".to_string());
    
    assert!(config.validate().is_ok());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_azure_no_subscription() {
    let mut config = CloudConfig::default();
    config.azure_enabled = true;
    config.azure_subscription_id = None;
    
    // Clear environment variable for test
    std::env::remove_var("AZURE_SUBSCRIPTION_ID");
    
    let result = config.validate();
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Azure subscription ID must be set"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_gcp_no_project() {
    let mut config = CloudConfig::default();
    config.gcp_enabled = true;
    config.gcp_project_id = None;
    
    // Clear environment variable for test
    std::env::remove_var("GCP_PROJECT_ID");
    
    let result = config.validate();
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("GCP project ID must be set"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_manager_initialization_validates_config() {
    use poolai::cloud::CloudManager;
    
    let mut config = CloudConfig::default();
    config.kubernetes_enabled = true;
    config.kubernetes_namespace = String::new(); // Invalid
    
    let manager = CloudManager::new(config);
    let result = manager.initialize().await;
    
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Kubernetes namespace cannot be empty"));
    }
}
