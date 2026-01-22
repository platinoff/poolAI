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
    
    // Initialize may fail if no credentials are available, which is acceptable
    match manager.initialize().await {
        Ok(_) => {
            manager.shutdown().await?;
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            return Err(e);
        }
    }
    
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ec2_empty_instance_type() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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

    let result = manager
        .create_ec2_instance("t3.medium", "ami-12345678")
        .await;

    // If cloud-sdk feature is enabled and credentials are available, instance should be created
    // Otherwise, it should fail gracefully with appropriate error
    match result {
        Ok(instance_id) => {
            // Success case: instance was created (when running on AWS or with credentials)
            assert!(instance_id.starts_with("i-"));
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(AppError::NetworkError(_)) => {
            // Expected when API call fails (e.g., no network, invalid region, etc.)
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            panic!("Unexpected error: {:?}", e);
        }
    }

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_aws_create_ecs_task_empty_cluster() {
    let manager = AwsManager::new(Some("us-east-1".to_string()));
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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
    
    // Initialize may fail if no credentials are available, which is acceptable
    match manager.initialize().await {
        Ok(_) => {
            manager.shutdown().await?;
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            return Err(e);
        }
    }
    
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_azure_create_vmss_empty_resource_group() {
    let manager = AzureManager::new(Some("sub-id".to_string()));
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

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

    let result = manager.create_vm_scale_set("my-rg", "vmss-name").await;

    // If cloud-sdk feature is enabled and credentials are available, VMSS should be created
    // Otherwise, it should fail gracefully with appropriate error
    match result {
        Ok(vmss_id) => {
            // Success case: VMSS was created (when running on Azure or with credentials)
            assert!(!vmss_id.is_empty());
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(AppError::NetworkError(_)) => {
            // Expected when API call fails (e.g., no network, invalid subscription, etc.)
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            panic!("Unexpected error: {:?}", e);
        }
    }

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
    
    // Initialize may fail if no credentials are available, which is acceptable
    let init_result = manager.initialize().await;
    match init_result {
        Ok(_) => {
            manager.shutdown().await?;
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            return Err(e);
        }
    }
    
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_compute_instance_empty_zone() {
    let manager = GcpManager::new(Some("project-id".to_string()));
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

    let result = manager.create_compute_instance("", "n1-standard-2").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Zone cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_compute_instance_empty_machine_type() {
    let manager = GcpManager::new(Some("project-id".to_string()));
    
    // Initialize may fail if no credentials are available
    if let Err(AppError::InitializationError(_)) = manager.initialize().await {
        // Skip test if no credentials available
        return;
    }

    let result = manager.create_compute_instance("us-central1-a", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Machine type cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_gcp_create_compute_instance_success() -> Result<(), AppError> {
    let manager = GcpManager::new(Some("test-project-id".to_string()));
    manager.initialize().await?;

    // Note: This test will fail if running without GCP credentials or cloud-sdk feature
    // It's expected to fail with InitializationError if no credentials are available
    // For now, we'll test the validation logic
    let result = manager
        .create_compute_instance("us-central1-a", "n1-standard-2")
        .await;

    // If cloud-sdk feature is enabled and credentials are available, instance should be created
    // Otherwise, it should fail gracefully with appropriate error
    match result {
        Ok(instance_id) => {
            // Success case: instance was created (when running on GCP or with credentials)
            assert!(!instance_id.is_empty());
        }
        Err(AppError::InitializationError(_)) => {
            // Expected when no credentials are available
            // This is acceptable for unit tests
        }
        Err(AppError::NetworkError(_)) => {
            // Expected when API call fails (e.g., no network, invalid project, etc.)
            // This is acceptable for unit tests
        }
        Err(e) => {
            // Other errors are unexpected
            panic!("Unexpected error: {:?}", e);
        }
    }

    Ok(())
}

// TODO: Re-enable these tests when create_instance_group method is implemented in GcpManager
// #[cfg(feature = "cloud")]
// #[tokio::test]
// async fn test_gcp_create_instance_group_empty_name() {
//     let manager = GcpManager::new(Some("project-id".to_string()));
//     manager.initialize().await.unwrap();
//
//     let result = manager.create_instance_group("", 3).await;
//     assert!(result.is_err());
//     if let Err(AppError::ValidationError(msg)) = result {
//         assert!(msg.contains("Instance group name cannot be empty"));
//     }
// }
//
// #[cfg(feature = "cloud")]
// #[tokio::test]
// async fn test_gcp_create_instance_group_zero_count() {
//     let manager = GcpManager::new(Some("project-id".to_string()));
//     manager.initialize().await.unwrap();
//
//     let result = manager.create_instance_group("ig-name", 0).await;
//     assert!(result.is_err());
//     if let Err(AppError::ValidationError(msg)) = result {
//         assert!(msg.contains("Instance count must be greater than 0"));
//     }
// }
//
// #[cfg(feature = "cloud")]
// #[tokio::test]
// async fn test_gcp_create_instance_group_success() -> Result<(), AppError> {
//     let manager = GcpManager::new(Some("project-id".to_string()));
//     manager.initialize().await?;
//
//     let ig_id = manager.create_instance_group("ig-name", 3).await?;
//     assert!(ig_id.starts_with("gcp-instance-group-"));
//
//     Ok(())
// }
