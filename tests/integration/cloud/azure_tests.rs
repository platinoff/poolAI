//! Integration tests for Azure cloud provider
//!
//! Tests Azure VM Scale Set API interactions and token acquisition.

#[cfg(feature = "cloud-sdk")]
mod mock_servers;

#[cfg(feature = "cloud-sdk")]
use mock_servers::MockAzureServer;
#[cfg(feature = "cloud-sdk")]
use poolai::cloud::providers::azure::AzureManager;
#[cfg(feature = "cloud-sdk")]
use poolai::core::error::AppError;

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_manager_initialization() -> Result<(), AppError> {
    let manager = AzureManager::new(Some("test-subscription-id".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_manager_initialization_with_env_subscription() -> Result<(), AppError> {
    std::env::set_var("AZURE_SUBSCRIPTION_ID", "env-subscription-id");
    let manager = AzureManager::new(None);
    manager.initialize().await?;
    manager.shutdown().await?;
    std::env::remove_var("AZURE_SUBSCRIPTION_ID");
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_create_vmss_validation() {
    let manager = AzureManager::new(Some("test-sub".to_string()));
    manager.initialize().await.unwrap();

    // Test empty resource group
    let result = manager.create_vm_scale_set("", "vmss-name").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Resource group name cannot be empty"));
    }

    // Test empty VMSS name
    let result = manager.create_vm_scale_set("my-rg", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("VM Scale Set name cannot be empty"));
    }

    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_token_from_env_var() {
    std::env::set_var("AZURE_ACCESS_TOKEN", "test-token-from-env");

    let manager = AzureManager::new(Some("test-sub".to_string()));
    manager.initialize().await.unwrap();

    // Token acquisition should work with env var
    // We test this indirectly via create_vm_scale_set which requires a token
    let result = manager.create_vm_scale_set("test-rg", "test-vmss").await;
    // Should either succeed (if token is valid format) or fail with network/API error
    // but not with token acquisition error
    if let Err(AppError::InitializationError(msg)) = result {
        // Should not be a token acquisition error
        assert!(!msg.contains("Azure access token not found"));
    }

    std::env::remove_var("AZURE_ACCESS_TOKEN");
    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_token_acquisition_fallback() {
    // Remove all token sources
    std::env::remove_var("AZURE_ACCESS_TOKEN");

    let manager = AzureManager::new(Some("test-sub".to_string()));
    manager.initialize().await.unwrap();

    // Should fail with token acquisition error when no sources available
    let result = manager.create_vm_scale_set("test-rg", "test-vmss").await;
    assert!(result.is_err());
    if let Err(AppError::InitializationError(msg)) = result {
        assert!(
            msg.contains("Azure access token not found")
                || msg.contains("All authentication methods failed")
        );
    }

    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_azure_vmss_e2e_with_mock_server() -> Result<(), AppError> {
    std::env::set_var("AZURE_ACCESS_TOKEN", "mock-token-for-test");

    let mut mock = MockAzureServer::new().await;
    let _m = mock.mock_vmss_creation("test-sub", "test-rg").await;
    let base = mock.url();

    let manager = AzureManager::new(Some("test-sub".to_string()));
    manager.set_base_url_override(Some(base)).await;
    manager.initialize().await?;

    let id = manager.create_vm_scale_set("test-rg", "test-vmss").await?;
    assert!(
        id.contains("test-vmss"),
        "expected id to contain test-vmss, got {}",
        id
    );

    manager.shutdown().await?;
    std::env::remove_var("AZURE_ACCESS_TOKEN");
    Ok(())
}
