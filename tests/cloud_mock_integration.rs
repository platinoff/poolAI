//! Cloud mock server integration tests
//!
//! Wires `tests/integration/cloud/` (mock servers for AWS, Azure, GCP).
//! Run: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk`
//! Note: cloud-sdk requires Rust 1.88+ (AWS SDK). CI runs these when enabled.

mod integration;
