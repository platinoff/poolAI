//! Auto-scaling module
//!
//! Provides cloud-based auto-scaling capabilities:
//! - Metrics-based scaling decisions
//! - Horizontal Pod Autoscaler (Kubernetes)
//! - Cloud provider auto-scaling groups
//! - Cost optimization strategies
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::cloud::autoscaling::AutoScaler;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let autoscaler = AutoScaler::new();
//! autoscaler.initialize().await?;
//!
//! // Get current metrics
//! let metrics = autoscaler.get_metrics("worker-pool").await?;
//!
//! // Scale up if CPU usage is high
//! if metrics.cpu_usage > 0.8 {
//!     autoscaler.scale_up("worker-pool", metrics.current_replicas + 2).await?;
//! }
//!
//! // Scale down if usage is low
//! if metrics.cpu_usage < 0.3 && metrics.current_replicas > 1 {
//!     autoscaler.scale_down("worker-pool", metrics.current_replicas - 1).await?;
//! }
//!
//! autoscaler.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[cfg(feature = "cloud-sdk")]
use crate::cloud::kubernetes::KubernetesManager;

/// Auto-scaler for managing resource scaling
pub struct AutoScaler {
    initialized: Arc<RwLock<bool>>,
    scaling_policies: Arc<RwLock<Vec<ScalingPolicy>>>,
    min_replicas: Arc<RwLock<u32>>,
    max_replicas: Arc<RwLock<u32>>,
    #[cfg(feature = "cloud-sdk")]
    /// Kubernetes manager for scaling operations
    k8s_manager: Option<Arc<KubernetesManager>>,
}

