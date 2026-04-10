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
    let config = CloudConfig {
        kubernetes_enabled: true,
        kubernetes_namespace: String::new(),
        ..Default::default()
    };

    let result = config.validate();
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Kubernetes namespace cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_kubernetes_valid() {
    let config = CloudConfig {
        kubernetes_enabled: true,
        kubernetes_namespace: "poolai".to_string(),
        ..Default::default()
    };

    assert!(config.validate().is_ok());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_aws_no_region() {
    let config = CloudConfig {
        aws_enabled: true,
        aws_region: None,
        ..Default::default()
    };

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
    let config = CloudConfig {
        aws_enabled: true,
        aws_region: Some("us-east-1".to_string()),
        ..Default::default()
    };

    assert!(config.validate().is_ok());
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_cloud_config_azure_no_subscription() {
    let config = CloudConfig {
        azure_enabled: true,
        azure_subscription_id: None,
        ..Default::default()
    };

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
    let config = CloudConfig {
        gcp_enabled: true,
        gcp_project_id: None,
        ..Default::default()
    };

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

    let config = CloudConfig {
        kubernetes_enabled: true,
        kubernetes_namespace: String::new(), // Invalid
        ..Default::default()
    };

    let manager = CloudManager::new(config);
    let result = manager.initialize().await;

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Kubernetes namespace cannot be empty"));
    }
}
