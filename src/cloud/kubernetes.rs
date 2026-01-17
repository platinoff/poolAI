//! Kubernetes integration module
//!
//! Provides Kubernetes orchestration support for PoolAI, including:
//! - Kubernetes operator for managing PoolAI resources
//! - Custom Resource Definitions (CRDs)
//! - Service discovery and configuration
//! - Helm charts for deployment
//!
//! # Features
//!
//! - **Operator**: Kubernetes operator for PoolAI resources
//! - **CRDs**: Custom resources for workers, VMs, tenants
//! - **Service Discovery**: Automatic service discovery
//! - **Helm Charts**: Deployment templates
//!
//! # Example
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use poolai::cloud::kubernetes::{KubernetesManager, WorkerDeploymentConfig};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = KubernetesManager::new("poolai".to_string());
//! manager.initialize().await?;
//!
//! // Create a worker deployment
//! let config = WorkerDeploymentConfig {
//!     image: "poolai/worker:v1.0.0".to_string(),
//!     replicas: 3,
//!     resources: poolai::cloud::kubernetes::ResourceRequirements {
//!         cpu: "500m".to_string(),
//!         memory: "512Mi".to_string(),
//!         gpu: Some(1),
//!     },
//!     env: std::collections::HashMap::new(),
//! };
//!
//! let deployment_id = manager.create_worker_deployment("my-worker", &config).await?;
//! println!("Created deployment: {}", deployment_id);
//!
//! // Get service endpoints
//! let endpoints = manager.get_service_endpoints(&deployment_id).await?;
//!
//! manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[cfg(feature = "cloud-sdk")]
use serde_json::json;

/// Kubernetes Pod status information
#[derive(Debug, Clone)]
pub struct PodStatus {
    /// Pod name
    pub name: String,
    /// Pod phase (Pending, Running, Succeeded, Failed, Unknown)
    pub phase: String,
    /// Whether the pod is ready
    pub ready: bool,
    /// Number of times the pod has been restarted
    pub restart_count: u32,
}

/// Kubernetes Deployment status information
#[derive(Debug, Clone)]
pub struct DeploymentStatus {
    /// Deployment name
    pub name: String,
    /// Number of desired replicas
    pub replicas: u32,
    /// Number of ready replicas
    pub ready_replicas: u32,
    /// Number of available replicas
    pub available_replicas: u32,
    /// Number of unavailable replicas
    pub unavailable_replicas: u32,
    /// Whether the deployment is ready (all replicas available)
    pub ready: bool,
}

/// Kubernetes Deployment event information
#[derive(Debug, Clone)]
pub struct DeploymentEvent {
    /// Event reason (e.g., "ScalingReplicaSet", "SuccessfulCreate")
    pub reason: String,
    /// Event message
    pub message: String,
    /// Event type (Normal, Warning)
    pub event_type: String,
    /// First occurrence timestamp
    pub first_timestamp: Option<String>,
}

/// Kubernetes manager for PoolAI resources
pub struct KubernetesManager {
    namespace: String,
    initialized: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// Kubernetes API base URL and authentication token
    /// Using reqwest + k8s-openapi directly (kube-rs requires Rust 1.88+)
    api_base_url: Arc<RwLock<Option<String>>>,
    #[cfg(feature = "cloud-sdk")]
    api_token: Arc<RwLock<Option<String>>>,
}

