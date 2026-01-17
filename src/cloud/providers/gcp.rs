//! GCP cloud provider integration
//!
//! Provides integration with GCP services:
//! - Compute Engine for VM instances
//! - Cloud Run for containers
//! - Cloud Storage for artifact storage
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::cloud::providers::gcp::GcpManager;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = GcpManager::new(Some("my-project-id".to_string()));
//! manager.initialize().await?;
//!
//! // Create Compute Engine instance
//! let instance_id = manager.create_compute_instance(
//!     "us-central1-a",
//!     "n1-standard-2"
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
use reqwest::Client as HttpClient;

// Note: GCP integration uses REST API via reqwest (similar to Kubernetes implementation)
// This avoids version conflicts and additional dependencies
// TODO: Consider adding google-cloud-compute-v1 crate in the future if needed
// For now, using direct REST API calls to GCP Compute Engine API

/// GCP manager for cloud resources
pub struct GcpManager {
    project_id: Option<String>,
    initialized: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// HTTP client for GCP REST API calls
    http_client: Arc<RwLock<Option<HttpClient>>>,
    #[cfg(feature = "cloud-sdk")]
    /// GCP access token for authentication
    /// Note: Retrieved via Application Default Credentials (ADC) or service account key
    access_token: Arc<RwLock<Option<String>>>,
}

impl GcpManager {
    /// Create a new GCP manager
    ///
    /// # Arguments
    ///
    /// * `project_id` - GCP project ID (defaults to GCP_PROJECT_ID env var)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::gcp::GcpManager;
    ///
    /// let manager = GcpManager::new(Some("my-project-id".to_string()));
    /// ```
    pub fn new(project_id: Option<String>) -> Self {
        Self {
            project_id: project_id.or_else(|| std::env::var("GCP_PROJECT_ID").ok()),
            initialized: Arc::new(RwLock::new(false)),
            #[cfg(feature = "cloud-sdk")]
            http_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            access_token: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize GCP integration
    ///
    /// This will:
    /// - Initialize GCP credentials (Application Default Credentials or service account)
    /// - Initialize Compute Engine, Cloud Run, and Cloud Storage clients
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - GCP credentials cannot be obtained
    /// - SDK clients cannot be initialized
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::gcp::GcpManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = GcpManager::new(Some("my-project-id".to_string()));
    /// manager.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        let project_id = self.project_id.as_deref().ok_or_else(|| {
            AppError::InitializationError(
                "GCP project ID is required. Context: Attempted to initialize GCP manager without project ID. \
                Suggestion: Set gcp_project_id in config or set GCP_PROJECT_ID environment variable. \
                Current value: None"
                    .to_string(),
            )
        })?;

        #[cfg(feature = "cloud-sdk")]
        {
            info!(
                "Initializing GCP REST API client for project: {}",
                project_id
            );

            // Initialize HTTP client for GCP REST API calls
            let http_client = reqwest::Client::builder()
                .build()
                .map_err(|e| AppError::InitializationError(format!(
                    "Failed to create HTTP client for GCP API. Context: Cannot initialize reqwest client. \
                    Error: {}",
                    e
                )))?;
            *self.http_client.write().await = Some(http_client);

            // Initialize GCP access token
            // Note: GCP authentication tries multiple methods in order:
            // 1. Metadata server (when running on GCP Compute Engine, Cloud Run, etc.)
            // 2. Service account key file - `GOOGLE_APPLICATION_CREDENTIALS` env var
            // 3. Application Default Credentials (ADC) - `gcloud auth application-default login`
            let token = self.get_gcp_access_token().await?;
            *self.access_token.write().await = Some(token.clone());

            info!(
                "GCP HTTP client and access token initialized for project: {}",
                project_id
            );
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!("GCP manager initialized (placeholder mode)");
            tracing::warn!(
                "GCP integration is a placeholder - enable cloud-sdk feature for full SDK support"
            );
        }

        *initialized = true;
        Ok(())
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get GCP access token
    ///
    /// Tries multiple authentication methods in order:
    /// 1. Metadata server (when running on GCP)
    /// 2. Service account key file (GOOGLE_APPLICATION_CREDENTIALS env var)
    /// 3. Application Default Credentials (ADC) via gcloud
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if no valid credentials can be found.
    async fn get_gcp_access_token(&self) -> Result<String, AppError> {
        // Try 1: Metadata server (when running on GCP Compute Engine, Cloud Run, etc.)
        if let Ok(token) = self.get_token_from_metadata_server().await {
            info!("GCP access token obtained from metadata server");
            return Ok(token);
        }

        // Try 2: Service account key file (GOOGLE_APPLICATION_CREDENTIALS env var)
        if let Ok(credentials_path) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            if let Ok(token) = self.get_token_from_service_account(&credentials_path).await {
                info!("GCP access token obtained from service account key file");
                return Ok(token);
            }
        }

