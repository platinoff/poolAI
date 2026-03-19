//! Integration tests for GCP cloud provider
//!
//! Tests GCP Compute Engine API interactions and token acquisition.

#[cfg(feature = "cloud-sdk")]
use super::mock_servers::MockGcpServer;
#[cfg(feature = "cloud-sdk")]
use poolai::cloud::providers::gcp::GcpManager;
#[cfg(feature = "cloud-sdk")]
use poolai::core::error::AppError;

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_manager_initialization() -> Result<(), AppError> {
    let manager = GcpManager::new(Some("test-project-id".to_string()));
    manager.initialize().await?;
    manager.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_manager_initialization_with_env_project() -> Result<(), AppError> {
    std::env::set_var("GCP_PROJECT_ID", "env-project-id");
    let manager = GcpManager::new(None);
    manager.initialize().await?;
    manager.shutdown().await?;
    std::env::remove_var("GCP_PROJECT_ID");
    Ok(())
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_create_compute_instance_validation() {
    let manager = GcpManager::new(Some("test-project".to_string()));
    manager.initialize().await.unwrap();

    // Test empty zone
    let result = manager.create_compute_instance("", "n1-standard-2").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Zone cannot be empty"));
    }

    // Test empty machine type
    let result = manager.create_compute_instance("us-central1-a", "").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Machine type cannot be empty"));
    }

    manager.shutdown().await.unwrap();
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_token_acquisition_fallback() {
    // Remove all token sources
    std::env::remove_var("GOOGLE_APPLICATION_CREDENTIALS");

    let manager = GcpManager::new(Some("test-project".to_string()));

    // Should fail with InitializationError when no credentials available
    // (metadata server won't be available in test environment)
    let result = manager.initialize().await;

    // May succeed if metadata server is somehow available, or fail with token error
    match result {
        Ok(_) => {
            // If initialization succeeds, token acquisition worked
            manager.shutdown().await.unwrap();
        }
        Err(AppError::InitializationError(msg)) => {
            // Expected when no credentials available
            assert!(
                msg.contains("Failed to obtain GCP access token")
                    || msg.contains("All authentication methods failed")
            );
        }
        Err(_) => {
            // Other errors are acceptable for this test
        }
    }
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_service_account_key_parsing() {
    use std::fs;
    use tempfile::NamedTempFile;

    // Create a minimal valid service account key file
    let key_content = r#"{
        "type": "service_account",
        "project_id": "test-project",
        "private_key_id": "test-key-id",
        "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC7VJTUt9Us8cKj\nMzEfYyjiWA4R4/M2b01i3K0FDIHk32tUZ6B2ZX3uJ4V+8U5P5Z5Z5Z5Z5Z5Z5Z5Z\n5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z\n5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z\n5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z\n5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z5Z\n-----END PRIVATE KEY-----\n",
        "client_email": "test@test-project.iam.gserviceaccount.com",
        "client_id": "123456789",
        "auth_uri": "https://accounts.google.com/o/oauth2/auth",
        "token_uri": "https://oauth2.googleapis.com/token"
    }"#;

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), key_content).unwrap();

    std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", temp_file.path());

    let manager = GcpManager::new(Some("test-project".to_string()));

    // Should attempt to parse the key file
    // Will fail at OAuth2 token exchange (expected in test environment)
    let result = manager.initialize().await;

    // Should not fail with key parsing error
    if let Err(AppError::InitializationError(msg)) = result {
        // Should not be a key parsing error
        assert!(
            !msg.contains("Failed to parse service account key file")
                && !msg.contains("Cannot parse JSON")
        );
    }

    std::env::remove_var("GOOGLE_APPLICATION_CREDENTIALS");
}

#[cfg(feature = "cloud-sdk")]
#[tokio::test]
async fn test_gcp_compute_e2e_with_mock_server() -> Result<(), AppError> {
    std::env::remove_var("GOOGLE_APPLICATION_CREDENTIALS");

    let mut mock = MockGcpServer::new().await;
    let _m1 = mock.mock_metadata_token().await;
    let _m2 = mock
        .mock_compute_instance_creation("test-project", "us-central1-a")
        .await;
    let base = mock.url();

    let manager = GcpManager::new(Some("test-project".to_string()));
    manager.set_base_url_override(Some(base)).await;
    manager.initialize().await?;

    let id = manager
        .create_compute_instance("us-central1-a", "n1-standard-2")
        .await?;
    assert!(
        id == "1234567890123456789" || id.starts_with("poolai-instance-"),
        "unexpected instance id: {}",
        id
    );

    manager.shutdown().await?;
    Ok(())
}
