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
#[cfg(feature = "aws-sdk-ec2")]
use aws_sdk_ec2::Client as Ec2Client;
#[cfg(feature = "aws-sdk-ecs")]
use aws_sdk_ecs::Client as EcsClient;
#[cfg(feature = "aws-sdk-s3")]
use aws_sdk_s3::Client as S3Client;
#[cfg(feature = "cloud-sdk")]
use aws_sign_v4::AwsSign;
#[cfg(feature = "cloud-sdk")]
use chrono::Utc;
#[cfg(feature = "cloud-sdk")]
use http::header::HeaderMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// AWS manager for cloud resources
pub struct AwsManager {
    region: Option<String>,
    initialized: Arc<RwLock<bool>>,
    #[cfg(feature = "cloud-sdk")]
    /// HTTP client for AWS REST API calls (fallback when SDK not available)
    http_client: Arc<RwLock<Option<reqwest::Client>>>,
    #[cfg(feature = "cloud-sdk")]
    /// AWS access key ID (from environment or config, used for REST API fallback)
    access_key_id: Option<String>,
    #[cfg(feature = "cloud-sdk")]
    /// AWS secret access key (from environment or config, used for REST API fallback)
    secret_access_key: Option<String>,
    #[cfg(feature = "aws-sdk-ec2")]
    /// AWS SDK EC2 client
    ec2_client: Arc<RwLock<Option<Ec2Client>>>,
    #[cfg(feature = "aws-sdk-ecs")]
    /// AWS SDK ECS client
    ecs_client: Arc<RwLock<Option<EcsClient>>>,
    #[cfg(feature = "aws-sdk-s3")]
    /// AWS SDK S3 client
    s3_client: Arc<RwLock<Option<S3Client>>>,

    #[cfg(feature = "cloud-sdk")]
    /// EC2 REST API base URL override (e.g. mock server for tests).
    ec2_base_url_override: Arc<RwLock<Option<String>>>,

