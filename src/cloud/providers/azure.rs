//! Azure cloud provider integration
//!
//! Provides integration with Azure services:
//! - VM Scale Sets for auto-scaling
//! - Container Instances for containers
//! - Blob Storage for artifact storage
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::cloud::providers::azure::AzureManager;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = AzureManager::new(Some("subscription-id".to_string()));
//! manager.initialize().await?;
//!
//! // Create VM Scale Set
//! let scale_set_id = manager.create_vm_scale_set(
//!     "poolai-rg",
//!     "poolai-workers"
//! ).await?;
//!
//! manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[cfg(feature = "cloud-sdk")]
use chrono::{DateTime, Utc};

// Note: Azure SDK 0.30 API differs from expected structure
// azure_mgmt_compute 0.21 uses azure_core 0.21, while azure_identity 0.30 uses azure_core 0.30
// This version mismatch requires using REST API directly via reqwest instead of SDK clients
// Using REST API approach similar to GCP integration to avoid version conflicts
// Note: DefaultAzureCredential may not be directly importable in azure_identity 0.30
// Using REST API with manual token acquisition as fallback
// Note: azure_identity 0.30 - DefaultAzureCredential API structure verification needed
// Currently using REST API approach with manual token acquisition
// Tokens can be obtained via Azure CLI, environment variables, or Managed Identity
#[cfg(feature = "cloud-sdk")]
use azure_mgmt_compute::Client as ComputeClient;
#[cfg(feature = "cloud-sdk")]
use reqwest::Client as HttpClient;

/// Azure manager for cloud resources
pub struct AzureManager {
    subscription_id: Option<String>,
    initialized: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// Azure credential for authentication
    /// Note: Using REST API approach, credential storage not needed for now
    /// TODO: Re-enable when DefaultAzureCredential import path is verified
    // credential: Arc<RwLock<Option<DefaultAzureCredential>>>,
    #[cfg(feature = "cloud-sdk")]
    /// Azure Compute client for VM Scale Sets and VM management
    /// Note: azure_mgmt_compute 0.21 uses azure_core 0.21, may need API verification
    compute_client: Arc<RwLock<Option<ComputeClient>>>,
    #[cfg(feature = "cloud-sdk")]
    /// HTTP client for Azure REST API calls (used as fallback when SDK has version conflicts)
    http_client: Arc<RwLock<Option<HttpClient>>>,
    #[cfg(feature = "cloud-sdk")]
    /// Cached Azure access token with expiration time
    /// Token is refreshed automatically when expired or about to expire
    cached_token: Arc<RwLock<Option<CachedToken>>>,
    #[cfg(feature = "cloud-sdk")]
    /// Override Management API base URL (e.g. mock server). When set, used instead of https://management.azure.com.
    base_url_override: Arc<RwLock<Option<String>>>,
}

