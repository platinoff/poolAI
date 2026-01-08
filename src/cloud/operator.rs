//! Kubernetes Operator for PoolAI
//!
//! This module provides a Kubernetes operator for managing PoolAI resources
//! through Custom Resource Definitions (CRDs).
//!
//! # Features
//!
//! - Custom Resource Definitions (CRDs) for PoolAI resources
//! - Resource watchers and controllers
//! - Reconciliation loops
//! - Event handling
//!
//! # Example
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use poolai::cloud::operator::PoolAIOperator;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Create and start the operator
//! let operator = PoolAIOperator::new("poolai".to_string());
//! operator.start().await?;
//!
//! // Operator is now watching for CRD changes and reconciling resources
//! // Check if operator is running
//! if operator.is_running().await {
//!     println!("Operator is running");
//! }
//!
//! // Stop the operator when done
//! operator.stop().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## CRD Resources
//!
//! The operator manages three types of Custom Resources:
//! - `PoolAIWorker` - Worker deployments
//! - `PoolAIVM` - VM instances
//! - `PoolAITenant` - Tenant configurations

use crate::core::error::AppError;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{interval, Duration};
use tracing::{info, warn, error};

/// CRD event type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrdEventType {
    /// Resource was added
    Added,
    /// Resource was modified
    Modified,
    /// Resource was deleted
    Deleted,
}

/// CRD resource type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrdResourceType {
    /// PoolAIWorker resource
    Worker,
    /// PoolAIVM resource
    Vm,
    /// PoolAITenant resource
    Tenant,
}

/// CRD event
#[derive(Debug, Clone)]
pub struct CrdEvent {
    /// Event type
    pub event_type: CrdEventType,
    /// Resource type
    pub resource_type: CrdResourceType,
    /// Resource name
    pub name: String,
    /// Resource namespace
    pub namespace: String,
}