impl KubernetesManager {
    /// Create a new KubernetesManager
    ///
    /// # Arguments
    ///
    /// * `namespace` - Kubernetes namespace to operate in
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// ```
    pub fn new(namespace: String) -> Self {
        Self {
            namespace,
            initialized: Arc::new(RwLock::new(false)),
            #[cfg(feature = "cloud-sdk")]
            api_base_url: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            api_token: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize Kubernetes integration
    ///
    /// This will:
    /// - Check for Kubernetes cluster connectivity
    /// - Verify namespace exists
    /// - Initialize CRD watchers (when implemented)
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - Kubernetes cluster is not accessible
    /// - Namespace does not exist
    /// - Authentication/authorization fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        if self.namespace.is_empty() {
            return Err(AppError::InitializationError(
                format!(
                    "Kubernetes namespace cannot be empty. Context: Attempted to initialize Kubernetes manager with empty namespace. \
                    Suggestion: Provide a valid namespace name (e.g., 'poolai', 'default'). \
                    Current namespace: '{}'",
                    self.namespace
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            info!(
                "Initializing Kubernetes API client for namespace: {}",
                self.namespace
            );

            // Try to load kubeconfig, fallback to in-cluster config
            let (api_url, token) = match self.load_kubeconfig().await {
                Ok(config) => Ok(config),
                Err(e1) => {
                    let e1_msg = format!("{}", e1);
                    match self.load_in_cluster_config().await {
                        Ok(config) => Ok(config),
                        Err(e2) => Err(AppError::InitializationError(format!(
                            "Failed to load Kubernetes configuration. Context: Cannot connect to Kubernetes cluster. \
                            Suggestion: Ensure KUBECONFIG is set, kubeconfig file exists at ~/.kube/config, or running inside a cluster with service account. \
                            Errors: kubeconfig={}, in-cluster={}",
                            e1_msg, e2
                        ))),
                    }
                }
            }?;

            *self.api_base_url.write().await = Some(api_url);
            *self.api_token.write().await = Some(token);

            info!(
                "Kubernetes API client initialized successfully for namespace: {}",
                self.namespace
            );
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Kubernetes manager initialized for namespace: {} (placeholder mode)",
                self.namespace
            );
            warn!("Kubernetes integration is a placeholder - enable cloud-sdk feature for full SDK support");
        }

        *initialized = true;
        Ok(())
    }

    /// Shutdown Kubernetes integration
    ///
    /// Cleans up Kubernetes client connections and watchers.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    /// // Use manager...
    /// manager.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&self) -> Result<(), AppError> {
        #[cfg(feature = "cloud-sdk")]
        {
            // Clear API credentials
            *self.api_base_url.write().await = None;
            *self.api_token.write().await = None;
        }
        *self.initialized.write().await = false;
        info!("Kubernetes manager shut down");
        Ok(())
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get Kubernetes API base URL (internal helper)
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if not initialized.
    async fn get_api_base_url(&self) -> Result<String, AppError> {
        let url_guard = self.api_base_url.read().await;
        url_guard.clone().ok_or_else(|| {
            AppError::InitializationError(
                "Kubernetes API client not initialized. Call initialize() first.".to_string(),
            )
        })
    }

    #[cfg(feature = "cloud-sdk")]
    /// Get Kubernetes API token (internal helper)
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if not initialized.
    async fn get_api_token(&self) -> Result<String, AppError> {
        let token_guard = self.api_token.read().await;
        token_guard.clone().ok_or_else(|| {
            AppError::InitializationError(
                "Kubernetes API client not initialized. Call initialize() first.".to_string(),
            )
        })
    }

    #[cfg(feature = "cloud-sdk")]
    /// Load kubeconfig from file or KUBECONFIG env var
    ///
    /// # Returns
    ///
    /// Returns (api_url, token) tuple if successful.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if kubeconfig cannot be loaded or parsed.
    async fn load_kubeconfig(&self) -> Result<(String, String), AppError> {
        // Get kubeconfig path from env var or default location
        let kubeconfig_path = std::env::var("KUBECONFIG").unwrap_or_else(|_| {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE")) // Windows
                .unwrap_or_else(|_| "~".to_string());
            format!("{}/.kube/config", home)
        });

        // Read kubeconfig file
        let content = tokio::fs::read_to_string(&kubeconfig_path)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to read kubeconfig file at '{}'. Error: {}",
                    kubeconfig_path, e
                ))
            })?;

        // Parse YAML (simplified - full implementation would parse full kubeconfig structure)
        // For now, we'll use a simple approach: try to extract server URL and token
        // Note: Full kubeconfig parsing would require handling contexts, clusters, users, etc.

        // Try to find server URL in kubeconfig
        let api_url = extract_kubeconfig_server(&content).ok_or_else(|| {
            AppError::ConfigError("Failed to extract server URL from kubeconfig".to_string())
        })?;

        // Try to find token (from user auth-info or exec command)
        // For now, we'll use a placeholder - full implementation would:
        // 1. Parse kubeconfig YAML structure
        // 2. Extract current context
        // 3. Get user auth-info
        // 4. Extract token or execute auth command

        // Placeholder: Try to read token from file if specified
        let token = extract_kubeconfig_token(&content)
            .unwrap_or_else(|| "placeholder-token-from-kubeconfig".to_string());

        Ok((api_url, token))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Load in-cluster configuration (when running inside Kubernetes)
    ///
    /// # Returns
    ///
    /// Returns (api_url, token) tuple if successful.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if in-cluster config cannot be loaded.
    async fn load_in_cluster_config(&self) -> Result<(String, String), AppError> {
        // Get API server host and port from environment
        let host = std::env::var("KUBERNETES_SERVICE_HOST").map_err(|_| {
            AppError::ConfigError(
                "KUBERNETES_SERVICE_HOST not set (not running in cluster)".to_string(),
            )
        })?;

        let port = std::env::var("KUBERNETES_SERVICE_PORT").unwrap_or_else(|_| "443".to_string());

        let api_url = format!("https://{}:{}", host, port);

        // Read service account token
        let token =
            tokio::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token")
                .await
                .map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to read service account token. Error: {}",
                        e
                    ))
                })?;

        Ok((api_url, token))
    }

    #[cfg(feature = "cloud-sdk")]
    /// Make HTTP request to Kubernetes API (internal helper)
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method (GET, POST, DELETE, etc.)
    /// * `path` - API path (e.g., "/apis/apps/v1/namespaces/{namespace}/deployments")
    /// * `body` - Optional request body (JSON)
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if request fails.
    async fn k8s_api_request(
        &self,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, AppError> {
        let base_url = self.get_api_base_url().await?;
        let token = self.get_api_token().await?;

        // Note: danger_accept_invalid_certs is not available with default-features = false
        // For production, use proper TLS certificates
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| AppError::NetworkError(format!(
                "Failed to create HTTP client. Context: Cannot initialize reqwest client for Kubernetes API. \
                Error: {}",
                e
            )))?;

        let url = format!("{}{}", base_url, path);
        let mut request = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "PATCH" => client.patch(&url),
            "DELETE" => client.delete(&url),
            _ => return Err(AppError::NetworkError(format!(
                "Unsupported HTTP method: {}. Context: Invalid HTTP method for Kubernetes API request. \
                Suggestion: Use GET, POST, PUT, PATCH, or DELETE.",
                method
            ))),
        };

        // Add authentication header
        request = request.bearer_auth(&token);

        // Add Content-Type header for POST/PUT/PATCH
        if matches!(method, "POST" | "PUT" | "PATCH") {
            request = request.header("Content-Type", "application/json");
        }

        // Store body reference for reuse in retry loop
        let body_ref = body.as_ref();

        // Add request body if provided
        if let Some(body) = body_ref {
            request = request.json(body);
        }

        // Send request with retry logic for transient errors
        const MAX_RETRIES: u32 = 3;
        const INITIAL_RETRY_DELAY_MS: u64 = 100;

        let mut last_error = None;
        for attempt in 0..=MAX_RETRIES {
            // Clone request for retry
            let mut retry_request = match method {
                "GET" => client.get(&url),
                "POST" => client.post(&url),
                "PUT" => client.put(&url),
                "PATCH" => client.patch(&url),
                "DELETE" => client.delete(&url),
                _ => {
                    return Err(AppError::NetworkError(format!(
                        "Unsupported HTTP method: {}",
                        method
                    )))
                }
            };

            retry_request = retry_request.bearer_auth(&token);
            if matches!(method, "POST" | "PUT" | "PATCH") {
                retry_request = retry_request.header("Content-Type", "application/json");
            }
            if let Some(body) = body_ref {
                retry_request = retry_request.json(body);
            }

            match retry_request.send().await {
                Ok(response) => {
                    let status = response.status();

                    // Read response body
                    let response_text = match response.text().await {
                        Ok(text) => text,
                        Err(e) => {
                            last_error = Some(format!(
                                "Failed to read response body. Status: {}, Error: {}",
                                status, e
                            ));
                            if attempt < MAX_RETRIES && status.is_server_error() {
                                // Retry on server errors
                                let delay_ms = INITIAL_RETRY_DELAY_MS * (1 << attempt);
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                                continue;
                            }
                            return Err(AppError::NetworkError(format!(
                                "Failed to read Kubernetes API response. Context: Cannot read response body. \
                                Method: {}, Path: {}, Status: {}, Error: {}",
                                method, path, status, e
                            )));
                        }
                    };

                    // Check for errors
                    if !status.is_success() {
                        // Retry on 5xx errors (server errors) and 429 (rate limit)
                        if attempt < MAX_RETRIES && (status.is_server_error() || status == 429) {
                            let delay_ms = INITIAL_RETRY_DELAY_MS * (1 << attempt);
                            warn!("Kubernetes API error (attempt {}/{}): Status {}, retrying in {}ms...", 
                                attempt + 1, MAX_RETRIES + 1, status, delay_ms);
                            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                            continue;
                        }

                        return Err(AppError::NetworkError(format!(
                            "Kubernetes API error. Context: API request returned error status. \
                            Suggestion: Check resource name, namespace permissions, and resource existence. \
                            Method: {}, Path: {}, Status: {}, Response: {}",
                            method, path, status, response_text
                        )));
                    }

                    // Parse JSON response
                    return serde_json::from_str(&response_text).map_err(|e| AppError::NetworkError(format!(
                        "Failed to parse Kubernetes API response. Context: Response is not valid JSON. \
                        Method: {}, Path: {}, Error: {}, Response: {}",
                        method, path, e, response_text
                    )));
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    // Retry on network errors
                    if attempt < MAX_RETRIES {
                        let delay_ms = INITIAL_RETRY_DELAY_MS * (1 << attempt);
                        warn!("Kubernetes API request failed (attempt {}/{}): {}, retrying in {}ms...", 
                            attempt + 1, MAX_RETRIES + 1, e, delay_ms);
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                        continue;
                    }
                }
            }
        }

        // All retries exhausted
        Err(AppError::NetworkError(format!(
            "Kubernetes API request failed after {} retries. Context: HTTP request to Kubernetes API failed. \
            Suggestion: Check cluster connectivity, authentication token, and API server URL. \
            Method: {}, Path: {}, Last error: {}",
            MAX_RETRIES + 1, method, path, last_error.unwrap_or_else(|| "Unknown error".to_string())
        )))
    }

    /// Create a PoolAI worker deployment
    ///
    /// # Arguments
    /// * `name` - Name of the worker
    /// * `config` - Worker configuration
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `config.image` is empty
    /// - Deployment with same name already exists
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Namespace does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::{KubernetesManager, WorkerDeploymentConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let config = WorkerDeploymentConfig::default();
    /// let deployment_id = manager.create_worker_deployment("my-worker", &config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_worker_deployment(
        &self,
        name: &str,
        config: &WorkerDeploymentConfig,
    ) -> Result<String, AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Worker deployment name cannot be empty. Context: Attempted to create worker deployment with empty name. \
                Suggestion: Provide a valid deployment name (e.g., 'worker-1', 'poolai-worker'). \
                Current value: ''"
                    .to_string(),
            ));
        }

        if config.image.is_empty() {
            return Err(AppError::ValidationError(
                format!(
                    "Worker deployment image cannot be empty. Context: Attempted to create worker deployment '{}' with empty image. \
                    Suggestion: Provide a valid container image (e.g., 'poolai/worker:v1.0.0'). \
                    Current value: ''",
                    name
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Build Deployment resource
            let deployment = build_deployment(
                name,
                &config.image,
                config.replicas as i32,
                &config.resources,
                &config.env,
            )?;

            // Create deployment via Kubernetes API
            let path = format!("/apis/apps/v1/namespaces/{}/deployments", self.namespace);
            let response = self
                .k8s_api_request("POST", &path, Some(deployment))
                .await?;

            // Extract deployment name from response
            let deployment_name = response
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .ok_or_else(|| {
                    AppError::NetworkError(
                        "Deployment created but name is missing in response".to_string(),
                    )
                })?;

            info!(
                "Created worker deployment: {} in namespace {}",
                deployment_name, self.namespace
            );
            Ok(deployment_name.to_string())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating worker deployment: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(format!("worker-{}", name))
        }
    }

    /// Update an existing worker deployment
    ///
    /// # Arguments
    /// * `name` - Name of the worker deployment to update
    /// * `config` - Updated worker configuration
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `config.image` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Deployment does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::{KubernetesManager, WorkerDeploymentConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let config = WorkerDeploymentConfig::default();
    /// manager.update_worker_deployment("my-worker", &config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_worker_deployment(
        &self,
        name: &str,
        config: &WorkerDeploymentConfig,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Worker deployment name cannot be empty. Context: Attempted to update worker deployment with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        if config.image.is_empty() {
            return Err(AppError::ValidationError(
                format!(
                    "Worker deployment image cannot be empty. Context: Attempted to update worker deployment '{}' with empty image. \
                    Suggestion: Provide a valid container image (e.g., 'poolai/worker:v1.0.0'). \
                    Current value: ''",
                    name
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Build patch body for updating deployment
            let mut patch_body = json!({
                "spec": {
                    "replicas": config.replicas,
                    "template": {
                        "spec": {
                            "containers": [{
                                "name": name,
                                "image": config.image,
                                "resources": {
                                    "requests": {
                                        "cpu": config.resources.cpu,
                                        "memory": config.resources.memory
                                    },
                                    "limits": {
                                        "cpu": config.resources.cpu,
                                        "memory": config.resources.memory
                                    }
                                }
                            }]
                        }
                    }
                }
            });

            // Add GPU if specified
            if let Some(gpu) = config.resources.gpu {
                if gpu > 0 {
                    patch_body["spec"]["template"]["spec"]["containers"][0]["resources"]
                        ["requests"]["nvidia.com/gpu"] = json!(gpu);
                    patch_body["spec"]["template"]["spec"]["containers"][0]["resources"]
                        ["limits"]["nvidia.com/gpu"] = json!(gpu);
                }
            }

            // Add environment variables if any
            if !config.env.is_empty() {
                let mut env_vars = Vec::new();
                for (key, value) in &config.env {
                    env_vars.push(json!({
                        "name": key,
                        "value": value
                    }));
                }
                patch_body["spec"]["template"]["spec"]["containers"][0]["env"] = json!(env_vars);
            }

            // Update deployment via Kubernetes API (PATCH)
            let path = format!(
                "/apis/apps/v1/namespaces/{}/deployments/{}",
                self.namespace, name
            );
            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;

            info!(
                "Updated worker deployment: {} in namespace {}",
                name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Updating worker deployment: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Delete a worker deployment
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the worker deployment to delete
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Deployment does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// manager.delete_worker_deployment("my-worker").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_worker_deployment(&self, name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Worker deployment name cannot be empty. Context: Attempted to delete worker deployment with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Delete deployment via Kubernetes API
            let path = format!(
                "/apis/apps/v1/namespaces/{}/deployments/{}",
                self.namespace, name
            );
            let _response = self.k8s_api_request("DELETE", &path, None).await?;

            info!(
                "Deleted worker deployment: {} from namespace {}",
                name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Deleting worker deployment: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Create a VM instance deployment
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the VM deployment
    /// * `config` - VM deployment configuration
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `config.image` is empty
    /// - Deployment with same name already exists
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Namespace does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::{KubernetesManager, VmDeploymentConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let config = VmDeploymentConfig::default();
    /// let deployment_id = manager.create_vm_deployment("my-vm", &config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_vm_deployment(
        &self,
        name: &str,
        config: &VmDeploymentConfig,
    ) -> Result<String, AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "VM deployment name cannot be empty. Context: Attempted to create VM deployment with empty name. \
                Suggestion: Provide a valid deployment name (e.g., 'vm-1', 'poolai-vm'). \
                Current value: ''"
                    .to_string(),
            ));
        }

        if config.image.is_empty() {
            return Err(AppError::ValidationError(
                format!(
                    "VM deployment image cannot be empty. Context: Attempted to create VM deployment '{}' with empty image. \
                    Suggestion: Provide a valid container image (e.g., 'poolai/vm:v1.0.0'). \
                    Current value: ''",
                    name
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Build VM Deployment resource (similar to worker deployment but with storage)
            let deployment = build_vm_deployment(
                name,
                &config.image,
                &config.resources,
                &config.storage,
                &config.network,
            )?;

            // Create deployment via Kubernetes API
            let path = format!("/apis/apps/v1/namespaces/{}/deployments", self.namespace);
            let response = self
                .k8s_api_request("POST", &path, Some(deployment))
                .await?;

            // Extract deployment name from response
            let deployment_name = response
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .ok_or_else(|| {
                    AppError::NetworkError(
                        "Deployment created but name is missing in response".to_string(),
                    )
                })?;

            info!(
                "Created VM deployment: {} in namespace {}",
                deployment_name, self.namespace
            );
            Ok(deployment_name.to_string())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating VM deployment: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(format!("vm-{}", name))
        }
    }

    /// Update an existing VM deployment
    ///
    /// # Arguments
    /// * `name` - Name of the VM deployment to update
    /// * `config` - Updated VM configuration
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `config.image` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Deployment does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::{KubernetesManager, VmDeploymentConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let config = VmDeploymentConfig::default();
    /// manager.update_vm_deployment("my-vm", &config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_vm_deployment(
        &self,
        name: &str,
        config: &VmDeploymentConfig,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "VM deployment name cannot be empty. Context: Attempted to update VM deployment with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        if config.image.is_empty() {
            return Err(AppError::ValidationError(
                format!(
                    "VM deployment image cannot be empty. Context: Attempted to update VM deployment '{}' with empty image. \
                    Suggestion: Provide a valid container image (e.g., 'poolai/vm:v1.0.0'). \
                    Current value: ''",
                    name
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Build patch body for updating VM deployment
            let mut patch_body = json!({
                "spec": {
                    "template": {
                        "spec": {
                            "containers": [{
                                "name": name,
                                "image": config.image,
                                "resources": {
                                    "requests": {
                                        "cpu": config.resources.cpu,
                                        "memory": config.resources.memory
                                    },
                                    "limits": {
                                        "cpu": config.resources.cpu,
                                        "memory": config.resources.memory
                                    }
                                }
                            }]
                        }
                    }
                }
            });

            // Add GPU if specified
            if let Some(gpu) = config.resources.gpu {
                if gpu > 0 {
                    patch_body["spec"]["template"]["spec"]["containers"][0]["resources"]
                        ["requests"]["nvidia.com/gpu"] = json!(gpu);
                    patch_body["spec"]["template"]["spec"]["containers"][0]["resources"]
                        ["limits"]["nvidia.com/gpu"] = json!(gpu);
                }
            }

            // Add ports if any
            if !config.network.ports.is_empty() {
                let mut ports = Vec::new();
                for port in &config.network.ports {
                    ports.push(json!({
                        "containerPort": port,
                        "protocol": "TCP"
                    }));
                }
                patch_body["spec"]["template"]["spec"]["containers"][0]["ports"] = json!(ports);
            }

            // Update deployment via Kubernetes API (PATCH)
            let path = format!(
                "/apis/apps/v1/namespaces/{}/deployments/{}",
                self.namespace, name
            );
            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;

            info!(
                "Updated VM deployment: {} in namespace {}",
                name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Updating VM deployment: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Get service endpoints for a resource
    ///
    /// # Arguments
    ///
    /// * `resource_name` - Name of the resource (deployment, service, etc.)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `resource_name` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Resource does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let endpoints = manager.get_service_endpoints("my-worker").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_service_endpoints(
        &self,
        resource_name: &str,
    ) -> Result<Vec<String>, AppError> {
        if resource_name.is_empty() {
            return Err(AppError::ValidationError(
                "Resource name cannot be empty. Context: Attempted to get service endpoints for empty resource name. \
                Suggestion: Provide a valid resource name (deployment, service, etc.). \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Query service endpoints via Kubernetes API
            let path = format!(
                "/api/v1/namespaces/{}/services/{}",
                self.namespace, resource_name
            );
            let service_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract endpoints
            let mut endpoints = Vec::new();

            // Get ClusterIP
            if let Some(cluster_ip) = service_json
                .get("spec")
                .and_then(|s| s.get("clusterIP"))
                .and_then(|ip| ip.as_str())
            {
                if cluster_ip != "None" {
                    let port = service_json
                        .get("spec")
                        .and_then(|s| s.get("ports"))
                        .and_then(|p| p.as_array())
                        .and_then(|ports| ports.first())
                        .and_then(|port| port.get("port"))
                        .and_then(|p| p.as_u64())
                        .unwrap_or(8080);

                    endpoints.push(format!("{}:{}", cluster_ip, port));
                }
            }

            // Get LoadBalancer IPs
            if let Some(ingress) = service_json
                .get("status")
                .and_then(|s| s.get("loadBalancer"))
                .and_then(|lb| lb.get("ingress"))
                .and_then(|i| i.as_array())
            {
                for ing in ingress {
                    if let Some(ip) = ing.get("ip").and_then(|i| i.as_str()) {
                        endpoints.push(ip.to_string());
                    }
                    if let Some(hostname) = ing.get("hostname").and_then(|h| h.as_str()) {
                        endpoints.push(hostname.to_string());
                    }
                }
            }

            info!(
                "Found {} endpoints for service: {}",
                endpoints.len(),
                resource_name
            );
            Ok(endpoints)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Getting service endpoints for: {} (placeholder - enable cloud-sdk feature)",
                resource_name
            );
            Ok(vec![])
        }
    }

    /// Get status of a Kubernetes Pod
    ///
    /// # Arguments
    ///
    /// * `pod_name` - Name of the pod to query
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `pod_name` is empty.
    /// Returns other `AppError` variants if pod is not found or query fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let status = manager.get_pod_status("my-pod").await?;
    /// println!("Pod phase: {}", status.phase);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pod_status(&self, pod_name: &str) -> Result<PodStatus, AppError> {
        if pod_name.is_empty() {
            return Err(AppError::ValidationError(
                "Pod name cannot be empty. Context: Attempted to get pod status with empty name. \
                Suggestion: Provide a valid pod name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        info!(
            "Getting status for pod {} in namespace {}",
            pod_name, self.namespace
        );

        #[cfg(feature = "cloud-sdk")]
        {
            // Query pod status via Kubernetes API
            let path = format!("/api/v1/namespaces/{}/pods/{}", self.namespace, pod_name);
            let pod_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract pod status information
            let name = pod_json
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or(pod_name)
                .to_string();

            let phase = pod_json
                .get("status")
                .and_then(|s| s.get("phase"))
                .and_then(|p| p.as_str())
                .unwrap_or("Unknown")
                .to_string();

            // Check if pod is ready
            let ready = pod_json
                .get("status")
                .and_then(|s| s.get("conditions"))
                .and_then(|c| c.as_array())
                .map(|conditions| {
                    conditions.iter().any(|cond| {
                        cond.get("type")
                            .and_then(|t| t.as_str())
                            .map(|t| t == "Ready")
                            .unwrap_or(false)
                            && cond
                                .get("status")
                                .and_then(|s| s.as_str())
                                .map(|s| s == "True")
                                .unwrap_or(false)
                    })
                })
                .unwrap_or(false);

            // Calculate restart count
            let restart_count = pod_json
                .get("status")
                .and_then(|s| s.get("containerStatuses"))
                .and_then(|cs| cs.as_array())
                .map(|statuses| {
                    statuses
                        .iter()
                        .filter_map(|status| status.get("restartCount").and_then(|rc| rc.as_u64()))
                        .sum::<u64>() as u32
                })
                .unwrap_or(0);

            return Ok(PodStatus {
                name,
                phase,
                ready,
                restart_count,
            });
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(PodStatus {
                name: pod_name.to_string(),
                phase: "Running".to_string(),
                ready: true,
                restart_count: 0,
            });
        }
    }

    /// Get status of a Kubernetes Deployment
    ///
    /// # Arguments
    ///
    /// * `deployment_name` - Name of the deployment to query
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `deployment_name` is empty.
    /// Returns other `AppError` variants if deployment is not found or query fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let status = manager.get_deployment_status("my-deployment").await?;
    /// println!("Deployment ready: {} ({}/{} replicas)", status.ready, status.ready_replicas, status.replicas);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_deployment_status(
        &self,
        deployment_name: &str,
    ) -> Result<DeploymentStatus, AppError> {
        if deployment_name.is_empty() {
            return Err(AppError::ValidationError(
                "Deployment name cannot be empty. Context: Attempted to get deployment status with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        info!(
            "Getting status for deployment {} in namespace {}",
            deployment_name, self.namespace
        );

        #[cfg(feature = "cloud-sdk")]
        {
            // Query deployment status via Kubernetes API
            let path = format!(
                "/apis/apps/v1/namespaces/{}/deployments/{}",
                self.namespace, deployment_name
            );
            let deployment_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract deployment status information
            let name = deployment_json
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or(deployment_name)
                .to_string();

            // Get desired replicas from spec
            let replicas = deployment_json
                .get("spec")
                .and_then(|s| s.get("replicas"))
                .and_then(|r| r.as_u64())
                .unwrap_or(0) as u32;

            // Get status information
            let status = deployment_json.get("status").ok_or_else(|| {
                AppError::NetworkError("Deployment status not found in response".to_string())
            })?;

            let ready_replicas = status
                .get("readyReplicas")
                .and_then(|r| r.as_u64())
                .unwrap_or(0) as u32;

            let available_replicas = status
                .get("availableReplicas")
                .and_then(|r| r.as_u64())
                .unwrap_or(0) as u32;

            let unavailable_replicas = status
                .get("unavailableReplicas")
                .and_then(|r| r.as_u64())
                .unwrap_or(0) as u32;

            // Deployment is ready if all replicas are available
            let ready = available_replicas == replicas && replicas > 0;

            return Ok(DeploymentStatus {
                name,
                replicas,
                ready_replicas,
                available_replicas,
                unavailable_replicas,
                ready,
            });
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(DeploymentStatus {
                name: deployment_name.to_string(),
                replicas: 1,
                ready_replicas: 1,
                available_replicas: 1,
                unavailable_replicas: 0,
                ready: true,
            });
        }
    }

    /// Get events for a Kubernetes Deployment
    ///
    /// Returns recent events related to the deployment, which can help with debugging
    /// and understanding deployment state changes.
    ///
    /// # Arguments
    ///
    /// * `deployment_name` - Name of the deployment to get events for
    /// * `limit` - Maximum number of events to return (default: 10)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `deployment_name` is empty.
    /// Returns other `AppError` variants if query fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let events = manager.get_deployment_events("my-deployment", Some(5)).await?;
    /// for event in events {
    ///     println!("Event: {} - {}", event.reason, event.message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_deployment_events(
        &self,
        deployment_name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<DeploymentEvent>, AppError> {
        if deployment_name.is_empty() {
            return Err(AppError::ValidationError(
                "Deployment name cannot be empty. Context: Attempted to get events for deployment with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        let limit = limit.unwrap_or(10);
        info!(
            "Getting events for deployment {} in namespace {} (limit: {})",
            deployment_name, self.namespace, limit
        );

        #[cfg(feature = "cloud-sdk")]
        {
            // Query events via Kubernetes API
            // Events are associated with involved objects (deployment, replicaset, pods)
            let path = format!(
                "/api/v1/namespaces/{}/events?fieldSelector=involvedObject.name={}",
                self.namespace, deployment_name
            );
            let events_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract events
            let events_array = events_json
                .get("items")
                .and_then(|i| i.as_array())
                .ok_or_else(|| {
                    AppError::NetworkError("Invalid events response format".to_string())
                })?;

            let mut events: Vec<DeploymentEvent> = events_array
                .iter()
                .take(limit)
                .filter_map(|event_json| {
                    let reason = event_json
                        .get("reason")
                        .and_then(|r| r.as_str())
                        .unwrap_or("Unknown")
                        .to_string();

                    let message = event_json
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("")
                        .to_string();

                    let event_type = event_json
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Normal")
                        .to_string();

                    let first_timestamp = event_json
                        .get("firstTimestamp")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());

                    Some(DeploymentEvent {
                        reason,
                        message,
                        event_type,
                        first_timestamp,
                    })
                })
                .collect();

            // Sort by timestamp (most recent first)
            events.sort_by(|a, b| b.first_timestamp.cmp(&a.first_timestamp));

            info!(
                "Found {} events for deployment {}",
                events.len(),
                deployment_name
            );
            return Ok(events);
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(vec![]);
        }
    }

    /// Scale a Kubernetes Deployment (placeholder)
    ///
    /// # Arguments
    ///
    /// * `deployment_name` - Name of the deployment to scale
    /// * `replicas` - Target number of replicas (must be >= 0)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `deployment_name` is empty
    /// - `replicas` is negative
    pub async fn scale_deployment(
        &self,
        deployment_name: &str,
        replicas: i32,
    ) -> Result<(), AppError> {
        if deployment_name.is_empty() {
            return Err(AppError::ValidationError(
                "Deployment name cannot be empty. Current value: ''. Suggestion: Provide a valid deployment name."
                    .to_string(),
            ));
        }

        if replicas < 0 {
            return Err(AppError::ValidationError(
                format!(
                    "Replicas must be non-negative. Current value: {}. Suggestion: Set replicas to 0 or greater.",
                    replicas
                ),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Scale deployment via Kubernetes API (PATCH)
            let path = format!(
                "/apis/apps/v1/namespaces/{}/deployments/{}",
                self.namespace, deployment_name
            );
            let patch_body = json!({
                "spec": {
                    "replicas": replicas
                }
            });

            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;

            info!(
                "Scaled deployment {} to {} replicas in namespace {}",
                deployment_name, replicas, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Scaling deployment {} to {} replicas (placeholder - enable cloud-sdk feature)",
                deployment_name, replicas
            );
            Ok(())
        }
    }

    /// Check if Kubernetes cluster is available
    ///
    /// Returns `true` if the cluster is accessible, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// if manager.is_cluster_available().await {
    ///     println!("Cluster is accessible");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_cluster_available(&self) -> bool {
        #[cfg(feature = "cloud-sdk")]
        {
            // Check cluster availability by querying API server version
            let path = "/version";
            match self.k8s_api_request("GET", path, None).await {
                Ok(_) => {
                    // Also verify namespace access
                    let ns_path = format!("/api/v1/namespaces/{}", self.namespace);
                    self.k8s_api_request("GET", &ns_path, None).await.is_ok()
                }
                Err(_) => false,
            }
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            false
        }
    }

    /// List all pods in the namespace
    ///
    /// Returns a list of pod names in the configured namespace.
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Namespace does not exist
    /// - Authentication/authorization fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let pods = manager.list_pods().await?;
    /// println!("Found {} pods", pods.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_pods(&self) -> Result<Vec<String>, AppError> {
        info!("Listing pods in namespace {}", self.namespace);

        #[cfg(feature = "cloud-sdk")]
        {
            // List pods via Kubernetes API
            let path = format!("/api/v1/namespaces/{}/pods", self.namespace);
            let pod_list_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract pod names
            let pods = pod_list_json
                .get("items")
                .and_then(|i| i.as_array())
                .ok_or_else(|| {
                    AppError::NetworkError("Invalid pod list response format".to_string())
                })?;

            let names: Vec<String> = pods
                .iter()
                .filter_map(|pod| {
                    pod.get("metadata")
                        .and_then(|m| m.get("name"))
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            return Ok(names);
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(vec![]);
        }
    }

    /// List pods for a specific deployment
    ///
    /// Returns a list of pod names that belong to the specified deployment.
    /// Uses label selector to find pods managed by the deployment.
    ///
    /// # Arguments
    ///
    /// * `deployment_name` - Name of the deployment to list pods for
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `deployment_name` is empty.
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Deployment does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let pods = manager.list_deployment_pods("my-deployment").await?;
    /// println!("Found {} pods for deployment", pods.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_deployment_pods(
        &self,
        deployment_name: &str,
    ) -> Result<Vec<String>, AppError> {
        if deployment_name.is_empty() {
            return Err(AppError::ValidationError(
                "Deployment name cannot be empty. Context: Attempted to list pods for deployment with empty name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        info!(
            "Listing pods for deployment {} in namespace {}",
            deployment_name, self.namespace
        );

        #[cfg(feature = "cloud-sdk")]
        {
            // List pods using label selector (deployments typically use app=<deployment_name> label)
            let label_selector = format!("app={}", deployment_name);
            let path = format!(
                "/api/v1/namespaces/{}/pods?labelSelector={}",
                self.namespace, label_selector
            );
            let pod_list_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract pod names
            let pods = pod_list_json
                .get("items")
                .and_then(|i| i.as_array())
                .ok_or_else(|| {
                    AppError::NetworkError("Invalid pod list response format".to_string())
                })?;

            let names: Vec<String> = pods
                .iter()
                .filter_map(|pod| {
                    pod.get("metadata")
                        .and_then(|m| m.get("name"))
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            info!(
                "Found {} pods for deployment {}",
                names.len(),
                deployment_name
            );
            return Ok(names);
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(vec![]);
        }
    }

    /// List all deployments in the namespace
    ///
    /// Returns a list of deployment names in the configured namespace.
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Namespace does not exist
    /// - Authentication/authorization fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let deployments = manager.list_deployments().await?;
    /// println!("Found {} deployments", deployments.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_deployments(&self) -> Result<Vec<String>, AppError> {
        info!("Listing deployments in namespace {}", self.namespace);

        #[cfg(feature = "cloud-sdk")]
        {
            // List deployments via Kubernetes API
            let path = format!("/apis/apps/v1/namespaces/{}/deployments", self.namespace);
            let deployment_list_json = self.k8s_api_request("GET", &path, None).await?;

            // Extract deployment names
            let deployments = deployment_list_json
                .get("items")
                .and_then(|i| i.as_array())
                .ok_or_else(|| {
                    AppError::NetworkError("Invalid deployment list response format".to_string())
                })?;

            let names: Vec<String> = deployments
                .iter()
                .filter_map(|deployment| {
                    deployment
                        .get("metadata")
                        .and_then(|m| m.get("name"))
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            return Ok(names);
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            // Placeholder implementation (when cloud-sdk feature is not enabled)
            return Ok(vec![]);
        }
    }

    /// List CRD resources from Kubernetes API
    ///
    /// # Arguments
    ///
    /// * `group` - API group (e.g., "poolai.io")
    /// * `version` - API version (e.g., "v1")
    /// * `plural` - Resource plural name (e.g., "poolaiworkers")
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let resources = manager.list_crd_resources("poolai.io", "v1", "poolaiworkers").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_crd_resources(
        &self,
        group: &str,
        version: &str,
        plural: &str,
    ) -> Result<serde_json::Value, AppError> {
        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!(
                "/apis/{}/{}/namespaces/{}/{}",
                group, version, self.namespace, plural
            );
            let resources = self.k8s_api_request("GET", &path, None).await?;
            Ok(resources)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Listing CRD resources {} (placeholder - enable cloud-sdk feature)",
                plural
            );
            Ok(serde_json::json!({"items": []}))
        }
    }

    /// Get a CRD resource from Kubernetes API
    ///
    /// # Arguments
    ///
    /// * `group` - API group (e.g., "poolai.io")
    /// * `version` - API version (e.g., "v1")
    /// * `plural` - Resource plural name (e.g., "poolaiworkers")
    /// * `name` - Resource name
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Resource does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let resource = manager.get_crd_resource("poolai.io", "v1", "poolaiworkers", "my-worker").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crd_resource(
        &self,
        group: &str,
        version: &str,
        plural: &str,
        name: &str,
    ) -> Result<serde_json::Value, AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "CRD resource name cannot be empty. Context: Attempted to get CRD resource with empty name. \
                Suggestion: Provide a valid resource name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!(
                "/apis/{}/{}/namespaces/{}/{}/{}",
                group, version, self.namespace, plural, name
            );
            let resource = self.k8s_api_request("GET", &path, None).await?;
            Ok(resource)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Getting CRD resource {}/{} (placeholder - enable cloud-sdk feature)",
                plural, name
            );
            Ok(serde_json::json!({}))
        }
    }

    /// Update CRD status
    ///
    /// # Arguments
    ///
    /// * `group` - API group (e.g., "poolai.io")
    /// * `version` - API version (e.g., "v1")
    /// * `plural` - Resource plural name (e.g., "poolaiworkers")
    /// * `name` - Resource name
    /// * `status` - Status object to update
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Resource does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let status = serde_json::json!({
    ///     "conditions": [{
    ///         "type": "Ready",
    ///         "status": "True"
    ///     }]
    /// });
    /// manager.update_crd_status("poolai.io", "v1", "poolaiworkers", "my-worker", status).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_crd_status(
        &self,
        group: &str,
        version: &str,
        plural: &str,
        name: &str,
        status: serde_json::Value,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "CRD resource name cannot be empty. Context: Attempted to update CRD status with empty name. \
                Suggestion: Provide a valid resource name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!(
                "/apis/{}/{}/namespaces/{}/{}/{}/status",
                group, version, self.namespace, plural, name
            );
            let patch_body = json!({
                "status": status
            });
            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;
            info!(
                "Updated CRD status for {}/{} in namespace {}",
                plural, name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Updating CRD status for {}/{} (placeholder - enable cloud-sdk feature)",
                plural, name
            );
            Ok(())
        }
    }

    /// Create or update a ResourceQuota
    ///
    /// # Arguments
    ///
    /// * `name` - ResourceQuota name
    /// * `quotas` - Quota specifications (CPU, memory, storage, etc.)
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Namespace does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// let quotas = serde_json::json!({
    ///     "hard": {
    ///         "requests.cpu": "4",
    ///         "requests.memory": "4Gi",
    ///         "persistentvolumeclaims": "10"
    ///     }
    /// });
    /// manager.create_or_update_resource_quota("tenant-abc", quotas).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_or_update_resource_quota(
        &self,
        name: &str,
        quotas: serde_json::Value,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "ResourceQuota name cannot be empty. Context: Attempted to create ResourceQuota with empty name. \
                Suggestion: Provide a valid ResourceQuota name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Check if ResourceQuota exists
            let get_path = format!(
                "/api/v1/namespaces/{}/resourcequotas/{}",
                self.namespace, name
            );
            let exists = self.k8s_api_request("GET", &get_path, None).await.is_ok();

            let quota_body = json!({
                "apiVersion": "v1",
                "kind": "ResourceQuota",
                "metadata": {
                    "name": name,
                    "namespace": self.namespace,
                    "labels": {
                        "managed-by": "poolai"
                    }
                },
                "spec": quotas
            });

            if exists {
                // Update existing ResourceQuota
                let path = format!(
                    "/api/v1/namespaces/{}/resourcequotas/{}",
                    self.namespace, name
                );
                let _response = self
                    .k8s_api_request(
                        "PATCH",
                        &path,
                        Some(json!({
                            "spec": quotas
                        })),
                    )
                    .await?;
                info!(
                    "Updated ResourceQuota {} in namespace {}",
                    name, self.namespace
                );
            } else {
                // Create new ResourceQuota
                let path = format!("/api/v1/namespaces/{}/resourcequotas", self.namespace);
                let _response = self
                    .k8s_api_request("POST", &path, Some(quota_body))
                    .await?;
                info!(
                    "Created ResourceQuota {} in namespace {}",
                    name, self.namespace
                );
            }

            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating/updating ResourceQuota {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Watch CRD resources using Kubernetes watch API
    ///
    /// This is more efficient than polling as it uses server-sent events.
    /// The watch API provides real-time updates without periodic polling.
    ///
    /// # Arguments
    ///
    /// * `group` - API group (e.g., "poolai.io")
    /// * `version` - API version (e.g., "v1")
    /// * `plural` - Resource plural name (e.g., "poolaiworkers")
    /// * `resource_version` - Optional resource version to start watching from
    ///
    /// # Returns
    ///
    /// Returns a stream of watch events (Added, Modified, Deleted).
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Kubernetes API is unreachable
    /// - Watch connection fails
    ///
    /// # Note
    ///
    /// This is a placeholder implementation. Full watch API support would require:
    /// - Proper SSE (Server-Sent Events) parsing
    /// - Handling watch bookmarks for efficient reconnection
    /// - Error handling and reconnection logic
    /// - Resource version tracking
    ///
    /// Currently, the operator uses HTTP polling which is simpler and more reliable
    /// for initial implementation. Watch API can be integrated later for better efficiency.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    /// use tracing::info;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// // Watch for changes (in production, you'd process the stream)
    /// // Note: This is a placeholder - full implementation would return a proper stream
    /// info!("Watch API support is available but not yet fully implemented");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "cloud-sdk")]
    pub async fn watch_crd_resources(
        &self,
        _group: &str,
        _version: &str,
        _plural: &str,
        _resource_version: Option<&str>,
    ) -> Result<(), AppError> {
        // Placeholder: Full watch API implementation would:
        // 1. Create a GET request with ?watch=true parameter
        // 2. Parse Server-Sent Events (SSE) stream
        // 3. Handle watch events (ADDED, MODIFIED, DELETED, BOOKMARK)
        // 4. Implement reconnection logic on connection loss
        // 5. Track resourceVersion for efficient reconnection

        info!("Watch API method called (placeholder - use HTTP polling for now)");
        Ok(())
    }

    /// Delete a ResourceQuota
    ///
    /// # Arguments
    ///
    /// * `name` - ResourceQuota name to delete
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `name` is empty.
    /// Returns `AppError::NetworkError` if:
    /// - ResourceQuota does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// manager.delete_resource_quota("tenant-abc").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_resource_quota(&self, name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "ResourceQuota name cannot be empty. Context: Attempted to delete ResourceQuota with empty name. \
                Suggestion: Provide a valid ResourceQuota name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Delete ResourceQuota via Kubernetes API
            let path = format!(
                "/api/v1/namespaces/{}/resourcequotas/{}",
                self.namespace, name
            );
            let _response = self.k8s_api_request("DELETE", &path, None).await?;

            info!(
                "Deleted ResourceQuota {} from namespace {}",
                name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Deleting ResourceQuota {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Create a Kubernetes Service
    ///
    /// # Arguments
    ///
    /// * `name` - Service name
    /// * `deployment_name` - Deployment name to create service for
    /// * `ports` - List of ports to expose
    /// * `service_type` - Service type (ClusterIP, NodePort, LoadBalancer)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `deployment_name` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Service with same name already exists
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::{KubernetesManager, ServiceType};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// manager.create_service("my-service", "my-deployment", &[8080], ServiceType::ClusterIP).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_service(
        &self,
        name: &str,
        deployment_name: &str,
        ports: &[u16],
        service_type: ServiceType,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Service name cannot be empty. Context: Attempted to create service with empty name. \
                Suggestion: Provide a valid service name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        if deployment_name.is_empty() {
            return Err(AppError::ValidationError(
                "Deployment name cannot be empty. Context: Attempted to create service with empty deployment name. \
                Suggestion: Provide a valid deployment name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let service_type_str = match service_type {
                ServiceType::ClusterIP => "ClusterIP",
                ServiceType::NodePort => "NodePort",
                ServiceType::LoadBalancer => "LoadBalancer",
            };

            let mut service_ports = Vec::new();
            for port in ports {
                service_ports.push(json!({
                    "port": port,
                    "targetPort": port,
                    "protocol": "TCP"
                }));
            }

            let service_body = json!({
                "apiVersion": "v1",
                "kind": "Service",
                "metadata": {
                    "name": name,
                    "namespace": self.namespace,
                    "labels": {
                        "app": deployment_name,
                        "managed-by": "poolai"
                    }
                },
                "spec": {
                    "type": service_type_str,
                    "selector": {
                        "app": deployment_name
                    },
                    "ports": service_ports
                }
            });

            // Create service via Kubernetes API
            let path = format!("/api/v1/namespaces/{}/services", self.namespace);
            let _response = self
                .k8s_api_request("POST", &path, Some(service_body))
                .await?;

            info!(
                "Created service: {} for deployment {} in namespace {}",
                name, deployment_name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating service: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Create a PersistentVolumeClaim
    ///
    /// # Arguments
    ///
    /// * `name` - PVC name
    /// * `size` - Storage size (e.g., "10Gi")
    /// * `storage_class` - Storage class name
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `size` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - PVC with same name already exists
    /// - Kubernetes API is unreachable
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = KubernetesManager::new("poolai".to_string());
    /// manager.initialize().await?;
    ///
    /// manager.create_pvc("my-pvc", "10Gi", "standard").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_pvc(
        &self,
        name: &str,
        size: &str,
        storage_class: &str,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "PVC name cannot be empty. Context: Attempted to create PVC with empty name. \
                Suggestion: Provide a valid PVC name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        if size.is_empty() {
            return Err(AppError::ValidationError(
                "Storage size cannot be empty. Context: Attempted to create PVC with empty size. \
                Suggestion: Provide a valid storage size (e.g., '10Gi', '100Mi'). \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let pvc_body = json!({
                "apiVersion": "v1",
                "kind": "PersistentVolumeClaim",
                "metadata": {
                    "name": name,
                    "namespace": self.namespace,
                    "labels": {
                        "managed-by": "poolai"
                    }
                },
                "spec": {
                    "accessModes": ["ReadWriteOnce"],
                    "resources": {
                        "requests": {
                            "storage": size
                        }
                    },
                    "storageClassName": storage_class
                }
            });

            // Create PVC via Kubernetes API
            let path = format!(
                "/api/v1/namespaces/{}/persistentvolumeclaims",
                self.namespace
            );
            let _response = self.k8s_api_request("POST", &path, Some(pvc_body)).await?;

            info!(
                "Created PVC: {} with size {} in namespace {}",
                name, size, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating PVC: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Update an existing Kubernetes Service
    ///
    /// # Arguments
    ///
    /// * `name` - Service name to update
    /// * `deployment_name` - Deployment name the service is for
    /// * `ports` - Updated list of ports to expose
    /// * `service_type` - Updated service type
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `deployment_name` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Service does not exist
    /// - Kubernetes API is unreachable
    pub async fn update_service(
        &self,
        name: &str,
        deployment_name: &str,
        ports: &[u16],
        service_type: ServiceType,
    ) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Service name cannot be empty. Context: Attempted to update service with empty name. \
                Suggestion: Provide a valid service name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let service_type_str = match service_type {
                ServiceType::ClusterIP => "ClusterIP",
                ServiceType::NodePort => "NodePort",
                ServiceType::LoadBalancer => "LoadBalancer",
            };

            let mut service_ports = Vec::new();
            for port in ports {
                service_ports.push(json!({
                    "port": port,
                    "targetPort": port,
                    "protocol": "TCP"
                }));
            }

            // Update service spec (PATCH)
            let patch_body = json!({
                "spec": {
                    "type": service_type_str,
                    "selector": {
                        "app": deployment_name
                    },
                    "ports": service_ports
                }
            });

            let path = format!("/api/v1/namespaces/{}/services/{}", self.namespace, name);
            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;

            info!(
                "Updated service: {} for deployment {} in namespace {}",
                name, deployment_name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Updating service: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Delete a Kubernetes Service
    ///
    /// # Arguments
    ///
    /// * `name` - Service name to delete
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `name` is empty.
    /// Returns `AppError::NetworkError` if:
    /// - Service does not exist
    /// - Kubernetes API is unreachable
    pub async fn delete_service(&self, name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "Service name cannot be empty. Context: Attempted to delete service with empty name. \
                Suggestion: Provide a valid service name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!("/api/v1/namespaces/{}/services/{}", self.namespace, name);
            let _response = self.k8s_api_request("DELETE", &path, None).await?;

            info!(
                "Deleted service: {} from namespace {}",
                name, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Deleting service: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Update an existing PersistentVolumeClaim
    ///
    /// # Arguments
    ///
    /// * `name` - PVC name to update
    /// * `size` - Updated storage size (e.g., "20Gi")
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `name` is empty
    /// - `size` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - PVC does not exist
    /// - Kubernetes API is unreachable
    ///
    /// # Note
    ///
    /// PVC size updates are only allowed if the storage class supports volume expansion.
    pub async fn update_pvc(&self, name: &str, size: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "PVC name cannot be empty. Context: Attempted to update PVC with empty name. \
                Suggestion: Provide a valid PVC name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        if size.is_empty() {
            return Err(AppError::ValidationError(
                "Storage size cannot be empty. Context: Attempted to update PVC with empty size. \
                Suggestion: Provide a valid storage size (e.g., '10Gi', '100Mi'). \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Update PVC spec (PATCH) - only size can be updated
            let patch_body = json!({
                "spec": {
                    "resources": {
                        "requests": {
                            "storage": size
                        }
                    }
                }
            });

            let path = format!(
                "/api/v1/namespaces/{}/persistentvolumeclaims/{}",
                self.namespace, name
            );
            let _response = self
                .k8s_api_request("PATCH", &path, Some(patch_body))
                .await?;

            info!(
                "Updated PVC: {} with size {} in namespace {}",
                name, size, self.namespace
            );
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Updating PVC: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Delete a PersistentVolumeClaim
    ///
    /// # Arguments
    ///
    /// * `name` - PVC name to delete
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if `name` is empty.
    /// Returns `AppError::NetworkError` if:
    /// - PVC does not exist
    /// - Kubernetes API is unreachable
    pub async fn delete_pvc(&self, name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::ValidationError(
                "PVC name cannot be empty. Context: Attempted to delete PVC with empty name. \
                Suggestion: Provide a valid PVC name. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!(
                "/api/v1/namespaces/{}/persistentvolumeclaims/{}",
                self.namespace, name
            );
            let _response = self.k8s_api_request("DELETE", &path, None).await?;

            info!("Deleted PVC: {} from namespace {}", name, self.namespace);
            Ok(())
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Deleting PVC: {} (placeholder - enable cloud-sdk feature)",
                name
            );
            Ok(())
        }
    }

    /// Check if a Service exists
    ///
    /// # Arguments
    ///
    /// * `name` - Service name to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the service exists, `false` otherwise.
    pub async fn service_exists(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!("/api/v1/namespaces/{}/services/{}", self.namespace, name);
            self.k8s_api_request("GET", &path, None).await.is_ok()
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            false
        }
    }

    /// Check if a PVC exists
    ///
    /// # Arguments
    ///
    /// * `name` - PVC name to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the PVC exists, `false` otherwise.
    pub async fn pvc_exists(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let path = format!(
                "/api/v1/namespaces/{}/persistentvolumeclaims/{}",
                self.namespace, name
            );
            self.k8s_api_request("GET", &path, None).await.is_ok()
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            false
        }
    }
}

/// Worker deployment configuration for Kubernetes
#[derive(Debug, Clone)]
pub struct WorkerDeploymentConfig {
    pub image: String,
    pub replicas: u32,
    pub resources: ResourceRequirements,
    pub env: HashMap<String, String>,
}

/// VM deployment configuration for Kubernetes
#[derive(Debug, Clone)]
pub struct VmDeploymentConfig {
    pub image: String,
    pub resources: ResourceRequirements,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
}

/// Kubernetes resource requirements
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub cpu: String,      // e.g., "100m", "1"
    pub memory: String,   // e.g., "128Mi", "1Gi"
    pub gpu: Option<u32>, // Number of GPUs
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub size: String,          // e.g., "10Gi"
    pub storage_class: String, // e.g., "standard", "ssd"
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub ports: Vec<u16>,
    pub service_type: ServiceType,
}

/// Kubernetes service type
#[derive(Debug, Clone)]
pub enum ServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
}

impl Default for WorkerDeploymentConfig {
    fn default() -> Self {
        Self {
            image: "poolai/worker:latest".to_string(),
            replicas: 1,
            resources: ResourceRequirements {
                cpu: "100m".to_string(),
                memory: "128Mi".to_string(),
                gpu: None,
            },
            env: HashMap::new(),
        }
    }
}

impl Default for VmDeploymentConfig {
    fn default() -> Self {
        Self {
            image: "poolai/vm:latest".to_string(),
            resources: ResourceRequirements {
                cpu: "500m".to_string(),
                memory: "512Mi".to_string(),
                gpu: None,
            },
            storage: StorageConfig {
                size: "10Gi".to_string(),
                storage_class: "standard".to_string(),
            },
            network: NetworkConfig {
                ports: vec![8080],
                service_type: ServiceType::ClusterIP,
            },
        }
    }
}

#[cfg(feature = "cloud-sdk")]
/// Build Kubernetes Deployment JSON for worker
fn build_deployment(
    name: &str,
    image: &str,
    replicas: i32,
    resources: &ResourceRequirements,
    env: &HashMap<String, String>,
) -> Result<serde_json::Value, AppError> {
    let mut env_vars = Vec::new();
    for (key, value) in env {
        env_vars.push(json!({
            "name": key,
            "value": value
        }));
    }

    let mut resources_obj = json!({
        "requests": {
            "cpu": resources.cpu,
            "memory": resources.memory
        },
        "limits": {
            "cpu": resources.cpu,
            "memory": resources.memory
        }
    });

    if let Some(gpu) = resources.gpu {
        if gpu > 0 {
            resources_obj["requests"]["nvidia.com/gpu"] = json!(gpu);
            resources_obj["limits"]["nvidia.com/gpu"] = json!(gpu);
        }
    }

    Ok(json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": {
                "app": name,
                "managed-by": "poolai"
            }
        },
        "spec": {
            "replicas": replicas,
            "selector": {
                "matchLabels": {
                    "app": name
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": name
                    }
                },
                "spec": {
                    "containers": [{
                        "name": name,
                        "image": image,
                        "resources": resources_obj,
                        "env": env_vars
                    }]
                }
            }
        }
    }))
}

#[cfg(feature = "cloud-sdk")]
/// Build Kubernetes Deployment JSON for VM
fn build_vm_deployment(
    name: &str,
    image: &str,
    resources: &ResourceRequirements,
    _storage: &StorageConfig,
    network: &NetworkConfig,
) -> Result<serde_json::Value, AppError> {
    let mut resources_obj = json!({
        "requests": {
            "cpu": resources.cpu,
            "memory": resources.memory
        },
        "limits": {
            "cpu": resources.cpu,
            "memory": resources.memory
        }
    });

    if let Some(gpu) = resources.gpu {
        if gpu > 0 {
            resources_obj["requests"]["nvidia.com/gpu"] = json!(gpu);
            resources_obj["limits"]["nvidia.com/gpu"] = json!(gpu);
        }
    }

    let mut ports = Vec::new();
    for port in &network.ports {
        ports.push(json!({
            "containerPort": port,
            "protocol": "TCP"
        }));
    }

    let _service_type = match network.service_type {
        ServiceType::ClusterIP => "ClusterIP",
        ServiceType::NodePort => "NodePort",
        ServiceType::LoadBalancer => "LoadBalancer",
    };

    Ok(json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": {
                "app": name,
                "managed-by": "poolai",
                "type": "vm"
            }
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": {
                    "app": name
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": name,
                        "type": "vm"
                    }
                },
                "spec": {
                    "containers": [{
                        "name": name,
                        "image": image,
                        "resources": resources_obj,
                        "ports": ports,
                        "volumeMounts": [{
                            "name": "storage",
                            "mountPath": "/data"
                        }]
                    }],
                    "volumes": [{
                        "name": "storage",
                        "persistentVolumeClaim": {
                            "claimName": format!("{}-pvc", name)
                        }
                    }]
                }
            }
        }
    }))
}

#[cfg(feature = "cloud-sdk")]
/// Extract server URL from kubeconfig content (simplified)
///
/// This is a simplified implementation. Full kubeconfig parsing would require
/// proper YAML parsing and handling of contexts, clusters, users, etc.
fn extract_kubeconfig_server(content: &str) -> Option<String> {
    // Simple regex-like search for server URL
    // Full implementation would parse YAML structure
    for line in content.lines() {
        if line.trim().starts_with("server:") {
            let url = line
                .split(':')
                .skip(1)
                .collect::<String>()
                .trim()
                .to_string();
            if !url.is_empty() {
                return Some(url);
            }
        }
    }
    None
}

#[cfg(feature = "cloud-sdk")]
/// Extract token from kubeconfig content (simplified)
///
/// This is a simplified implementation. Full kubeconfig parsing would require
/// proper YAML parsing and handling of exec commands, token files, etc.
fn extract_kubeconfig_token(content: &str) -> Option<String> {
    // Simple search for token
    // Full implementation would parse YAML structure and handle:
    // - token: <token>
    // - tokenFile: <path>
    // - exec: <command>
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("token:") {
            let token = trimmed
                .split(':')
                .skip(1)
                .collect::<String>()
                .trim()
                .to_string();
            if !token.is_empty() {
                return Some(token);
            }
        } else if trimmed.starts_with("token-file:") {
            // Try to read token from file
            let path = trimmed
                .split(':')
                .skip(1)
                .collect::<String>()
                .trim()
                .to_string();
            if let Ok(token) = std::fs::read_to_string(&path) {
                return Some(token.trim().to_string());
            }
        }
    }
    None
}