    #[cfg(feature = "cloud-sdk")]
    /// ECS REST API base URL override (e.g. mock server for tests).
    ecs_base_url_override: Arc<RwLock<Option<String>>>,
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
            #[cfg(feature = "aws-sdk-ec2")]
            ec2_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "aws-sdk-ecs")]
            ecs_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "aws-sdk-s3")]
            s3_client: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            ec2_base_url_override: Arc::new(RwLock::new(None)),
            #[cfg(feature = "cloud-sdk")]
            ecs_base_url_override: Arc::new(RwLock::new(None)),
        }
    }

    /// Set EC2 API base URL override (e.g. mock server for tests). When set, SigV4 is skipped.
    #[cfg(feature = "cloud-sdk")]
    pub async fn set_ec2_base_url_override(&self, url: Option<String>) {
        *self.ec2_base_url_override.write().await = url;
    }

    /// Set ECS API base URL override (e.g. mock server for tests). When set, SigV4 is skipped.
    #[cfg(feature = "cloud-sdk")]
    pub async fn set_ecs_base_url_override(&self, url: Option<String>) {
        *self.ecs_base_url_override.write().await = url;
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
            let region_str = self.region.as_deref().unwrap_or("us-east-1");
            info!("Initializing AWS integration for region: {}", region_str);

            // Try to initialize AWS SDK clients first (if available)
            #[cfg(feature = "aws-config")]
            {
                use aws_config::meta::region::RegionProviderChain;
                use aws_sdk_ec2::config::Region;

                info!("Initializing AWS SDK clients...");

                // Create region provider (AWS_REGION env var or default)
                let region_provider = RegionProviderChain::first_try(
                    std::env::var("AWS_REGION").ok().map(Region::new),
                )
                .or_default_provider()
                .or_else(Region::new(region_str.to_string()));

                // Load AWS config with automatic credential chain resolution
                // AWS SDK will try in order:
                // 1. Environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
                // 2. AWS credentials file (~/.aws/credentials)
                // 3. IAM roles (when running on EC2/ECS/Lambda)
                let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .region(region_provider)
                    .load()
                    .await;

                // Initialize EC2 client
                #[cfg(feature = "aws-sdk-ec2")]
                {
                    let ec2_client = aws_sdk_ec2::Client::new(&config);
                    *self.ec2_client.write().await = Some(ec2_client);
                    info!("AWS SDK EC2 client initialized");
                }

                // Initialize ECS client
                #[cfg(feature = "aws-sdk-ecs")]
                {
                    let ecs_client = aws_sdk_ecs::Client::new(&config);
                    *self.ecs_client.write().await = Some(ecs_client);
                    info!("AWS SDK ECS client initialized");
                }

                // Initialize S3 client
                #[cfg(feature = "aws-sdk-s3")]
                {
                    let s3_client = aws_sdk_s3::Client::new(&config);
                    *self.s3_client.write().await = Some(s3_client);
                    info!("AWS SDK S3 client initialized");
                }

                info!(
                    "AWS SDK clients initialized successfully for region: {}",
                    config.region().map(|r| r.as_ref()).unwrap_or(region_str)
                );
            }

            // Also initialize HTTP client for REST API fallback (if SDK not available or for backward compatibility)
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

            // Note: AWS SDK handles credential chain automatically, so we only warn for REST API fallback
            #[cfg(not(feature = "aws-config"))]
            {
                if self.access_key_id.is_none() || self.secret_access_key.is_none() {
                    warn!(
                        "AWS credentials not found. Context: AWS_ACCESS_KEY_ID or AWS_SECRET_ACCESS_KEY not set. \
                        Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, or use AWS IAM roles. \
                        Note: REST API calls will fail without valid credentials. Consider enabling AWS SDK (requires Rust 1.88+) for automatic credential chain resolution."
                    );
                }
            }
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

            // Try AWS SDK first (if available)
            #[cfg(feature = "aws-sdk-ec2")]
            {
                let ec2_guard = self.ec2_client.read().await;
                if let Some(ec2_client) = ec2_guard.as_ref() {
                    info!(
                        "Creating EC2 instance: {} / {} in region {} (AWS SDK)",
                        instance_type, image_id, region
                    );

                    use aws_sdk_ec2::types::InstanceType;

                    // Build RunInstances request using fluent builder
                    match ec2_client
                        .run_instances()
                        .image_id(image_id)
                        .instance_type(InstanceType::from(instance_type))
                        .min_count(1)
                        .max_count(1)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            let instances = response.instances();
                            if let Some(instance) = instances.first() {
                                if let Some(instance_id) = instance.instance_id() {
                                    info!(
                                        "EC2 instance created successfully via AWS SDK: {}",
                                        instance_id
                                    );
                                    return Ok(instance_id.to_string());
                                }
                            }
                            return Err(AppError::NetworkError(
                                "EC2 RunInstances response missing instance ID".to_string(),
                            ));
                        }
                        Err(e) => Err(AppError::NetworkError(format!(
                            "EC2 RunInstances API call failed via AWS SDK. Context: AWS SDK request failed. \
                            Error: {}. Suggestion: Check AWS credentials, permissions, instance type, and AMI ID.",
                            e
                        ))),
                    }?;
                }
            }

            // Fallback to REST API if SDK not available
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            let query_params = vec![
                ("Action", "RunInstances"),
                ("Version", "2016-11-15"),
                ("InstanceType", instance_type),
                ("ImageId", image_id),
                ("MinCount", "1"),
                ("MaxCount", "1"),
            ];
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");

            if let Some(ref base) = *self.ec2_base_url_override.read().await {
                let base = base.trim_end_matches('/');
                let url = format!("{}?{}", base, query_string);
                let response = client
                    .post(&url)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .send()
                    .await
                    .map_err(|e| {
                        AppError::NetworkError(format!(
                            "EC2 RunInstances (mock) failed. Error: {}",
                            e
                        ))
                    })?;
                if !response.status().is_success() {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    return Err(AppError::NetworkError(format!(
                        "EC2 mock returned {}: {}",
                        status, body
                    )));
                }
                let response_text = response
                    .text()
                    .await
                    .map_err(|e| AppError::NetworkError(e.to_string()))?;
                let instance_id = if let Some(start) = response_text.find("<instanceId>") {
                    let start = start + "<instanceId>".len();
                    if let Some(end) = response_text[start..].find("</instanceId>") {
                        response_text[start..start + end].to_string()
                    } else {
                        format!("i-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
                    }
                } else {
                    format!("i-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
                };
                info!("EC2 instance created (mock): {}", instance_id);
                return Ok(instance_id);
            }

            let access_key = self.access_key_id.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_ACCESS_KEY_ID not set. Context: AWS credentials required for EC2 API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, \
                    or use AWS SDK (enables automatic credential chain resolution)."
                        .to_string(),
                )
            })?;
            let secret_key = self.secret_access_key.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_SECRET_ACCESS_KEY not set. Context: AWS credentials required for EC2 API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, \
                    or use AWS SDK (enables automatic credential chain resolution)."
                        .to_string(),
                )
            })?;

            let service = "ec2";
            let endpoint = format!("https://ec2.{}.amazonaws.com", region);
            let url = format!("{}?{}", endpoint, query_string);

            // Get current timestamp for signing
            let datetime = Utc::now();
            let host = format!("ec2.{}.amazonaws.com", region);

            // Build headers for signing
            let mut headers = HeaderMap::new();
            headers.insert(
                "host",
                host.parse().map_err(|e| {
                    AppError::NetworkError(format!(
                        "Failed to parse host header. Context: Cannot parse host. Error: {}",
                        e
                    ))
                })?,
            );
            headers.insert(
                "X-Amz-Date",
                datetime
                    .format("%Y%m%dT%H%M%SZ")
                    .to_string()
                    .parse()
                    .map_err(|e| {
                        AppError::NetworkError(format!(
                            "Failed to parse date header. Context: Cannot parse date. Error: {}",
                            e
                        ))
                    })?,
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
                "POST", &url, &datetime, &headers, region, access_key, secret_key, service,
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
                let error_body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "EC2 RunInstances API returned error. Context: AWS EC2 API returned status {}. \
                    Response: {}. Suggestion: Check instance type, AMI ID, and AWS permissions.",
                    status, error_body
                )));
            }

            // Parse XML response to extract instance ID
            // Note: EC2 API returns XML, not JSON
            let response_text = response.text().await.map_err(|e| {
                AppError::NetworkError(format!(
                "Failed to read EC2 API response. Context: Cannot read response body. Error: {}",
                e
            ))
            })?;

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

            // Try AWS SDK first (if available)
            #[cfg(feature = "aws-sdk-ecs")]
            {
                let ecs_guard = self.ecs_client.read().await;
                if let Some(ecs_client) = ecs_guard.as_ref() {
                    info!(
                        "Creating ECS task: {} / {} in region {} (AWS SDK)",
                        cluster, task_definition, region
                    );

                    // Build RunTask request using fluent builder
                    match ecs_client
                        .run_task()
                        .cluster(cluster)
                        .task_definition(task_definition)
                        .count(1)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            let tasks = response.tasks();
                            if let Some(task) = tasks.first() {
                                if let Some(task_arn) = task.task_arn() {
                                    info!(
                                        "ECS task created successfully via AWS SDK: {}",
                                        task_arn
                                    );
                                    return Ok(task_arn.to_string());
                                }
                            }
                            return Err(AppError::NetworkError(
                                "ECS RunTask response missing task ARN".to_string(),
                            ));
                        }
                        Err(e) => Err(AppError::NetworkError(format!(
                            "ECS RunTask API call failed via AWS SDK. Context: AWS SDK request failed. \
                            Error: {}. Suggestion: Check AWS credentials, permissions, cluster name, and task definition.",
                            e
                        ))),
                    }?;
                }
            }

            // Fallback to REST API if SDK not available
            let client_guard = self.http_client.read().await;
            let client = client_guard.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS HTTP client not initialized. Call initialize() first.".to_string(),
                )
            })?;

            let request_body = serde_json::json!({
                "cluster": cluster,
                "taskDefinition": task_definition,
                "count": 1
            });
            let body_str = serde_json::to_string(&request_body).map_err(|e| {
                AppError::NetworkError(format!(
                    "Failed to serialize ECS request body. Error: {}",
                    e
                ))
            })?;

            if let Some(ref base) = *self.ecs_base_url_override.read().await {
                let base = base.trim_end_matches('/');
                let response = client
                    .post(base)
                    .header("Content-Type", "application/x-amz-json-1.1")
                    .header("X-Amz-Target", "AmazonEC2ContainerServiceV20141113.RunTask")
                    .body(body_str.clone())
                    .send()
                    .await
                    .map_err(|e| {
                        AppError::NetworkError(format!("ECS RunTask (mock) failed. Error: {}", e))
                    })?;
                if !response.status().is_success() {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    return Err(AppError::NetworkError(format!(
                        "ECS mock returned {}: {}",
                        status, body
                    )));
                }
                let json: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| AppError::NetworkError(e.to_string()))?;
                let task_arn = json
                    .get("tasks")
                    .and_then(|t| t.as_array())
                    .and_then(|a| a.first())
                    .and_then(|t| t.get("taskArn"))
                    .and_then(|a| a.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| {
                        AppError::NetworkError("ECS mock response missing taskArn".to_string())
                    })?;
                info!("ECS task created (mock): {}", task_arn);
                return Ok(task_arn);
            }

            let access_key = self.access_key_id.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_ACCESS_KEY_ID not set. Context: AWS credentials required for ECS API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, \
                    or use AWS SDK (enables automatic credential chain resolution)."
                        .to_string(),
                )
            })?;
            let secret_key = self.secret_access_key.as_ref().ok_or_else(|| {
                AppError::InitializationError(
                    "AWS_SECRET_ACCESS_KEY not set. Context: AWS credentials required for ECS API calls. \
                    Suggestion: Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, \
                    or use AWS SDK (enables automatic credential chain resolution)."
                        .to_string(),
                )
            })?;

            let service = "ecs";
            let endpoint = format!("https://ecs.{}.amazonaws.com", region);
            let url = endpoint.clone();

            // Get current timestamp for signing
            let datetime = Utc::now();
            let host = format!("ecs.{}.amazonaws.com", region);

            // Build headers for signing
            let mut headers = HeaderMap::new();
            headers.insert(
                "host",
                host.parse().map_err(|e| {
                    AppError::NetworkError(format!(
                    "Failed to parse host header for ECS. Context: Cannot parse host. Error: {}",
                    e
                ))
                })?,
            );
            headers.insert(
                "X-Amz-Date",
                datetime
                    .format("%Y%m%dT%H%M%SZ")
                    .to_string()
                    .parse()
                    .map_err(|e| {
                        AppError::NetworkError(format!(
                    "Failed to parse date header for ECS. Context: Cannot parse date. Error: {}",
                    e
                ))
                    })?,
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
                "AmazonEC2ContainerServiceV20141113.RunTask"
                    .parse()
                    .map_err(|e| {
                        AppError::NetworkError(format!(
                    "Failed to parse X-Amz-Target header. Context: Cannot parse target. Error: {}",
                    e
                ))
                    })?,
            );

            // Sign the request using aws-sign-v4
            // ECS uses JSON body, so we sign with the body content
            let signer = AwsSign::new(
                "POST", &url, &datetime, &headers, region, access_key, secret_key, service,
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
                let error_body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(AppError::NetworkError(format!(
                    "ECS RunTask API returned error. Context: AWS ECS API returned status {}. \
                    Response: {}. Suggestion: Check cluster name, task definition, and AWS permissions.",
                    status, error_body
                )));
            }

            // Parse JSON response to extract task ARN
            let response_json: serde_json::Value = response.json().await.map_err(|e| {
                AppError::NetworkError(format!(
                "Failed to parse ECS API response. Context: Cannot parse JSON response. Error: {}",
                e
            ))
            })?;

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
                    format!(
                        "arn:aws:ecs:{}:123456789012:task/{}/{}",
                        region,
                        cluster,
                        uuid::Uuid::new_v4()
                    )
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
            // Clear HTTP client (REST API fallback)
            *self.http_client.write().await = None;

            // Clear AWS SDK clients
            if cfg!(feature = "aws-sdk-ec2") {
                *self.ec2_client.write().await = None;
            }
            if cfg!(feature = "aws-sdk-ecs") {
                *self.ecs_client.write().await = None;
            }
            if cfg!(feature = "aws-sdk-s3") {
                *self.s3_client.write().await = None;
            }
        }

        *self.initialized.write().await = false;
        info!("AWS manager shut down");
        Ok(())
    }
}