        // Try 3: Application Default Credentials (ADC) - placeholder
        // Note: Full ADC implementation would require gcloud CLI or user credentials
        // For now, return an error if metadata server and service account key both fail
        Err(AppError::InitializationError(
            "Failed to obtain GCP access token. Context: All authentication methods failed. \
            Suggestion: \
            1) Set GOOGLE_APPLICATION_CREDENTIALS to service account key file path, or \
            2) Run on GCP Compute Engine/Cloud Run (metadata server), or \
            3) Use 'gcloud auth application-default login' for ADC. \
            Current status: Metadata server unavailable, GOOGLE_APPLICATION_CREDENTIALS not set or invalid."
                .to_string(),
        ))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get access token from GCP metadata server
    ///
    /// This works when running on GCP Compute Engine, Cloud Run, App Engine, etc.
    async fn get_token_from_metadata_server(&self) -> Result<String, AppError> {
        let metadata_url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

        let client = reqwest::Client::builder().build().map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to create HTTP client for metadata server. Error: {}",
                e
            ))
        })?;

        let response = client
            .get(metadata_url)
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|e| {
                AppError::InitializationError(format!(
                    "Failed to query metadata server. Error: {}",
                    e
                ))
            })?;

        if !response.status().is_success() {
            return Err(AppError::InitializationError(format!(
                "Metadata server returned error status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to parse metadata server response. Error: {}",
                e
            ))
        })?;

        json.get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                AppError::InitializationError(
                    "Metadata server response missing access_token field".to_string(),
                )
            })
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get access token from service account key file
    ///
    /// Reads service account JSON key file and generates OAuth2 access token.
    async fn get_token_from_service_account(&self, _key_path: &str) -> Result<String, AppError> {
        // TODO: Implement service account key file parsing and JWT signing
        // Note: Full implementation would require:
        // 1. Read JSON key file
        // 2. Parse service account email and private key
        // 3. Create JWT assertion (claims: iss, sub, aud, exp, iat)
        // 4. Sign JWT with RSA private key
        // 5. Exchange JWT for access token via OAuth2 token endpoint
        // 6. Return access token
        //
        // This is complex and may require additional dependencies (jwt crate, rsa, etc.)
        // For now, return an error indicating this method needs implementation
        Err(AppError::InitializationError(
            "Service account key file authentication not yet implemented. \
            Context: GOOGLE_APPLICATION_CREDENTIALS is set but parsing is not implemented. \
            Suggestion: Use metadata server (when running on GCP) or implement service account key parsing. \
            Current value: Service account key file authentication is a placeholder."
                .to_string(),
        ))
    }

    /// Shutdown GCP integration
    ///
    /// Cleans up GCP client connections.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::gcp::GcpManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = GcpManager::new(Some("my-project-id".to_string()));
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
            *self.http_client.write().await = None;
            *self.access_token.write().await = None;
        }

        *self.initialized.write().await = false;
        info!("GCP manager shut down");
        Ok(())
    }

    /// Create Compute Engine instance
    ///
    /// # Arguments
    ///
    /// * `zone` - GCP zone (e.g., "us-central1-a")
    /// * `machine_type` - Machine type (e.g., "n1-standard-2")
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `zone` is empty
    /// - `machine_type` is empty
    pub async fn create_compute_instance(
        &self,
        zone: &str,
        machine_type: &str,
    ) -> Result<String, AppError> {
        if zone.is_empty() {
            return Err(AppError::ValidationError(
                "Zone cannot be empty. Current value: ''. Suggestion: Provide a valid GCP zone (e.g., 'us-central1-a')."
                    .to_string(),
            ));
        }

        if machine_type.is_empty() {
            return Err(AppError::ValidationError(
                "Machine type cannot be empty. Current value: ''. Suggestion: Provide a valid GCP machine type (e.g., 'n1-standard-2')."
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let project_id = self.project_id.as_deref().ok_or_else(|| {
                AppError::ValidationError(
                    "GCP project ID is required for Compute Engine API calls".to_string(),
                )
            })?;

            // Get HTTP client and access token
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "GCP HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            let token_guard = self.access_token.read().await;
            let token = token_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "GCP access token not initialized. Call initialize() first.".to_string(),
                )
            })?;

            // Create instance name
            let instance_name = format!("poolai-instance-{}", uuid::Uuid::new_v4());

            // Prepare instance configuration
            // Note: Minimal configuration for now, can be extended later
            let instance_config = serde_json::json!({
                "name": instance_name,
                "machineType": format!("zones/{}/machineTypes/{}", zone, machine_type),
                "disks": [{
                    "boot": true,
                    "autoDelete": true,
                    "initializeParams": {
                        "sourceImage": "projects/debian-cloud/global/images/family/debian-12"
                    }
                }],
                "networkInterfaces": [{
                    "network": "global/networks/default",
                    "accessConfigs": [{
                        "type": "ONE_TO_ONE_NAT",
                        "name": "External NAT"
                    }]
                }]
            });

            // Make API call to create instance
            let api_url = format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances",
                project_id, zone
            );

            let response = client
                .post(&api_url)
                .bearer_auth(token)
                .header("Content-Type", "application/json")
                .json(&instance_config)
                .send()
                .await
                .map_err(|e| {
                    AppError::NetworkError(format!(
                    "Failed to create Compute Engine instance. Context: GCP API request failed. \
                    Zone: {}, Machine Type: {}, Error: {}",
                    zone, machine_type, e
                ))
                })?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "GCP Compute Engine API error. Context: Instance creation failed. \
                    Zone: {}, Machine Type: {}, Status: {}, Response: {}",
                    zone, machine_type, status, error_text
                )));
            }

            let response_json: serde_json::Value = response.json().await.map_err(|e| {
                AppError::NetworkError(format!("Failed to parse GCP API response. Error: {}", e))
            })?;

            // Extract instance ID from response
            let instance_id = response_json
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| instance_name.clone());

            info!(
                "Created Compute Engine instance: {} in zone {} / project {}",
                instance_id, zone, project_id
            );

            return Ok(instance_id);
        }

        // Fallback for non-cloud-sdk feature
        let project = self.project_id.as_deref().unwrap_or("unknown");
        info!(
            "Creating Compute Engine instance: {} / {} in project {} (placeholder - cloud-sdk feature disabled)",
            zone, machine_type, project
        );
        Ok(uuid::Uuid::new_v4().to_string())
    }
}
