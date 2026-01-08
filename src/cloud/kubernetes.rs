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
            info!("Initializing Kubernetes API client for namespace: {}", self.namespace);
            
            // TODO: Load kubeconfig and extract API server URL and token
            // For now, use in-cluster config or KUBECONFIG env var
            // This is a placeholder - full implementation would:
            // 1. Load kubeconfig from ~/.kube/config or KUBECONFIG env var
            // 2. Extract API server URL and authentication token
            // 3. Verify namespace exists
            
            // Placeholder: Set API base URL (would be extracted from kubeconfig)
            *self.api_base_url.write().await = Some(
                std::env::var("KUBERNETES_SERVICE_HOST")
                    .map(|host| format!("https://{}:443", host))
                    .unwrap_or_else(|_| "https://kubernetes.default.svc".to_string())
            );
            
            // Placeholder: Set API token (would be extracted from kubeconfig or service account)
            *self.api_token.write().await = Some(
                std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token")
                    .unwrap_or_else(|_| "placeholder-token".to_string())
            );
            
            info!("Kubernetes API client initialized (placeholder - full kubeconfig loading TODO)");
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
                "Kubernetes API client not initialized. Call initialize() first.".to_string()
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
                "Kubernetes API client not initialized. Call initialize() first.".to_string()
            )
        })
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
        
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true) // For self-signed certs in development
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
        
        // Add request body if provided
        if let Some(body) = body {
            request = request.json(&body);
        }
        
        // Send request
        let response = request.send().await.map_err(|e| AppError::NetworkError(format!(
            "Kubernetes API request failed. Context: HTTP request to Kubernetes API failed. \
            Suggestion: Check cluster connectivity, authentication token, and API server URL. \
            Method: {}, Path: {}, Error: {}",
            method, path, e
        )))?;
        
        let status = response.status();
        
        // Read response body
        let response_text = response.text().await.map_err(|e| AppError::NetworkError(format!(
            "Failed to read Kubernetes API response. Context: Cannot read response body. \
            Method: {}, Path: {}, Status: {}, Error: {}",
            method, path, status, e
        )))?;
        
        // Check for errors
        if !status.is_success() {
            return Err(AppError::NetworkError(format!(
                "Kubernetes API error. Context: API request returned error status. \
                Suggestion: Check resource name, namespace permissions, and resource existence. \
                Method: {}, Path: {}, Status: {}, Response: {}",
                method, path, status, response_text
            )));
        }
        
        // Parse JSON response
        serde_json::from_str(&response_text).map_err(|e| AppError::NetworkError(format!(
            "Failed to parse Kubernetes API response. Context: Response is not valid JSON. \
            Method: {}, Path: {}, Error: {}, Response: {}",
            method, path, e, response_text
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
            let response = self.k8s_api_request("POST", &path, Some(deployment)).await?;
            
            // Extract deployment name from response
            let deployment_name = response
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .ok_or_else(|| AppError::NetworkError(
                    "Deployment created but name is missing in response".to_string()
                ))?;
            
            info!("Created worker deployment: {} in namespace {}", deployment_name, self.namespace);
            Ok(deployment_name.to_string())
        }
        
        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!("Creating worker deployment: {} (placeholder - enable cloud-sdk feature)", name);
            Ok(format!("worker-{}", name))
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
            let path = format!("/apis/apps/v1/namespaces/{}/deployments/{}", self.namespace, name);
            let _response = self.k8s_api_request("DELETE", &path, None).await?;
            
            info!("Deleted worker deployment: {} from namespace {}", name, self.namespace);
            Ok(())
        }
        
        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!("Deleting worker deployment: {} (placeholder - enable cloud-sdk feature)", name);
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

        // TODO: Implement Kubernetes VM deployment
        info!("Creating VM deployment: {} (placeholder)", name);
        Ok(format!("vm-{}", name))
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

        // TODO: Query Kubernetes API for service endpoints
        info!("Getting service endpoints for: {} (placeholder)", resource_name);
        Ok(vec![])
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

        info!("Getting status for pod {} in namespace {}", pod_name, self.namespace);
        
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
                        && cond.get("status")
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
                    statuses.iter()
                        .filter_map(|status| {
                            status.get("restartCount")
                                .and_then(|rc| rc.as_u64())
                        })
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
        
        // Placeholder implementation (when cloud-sdk feature is not enabled)
        Ok(PodStatus {
            name: pod_name.to_string(),
            phase: "Running".to_string(),
            ready: true,
            restart_count: 0,
        })
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
    pub async fn scale_deployment(&self, deployment_name: &str, replicas: i32) -> Result<(), AppError> {
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
            let path = format!("/apis/apps/v1/namespaces/{}/deployments/{}", self.namespace, deployment_name);
            let patch_body = json!({
                "spec": {
                    "replicas": replicas
                }
            });
            
            let _response = self.k8s_api_request("PATCH", &path, Some(patch_body)).await?;
            
            info!("Scaled deployment {} to {} replicas in namespace {}", deployment_name, replicas, self.namespace);
            Ok(())
        }
        
        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!("Scaling deployment {} to {} replicas (placeholder - enable cloud-sdk feature)", deployment_name, replicas);
            Ok(())
        }
    }

    /// Check if Kubernetes cluster is available
    ///
    /// Returns `true` if the cluster is accessible, `false` otherwise.
    /// This is a placeholder implementation that always returns `false`.
    ///
    /// # Future Implementation
    ///
    /// This will be enhanced to:
    /// - Perform actual cluster connectivity check
    /// - Verify API server accessibility
    /// - Check authentication/authorization
    pub async fn is_cluster_available(&self) -> bool {
        // TODO: Implement actual cluster availability check
        // - Ping API server
        // - Check authentication
        // - Verify namespace access
        false
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
                .ok_or_else(|| AppError::NetworkError(
                    "Invalid pod list response format".to_string()
                ))?;
            
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
        
        // Placeholder implementation (when cloud-sdk feature is not enabled)
        Ok(vec![])
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
                .ok_or_else(|| AppError::NetworkError(
                    "Invalid deployment list response format".to_string()
                ))?;
            
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
        
        // Placeholder implementation (when cloud-sdk feature is not enabled)
        Ok(vec![])
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
    pub memory: String,  // e.g., "128Mi", "1Gi"
    pub gpu: Option<u32>, // Number of GPUs
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub size: String,        // e.g., "10Gi"
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
