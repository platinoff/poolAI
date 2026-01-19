//! Cloud integration tests with mock servers
//!
//! This module provides integration tests for cloud providers (AWS, Azure, GCP)
//! using mock HTTP servers to test API interactions without requiring real credentials.

#[cfg(feature = "cloud-sdk")]
mod token_acquisition_tests;

#[cfg(feature = "cloud-sdk")]
mod mock_servers;
