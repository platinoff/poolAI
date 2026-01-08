//! Cloud provider integrations
//!
//! Provides integration with major cloud providers:
//! - AWS (EC2, ECS, Lambda, S3)
//! - Azure (VM Scale Sets, Container Instances, Blob Storage)
//! - GCP (Compute Engine, Cloud Run, Cloud Storage)
//!
//! # Example
//!
//! ## Multi-Cloud Usage
//!
//! ```rust,no_run
//! use poolai::cloud::providers::{aws::AwsManager, azure::AzureManager, gcp::GcpManager};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Initialize multiple cloud providers
//! let aws = AwsManager::new(Some("us-east-1".to_string()));
//! let azure = AzureManager::new(Some("subscription-id".to_string()));
//! let gcp = GcpManager::new(Some("project-id".to_string()));
//!
//! aws.initialize().await?;
//! azure.initialize().await?;
//! gcp.initialize().await?;
//!
//! // Use providers as needed
//! # Ok(())
//! # }
//! ```

pub mod aws;
pub mod azure;
pub mod gcp;
