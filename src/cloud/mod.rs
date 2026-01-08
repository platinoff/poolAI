//! Cloud Integration module
//!
//! Provides cloud infrastructure integration for PoolAI, including:
//! - Kubernetes orchestration support
//! - Multi-cloud provider integration (AWS, Azure, GCP)
//! - Cloud-based auto-scaling
//! - Advanced load balancing
//!
//! # Features
//!
//! - **Kubernetes Support**: Operator, CRDs, Helm charts, service discovery
//! - **Cloud Providers**: AWS, Azure, GCP integration
//! - **Auto-scaling**: Metrics-based horizontal scaling
//! - **Load Balancing**: Cloud load balancers with health checks
//!
//! # Example
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use poolai::cloud::{CloudManager, CloudConfig};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Create cloud manager with Kubernetes enabled
//! let config = CloudConfig {
//!     kubernetes_enabled: true,
//!     kubernetes_namespace: "poolai".to_string(),
//!     ..Default::default()
//! };
//! 
//! let manager = CloudManager::new(config);
//! manager.initialize().await?;
//! 
//! // Use Kubernetes manager
//! if let Some(k8s) = manager.kubernetes() {
//!     let deployment = k8s.create_worker_deployment(
//!         "my-worker",
//!         &poolai::cloud::kubernetes::WorkerDeploymentConfig::default()
//!     ).await?;
//!     println!("Created deployment: {}", deployment);
//! }
//! 
//! manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Multi-Cloud Configuration
//!
//! ```rust,no_run
//! use poolai::cloud::{CloudManager, CloudConfig};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Enable multiple cloud providers
//! let config = CloudConfig {
//!     kubernetes_enabled: true,
//!     aws_enabled: true,
//!     aws_region: Some("us-east-1".to_string()),
//!     azure_enabled: true,
//!     gcp_enabled: true,
//!     autoscaling_enabled: true,
//!     loadbalancing_enabled: true,
//!     ..Default::default()
//! };
//! 
//! let manager = CloudManager::new(config);
//! manager.initialize().await?;
//! 
//! // Use auto-scaler
//! if let Some(autoscaler) = manager.autoscaler() {
//!     autoscaler.scale_up("worker-pool", 5).await?;
//! }
//! 
//! # Ok(())
//! # }
//! ```

pub mod kubernetes;
pub mod providers;
pub mod autoscaling;
pub mod loadbalancing;

use crate::core::error::AppError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Cloud integration configuration
///
/// Configures which cloud features are enabled and their settings.
/// All features are optional and can be enabled independently.
///
/// # Example
///
/// ```rust
/// use poolai::cloud::CloudConfig;
///
/// let config = CloudConfig {
///     kubernetes_enabled: true,
///     kubernetes_namespace: "poolai".to_string(),
///     aws_enabled: true,
///     aws_region: Some("us-east-1".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CloudConfig {
    /// Enable Kubernetes integration
    pub kubernetes_enabled: bool,
    /// Kubernetes namespace for PoolAI resources
    pub kubernetes_namespace: String,
    /// Enable AWS cloud provider integration
    pub aws_enabled: bool,
    /// AWS region (defaults to AWS_REGION env var if not set)
    pub aws_region: Option<String>,
    /// Enable Azure cloud provider integration
    pub azure_enabled: bool,
    /// Azure subscription ID (defaults to AZURE_SUBSCRIPTION_ID env var if not set)
    pub azure_subscription_id: Option<String>,
    /// Enable GCP cloud provider integration
    pub gcp_enabled: bool,
    /// GCP project ID (defaults to GCP_PROJECT_ID env var if not set)
    pub gcp_project_id: Option<String>,
    /// Enable auto-scaling features
    pub autoscaling_enabled: bool,
    /// Enable load balancing features
    pub loadbalancing_enabled: bool,
}