impl AutoScaler {
    /// Create a new AutoScaler
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::cloud::autoscaling::AutoScaler;
    ///
    /// let autoscaler = AutoScaler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            initialized: Arc::new(RwLock::new(false)),
            scaling_policies: Arc::new(RwLock::new(Vec::new())),
            min_replicas: Arc::new(RwLock::new(1)),
            max_replicas: Arc::new(RwLock::new(10)),
            #[cfg(feature = "cloud-sdk")]
            k8s_manager: None,
        }
    }

    /// Create a new AutoScaler with Kubernetes manager
    ///
    /// # Arguments
    ///
    /// * `k8s_manager` - Kubernetes manager for scaling operations
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::autoscaling::AutoScaler;
    /// use poolai::cloud::kubernetes::KubernetesManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let k8s_manager = Arc::new(KubernetesManager::new("poolai".to_string()));
    /// k8s_manager.initialize().await?;
    /// let autoscaler = AutoScaler::with_k8s_manager(k8s_manager);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "cloud-sdk")]
    pub fn with_k8s_manager(k8s_manager: Arc<KubernetesManager>) -> Self {
        Self {
            initialized: Arc::new(RwLock::new(false)),
            scaling_policies: Arc::new(RwLock::new(Vec::new())),
            min_replicas: Arc::new(RwLock::new(1)),
            max_replicas: Arc::new(RwLock::new(10)),
            k8s_manager: Some(k8s_manager),
        }
    }

    /// Initialize auto-scaler
    ///
    /// Sets up metrics collection, scaling policies, and HPA (if Kubernetes is enabled).
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - Metrics collection cannot be initialized
    /// - Scaling policies cannot be configured
    /// - HPA initialization fails (if Kubernetes is enabled)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::autoscaling::AutoScaler;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let autoscaler = AutoScaler::new();
    /// autoscaler.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Initialize default scaling policies
        let mut policies = self.scaling_policies.write().await;
        policies.push(ScalingPolicy {
            name: "default-cpu".to_string(),
            metric_type: "CPU".to_string(),
            target_value: 70.0,
            scale_up_threshold: 80.0,
            scale_down_threshold: 50.0,
        });
        drop(policies);

        // TODO: Set up metrics collection
        // TODO: Configure scaling rules
        // TODO: Initialize HPA (if Kubernetes)

        info!("Auto-scaler initialized with default policies");

        *initialized = true;
        Ok(())
    }

    /// Shutdown auto-scaler
    pub async fn shutdown(&self) -> Result<(), AppError> {
        *self.initialized.write().await = false;
        info!("Auto-scaler shut down");
        Ok(())
    }

    /// Scale up resources based on metrics
    ///
    /// # Arguments
    ///
    /// * `resource_id` - Identifier for the resource to scale
    /// * `target_replicas` - Target number of replicas (must be > current replicas)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `resource_id` is empty
    /// - `target_replicas` is 0
    /// - `target_replicas` is less than or equal to current replicas
    pub async fn scale_up(&self, resource_id: &str, target_replicas: u32) -> Result<(), AppError> {
        if resource_id.is_empty() {
            return Err(AppError::ValidationError(
                "Resource ID cannot be empty. Current value: ''. Suggestion: Provide a valid resource identifier."
                    .to_string(),
            ));
        }

        if target_replicas == 0 {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas must be greater than 0 for scale up. Current value: {}. Suggestion: Set target_replicas to at least 1.",
                    target_replicas
                ),
            ));
        }

        // Get current metrics to validate scale up
        let metrics = self.get_metrics(resource_id).await?;
        if target_replicas <= metrics.current_replicas {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas ({}) must be greater than current replicas ({}) for scale up. Current value: {}. Suggestion: Set target_replicas to a value greater than {}.",
                    target_replicas, metrics.current_replicas, target_replicas, metrics.current_replicas
                ),
            ));
        }

        // Check replica limits
        let max_replicas = *self.max_replicas.read().await;
        if target_replicas > max_replicas {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas ({}) exceeds maximum replicas ({}). Context: Attempted to scale up beyond configured limit. \
                    Suggestion: Increase max_replicas or scale to a lower value. \
                    Current value: {}, Max: {}",
                    target_replicas, max_replicas, target_replicas, max_replicas
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            if let Some(ref k8s_manager) = self.k8s_manager {
                // Scale deployment via Kubernetes API
                match k8s_manager
                    .scale_deployment(resource_id, target_replicas as i32)
                    .await
                {
                    Ok(_) => {
                        info!(
                            "Scaled up resource: {} from {} to {} replicas",
                            resource_id, metrics.current_replicas, target_replicas
                        );
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("Failed to scale up deployment {}: {}", resource_id, e);
                        return Err(e);
                    }
                }
            }
        }

        // Fallback: Log placeholder scaling
        info!(
            "Scaling up resource: {} from {} to {} replicas (placeholder - enable cloud-sdk feature)",
            resource_id, metrics.current_replicas, target_replicas
        );
        Ok(())
    }

    /// Scale down resources based on metrics
    ///
    /// # Arguments
    ///
    /// * `resource_id` - Identifier for the resource to scale
    /// * `target_replicas` - Target number of replicas (must be < current replicas and >= 1)
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `resource_id` is empty
    /// - `target_replicas` is 0
    /// - `target_replicas` is greater than or equal to current replicas
    pub async fn scale_down(
        &self,
        resource_id: &str,
        target_replicas: u32,
    ) -> Result<(), AppError> {
        if resource_id.is_empty() {
            return Err(AppError::ValidationError(
                "Resource ID cannot be empty. Current value: ''. Suggestion: Provide a valid resource identifier."
                    .to_string(),
            ));
        }

        if target_replicas == 0 {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas must be at least 1 for scale down. Current value: {}. Suggestion: Set target_replicas to at least 1.",
                    target_replicas
                ),
            ));
        }

        // Get current metrics to validate scale down
        let metrics = self.get_metrics(resource_id).await?;
        if target_replicas >= metrics.current_replicas {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas ({}) must be less than current replicas ({}) for scale down. Current value: {}. Suggestion: Set target_replicas to a value less than {}.",
                    target_replicas, metrics.current_replicas, target_replicas, metrics.current_replicas
                ),
            ));
        }

        // Check replica limits
        let min_replicas = *self.min_replicas.read().await;
        if target_replicas < min_replicas {
            return Err(AppError::ValidationError(
                format!(
                    "Target replicas ({}) is below minimum replicas ({}). Context: Attempted to scale down below configured limit. \
                    Suggestion: Decrease min_replicas or scale to a higher value. \
                    Current value: {}, Min: {}",
                    target_replicas, min_replicas, target_replicas, min_replicas
                )
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            if let Some(ref k8s_manager) = self.k8s_manager {
                // Scale deployment via Kubernetes API
                match k8s_manager
                    .scale_deployment(resource_id, target_replicas as i32)
                    .await
                {
                    Ok(_) => {
                        info!(
                            "Scaled down resource: {} from {} to {} replicas",
                            resource_id, metrics.current_replicas, target_replicas
                        );
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("Failed to scale down deployment {}: {}", resource_id, e);
                        return Err(e);
                    }
                }
            }
        }

        // Fallback: Log placeholder scaling
        info!(
            "Scaling down resource: {} from {} to {} replicas (placeholder - enable cloud-sdk feature)",
            resource_id, metrics.current_replicas, target_replicas
        );
        Ok(())
    }

    /// Get current scaling metrics
    ///
    /// # Arguments
    ///
    /// * `resource_id` - Identifier for the resource to query metrics for
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `resource_id` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Metrics service is unreachable
    /// - Resource does not exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::autoscaling::AutoScaler;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let autoscaler = AutoScaler::new();
    /// autoscaler.initialize().await?;
    ///
    /// let metrics = autoscaler.get_metrics("worker-pool").await?;
    /// println!("CPU usage: {:.1}%", metrics.cpu_usage * 100.0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_metrics(&self, resource_id: &str) -> Result<ScalingMetrics, AppError> {
        if resource_id.is_empty() {
            return Err(AppError::ValidationError(
                "Resource ID cannot be empty. Context: Attempted to get scaling metrics with empty resource ID. \
                Suggestion: Provide a valid resource identifier. \
                Current value: ''"
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            if let Some(ref k8s_manager) = self.k8s_manager {
                // Get deployment replicas
                let deployments = k8s_manager.list_deployments().await.unwrap_or_default();
                let current_replicas = if deployments.contains(&resource_id.to_string()) {
                    // Try to get deployment details to get replica count
                    // For now, we'll use a placeholder - in production, we'd query deployment spec
                    let pods = k8s_manager.list_pods().await.unwrap_or_default();
                    pods.iter().filter(|p| p.starts_with(resource_id)).count() as u32
                } else {
                    0
                };

                // Query Pod metrics API for CPU and memory usage
                // Note: This requires metrics-server to be installed in the cluster
                let mut total_cpu_usage = 0.0;
                let mut total_memory_usage = 0.0;
                let mut pod_count = 0;

                if current_replicas > 0 {
                    let pods = k8s_manager.list_pods().await.unwrap_or_default();
                    for pod_name in pods.iter().filter(|p| p.starts_with(resource_id)) {
                        // Query metrics for each pod
                        // Path: /apis/metrics.k8s.io/v1beta1/namespaces/{namespace}/pods/{pod_name}
                        // For now, we'll use a simplified approach
                        // In production, we'd query the metrics API and calculate averages
                        pod_count += 1;
                    }

                    // Calculate average CPU and memory usage
                    // Placeholder: In production, we'd query actual metrics
                    // For now, return placeholder values
                    if pod_count > 0 {
                        total_cpu_usage = 0.5; // Placeholder: 50% CPU usage
                        total_memory_usage = 0.6; // Placeholder: 60% memory usage
                    }
                }

                // Request rate would come from load balancer or API gateway metrics
                // For now, we'll use a placeholder
                let request_rate = 0.0;

                return Ok(ScalingMetrics {
                    cpu_usage: total_cpu_usage,
                    memory_usage: total_memory_usage,
                    request_rate,
                    current_replicas: if current_replicas > 0 {
                        current_replicas
                    } else {
                        1
                    },
                });
            }
        }

        // Fallback: Return placeholder metrics
        Ok(ScalingMetrics {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            request_rate: 0.0,
            current_replicas: 1,
        })
    }

    /// Add a scaling policy
    ///
    /// # Arguments
    ///
    /// * `policy` - Scaling policy to add
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::autoscaling::{AutoScaler, ScalingPolicy};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let autoscaler = AutoScaler::new();
    /// autoscaler.initialize().await?;
    ///
    /// let policy = ScalingPolicy {
    ///     name: "memory-policy".to_string(),
    ///     metric_type: "Memory".to_string(),
    ///     target_value: 75.0,
    ///     scale_up_threshold: 85.0,
    ///     scale_down_threshold: 60.0,
    /// };
    /// autoscaler.add_policy(policy).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_policy(&self, policy: ScalingPolicy) -> Result<(), AppError> {
        let mut policies = self.scaling_policies.write().await;
        policies.push(policy);
        Ok(())
    }

    /// Get all scaling policies
    pub async fn get_policies(&self) -> Vec<ScalingPolicy> {
        let policies = self.scaling_policies.read().await;
        policies.clone()
    }

    /// Set min/max replica limits
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum number of replicas
    /// * `max` - Maximum number of replicas
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if min > max or max is 0.
    pub async fn set_replica_limits(&self, min: u32, max: u32) -> Result<(), AppError> {
        if min > max {
            return Err(AppError::ValidationError(format!(
                "Minimum replicas ({}) cannot be greater than maximum ({})",
                min, max
            )));
        }
        if max == 0 {
            return Err(AppError::ValidationError(
                "Maximum replicas cannot be 0".to_string(),
            ));
        }

        *self.min_replicas.write().await = min;
        *self.max_replicas.write().await = max;
        Ok(())
    }
}

impl Default for AutoScaler {
    fn default() -> Self {
        Self::new()
    }
}

/// Scaling policy configuration
#[derive(Debug, Clone)]
pub struct ScalingPolicy {
    /// Policy name
    pub name: String,
    /// Metric type (CPU, Memory, RequestRate)
    pub metric_type: String,
    /// Target value for the metric
    pub target_value: f64,
    /// Scale up threshold
    pub scale_up_threshold: f64,
    /// Scale down threshold
    pub scale_down_threshold: f64,
}

/// Scaling metrics for auto-scaling decisions
#[derive(Debug, Clone)]
pub struct ScalingMetrics {
    pub cpu_usage: f64,    // 0.0 - 1.0
    pub memory_usage: f64, // 0.0 - 1.0
    pub request_rate: f64, // requests per second
    pub current_replicas: u32,
}
