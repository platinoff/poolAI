//! Integration test harness (cloud mock servers, etc.)
//!
//! Run: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk`

#[cfg(feature = "cloud-sdk")]
pub mod cloud;