impl CloudConfig {
    /// Validate configuration
    ///
    /// Checks that configuration is valid and returns an error if any issues are found.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - Kubernetes namespace is empty when Kubernetes is enabled
    /// - AWS region is not set when AWS is enabled
    /// - Azure subscription ID is not set when Azure is enabled
    /// - GCP project ID is not set when GCP is enabled
    pub fn validate(&self) -> Result<(), AppError> {
        if self.kubernetes_enabled && self.kubernetes_namespace.is_empty() {
            return Err(AppError::ValidationError(format!(
                "Kubernetes namespace cannot be empty when Kubernetes is enabled. Current value: '{}'. Suggestion: Set a valid namespace like 'poolai' or 'default'.",
                self.kubernetes_namespace
            )));
        }

        if self.aws_enabled {
            let region = self.aws_region.as_deref()
                .or_else(|| std::env::var("AWS_REGION").ok().as_deref());
            if region.is_none() || region.unwrap().is_empty() {
                return Err(AppError::ValidationError(
                    "AWS region must be set when AWS is enabled. Current value: None. Suggestion: Set aws_region in config or set AWS_REGION environment variable."
                        .to_string(),
                ));
            }
        }

        if self.azure_enabled {
            let subscription_id = self.azure_subscription_id.as_deref()
                .or_else(|| std::env::var("AZURE_SUBSCRIPTION_ID").ok().as_deref());
            if subscription_id.is_none() || subscription_id.unwrap().is_empty() {
                return Err(AppError::ValidationError(
                    "Azure subscription ID must be set when Azure is enabled. Current value: None. Suggestion: Set azure_subscription_id in config or set AZURE_SUBSCRIPTION_ID environment variable."
                        .to_string(),
                ));
            }
        }

        if self.gcp_enabled {
            let project_id = self.gcp_project_id.as_deref()
                .or_else(|| std::env::var("GCP_PROJECT_ID").ok().as_deref());
            if project_id.is_none() || project_id.unwrap().is_empty() {
                return Err(AppError::ValidationError(
                    "GCP project ID must be set when GCP is enabled. Current value: None. Suggestion: Set gcp_project_id in config or set GCP_PROJECT_ID environment variable."
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            kubernetes_enabled: false,
            kubernetes_namespace: "default".to_string(),
            aws_enabled: false,
            aws_region: None,
            azure_enabled: false,
            azure_subscription_id: None,
            gcp_enabled: false,
            gcp_project_id: None,
            autoscaling_enabled: false,
            loadbalancing_enabled: false,
        }
    }
}

/// Cloud Manager - orchestrates all cloud integration features
pub struct CloudManager {
    kubernetes: Option<kubernetes::KubernetesManager>,
    aws: Option<providers::aws::AwsManager>,
    azure: Option<providers::azure::AzureManager>,
    gcp: Option<providers::gcp::GcpManager>,
    autoscaler: Option<autoscaling::AutoScaler>,
    loadbalancer: Option<loadbalancing::LoadBalancer>,
    config: CloudConfig,
    initialized: Arc<RwLock<bool>>,
}

impl CloudManager {
    /// Create a new CloudManager with the given configuration
    pub fn new(config: CloudConfig) -> Self {
        let kubernetes = if config.kubernetes_enabled {
            Some(kubernetes::KubernetesManager::new(
                config.kubernetes_namespace.clone(),
            ))
        } else {
            None
        };

        let aws = if config.aws_enabled {
            Some(providers::aws::AwsManager::new(
                config.aws_region.clone(),
            ))
        } else {
            None
        };

        let azure = if config.azure_enabled {
            Some(providers::azure::AzureManager::new(
                config.azure_subscription_id.clone(),
            ))
        } else {
            None
        };

        let gcp = if config.gcp_enabled {
            Some(providers::gcp::GcpManager::new(
                config.gcp_project_id.clone(),
            ))
        } else {
            None
        };

        let autoscaler = if config.autoscaling_enabled {
            Some(autoscaling::AutoScaler::new())
        } else {
            None
        };

        let loadbalancer = if config.loadbalancing_enabled {
            Some(loadbalancing::LoadBalancer::new())
        } else {
            None
        };

        Self {
            kubernetes,
            aws,
            azure,
            gcp,
            autoscaler,
            loadbalancer,
            config: config.clone(),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize cloud integration
    ///
    /// Validates configuration and initializes all enabled cloud features.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if configuration is invalid.
    /// Returns other `AppError` variants if initialization fails.
    pub async fn initialize(&self) -> Result<(), AppError> {
        // Validate configuration
        self.config.validate()?;

        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        if let Some(ref k8s) = self.kubernetes {
            k8s.initialize().await?;
        }

        if let Some(ref aws) = self.aws {
            aws.initialize().await?;
        }

        if let Some(ref azure) = self.azure {
            azure.initialize().await?;
        }

        if let Some(ref gcp) = self.gcp {
            gcp.initialize().await?;
        }

        if let Some(ref autoscaler) = self.autoscaler {
            autoscaler.initialize().await?;
        }

        if let Some(ref loadbalancer) = self.loadbalancer {
            loadbalancer.initialize().await?;
        }

        *initialized = true;
        info!("Cloud manager initialized");
        Ok(())
    }

    /// Shutdown cloud integration
    pub async fn shutdown(&self) -> Result<(), AppError> {
        if let Some(ref k8s) = self.kubernetes {
            k8s.shutdown().await?;
        }

        if let Some(ref aws) = self.aws {
            aws.shutdown().await?;
        }

        if let Some(ref azure) = self.azure {
            azure.shutdown().await?;
        }

        if let Some(ref gcp) = self.gcp {
            gcp.shutdown().await?;
        }

        if let Some(ref autoscaler) = self.autoscaler {
            autoscaler.shutdown().await?;
        }

        if let Some(ref loadbalancer) = self.loadbalancer {
            loadbalancer.shutdown().await?;
        }

        *self.initialized.write().await = false;
        info!("Cloud manager shut down");
        Ok(())
    }

    /// Get Kubernetes manager (if enabled)
    pub fn kubernetes(&self) -> Option<&kubernetes::KubernetesManager> {
        self.kubernetes.as_ref()
    }

    /// Get AWS manager (if enabled)
    pub fn aws(&self) -> Option<&providers::aws::AwsManager> {
        self.aws.as_ref()
    }

    /// Get Azure manager (if enabled)
    pub fn azure(&self) -> Option<&providers::azure::AzureManager> {
        self.azure.as_ref()
    }

    /// Get GCP manager (if enabled)
    pub fn gcp(&self) -> Option<&providers::gcp::GcpManager> {
        self.gcp.as_ref()
    }

    /// Get auto-scaler (if enabled)
    pub fn autoscaler(&self) -> Option<&autoscaling::AutoScaler> {
        self.autoscaler.as_ref()
    }

    /// Get load balancer (if enabled)
    pub fn loadbalancer(&self) -> Option<&loadbalancing::LoadBalancer> {
        self.loadbalancer.as_ref()
    }
}

impl Default for CloudManager {
    fn default() -> Self {
        Self::new(CloudConfig::default())
    }
}
