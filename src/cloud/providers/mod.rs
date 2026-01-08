//! Cloud provider integrations
//!
//! Provides integration with major cloud providers:
//! - AWS (EC2, ECS, Lambda, S3)
//! - Azure (VM Scale Sets, Container Instances, Blob Storage)
//! - GCP (Compute Engine, Cloud Run, Cloud Storage)

pub mod aws;
pub mod azure;
pub mod gcp;
