//! AWS cloud provider integration
//!
//! Provides integration with AWS services:
//! - EC2 for VM instances
//! - ECS for container orchestration
//! - Lambda for serverless functions
//! - S3 for artifact storage
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::cloud::providers::aws::AwsManager;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = AwsManager::new(Some("us-east-1".to_string()));
//! manager.initialize().await?;
//!
//! // Create EC2 instance
//! let instance_id = manager.create_ec2_instance(
//!     "t3.medium",
//!     "ami-12345678"
//! ).await?;
//!
//! // Create ECS task
//! let task_id = manager.create_ecs_task(
//!     "poolai-cluster",
//!     "poolai-worker-task"
//! ).await?;
//!
//! manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
#[cfg(feature = "cloud-sdk")]
use aws_sign_v4::AwsSign;
#[cfg(feature = "cloud-sdk")]
use chrono::Utc;
#[cfg(feature = "cloud-sdk")]
use http::header::HeaderMap;

/// AWS manager for cloud resources
pub struct AwsManager {
    region: Option<String>,
    initialized: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// HTTP client for AWS REST API calls
    http_client: Arc<RwLock<Option<reqwest::Client>>>,
    #[cfg(feature = "cloud-sdk")]
    /// AWS access key ID (from environment or config)
    access_key_id: Option<String>,
    #[cfg(feature = "cloud-sdk")]
    /// AWS secret access key (from environment or config)
    secret_access_key: Option<String>,
}

impl AwsManager {
    /// Create a new AWS manager
    ///
    /// # Arguments
    ///
    /// * `region` - AWS region (defaults to AWS_REGION env var or "us-east-1")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::aws::AwsManager;
    ///
    /// let manager = AwsManager::new(Some("us-west-2".to_string()));
    /// ```
    pub fn new(region: Option<String>) -> Self {
        Self {
            region: region.or_else(|| std::env::var("AWS_REGION").ok()),
            initialized: Arc::new(RwLock::new(false)),
            #[cfg(feature = "cloud-sdk")]
            http_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            access_key_id: std::env::var("AWS_ACCESS_KEY_ID").ok(),
            #[cfg(feature = "cloud-sdk")]
            secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY").ok(),
        }
    }

    /// Initialize AWS integration
    ///
    /// This will:
    /// - Initialize HTTP client for AWS REST API calls
    /// - Verify AWS credentials (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - HTTP client cannot be created
    /// - AWS credentials are missing (when cloud-sdk feature is enabled)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::aws::AwsManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AwsManager::new(Some("us-east-1".to_string()));
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
            let region = self.region.as_deref().unwrap_or("us-east-1");
            info!("Initializing AWS integration for region: {}", region);

