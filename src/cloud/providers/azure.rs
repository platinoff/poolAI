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

// Note: Azure SDK 0.30 API differs from expected structure
// azure_mgmt_compute 0.21 uses azure_core 0.21, while azure_identity 0.30 uses azure_core 0.30
// This version mismatch may require workaround or API verification
#[cfg(feature = "cloud-sdk")]
use azure_identity::DefaultAzureCredential;
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
    /// Note: Currently placeholder - API needs verification for version compatibility
    credential: Arc<RwLock<Option<DefaultAzureCredential>>>,
    #[cfg(feature = "cloud-sdk")]
    /// Azure Compute client for VM Scale Sets and VM management
    /// Note: azure_mgmt_compute 0.21 uses azure_core 0.21, may need API verification
    compute_client: Arc<RwLock<Option<ComputeClient>>>,
    #[cfg(feature = "cloud-sdk")]
    /// HTTP client for Azure REST API calls (used as fallback when SDK has version conflicts)
    http_client: Arc<RwLock<Option<HttpClient>>>,
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
            #[cfg(feature = "cloud-sdk")]
            credential: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            compute_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            http_client: Arc::new(RwLock::new(None)),
        }
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
            // Note: DefaultAzureCredential tries multiple authentication methods:
            // 1. Environment variables (AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_TENANT_ID)
            // 2. Managed Identity (when running on Azure)
            // 3. Azure CLI (when logged in via `az login`)
            // 4. Azure PowerShell (when logged in)
            let credential = DefaultAzureCredential::default();

            // Store credential
            *self.credential.write().await = Some(credential);

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
            // Clear clients
            *self.compute_client.write().await = None;
            *self.http_client.write().await = None;
            *self.credential.write().await = None;
        }

        *self.initialized.write().await = false;
        info!("Azure manager shut down");
        Ok(())
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

            // Get access token from credential
            let credential_guard = self.credential.read().await;
            let credential = credential_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "Azure credential not initialized. Call initialize() first.".to_string(),
                )
            })?;

            // Get access token - DefaultAzureCredential implements TokenCredential trait
            // Note: TokenCredential requires scope, typically "https://management.azure.com/.default"
            let token = credential
                .token(&["https://management.azure.com/.default"])
                .await
                .map_err(|e| {
                    AppError::InitializationError(format!(
                        "Failed to obtain Azure access token. Context: Token request failed. \
                    Error: {}",
                        e
                    ))
                })?;

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
            let api_url = format!(
                "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}?api-version=2023-03-01",
                subscription_id, resource_group, name
            );

            let response = client
                .put(&api_url)
                .bearer_auth(token.token.secret())
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
        } else {
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
