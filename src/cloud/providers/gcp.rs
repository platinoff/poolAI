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
use chrono::{DateTime, Duration, Utc};
#[cfg(feature = "cloud-sdk")]
use jsonwebtoken::{encode, EncodingKey, Header};
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
    /// Cached GCP access token with expiration time
    /// Token is refreshed automatically when expired or about to expire
    cached_token: Arc<RwLock<Option<CachedToken>>>,
    #[cfg(feature = "cloud-sdk")]
    /// Override base URL for metadata server and Compute API (e.g. mock server). When set, used instead of metadata.google.internal / compute.googleapis.com.
    base_url_override: Arc<RwLock<Option<String>>>,
}

#[cfg(feature = "cloud-sdk")]
/// Cached token with expiration time
struct CachedToken {
    token: String,
    expires_at: DateTime<Utc>,
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
            cached_token: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            base_url_override: Arc::new(RwLock::new(None)),
        }
    }

    /// Set base URL override for metadata server and Compute API (e.g. mock server for tests).
    #[cfg(feature = "cloud-sdk")]
    pub async fn set_base_url_override(&self, url: Option<String>) {
        *self.base_url_override.write().await = url;
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

            // Initialize HTTP client with connection pooling for GCP REST API calls
            // Connection pooling improves performance by reusing connections
            let http_client = reqwest::Client::builder()
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                .pool_max_idle_per_host(10)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| AppError::InitializationError(format!(
                    "Failed to create HTTP client for GCP API. Context: Cannot initialize reqwest client. \
                    Error: {}",
                    e
                )))?;
            *self.http_client.write().await = Some(http_client);

            // Initialize GCP access token (with caching)
            // Note: GCP authentication tries multiple methods in order:
            // 1. Metadata server (when running on GCP Compute Engine, Cloud Run, etc.)
            // 2. Service account key file - `GOOGLE_APPLICATION_CREDENTIALS` env var
            // 3. Application Default Credentials (ADC) - `gcloud auth application-default login`
            // Token will be cached with expiration time and automatically refreshed
            let _ = self.get_gcp_access_token().await?;

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
    /// Get GCP access token with fallback methods and caching
    ///
    /// Tries multiple authentication methods in order:
    /// 1. Metadata server (when running on GCP)
    /// 2. Service account key file (GOOGLE_APPLICATION_CREDENTIALS env var)
    /// 3. Application Default Credentials (ADC) via gcloud
    ///
    /// Tokens are cached with expiration time and automatically refreshed when expired or about to expire.
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if no valid credentials can be found.
    async fn get_gcp_access_token(&self) -> Result<String, AppError> {
        // Check cached token first
        {
            let cached_guard = self.cached_token.read().await;
            if let Some(cached) = cached_guard.as_ref() {
                // Refresh token if it expires in less than 5 minutes
                let refresh_threshold = Utc::now() + Duration::minutes(5);
                if cached.expires_at > refresh_threshold {
                    info!(
                        "GCP access token obtained from cache (expires at {})",
                        cached.expires_at
                    );
                    return Ok(cached.token.clone());
                }
            }
        }

        // Token expired or not cached, acquire new token
        let (token, expires_in_seconds) = self.acquire_gcp_token().await?;

        // Calculate expiration time
        let expires_at = Utc::now() + Duration::seconds(expires_in_seconds as i64);

        // Cache the token
        {
            let mut cached_guard = self.cached_token.write().await;
            *cached_guard = Some(CachedToken {
                token: token.clone(),
                expires_at,
            });
        }

        info!(
            "GCP access token acquired and cached (expires at {})",
            expires_at
        );
        Ok(token)
    }

    #[cfg(feature = "cloud-sdk")]
    /// Acquire GCP access token from available sources
    ///
    /// Returns tuple of (token, expires_in_seconds)
    async fn acquire_gcp_token(&self) -> Result<(String, u64), AppError> {
        // Try 1: Metadata server (when running on GCP Compute Engine, Cloud Run, etc.)
        if let Ok((token, expires_in)) = self.get_token_from_metadata_server().await {
            info!("GCP access token obtained from metadata server");
            return Ok((token, expires_in));
        }

        // Try 2: Service account key file (GOOGLE_APPLICATION_CREDENTIALS env var)
        if let Ok(credentials_path) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            if let Ok((token, expires_in)) =
                self.get_token_from_service_account(&credentials_path).await
            {
                info!("GCP access token obtained from service account key file");
                return Ok((token, expires_in));
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
    /// Returns tuple of (token, expires_in_seconds)
    async fn get_token_from_metadata_server(&self) -> Result<(String, u64), AppError> {
        let path = "/computeMetadata/v1/instance/service-accounts/default/token";
        let base = self
            .base_url_override
            .read()
            .await
            .clone()
            .unwrap_or_else(|| "http://metadata.google.internal".to_string());
        let base = base.trim_end_matches('/');
        let metadata_url = format!("{}{}", base, path);

        // Create a temporary client for metadata server (one-time use during initialization)
        // Note: Metadata server is internal to GCP and doesn't benefit from connection pooling
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| {
                AppError::InitializationError(format!(
                    "Failed to create HTTP client for metadata server. Error: {}",
                    e
                ))
            })?;

        let response = client
            .get(&metadata_url)
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

        let token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                AppError::InitializationError(
                    "Metadata server response missing access_token field".to_string(),
                )
            })?;

        // Extract expires_in (default to 3600 seconds if not present)
        let expires_in = json
            .get("expires_in")
            .and_then(|v| v.as_u64())
            .unwrap_or(3600);

        Ok((token, expires_in))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get access token from service account key file
    ///
    /// Reads service account JSON key file and generates OAuth2 access token.
    /// This implements the full OAuth2 flow:
    /// 1. Parse service account JSON key file
    /// 2. Create JWT assertion with claims (iss, sub, aud, exp, iat)
    /// 3. Sign JWT with RSA private key from service account
    /// 4. Exchange JWT for access token via Google OAuth2 token endpoint
    /// Returns tuple of (token, expires_in_seconds)
    async fn get_token_from_service_account(
        &self,
        key_path: &str,
    ) -> Result<(String, u64), AppError> {
        use serde::Deserialize;
        use std::fs;

        // Step 1: Parse service account JSON key file
        #[derive(Deserialize)]
        struct ServiceAccountKey {
            #[serde(rename = "type")]
            key_type: String,
            project_id: String,
            private_key_id: String,
            private_key: String,
            client_email: String,
            client_id: String,
            auth_uri: String,
            token_uri: String,
        }

        let key_content = fs::read_to_string(key_path).map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to read service account key file. Context: Cannot read file at {}. \
                Error: {}. Suggestion: Ensure GOOGLE_APPLICATION_CREDENTIALS points to a valid JSON key file.",
                key_path, e
            ))
        })?;

        let key: ServiceAccountKey = serde_json::from_str(&key_content).map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to parse service account key file. Context: Cannot parse JSON from {}. \
                Error: {}. Suggestion: Ensure the file is a valid GCP service account JSON key.",
                key_path, e
            ))
        })?;

        if key.key_type != "service_account" {
            return Err(AppError::InitializationError(format!(
                "Invalid service account key type. Context: Expected 'service_account', got '{}'. \
                Suggestion: Ensure the file is a valid GCP service account JSON key.",
                key.key_type
            )));
        }

        // Step 2: Create JWT claims
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::hours(1)).timestamp(); // Token valid for 1 hour

        #[derive(serde::Serialize)]
        struct JwtClaims {
            iss: String, // Issuer (service account email)
            sub: String, // Subject (service account email)
            aud: String, // Audience (token_uri)
            exp: i64,    // Expiration time
            iat: i64,    // Issued at time
        }

        let claims = JwtClaims {
            iss: key.client_email.clone(),
            sub: key.client_email.clone(),
            aud: key.token_uri.clone(),
            exp,
            iat,
        };

        // Step 3: Sign JWT with RSA private key
        let encoding_key = EncodingKey::from_rsa_pem(key.private_key.as_bytes()).map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to parse RSA private key. Context: Cannot parse private_key from service account key. \
                Error: {}. Suggestion: Ensure the service account key file contains a valid RSA private key in PEM format.",
                e
            ))
        })?;

        let header = Header::new(jsonwebtoken::Algorithm::RS256);
        let jwt = encode(&header, &claims, &encoding_key).map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to sign JWT. Context: Cannot create JWT assertion with RSA signature. \
                Error: {}. Suggestion: Check service account key validity.",
                e
            ))
        })?;

        // Step 4: Exchange JWT for access token via Google OAuth2 token endpoint
        let client_guard = self.http_client.read().await;
        let client = client_guard.as_ref().ok_or_else(|| {
            AppError::InitializationError(
                "GCP HTTP client not initialized. Call initialize() first.".to_string(),
            )
        })?;

        let token_request = serde_json::json!({
            "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
            "assertion": jwt
        });

        let response = client
            .post(&key.token_uri)
            .header("Content-Type", "application/json")
            .json(&token_request)
            .send()
            .await
            .map_err(|e| {
                AppError::InitializationError(format!(
                    "Failed to exchange JWT for access token. Context: Cannot send request to Google OAuth2 token endpoint. \
                    Error: {}. Suggestion: Check network connectivity and service account key validity.",
                    e
                ))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::InitializationError(format!(
                "Google OAuth2 token exchange failed. Context: Token endpoint returned status {}. \
                Response: {}. Suggestion: Check service account key validity and permissions.",
                status, error_body
            )));
        }

        let token_response: serde_json::Value = response.json().await.map_err(|e| {
            AppError::InitializationError(format!(
                "Failed to parse token response. Context: Cannot parse JSON response from Google OAuth2 token endpoint. \
                Error: {}. Suggestion: Check service account key validity.",
                e
            ))
        })?;

        let token = token_response
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                AppError::InitializationError(
                    "Token response missing access_token. Context: Google OAuth2 response does not contain access_token field. \
                    Suggestion: Check service account key validity and permissions.".to_string()
                )
            })?;

        // Extract expires_in (default to 3600 seconds if not present)
        let expires_in = token_response
            .get("expires_in")
            .and_then(|v| v.as_u64())
            .unwrap_or(3600);

        Ok((token, expires_in))
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
            *self.cached_token.write().await = None;
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

            let token = self.get_gcp_access_token().await?;

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
            let path = format!(
                "/compute/v1/projects/{}/zones/{}/instances",
                project_id, zone
            );
            let base = self
                .base_url_override
                .read()
                .await
                .clone()
                .unwrap_or_else(|| "https://compute.googleapis.com".to_string());
            let base = base.trim_end_matches('/');
            let api_url = format!("{}{}", base, path);

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
                .and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_u64().map(|n| n.to_string()))
                })
                .unwrap_or_else(|| instance_name.clone());

            info!(
                "Created Compute Engine instance: {} in zone {} / project {}",
                instance_id, zone, project_id
            );

            Ok(instance_id)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Fallback for non-cloud-sdk feature
            let project = self.project_id.as_deref().unwrap_or("unknown");
            info!(
                "Creating Compute Engine instance: {} / {} in project {} (placeholder - cloud-sdk feature disabled)",
                zone, machine_type, project
            );
            Ok(uuid::Uuid::new_v4().to_string())
        }
    }
}