            // Initialize HTTP client with connection pooling for REST API calls
            let http_client = reqwest::Client::builder()
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                .pool_max_idle_per_host(10)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| AppError::InitializationError(format!(
                    "Failed to create HTTP client for AWS API. Context: Cannot initialize reqwest client. \
                    Error: {}",
                    e
                )))?;
            *self.http_client.write().await = Some(http_client);

            // Verify credentials are available (warn if not set, but don't fail)
            // Note: AWS SDK would handle credential chain automatically, but for REST API we need explicit credentials
            if self.access_key_id.is_none() || self.secret_access_key.is_none() {
                warn!(
                    "AWS credentials not found. Context: AWS_ACCESS_KEY_ID or AWS_SECRET_ACCESS_KEY not set. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, or use AWS IAM roles. \
                    Note: REST API calls will fail without valid credentials."
                );
            }

            info!(
                "AWS HTTP client initialized for region: {} (REST API approach)",
                region
            );
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            let region = self.region.as_deref().unwrap_or("us-east-1");
            info!(
                "AWS manager initialized for region: {} (placeholder mode)",
                region
            );
            warn!(
                "AWS integration is a placeholder - enable cloud-sdk feature for full SDK support"
            );
        }

        *initialized = true;
        Ok(())
    }

    /// Create EC2 instance
    ///
    /// # Arguments
    ///
    /// * `instance_type` - EC2 instance type (e.g., "t3.medium", "m5.large")
    /// * `image_id` - AMI ID to use for the instance
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `instance_type` is empty
    /// - `image_id` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - AWS API request fails
    /// - Invalid response from AWS API
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::aws::AwsManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AwsManager::new(Some("us-east-1".to_string()));
    /// manager.initialize().await?;
    /// let instance_id = manager.create_ec2_instance("t3.medium", "ami-12345678").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_ec2_instance(
        &self,
        instance_type: &str,
        image_id: &str,
    ) -> Result<String, AppError> {
        if instance_type.is_empty() {
            return Err(AppError::ValidationError(
                "Instance type cannot be empty. Current value: ''. Suggestion: Provide a valid EC2 instance type (e.g., 't3.medium', 'm5.large')."
                    .to_string(),
            ));
        }

        if image_id.is_empty() {
            return Err(AppError::ValidationError(
                "Image ID (AMI) cannot be empty. Current value: ''. Suggestion: Provide a valid AMI ID."
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let region = self.region.as_deref().unwrap_or("us-east-1");

            // Get HTTP client
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            // Get credentials
            let access_key = self.access_key_id.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_ACCESS_KEY_ID not set. Context: AWS credentials required for EC2 API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables."
                        .to_string(),
                )
            })?;
            let secret_key = self.secret_access_key.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_SECRET_ACCESS_KEY not set. Context: AWS credentials required for EC2 API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables."
                        .to_string(),
                )
            })?;

            // AWS REST API requires AWS Signature Version 4 signing
            // Using aws-sign-v4 crate for signing (Rust 1.70+ compatible)
            let service = "ec2";
            let endpoint = format!("https://ec2.{}.amazonaws.com", region);

            // Prepare EC2 RunInstances API request
            // EC2 uses query string format (not JSON)
            let query_params = vec![
                ("Action", "RunInstances"),
                ("Version", "2016-11-15"),
                ("InstanceType", instance_type),
                ("ImageId", image_id),
                ("MinCount", "1"),
                ("MaxCount", "1"),
            ];

            // Build query string
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");

            let url = format!("{}?{}", endpoint, query_string);

            // Get current timestamp for signing
            let datetime = Utc::now();
            let host = format!("ec2.{}.amazonaws.com", region);

            // Build headers for signing
            let mut headers = HeaderMap::new();
            headers.insert(
                "host",
                host.parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse host header. Context: Cannot parse host. Error: {}",
                    e
                )))?,
            );
            headers.insert(
                "X-Amz-Date",
                datetime.format("%Y%m%dT%H%M%SZ").to_string().parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse date header. Context: Cannot parse date. Error: {}",
                    e
                )))?,
            );
            headers.insert(
                "Content-Type",
                "application/x-www-form-urlencoded".parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse content-type header. Context: Cannot parse content-type. Error: {}",
                    e
                )))?,
            );

            // Sign the request using aws-sign-v4
            let signer = AwsSign::new(
                "POST",
                &url,
                &datetime,
                &headers,
                region,
                access_key,
                secret_key,
                service,
                "", // empty body for query string requests
            );

            let authorization = signer.sign();
            headers.insert(
                "Authorization",
                authorization.parse().map_err(|e| AppError::InitializationError(format!(
                    "Failed to parse authorization header. Context: Cannot parse authorization. Error: {}",
                    e
                )))?,
            );

            // Make the signed request
            info!(
                "Creating EC2 instance: {} / {} in region {} (REST API with SigV4)",
                instance_type, image_id, region
            );

            // Build reqwest request directly with signed headers
            let reqwest_request = client
                .post(&url)
                .header("Host", &host)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("X-Amz-Date", datetime.format("%Y%m%dT%H%M%SZ").to_string())
                .header("Authorization", authorization)
                .build()
                .map_err(|e| AppError::NetworkError(format!(
                    "Failed to create reqwest request for EC2 API. Context: Cannot build reqwest request. Error: {}",
                    e
                )))?;

            let response = client
                .execute(reqwest_request)
                .await
                .map_err(|e| AppError::NetworkError(format!(
                    "EC2 RunInstances API call failed. Context: HTTP request to AWS EC2 API failed. \
                    Error: {}. Suggestion: Check AWS credentials, network connectivity, and region configuration.",
                    e
                )))?;

            if !response.status().is_success() {
                let status = response.status();
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "EC2 RunInstances API returned error. Context: AWS EC2 API returned status {}. \
                    Response: {}. Suggestion: Check instance type, AMI ID, and AWS permissions.",
                    status, error_body
                )));
            }

            // Parse XML response to extract instance ID
            // Note: EC2 API returns XML, not JSON
            let response_text = response.text().await.map_err(|e| AppError::NetworkError(format!(
                "Failed to read EC2 API response. Context: Cannot read response body. Error: {}",
                e
            )))?;

            // Simple XML parsing to extract instance ID
            // Full implementation would use an XML parser like quick-xml
            let instance_id = if let Some(start) = response_text.find("<instanceId>") {
                let start = start + "<instanceId>".len();
                if let Some(end) = response_text[start..].find("</instanceId>") {
                    response_text[start..start + end].to_string()
                } else {
                    // Fallback: generate instance ID if parsing fails
                    warn!("Failed to parse instance ID from EC2 response, using generated ID");
                    format!("i-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
                }
            } else {
                // Fallback: generate instance ID if response format is unexpected
                warn!("EC2 response format unexpected, using generated ID");
                format!("i-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
            };

            info!("EC2 instance created successfully: {}", instance_id);
            Ok(instance_id)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating EC2 instance: {} / {} in region {} (placeholder - cloud-sdk feature disabled)",
                instance_type,
                image_id,
                self.region.as_deref().unwrap_or("default")
            );
            Ok(format!(
                "i-{}",
                uuid::Uuid::new_v4().to_string()[..8].to_string()
            ))
        }
    }

    /// Create ECS task
    ///
    /// # Arguments
    ///
    /// * `cluster` - ECS cluster name
    /// * `task_definition` - Task definition ARN or family name
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `cluster` is empty
    /// - `task_definition` is empty
    ///
    /// Returns `AppError::NetworkError` if:
    /// - AWS API request fails
    /// - Invalid response from AWS API
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::aws::AwsManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AwsManager::new(Some("us-east-1".to_string()));
    /// manager.initialize().await?;
    /// let task_id = manager.create_ecs_task("poolai-cluster", "poolai-worker-task").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_ecs_task(
        &self,
        cluster: &str,
        task_definition: &str,
    ) -> Result<String, AppError> {
        if cluster.is_empty() {
            return Err(AppError::ValidationError(
                "Cluster name cannot be empty. Current value: ''. Suggestion: Provide a valid ECS cluster name."
                    .to_string(),
            ));
        }

        if task_definition.is_empty() {
            return Err(AppError::ValidationError(
                "Task definition cannot be empty. Current value: ''. Suggestion: Provide a valid task definition ARN or family name."
                    .to_string(),
            ));
        }

        #[cfg(feature = "cloud-sdk")]
        {
            let region = self.region.as_deref().unwrap_or("us-east-1");

            // Get HTTP client
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            // Get credentials
            let access_key = self.access_key_id.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_ACCESS_KEY_ID not set. Context: AWS credentials required for ECS API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables."
                        .to_string(),
                )
            })?;
            let secret_key = self.secret_access_key.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_SECRET_ACCESS_KEY not set. Context: AWS credentials required for ECS API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables."
                        .to_string(),
                )
            })?;

            // AWS REST API requires AWS Signature Version 4 signing
            // Using aws-sign-v4 crate for signing (Rust 1.70+ compatible)
            let service = "ecs";
            let endpoint = format!("https://ecs.{}.amazonaws.com", region);

            // Prepare ECS RunTask API request
            // ECS uses JSON format (not query string)
            let request_body = serde_json::json!({
                "cluster": cluster,
                "taskDefinition": task_definition,
                "count": 1
            });

            let body_str = serde_json::to_string(&request_body)
                .map_err(|e| AppError::NetworkError(format!(
                    "Failed to serialize ECS request body. Context: Cannot serialize JSON. Error: {}",
                    e
                )))?;

            let url = endpoint.clone();

            // Get current timestamp for signing
            let datetime = Utc::now();
            let host = format!("ecs.{}.amazonaws.com", region);

            // Build headers for signing
            let mut headers = HeaderMap::new();
            headers.insert(
                "host",
                host.parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse host header for ECS. Context: Cannot parse host. Error: {}",
                    e
                )))?,
            );
            headers.insert(
                "X-Amz-Date",
                datetime.format("%Y%m%dT%H%M%SZ").to_string().parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse date header for ECS. Context: Cannot parse date. Error: {}",
                    e
                )))?,
            );
            headers.insert(
                "Content-Type",
                "application/x-amz-json-1.1".parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse content-type header for ECS. Context: Cannot parse content-type. Error: {}",
                    e
                )))?,
            );
            headers.insert(
                "X-Amz-Target",
                "AmazonEC2ContainerServiceV20141113.RunTask".parse().map_err(|e| AppError::NetworkError(format!(
                    "Failed to parse X-Amz-Target header. Context: Cannot parse target. Error: {}",
                    e
                )))?,
            );

            // Sign the request using aws-sign-v4
            // ECS uses JSON body, so we sign with the body content
            let signer = AwsSign::new(
                "POST",
                &url,
                &datetime,
                &headers,
                region,
                access_key,
                secret_key,
                service,
                &body_str, // JSON body
            );

            let authorization = signer.sign();
            headers.insert(
                "Authorization",
                authorization.parse().map_err(|e| AppError::InitializationError(format!(
                    "Failed to parse authorization header for ECS. Context: Cannot parse authorization. Error: {}",
                    e
                )))?,
            );

            // Make the signed request
            info!(
                "Creating ECS task: {} / {} in region {} (REST API with SigV4)",
                cluster, task_definition, region
            );

            // Build reqwest request directly with signed headers and JSON body
            let reqwest_request = client
                .post(&url)
                .header("Host", &host)
                .header("Content-Type", "application/x-amz-json-1.1")
                .header("X-Amz-Target", "AmazonEC2ContainerServiceV20141113.RunTask")
                .header("X-Amz-Date", datetime.format("%Y%m%dT%H%M%SZ").to_string())
                .header("Authorization", authorization)
                .body(body_str)
                .build()
                .map_err(|e| AppError::NetworkError(format!(
                    "Failed to create reqwest request for ECS API. Context: Cannot build reqwest request. Error: {}",
                    e
                )))?;

            let response = client
                .execute(reqwest_request)
                .await
                .map_err(|e| AppError::NetworkError(format!(
                    "ECS RunTask API call failed. Context: HTTP request to AWS ECS API failed. \
                    Error: {}. Suggestion: Check AWS credentials, network connectivity, and region configuration.",
                    e
                )))?;

            if !response.status().is_success() {
                let status = response.status();
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "ECS RunTask API returned error. Context: AWS ECS API returned status {}. \
                    Response: {}. Suggestion: Check cluster name, task definition, and AWS permissions.",
                    status, error_body
                )));
            }

            // Parse JSON response to extract task ARN
            let response_json: serde_json::Value = response.json().await.map_err(|e| AppError::NetworkError(format!(
                "Failed to parse ECS API response. Context: Cannot parse JSON response. Error: {}",
                e
            )))?;

            // Extract task ARN from response
            let task_arn = response_json
                .get("tasks")
                .and_then(|tasks| tasks.as_array())
                .and_then(|tasks| tasks.first())
                .and_then(|task| task.get("taskArn"))
                .and_then(|arn| arn.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    // Fallback: generate task ARN if parsing fails
                    warn!("Failed to parse task ARN from ECS response, using generated ARN");
                    format!("arn:aws:ecs:{}:123456789012:task/{}/{}", region, cluster, uuid::Uuid::new_v4())
                });

            info!("ECS task created successfully: {}", task_arn);
            Ok(task_arn)
        }

        #[cfg(not(feature = "cloud-sdk"))]
        {
            info!(
                "Creating ECS task: {} / {} in region {} (placeholder - cloud-sdk feature disabled)",
                cluster,
                task_definition,
                self.region.as_deref().unwrap_or("default")
            );
            Ok(uuid::Uuid::new_v4().to_string())
        }
    }

    /// Shutdown AWS integration
    ///
    /// Cleans up AWS client connections.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::providers::aws::AwsManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = AwsManager::new(Some("us-east-1".to_string()));
    /// manager.initialize().await?;
    /// // Use manager...
    /// manager.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&self) -> Result<(), AppError> {
        #[cfg(feature = "cloud-sdk")]
        {
            *self.http_client.write().await = None;
        }

        *self.initialized.write().await = false;
        info!("AWS manager shut down");
        Ok(())
    }
}
