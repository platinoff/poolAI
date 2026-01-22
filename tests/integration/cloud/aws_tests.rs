//! Integration tests for AWS cloud provider
//!
//! Tests AWS EC2 and ECS API interactions using mock servers.

#[cfg(feature = "cloud-sdk")]
mod mock_servers;

#[cfg(feature = "cloud-sdk")]
use mock_servers::MockAwsEc2Server;
#[cfg(feature = "cloud-sdk")]
use poolai::cloud::providers::aws::AwsManager;
#[cfg(feature = "cloud-sdk")]
use poolai::core::error::AppError;

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_aws_manager_initialization() -> Result<(), AppError> {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_aws_manager_initialization_with_env_region() -> Result<(), AppError> {
    std::env::set_var("AWS_REGION", "us-west-2");
    let manager = AwsManager::new(None);
    manager.initialize().await?;
    manager.shutdown().await?;
    std::env::remove_var("AWS_REGION");
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_aws_create_ec2_validation() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();

    // Test empty instance type
    let result = manager.create_ec2_instance("", "ami-12345678").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Instance type cannot be empty"));
    }

    // Test empty image ID
    let result = manager.create_ec2_instance("t3.medium", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Image ID (AMI) cannot be empty"));
    }

    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_aws_create_ecs_validation() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();

    // Test empty cluster name
    let result = manager.create_ecs_task("", "task-def").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Cluster name cannot be empty"));
    }

    // Test empty task definition
    let result = manager.create_ecs_task("my-cluster", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Task definition cannot be empty"));
    }

    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_aws_credentials_handling() {
    // Test that manager handles missing credentials gracefully
    std::env::remove_var("AWS_ACCESS_KEY_ID");
    std::env::remove_var("AWS_SECRET_ACCESS_KEY");

    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();

    // Should fail with InitializationError when credentials are missing
    let result = manager
        .create_ec2_instance("t3.medium", "ami-12345678")
        .await;
    assert!(result.is_err());
    if let Err(AppError::InitializationError(msg)) = result {
        assert!(msg.contains("AWS_ACCESS_KEY_ID") || msg.contains("AWS_SECRET_ACCESS_KEY"));
    }

    manager.shutdown().await.unwrap();
}

// Note: Full end-to-end tests with mock servers would require making endpoints configurable
// For now, these tests verify validation, error handling, and basic functionality