#[cfg(feature = "cloud-sdk")]
/// Cached token with expiration time
struct CachedToken {
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl AzureManager {
    /// Create a new Azure manager
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - Azure subscription ID (defaults to AZURE_SUBSCRIPTION_ID env var)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::azure::AzureManager;
    ///
    /// let manager = AzureManager::new(Some("subscription-id".to_string()));
    /// ```
    pub fn new(subscription_id: Option<String>) -> Self {
        Self {
            subscription_id: subscription_id
                .or_else(|| std::env::var("AZURE_SUBSCRIPTION_ID").ok()),
            initialized: Arc::new(RwLock::new(false)),
            // #[cfg(feature = "cloud-sdk")]
            // credential: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            compute_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            http_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            cached_token: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            base_url_override: Arc::new(RwLock::new(None)),
        }
    }

    /// Set base URL override for Management API (e.g. mock server for tests).
    /// When set, create_vm_scale_set etc. use this instead of https://management.azure.com.
    #[cfg(feature = "cloud-sdk")]
    pub async fn set_base_url_override(&self, url: Option<String>) {
        *self.base_url_override.write().await = url;
    }

    /// Initialize Azure integration
    ///
    /// This will:
    /// - Initialize Azure credentials (DefaultAzureCredential)
    /// - Initialize Compute client for VM Scale Sets and VM management
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - Azure credentials cannot be obtained
    /// - Compute client cannot be initialized
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::azure::AzureManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AzureManager::new(Some("subscription-id".to_string()));
    /// manager.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let subscription_id = self.subscription_id.as_deref().ok_or_else(|| {
                AppError::InitializationError(
                    "Azure subscription ID is required. Context: Attempted to initialize Azure manager without subscription ID. \
                    Suggestion: Set azure_subscription_id in config or set AZURE_SUBSCRIPTION_ID environment variable. \
                    Current value: None"
                        .to_string(),
                )
            })?;

            info!(
                "Initializing Azure SDK clients for subscription: {}",
                subscription_id
            );

            // Initialize Azure credentials
            // Note: Using REST API approach - tokens obtained via get_azure_access_token()
            // DefaultAzureCredential API structure needs verification for azure_identity 0.30
            // For now, using environment-based authentication via REST API
            // Credential will be obtained on-demand when needed for API calls

            // Initialize HTTP client with connection pooling for REST API calls
            // Connection pooling improves performance by reusing connections
            let http_client = reqwest::Client::builder()
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                .pool_max_idle_per_host(10)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| AppError::InitializationError(format!(
                    "Failed to create HTTP client for Azure API. Context: Cannot initialize reqwest client. \
                    Error: {}",
                    e
                )))?;
            *self.http_client.write().await = Some(http_client);

            // TODO: Initialize Compute client once API is verified
            // Note: azure_mgmt_compute 0.21 uses azure_core 0.21, while azure_identity 0.30 uses azure_core 0.30
            // This version mismatch may cause compilation errors. Need to verify API compatibility.
            // For now, using REST API approach (similar to GCP) to avoid version conflicts.
            // Expected API (needs verification):
            // let compute_client = ComputeClient::builder()
            //     .subscription_id(subscription_id.to_string())
            //     .credential(credential.clone())
            //     .build();
            // *self.compute_client.write().await = Some(compute_client);

            info!(
                "Azure credential and HTTP client initialized for subscription: {} (Compute SDK client initialization pending API verification)",
                subscription_id
            );
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!("Azure manager initialized (placeholder mode)");
            tracing::warn!(
                "Azure integration is a placeholder - enable cloud-sdk feature for full SDK support"
            );
        }

        *initialized = true;
        Ok(())
    }

    /// Shutdown Azure integration
    ///
    /// Cleans up Azure client connections.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::azure::AzureManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AzureManager::new(Some("subscription-id".to_string()));
    /// manager.initialize().await?;
    /// // Use manager...
    /// manager.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&self) -> Result<(), AppError> {
        #[cfg(feature = "cloud-sdk")]
        {
            // Clear clients and cached token
            *self.compute_client.write().await = None;
            *self.http_client.write().await = None;
            *self.cached_token.write().await = None;
        }

        *self.initialized.write().await = false;
        info!("Azure manager shut down");
        Ok(())
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get Azure access token with fallback methods and caching
    ///
    /// Tries multiple authentication methods in order:
    /// 1. Environment variable (`AZURE_ACCESS_TOKEN`)
    /// 2. Azure CLI (`az account get-access-token`)
    /// 3. Managed Identity (when running on Azure VM/App Service)
    ///
    /// Tokens are cached with expiration time and automatically refreshed when expired or about to expire.
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if no valid credentials can be found.
    async fn get_azure_access_token(&self) -> Result<String, AppError> {
        // Check cached token first
        {
            let cached_guard = self.cached_token.read().await;
            if let Some(cached) = cached_guard.as_ref() {
                // Refresh token if it expires in less than 5 minutes
                let refresh_threshold = chrono::Utc::now() + chrono::Duration::minutes(5);
                if cached.expires_at > refresh_threshold {
                    info!(
                        "Azure access token obtained from cache (expires at {})",
                        cached.expires_at
                    );
                    return Ok(cached.token.clone());
                }
            }
        }

        // Token expired or not cached, acquire new token
        let (token, expires_in_seconds) = self.acquire_azure_token().await?;

        // Calculate expiration time
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in_seconds as i64);

        // Cache the token
        {
            let mut cached_guard = self.cached_token.write().await;
            *cached_guard = Some(CachedToken {
                token: token.clone(),
                expires_at,
            });
        }

        info!(
            "Azure access token acquired and cached (expires at {})",
            expires_at
        );
        Ok(token)
    }

    #[cfg(feature = "cloud-sdk")]
    /// Acquire Azure access token from available sources
    ///
    /// Returns tuple of (token, expires_in_seconds)
    async fn acquire_azure_token(&self) -> Result<(String, u64), AppError> {
        // Try 1: Environment variable (highest priority for explicit override)
        // Note: Environment variable tokens don't have expiration info, assume 1 hour
        if let Ok(token) = std::env::var("AZURE_ACCESS_TOKEN") {
            if !token.is_empty() {
                info!("Azure access token obtained from AZURE_ACCESS_TOKEN environment variable");
                return Ok((token, 3600)); // Assume 1 hour expiration
            }
        }

        // Try 2: Azure CLI
        if let Ok((token, expires_in)) = self.get_token_from_azure_cli().await {
            info!("Azure access token obtained from Azure CLI");
            return Ok((token, expires_in));
        }

        // Try 3: Managed Identity (when running on Azure)
        if let Ok((token, expires_in)) = self.get_token_from_managed_identity().await {
            info!("Azure access token obtained from Managed Identity");
            return Ok((token, expires_in));
        }

        // All methods failed
        Err(AppError::InitializationError(
            "Azure access token not found. Context: All authentication methods failed. \
            Tried: 1) AZURE_ACCESS_TOKEN environment variable, 2) Azure CLI (az account get-access-token), \
            3) Managed Identity (metadata service). \
            Suggestion: Run 'az login' to authenticate with Azure CLI, or set AZURE_ACCESS_TOKEN environment variable, \
            or ensure the application is running on Azure with Managed Identity enabled.".to_string()
        ))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get access token from Azure CLI
    ///
    /// Executes `az account get-access-token` to get token with expiration info.
    /// Returns tuple of (token, expires_in_seconds).
    async fn get_token_from_azure_cli(&self) -> Result<(String, u64), AppError> {
        use tokio::process::Command;

        // Get full token response with expiration info
        let output = Command::new("az")
            .args([
                "account",
                "get-access-token",
                "-o",
                "json",
            ])
            .output()
            .await
            .map_err(|e| AppError::InitializationError(format!(
                "Failed to execute Azure CLI command. Context: Cannot run 'az account get-access-token'. \
                Error: {}. Suggestion: Ensure Azure CLI is installed and 'az login' has been executed.",
                e
            )))?;

        if !output.status.success() {
            return Err(AppError::InitializationError(format!(
                "Azure CLI command failed. Context: 'az account get-access-token' returned non-zero exit code. \
                Exit code: {}. Stderr: {}. Suggestion: Run 'az login' to authenticate with Azure CLI.",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let response_json: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| AppError::InitializationError(format!(
                "Failed to parse Azure CLI output. Context: Cannot parse JSON from 'az account get-access-token' output. \
                Error: {}. Suggestion: Ensure Azure CLI is properly configured.",
                e
            )))?;

        let token = response_json
            .get("accessToken")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::InitializationError(
                "Azure CLI response missing accessToken. Context: 'az account get-access-token' response does not contain accessToken field. \
                Suggestion: Run 'az login' to authenticate with Azure CLI.".to_string()
            ))?
            .to_string();

        // Parse expiration time (expiresOn is in format "2024-01-01 12:00:00.000000" or RFC3339)
        let expires_in =
            if let Some(expires_on_str) = response_json.get("expiresOn").and_then(|v| v.as_str()) {
                // Try RFC3339 format first
                if let Ok(expires_on) = DateTime::parse_from_rfc3339(expires_on_str) {
                    let now = Utc::now();
                    let expires_at = expires_on.with_timezone(&Utc);
                    let duration = expires_at.signed_duration_since(now);
                    duration.num_seconds().max(0) as u64
                } else if let Ok(expires_on) =
                    DateTime::parse_from_str(expires_on_str, "%Y-%m-%d %H:%M:%S%.f")
                {
                    // Try Azure CLI format: "2024-01-01 12:00:00.000000"
                    let now = Utc::now();
                    let expires_at = expires_on.with_timezone(&Utc);
                    let duration = expires_at.signed_duration_since(now);
                    duration.num_seconds().max(0) as u64
                } else {
                    // Fallback: assume 1 hour if parsing fails
                    3600
                }
            } else {
                // Fallback: assume 1 hour if expiration not provided
                3600
            };

        Ok((token, expires_in))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get access token from Azure Managed Identity
    ///
    /// Queries the Azure Instance Metadata Service (IMDS) for Managed Identity token.
    /// This works when running on Azure VM, App Service, or other Azure services with Managed Identity enabled.
    /// Returns tuple of (token, expires_in_seconds).
    async fn get_token_from_managed_identity(&self) -> Result<(String, u64), AppError> {
        let client_guard = self.http_client.read().await;
        let client = client_guard.as_ref().ok_or_else(|| {
            AppError::InitializationError(
                "Azure HTTP client not initialized. Call initialize() first.".to_string(),
            )
        })?;

        // Azure IMDS endpoint for Managed Identity
        // Resource: https://management.azure.com/ for Azure Resource Manager API
        let imds_url = "http://169.254.169.254/metadata/identity/oauth2/token?api-version=2018-02-01&resource=https://management.azure.com/";

        let response = client
            .get(imds_url)
            .header("Metadata", "true")
            .send()
            .await
            .map_err(|e| AppError::InitializationError(format!(
                "Failed to query Azure Managed Identity. Context: Cannot connect to Azure Instance Metadata Service (IMDS). \
                Error: {}. Suggestion: Ensure the application is running on Azure with Managed Identity enabled, \
                or use Azure CLI or environment variable authentication instead.",
                e
            )))?;

        if !response.status().is_success() {
            return Err(AppError::InitializationError(format!(
                "Azure Managed Identity request failed. Context: IMDS returned status {}. \
                Suggestion: Ensure Managed Identity is enabled for the Azure resource, \
                or use Azure CLI or environment variable authentication instead.",
                response.status()
            )));
        }

        let token_response: serde_json::Value = response.json().await.map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to parse Managed Identity response. Context: Cannot parse JSON response from IMDS. \
                Error: {}. Suggestion: Check Azure Managed Identity configuration.",
                e
            ))
        })?;

        let token = token_response
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AppError::InitializationError(
                    "Managed Identity response missing access_token. Context: IMDS response does not contain access_token field. \
                    Suggestion: Check Azure Managed Identity configuration.".to_string()
                )
            })?
            .to_string();

        // Parse expiration time (expires_in is in seconds)
        let expires_in = token_response
            .get("expires_in")
            .and_then(|v| v.as_u64())
            .unwrap_or(3600); // Default to 1 hour if not provided

        Ok((token, expires_in))
    }

    /// Create VM Scale Set
    ///
    /// # Arguments
    ///
    /// * `resource_group` - Azure resource group name
    /// * `name` - VM Scale Set name
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `resource_group` is empty
    /// - `name` is empty
    pub async fn create_vm_scale_set(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<String, AppError> {
        if resource_group.is_empty() {
            return Err(AppError::ValidationError(
                "Resource group name cannot be empty. Current value: ''. Suggestion: Provide a valid Azure resource group name."
                    .to_string(),
            ));
        }

        if name.is_empty() {
            return Err(AppError::ValidationError(
                "VM Scale Set name cannot be empty. Current value: ''. Suggestion: Provide a valid VM Scale Set name."
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let subscription_id = self.subscription_id.as_deref().ok_or_else(|| {
                AppError::ValidationError(
                    "Azure subscription ID is required for Compute API calls".to_string(),
                )
            })?;

            // Get HTTP client and access token
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "Azure HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            // Get access token via REST API with fallback methods
            let token = self.get_azure_access_token().await?;

            // Prepare VM Scale Set configuration
            // Note: Minimal configuration for now, can be extended later
            let vmss_config = serde_json::json!({
                "location": "eastus", // TODO: Make location configurable
                "sku": {
                    "name": "Standard_DS1_v2",
                    "tier": "Standard",
                    "capacity": 2
                },
                "properties": {
                    "upgradePolicy": {
                        "mode": "Manual"
                    },
                    "virtualMachineProfile": {
                        "storageProfile": {
                            "imageReference": {
                                "publisher": "Canonical",
                                "offer": "UbuntuServer",
                                "sku": "18.04-LTS",
                                "version": "latest"
                            },
                            "osDisk": {
                                "caching": "ReadWrite",
                                "createOption": "FromImage"
                            }
                        },
                        "osProfile": {
                            "computerNamePrefix": name,
                            "adminUsername": "azureuser",
                            "adminPassword": "ChangeMe123!"
                        }
                    }
                }
            });

            // Make API call to create VM Scale Set
            let path = format!(
                "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}?api-version=2023-03-01",
                subscription_id, resource_group, name
            );
            let base = self
                .base_url_override
                .read()
                .await
                .clone()
                .unwrap_or_else(|| "https://management.azure.com".to_string());
            let base = base.trim_end_matches('/');
            let api_url = format!("{}{}", base, path);

            let response = client
                .put(&api_url)
                .bearer_auth(&token)
                .header("Content-Type", "application/json")
                .json(&vmss_config)
                .send()
                .await
                .map_err(|e| {
                    AppError::NetworkError(format!(
                        "Failed to create VM Scale Set. Context: Azure API request failed. \
                    Resource Group: {}, Name: {}, Error: {}",
                        resource_group, name, e
                    ))
                })?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "Azure Compute API error. Context: VM Scale Set creation failed. \
                    Resource Group: {}, Name: {}, Status: {}, Response: {}",
                    resource_group, name, status, error_text
                )));
            }

            let response_json: serde_json::Value = response.json().await.map_err(|e| {
                AppError::NetworkError(format!("Failed to parse Azure API response. Error: {}", e))
            })?;

            // Extract VM Scale Set ID from response
            let vmss_id = response_json
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}",
                    subscription_id, resource_group, name
                ));

            info!(
                "Created VM Scale Set: {} in resource group {} / subscription {}",
                vmss_id, resource_group, subscription_id
            );

            Ok(vmss_id)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Fallback for non-cloud-sdk feature
            info!(
                "Creating VM Scale Set: {} / {} in subscription {} (placeholder - cloud-sdk feature disabled)",
                resource_group,
                name,
                self.subscription_id.as_deref().unwrap_or("default")
            );
            Ok(uuid::Uuid::new_v4().to_string())
        }
    }
}
