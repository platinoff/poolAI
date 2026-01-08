//! Unit tests for cloud provider managers (AWS, Azure, GCP)

#[cfg(feature = "cloud")]
use poolai::cloud::providers::aws::AwsManager;
#[cfg(feature = "cloud")]
use poolai::cloud::providers::azure::AzureManager;
#[cfg(feature = "cloud")]
use poolai::cloud::providers::gcp::GcpManager;
#[cfg(feature = "cloud")]
use poolai::AppError;

// AWS Tests
#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_manager_creation() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    let _ = manager;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_manager_initialization() -> Result<(), AppError> {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ec2_empty_instance_type() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_ec2_instance("", "ami-12345678").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Instance type cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ec2_empty_image_id() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_ec2_instance("t3.medium", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Image ID (AMI) cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ec2_success() -> Result<(), AppError> {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await?;
    
    let instance_id = manager.create_ec2_instance("t3.medium", "ami-12345678").await?;
    assert!(instance_id.starts_with("i-"));
    
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ecs_task_empty_cluster() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_ecs_task("", "task-def").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Cluster name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ecs_task_empty_definition() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_ecs_task("my-cluster", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Task definition cannot be empty"));
    }
}

// Azure Tests
#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_manager_creation() {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    let _ = manager;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_manager_initialization() -> Result<(), AppError> {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_create_vmss_empty_resource_group() {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_vm_scale_set("", "vmss-name").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Resource group name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_create_vmss_empty_name() {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_vm_scale_set("my-rg", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("VM Scale Set name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_create_vmss_success() -> Result<(), AppError> {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    manager.initialize().await?;
    
    let vmss_id = manager.create_vm_scale_set("my-rg", "vmss-name").await?;
    assert!(!vmss_id.is_empty());
    
    Ok(())
}

// GCP Tests
#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_manager_creation() {
    let manager = GcpManager::new(Some("project-id".to_string()));
    let _ = manager;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_manager_initialization() -> Result<(), AppError> {
    let manager = GcpManager::new(Some("project-id".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_instance_group_empty_name() {
    let manager = GcpManager::new(Some("project-id".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_instance_group("", 3).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Instance group name cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_instance_group_zero_count() {
    let manager = GcpManager::new(Some("project-id".to_string()));
    manager.initialize().await.unwrap();
    
    let result = manager.create_instance_group("ig-name", 0).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Instance count must be greater than 0"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_instance_group_success() -> Result<(), AppError> {
    let manager = GcpManager::new(Some("project-id".to_string()));
    manager.initialize().await?;
    
    let ig_id = manager.create_instance_group("ig-name", 3).await?;
    assert!(ig_id.starts_with("gcp-instance-group-"));
    
    Ok(())
}