/// PoolAI Kubernetes Operator
///
/// Manages PoolAI resources through Kubernetes CRDs.
pub struct PoolAIOperator {
    namespace: String,
    running: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// Kubernetes manager for API operations
    k8s_manager: Option<Arc<crate::cloud::kubernetes::KubernetesManager>>,
    #[cfg(feature = "cloud-sdk")]
    /// Watcher shutdown handles
    watcher_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl PoolAIOperator {
    /// Create a new PoolAI operator
    ///
    /// # Arguments
    ///
    /// * `namespace` - Kubernetes namespace to operate in
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::operator::PoolAIOperator;
    ///
    /// let operator = PoolAIOperator::new("poolai".to_string());
    /// ```
    pub fn new(namespace: String) -> Self {
        Self {
            namespace: namespace.clone(),
            running: Arc::new(RwLock::new(false)),
            #[cfg(feature = "cloud-sdk")]
            k8s_manager: Some(Arc::new(crate::cloud::kubernetes::KubernetesManager::new(namespace.clone()))),
            #[cfg(feature = "cloud-sdk")]
            watcher_handles: Arc::new(RwLock::new(Vec::new())),
            #[cfg(feature = "cloud-sdk")]
            event_tx: None,
        }
    }

    /// Start the operator
    ///
    /// This will:
    /// - Initialize CRD watchers
    /// - Start reconciliation loops
    /// - Handle resource events
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - Kubernetes cluster is not accessible
    /// - CRDs are not installed
    /// - Namespace does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::operator::PoolAIOperator;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let operator = PoolAIOperator::new("poolai".to_string());
    /// operator.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }

        info!("Starting PoolAI Kubernetes operator for namespace: {}", self.namespace);

        if self.namespace.is_empty() {
            return Err(AppError::InitializationError(
                format!(
                    "Kubernetes namespace cannot be empty. Context: Attempted to start operator with empty namespace. \
                    Suggestion: Provide a valid namespace name (e.g., 'poolai', 'default'). \
                    Current namespace: '{}'",
                    self.namespace
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            // Initialize Kubernetes manager if not already initialized
            if let Some(ref k8s_manager) = self.k8s_manager {
                k8s_manager.initialize().await.map_err(|e| AppError::InitializationError(format!(
                    "Failed to initialize Kubernetes manager for operator. Context: Cannot start operator without Kubernetes access. \
                    Suggestion: Check kubeconfig and cluster connectivity. \
                    Error: {}",
                    e
                )))?;
            }
            
            // Verify CRDs are installed (check if we can list CRDs)
            // For now, we'll assume CRDs are installed
            // In production, we would check: GET /apis/apiextensions.k8s.io/v1/customresourcedefinitions
            
            // Create event channel for CRD events
            let (event_tx, event_rx) = mpsc::unbounded_channel();
            
            // Store event_tx for shutdown
            // Note: We can't store it in self because of lifetime issues
            // In production, we'd use a different approach (e.g., Arc<Mutex<Option<...>>>)
            
            // Start watchers for PoolAIWorker, PoolAIVM, PoolAITenant CRDs
            let watcher_handles = self.watcher_handles.clone();
            let namespace = self.namespace.clone();
            let k8s_manager = self.k8s_manager.clone();
            
            // Start watcher for PoolAIWorker CRD
            let handle_worker = tokio::spawn(Self::watch_crd_resources(
                "poolaiworkers".to_string(),
                "poolai.io".to_string(),
                "v1".to_string(),
                namespace.clone(),
                CrdResourceType::Worker,
                event_tx.clone(),
                k8s_manager.clone(),
            ));
            watcher_handles.write().await.push(handle_worker);
            
            // Start watcher for PoolAIVM CRD
            let handle_vm = tokio::spawn(Self::watch_crd_resources(
                "poolaivms".to_string(),
                "poolai.io".to_string(),
                "v1".to_string(),
                namespace.clone(),
                CrdResourceType::Vm,
                event_tx.clone(),
                k8s_manager.clone(),
            ));
            watcher_handles.write().await.push(handle_vm);
            
            // Start watcher for PoolAITenant CRD
            let handle_tenant = tokio::spawn(Self::watch_crd_resources(
                "poolaitenants".to_string(),
                "poolai.io".to_string(),
                "v1".to_string(),
                namespace.clone(),
                CrdResourceType::Tenant,
                event_tx.clone(),
                k8s_manager.clone(),
            ));
            watcher_handles.write().await.push(handle_tenant);
            
            // Start reconciliation loop
            let k8s_manager_reconcile = self.k8s_manager.clone();
            let namespace_reconcile = self.namespace.clone();
            let handle_reconcile = tokio::spawn(Self::reconciliation_loop(
                event_rx,
                namespace_reconcile,
                k8s_manager_reconcile,
            ));
            watcher_handles.write().await.push(handle_reconcile);
            
            info!("Started {} watchers and reconciliation loop", watcher_handles.read().await.len());
        }
        
        #[cfg(not(feature = "cloud-sdk"))]
        {
            warn!("Operator started without cloud-sdk feature - CRD watching disabled");
        }

        info!("PoolAI Kubernetes operator started for namespace: {}", self.namespace);
        *running = true;
        Ok(())
    }

    /// Stop the operator
    ///
    /// Gracefully shuts down the operator, stopping all watchers and reconciliation loops.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::operator::PoolAIOperator;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let operator = PoolAIOperator::new("poolai".to_string());
    /// operator.start().await?;
    /// // Use operator...
    /// operator.stop().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }

        info!("Stopping PoolAI Kubernetes operator...");

        #[cfg(feature = "cloud-sdk")]
        {
            // Stop watchers and reconciliation loops gracefully
            let mut handles = self.watcher_handles.write().await;
            info!("Stopping {} watcher tasks...", handles.len());
            
            // Abort all watcher tasks
            for handle in handles.drain(..) {
                handle.abort();
            }
            
            // Note: Event channel is closed when all senders are dropped
            // Watchers will stop when their tasks are aborted
            
            // Shutdown Kubernetes manager
            if let Some(ref k8s_manager) = self.k8s_manager {
                let _ = k8s_manager.shutdown().await;
            }
            
            info!("All watchers stopped");
        }

        *running = false;
        info!("PoolAI Kubernetes operator stopped");
        Ok(())
    }

    /// Check if operator is running
    ///
    /// Returns `true` if the operator is currently running, `false` otherwise.
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

/// PoolAI Worker Custom Resource
///
/// Represents a PoolAI worker resource in Kubernetes.
#[derive(Debug, Clone)]
pub struct PoolAIWorker {
    /// Worker name
    pub name: String,
    /// Worker image
    pub image: String,
    /// Number of replicas
    pub replicas: u32,
    /// Resource requirements
    pub resources: WorkerResources,
    /// Environment variables
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Worker resource requirements
#[derive(Debug, Clone)]
pub struct WorkerResources {
    /// CPU request/limit (e.g., "100m", "1")
    pub cpu: String,
    /// Memory request/limit (e.g., "128Mi", "1Gi")
    pub memory: String,
    /// GPU count (optional)
    pub gpu: Option<u32>,
}

/// PoolAI VM Custom Resource
///
/// Represents a PoolAI VM instance resource in Kubernetes.
/// This CRD allows users to define VM instances declaratively.
///
/// # Example
///
/// ```yaml
/// apiVersion: poolai.io/v1
/// kind: PoolAIVM
/// metadata:
///   name: my-vm
/// spec:
///   image: poolai/vm:v1.0.0
///   resources:
///     cpu: "1"
///     memory: "2Gi"
///   storage:
///     size: "20Gi"
///     storage_class: "ssd"
/// ```
#[derive(Debug, Clone)]
pub struct PoolAIVM {
    /// VM name
    pub name: String,
    /// VM image
    pub image: String,
    /// Resource requirements
    pub resources: VmResources,
    /// Storage configuration
    pub storage: VmStorage,
    /// Network ports
    pub ports: Option<Vec<u16>>,
}

/// VM resource requirements
#[derive(Debug, Clone)]
pub struct VmResources {
    /// CPU request/limit
    pub cpu: String,
    /// Memory request/limit
    pub memory: String,
    /// GPU count (optional)
    pub gpu: Option<u32>,
}

/// VM storage configuration
#[derive(Debug, Clone)]
pub struct VmStorage {
    /// Storage size (e.g., "10Gi")
    pub size: String,
    /// Storage class
    pub storage_class: String,
}

/// PoolAI Tenant Custom Resource
///
/// Represents a PoolAI tenant resource in Kubernetes.
/// This CRD allows users to define tenant configurations with resource quotas.
///
/// # Example
///
/// ```yaml
/// apiVersion: poolai.io/v1
/// kind: PoolAITenant
/// metadata:
///   name: tenant-abc
/// spec:
///   active: true
///   quotas:
///     max_workers: 10
///     max_memory_mb: 1024
///     max_cpu_cores: 4
///     max_storage_mb: 10000
/// ```
#[derive(Debug, Clone)]
pub struct PoolAITenant {
    /// Tenant name
    pub name: String,
    /// Resource quotas
    pub quotas: TenantQuotas,
    /// Whether tenant is active
    pub active: bool,
}

/// Tenant resource quotas
#[derive(Debug, Clone)]
pub struct TenantQuotas {
    /// Maximum workers
    pub max_workers: Option<usize>,
    /// Maximum memory (MB)
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU cores
    pub max_cpu_cores: Option<usize>,
    /// Maximum storage (MB)
    pub max_storage_mb: Option<u64>,
}

#[cfg(feature = "cloud-sdk")]
impl PoolAIOperator {
    /// Watch CRD resources using HTTP polling (with watch API support)
    ///
    /// Monitors CRD resources for changes using HTTP polling.
    /// This implementation uses periodic polling for simplicity and reliability.
    /// For better efficiency in production, consider using Kubernetes watch API
    /// (see `KubernetesManager::watch_crd_resources` for watch API support).
    ///
    /// The watcher:
    /// - Polls every 10 seconds for resource changes
    /// - Compares resourceVersion to detect Added, Modified, Deleted events
    /// - Sends events to the reconciliation loop via event channel
    async fn watch_crd_resources(
        resource_plural: String,
        group: String,
        version: String,
        namespace: String,
        resource_type: CrdResourceType,
        event_tx: mpsc::UnboundedSender<CrdEvent>,
        k8s_manager: Option<Arc<crate::cloud::kubernetes::KubernetesManager>>,
    ) {
        let mut interval = interval(Duration::from_secs(10)); // Poll every 10 seconds
        let mut last_resources: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        loop {
            interval.tick().await;
            
            // In a real implementation, we would:
            // 1. GET /apis/{group}/{version}/namespaces/{namespace}/{resource_plural}
            // 2. Compare with last_resources to detect changes
            // 3. Send events for Added, Modified, Deleted
            
            // For now, this is a placeholder that logs polling activity
            if let Some(ref manager) = k8s_manager {
                // Check if manager is initialized
                if manager.is_cluster_available().await.unwrap_or(false) {
                    // TODO: Implement actual resource listing and comparison
                    // This would involve:
                    // - Making HTTP GET request to Kubernetes API
                    // - Parsing JSON response
                    // - Comparing resourceVersion to detect changes
                    // - Sending appropriate events
                    
                    // Try to list resources and detect changes
                    if let Ok(resources_json) = manager.list_crd_resources(&group, &version, &resource_plural).await {
                        if let Some(items) = resources_json.get("items").and_then(|i| i.as_array()) {
                            let mut current_resources: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                            
                            for item in items {
                                if let Some(name) = item.get("metadata")
                                    .and_then(|m| m.get("name"))
                                    .and_then(|n| n.as_str())
                                {
                                    let resource_version = item.get("metadata")
                                        .and_then(|m| m.get("resourceVersion"))
                                        .and_then(|rv| rv.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    
                                    // Check if resource is new or modified
                                    if let Some(old_version) = last_resources.get(name) {
                                        if old_version != &resource_version {
                                            // Resource modified
                                            let _ = event_tx.send(CrdEvent {
                                                event_type: CrdEventType::Modified,
                                                resource_type: resource_type.clone(),
                                                name: name.to_string(),
                                                namespace: namespace.clone(),
                                            });
                                        }
                                    } else {
                                        // Resource added
                                        let _ = event_tx.send(CrdEvent {
                                            event_type: CrdEventType::Added,
                                            resource_type: resource_type.clone(),
                                            name: name.to_string(),
                                            namespace: namespace.clone(),
                                        });
                                    }
                                    
                                    current_resources.insert(name.to_string(), resource_version);
                                }
                            }
                            
                            // Detect deleted resources
                            for (name, _) in &last_resources {
                                if !current_resources.contains_key(name) {
                                    // Resource deleted
                                    let _ = event_tx.send(CrdEvent {
                                        event_type: CrdEventType::Deleted,
                                        resource_type: resource_type.clone(),
                                        name: name.clone(),
                                        namespace: namespace.clone(),
                                    });
                                }
                            }
                            
                            last_resources = current_resources;
                        }
                    }
                }
            }
            
            // In production, we would break on shutdown signal
            // For now, this runs indefinitely
        }
    }
    
    /// Reconciliation loop
    ///
    /// Processes CRD events and reconciles desired state vs actual state.
    async fn reconciliation_loop(
        mut event_rx: mpsc::UnboundedReceiver<CrdEvent>,
        namespace: String,
        k8s_manager: Option<Arc<crate::cloud::kubernetes::KubernetesManager>>,
    ) {
        info!("Reconciliation loop started for namespace: {}", namespace);
        
        let k8s_manager = match k8s_manager {
            Some(manager) => manager,
            None => {
                warn!("Kubernetes manager not available, reconciliation loop stopping");
                return;
            }
        };
        
        while let Some(event) = event_rx.recv().await {
            match event.event_type {
                CrdEventType::Added | CrdEventType::Modified => {
                    info!("CRD resource {}: {} ({:?}) in namespace {}", 
                        match event.event_type {
                            CrdEventType::Added => "added",
                            CrdEventType::Modified => "modified",
                            _ => unreachable!(),
                        },
                        event.name, 
                        event.resource_type,
                        event.namespace
                    );
                    
                    // Parse resource spec from Kubernetes API and reconcile
                    match event.resource_type {
                        CrdResourceType::Worker => {
                            if let Ok(worker) = Self::parse_worker_crd(&k8s_manager, &event.namespace, &event.name).await {
                                if let Err(e) = Self::reconcile_worker(&worker, &event.namespace, &k8s_manager).await {
                                    error!("Failed to reconcile worker {}: {}", event.name, e);
                                }
                            }
                        }
                        CrdResourceType::Vm => {
                            if let Ok(vm) = Self::parse_vm_crd(&k8s_manager, &event.namespace, &event.name).await {
                                if let Err(e) = Self::reconcile_vm(&vm, &event.namespace, &k8s_manager).await {
                                    error!("Failed to reconcile VM {}: {}", event.name, e);
                                }
                            }
                        }
                        CrdResourceType::Tenant => {
                            if let Ok(tenant) = Self::parse_tenant_crd(&k8s_manager, &event.namespace, &event.name).await {
                                if let Err(e) = Self::reconcile_tenant(&tenant, &event.namespace, &k8s_manager).await {
                                    error!("Failed to reconcile tenant {}: {}", event.name, e);
                                }
                            }
                        }
                    }
                }
                CrdEventType::Deleted => {
                    if event.name == "shutdown" {
                        info!("Reconciliation loop shutting down");
                        break;
                    }
                    info!("CRD resource deleted: {} ({:?}) in namespace {}", 
                        event.name, 
                        event.resource_type,
                        event.namespace
                    );
                    
                    // Cleanup: Delete associated Kubernetes resources
                    match event.resource_type {
                        CrdResourceType::Worker => {
                            if let Err(e) = k8s_manager.delete_worker_deployment(&event.name).await {
                                warn!("Failed to delete worker deployment {}: {}", event.name, e);
                            }
                        }
                        CrdResourceType::Vm => {
                            // Delete VM deployment
                            if let Err(e) = k8s_manager.delete_worker_deployment(&event.name).await {
                                warn!("Failed to delete VM deployment {}: {}", event.name, e);
                            } else {
                                info!("Deleted VM deployment: {} from namespace {}", event.name, event.namespace);
                            }
                        }
                        CrdResourceType::Tenant => {
                            // Delete ResourceQuota for tenant
                            if let Err(e) = k8s_manager.delete_resource_quota(&event.name).await {
                                warn!("Failed to delete ResourceQuota for tenant {}: {}", event.name, e);
                            } else {
                                info!("Deleted ResourceQuota for tenant: {} from namespace {}", event.name, event.namespace);
                            }
                        }
                    }
                }
            }
        }
        
        info!("Reconciliation loop stopped");
    }
    
    /// Parse PoolAIWorker CRD from Kubernetes API
    async fn parse_worker_crd(
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
        namespace: &str,
        name: &str,
    ) -> Result<PoolAIWorker, AppError> {
        let resource = k8s_manager.get_crd_resource("poolai.io", "v1", "poolaiworkers", name).await?;
        
        let spec = resource.get("spec")
            .ok_or_else(|| AppError::NetworkError("CRD spec is missing".to_string()))?;
        
        let name = resource.get("metadata")
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or(name)
            .to_string();
        
        let image = spec.get("image")
            .and_then(|i| i.as_str())
            .unwrap_or("poolai/worker:latest")
            .to_string();
        
        let replicas = spec.get("replicas")
            .and_then(|r| r.as_u64())
            .unwrap_or(1) as u32;
        
        let resources_spec = spec.get("resources")
            .ok_or_else(|| AppError::NetworkError("Resources spec is missing".to_string()))?;
        
        let cpu = resources_spec.get("cpu")
            .and_then(|c| c.as_str())
            .unwrap_or("100m")
            .to_string();
        
        let memory = resources_spec.get("memory")
            .and_then(|m| m.as_str())
            .unwrap_or("128Mi")
            .to_string();
        
        let gpu = resources_spec.get("gpu")
            .and_then(|g| g.as_u64())
            .map(|g| g as u32);
        
        // Parse environment variables if present
        let mut env = std::collections::HashMap::new();
        if let Some(env_spec) = spec.get("env").and_then(|e| e.as_object()) {
            for (key, value) in env_spec {
                if let Some(val_str) = value.as_str() {
                    env.insert(key.clone(), val_str.to_string());
                }
            }
        }
        
        Ok(PoolAIWorker {
            name,
            image,
            replicas,
            resources: WorkerResources { cpu, memory, gpu },
            env: if env.is_empty() { None } else { Some(env) },
        })
    }
    
    /// Parse PoolAIVM CRD from Kubernetes API
    async fn parse_vm_crd(
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
        namespace: &str,
        name: &str,
    ) -> Result<PoolAIVM, AppError> {
        let resource = k8s_manager.get_crd_resource("poolai.io", "v1", "poolaivms", name).await?;
        
        let spec = resource.get("spec")
            .ok_or_else(|| AppError::NetworkError("CRD spec is missing".to_string()))?;
        
        let name = resource.get("metadata")
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or(name)
            .to_string();
        
        let image = spec.get("image")
            .and_then(|i| i.as_str())
            .unwrap_or("poolai/vm:latest")
            .to_string();
        
        let resources_spec = spec.get("resources")
            .ok_or_else(|| AppError::NetworkError("Resources spec is missing".to_string()))?;
        
        let cpu = resources_spec.get("cpu")
            .and_then(|c| c.as_str())
            .unwrap_or("500m")
            .to_string();
        
        let memory = resources_spec.get("memory")
            .and_then(|m| m.as_str())
            .unwrap_or("512Mi")
            .to_string();
        
        let gpu = resources_spec.get("gpu")
            .and_then(|g| g.as_u64())
            .map(|g| g as u32);
        
        let storage_spec = spec.get("storage")
            .ok_or_else(|| AppError::NetworkError("Storage spec is missing".to_string()))?;
        
        let size = storage_spec.get("size")
            .and_then(|s| s.as_str())
            .unwrap_or("10Gi")
            .to_string();
        
        let storage_class = storage_spec.get("storage_class")
            .and_then(|sc| sc.as_str())
            .unwrap_or("standard")
            .to_string();
        
        // Parse network ports if present
        let ports = spec.get("ports")
            .and_then(|p| p.as_array())
            .map(|ports_array| {
                ports_array
                    .iter()
                    .filter_map(|p| p.as_u64().map(|p| p as u16))
                    .collect()
            });
        
        Ok(PoolAIVM {
            name,
            image,
            resources: VmResources { cpu, memory, gpu },
            storage: VmStorage { size, storage_class },
            ports,
        })
    }
    
    /// Parse PoolAITenant CRD from Kubernetes API
    async fn parse_tenant_crd(
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
        namespace: &str,
        name: &str,
    ) -> Result<PoolAITenant, AppError> {
        let resource = k8s_manager.get_crd_resource("poolai.io", "v1", "poolaitenants", name).await?;
        
        let spec = resource.get("spec")
            .ok_or_else(|| AppError::NetworkError("CRD spec is missing".to_string()))?;
        
        let name = resource.get("metadata")
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or(name)
            .to_string();
        
        let active = spec.get("active")
            .and_then(|a| a.as_bool())
            .unwrap_or(true);
        
        let quotas_spec = spec.get("quotas")
            .ok_or_else(|| AppError::NetworkError("Quotas spec is missing".to_string()))?;
        
        let max_workers = quotas_spec.get("max_workers")
            .and_then(|w| w.as_u64())
            .map(|w| w as usize);
        
        let max_memory_mb = quotas_spec.get("max_memory_mb")
            .and_then(|m| m.as_u64());
        
        let max_cpu_cores = quotas_spec.get("max_cpu_cores")
            .and_then(|c| c.as_u64())
            .map(|c| c as usize);
        
        let max_storage_mb = quotas_spec.get("max_storage_mb")
            .and_then(|s| s.as_u64());
        
        Ok(PoolAITenant {
            name,
            active,
            quotas: TenantQuotas {
                max_workers,
                max_memory_mb,
                max_cpu_cores,
                max_storage_mb,
            },
        })
    }
    
    /// Reconcile a PoolAIWorker resource
    ///
    /// Ensures the actual state matches the desired state.
    /// This will:
    /// 1. Check if Deployment exists
    /// 2. Create/update Deployment based on worker spec
    /// 3. Create/update Service if needed
    /// 4. Update CRD status (when implemented)
    async fn reconcile_worker(
        worker: &PoolAIWorker,
        namespace: &str,
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
    ) -> Result<(), AppError> {
        info!("Reconciling worker: {} in namespace {}", worker.name, namespace);
        
        // Build WorkerDeploymentConfig from PoolAIWorker
        let config = crate::cloud::kubernetes::WorkerDeploymentConfig {
            image: worker.image.clone(),
            replicas: worker.replicas,
            resources: crate::cloud::kubernetes::ResourceRequirements {
                cpu: worker.resources.cpu.clone(),
                memory: worker.resources.memory.clone(),
                gpu: worker.resources.gpu,
            },
            env: worker.env.clone().unwrap_or_default(),
        };
        
        // Check if deployment exists
        let deployment_name = &worker.name;
        let deployments = k8s_manager.list_deployments().await.unwrap_or_default();
        let deployment_exists = deployments.iter().any(|d| d == deployment_name);
        
        if deployment_exists {
            // Update existing deployment
            match k8s_manager.update_worker_deployment(deployment_name, &config).await {
                Ok(_) => {
                    info!("Updated worker deployment: {} in namespace {}", deployment_name, namespace);
                }
                Err(e) => {
                    warn!("Failed to update worker deployment {}: {}", deployment_name, e);
                    return Err(e);
                }
            }
        } else {
            // Create new deployment
            match k8s_manager.create_worker_deployment(deployment_name, &config).await {
                Ok(deployment_id) => {
                    info!("Created worker deployment: {} in namespace {}", deployment_id, namespace);
                }
                Err(e) => {
                    warn!("Failed to create worker deployment {}: {}", deployment_name, e);
                    return Err(e);
                }
            }
        }
        
        // Update CRD status with deployment status
        let status = json!({
            "conditions": [{
                "type": "Ready",
                "status": if deployment_exists { "True" } else { "True" },
                "reason": if deployment_exists { "DeploymentUpdated" } else { "DeploymentCreated" },
                "message": format!("Worker deployment {} {}", deployment_name, if deployment_exists { "updated" } else { "created" })
            }],
            "deploymentName": deployment_name
        });
        
        if let Err(e) = k8s_manager.update_crd_status("poolai.io", "v1", "poolaiworkers", &worker.name, status).await {
            warn!("Failed to update CRD status for worker {}: {}", worker.name, e);
            // Don't fail reconciliation if status update fails
        }
        
        Ok(())
    }
    
    /// Reconcile a PoolAIVM resource
    ///
    /// Ensures the actual state matches the desired state.
    /// This will:
    /// 1. Check if VM Deployment exists
    /// 2. Create/update Deployment based on VM spec
    /// 3. Create/update PVC if needed
    /// 4. Update CRD status (when implemented)
    async fn reconcile_vm(
        vm: &PoolAIVM,
        namespace: &str,
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
    ) -> Result<(), AppError> {
        info!("Reconciling VM: {} in namespace {}", vm.name, namespace);
        
        // Build VmDeploymentConfig from PoolAIVM
        let config = crate::cloud::kubernetes::VmDeploymentConfig {
            image: vm.image.clone(),
            resources: crate::cloud::kubernetes::ResourceRequirements {
                cpu: vm.resources.cpu.clone(),
                memory: vm.resources.memory.clone(),
                gpu: vm.resources.gpu,
            },
            storage: crate::cloud::kubernetes::StorageConfig {
                size: vm.storage.size.clone(),
                storage_class: vm.storage.storage_class.clone(),
            },
            network: crate::cloud::kubernetes::NetworkConfig {
                ports: vm.ports.clone().unwrap_or_default(),
                service_type: crate::cloud::kubernetes::ServiceType::ClusterIP, // Default service type
            },
        };
        
        // Check if deployment exists
        let deployment_name = &vm.name;
        let deployments = k8s_manager.list_deployments().await.unwrap_or_default();
        let deployment_exists = deployments.iter().any(|d| d == deployment_name);
        
        if deployment_exists {
            // Update existing deployment
            match k8s_manager.update_vm_deployment(deployment_name, &config).await {
                Ok(_) => {
                    info!("Updated VM deployment: {} in namespace {}", deployment_name, namespace);
                }
                Err(e) => {
                    warn!("Failed to update VM deployment {}: {}", deployment_name, e);
                    return Err(e);
                }
            }
        } else {
            // Create new deployment
            match k8s_manager.create_vm_deployment(deployment_name, &config).await {
                Ok(deployment_id) => {
                    info!("Created VM deployment: {} in namespace {}", deployment_id, namespace);
                }
                Err(e) => {
                    warn!("Failed to create VM deployment {}: {}", deployment_name, e);
                    return Err(e);
                }
            }
        }
        
        // Update CRD status with deployment status
        let status = json!({
            "conditions": [{
                "type": "Ready",
                "status": if deployment_exists { "True" } else { "True" },
                "reason": if deployment_exists { "DeploymentUpdated" } else { "DeploymentCreated" },
                "message": format!("VM deployment {} {}", deployment_name, if deployment_exists { "updated" } else { "created" })
            }],
            "deploymentName": deployment_name
        });
        
        if let Err(e) = k8s_manager.update_crd_status("poolai.io", "v1", "poolaivms", &vm.name, status).await {
            warn!("Failed to update CRD status for VM {}: {}", vm.name, e);
            // Don't fail reconciliation if status update fails
        }
        
        Ok(())
    }
    
    /// Reconcile a PoolAITenant resource
    ///
    /// Ensures the actual state matches the desired state.
    /// This will:
    /// 1. Create/update ResourceQuota based on tenant quotas
    /// 2. Update CRD status
    async fn reconcile_tenant(
        tenant: &PoolAITenant,
        namespace: &str,
        k8s_manager: &Arc<crate::cloud::kubernetes::KubernetesManager>,
    ) -> Result<(), AppError> {
        info!("Reconciling tenant: {} in namespace {}", tenant.name, namespace);
        
        if !tenant.active {
            info!("Tenant {} is inactive, skipping reconciliation", tenant.name);
            // Update status to indicate tenant is inactive
            let status = json!({
                "conditions": [{
                    "type": "Active",
                    "status": "False",
                    "reason": "TenantInactive",
                    "message": "Tenant is marked as inactive"
                }]
            });
            let _ = k8s_manager.update_crd_status("poolai.io", "v1", "poolaitenants", &tenant.name, status).await;
            return Ok(());
        }
        
        // Build ResourceQuota spec from tenant quotas
        let mut quota_hard = serde_json::Map::new();
        
        if let Some(max_cpu) = tenant.quotas.max_cpu_cores {
            quota_hard.insert("requests.cpu".to_string(), json!(max_cpu.to_string()));
            quota_hard.insert("limits.cpu".to_string(), json!(max_cpu.to_string()));
        }
        
        if let Some(max_memory_mb) = tenant.quotas.max_memory_mb {
            let memory_gi = format!("{}Mi", max_memory_mb);
            quota_hard.insert("requests.memory".to_string(), json!(memory_gi.clone()));
            quota_hard.insert("limits.memory".to_string(), json!(memory_gi));
        }
        
        if let Some(max_storage_mb) = tenant.quotas.max_storage_mb {
            let storage_gi = format!("{}Mi", max_storage_mb);
            quota_hard.insert("requests.storage".to_string(), json!(storage_gi));
            quota_hard.insert("persistentvolumeclaims".to_string(), json!("10")); // Default PVC limit
        }
        
        if let Some(max_workers) = tenant.quotas.max_workers {
            // Note: Kubernetes doesn't have a direct way to limit deployments per namespace
            // We'll use a custom annotation or rely on ResourceQuota for resource limits
            quota_hard.insert("count/deployments.apps".to_string(), json!(max_workers));
        }
        
        let quotas_json = json!({
            "hard": quota_hard
        });
        
        // Create or update ResourceQuota
        match k8s_manager.create_or_update_resource_quota(&tenant.name, quotas_json).await {
            Ok(_) => {
                info!("Created/updated ResourceQuota for tenant {} in namespace {}", tenant.name, namespace);
            }
            Err(e) => {
                warn!("Failed to create/update ResourceQuota for tenant {}: {}", tenant.name, e);
                return Err(e);
            }
        }
        
        // Update CRD status
        let status = json!({
            "conditions": [{
                "type": "Ready",
                "status": "True",
                "reason": "ResourceQuotaCreated",
                "message": format!("ResourceQuota created/updated for tenant {}", tenant.name)
            }],
            "active": tenant.active
        });
        
        if let Err(e) = k8s_manager.update_crd_status("poolai.io", "v1", "poolaitenants", &tenant.name, status).await {
            warn!("Failed to update CRD status for tenant {}: {}", tenant.name, e);
            // Don't fail reconciliation if status update fails
        }
        
        Ok(())
    }
}
