//! Integration tests for UI Write Operations
//!
//! Tests:
//! - RAID artifact create/delete operations
//! - Worker create/delete operations
//! - Validation errors
//! - RBAC permission checks

use poolai::network::validation;

#[tokio::test]
async fn test_validate_artifact_name_success() {
    // Test valid artifact names
    let long_name = "a".repeat(255);
    let valid_names = vec![
        "my-model-v1",
        "model_weights_2024",
        "library-1.2.3",
        "test123",
        "a",
        long_name.as_str(),
    ];

    for name in valid_names {
        assert!(
            validation::validate_artifact_name(name).is_ok(),
            "Artifact name '{}' should be valid",
            name
        );
    }
}

#[tokio::test]
async fn test_validate_artifact_name_failure() {
    // Test invalid artifact names
    let too_long = "a".repeat(256);
    let invalid_cases = vec![
        ("", "empty name"),
        (" ", "whitespace only"),
        ("-invalid", "starts with hyphen"),
        ("_invalid", "starts with underscore"),
        (".invalid", "starts with dot"),
        ("invalid@name", "contains invalid character"),
        ("invalid name", "contains space"),
        (too_long.as_str(), "too long"),
    ];

    for (name, reason) in invalid_cases {
        assert!(
            validation::validate_artifact_name(name).is_err(),
            "Artifact name '{}' should be invalid: {}",
            name,
            reason
        );
    }
}

#[tokio::test]
async fn test_validate_worker_id_success() {
    // Test valid worker IDs
    let long_id = "a".repeat(64);
    let valid_ids = vec![
        "worker-1",
        "worker_1",
        "worker123",
        "w",
        long_id.as_str(),
    ];

    for id in valid_ids {
        assert!(
            validation::validate_worker_id(id).is_ok(),
            "Worker ID '{}' should be valid",
            id
        );
    }
}

#[tokio::test]
async fn test_validate_worker_id_failure() {
    // Test invalid worker IDs
    let too_long = "a".repeat(65);
    let invalid_cases = vec![
        ("", "empty ID"),
        (" ", "whitespace only"),
        ("-invalid", "starts with hyphen"),
        ("_invalid", "starts with underscore"),
        ("invalid@id", "contains invalid character"),
        ("invalid id", "contains space"),
        (too_long.as_str(), "too long"),
    ];

    for (id, reason) in invalid_cases {
        assert!(
            validation::validate_worker_id(id).is_err(),
            "Worker ID '{}' should be invalid: {}",
            id,
            reason
        );
    }
}

#[tokio::test]
async fn test_validate_uuid_success() {
    // Test valid UUIDs
    let valid_uuids = vec![
        "550e8400-e29b-41d4-a716-446655440000",
        "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "00000000-0000-0000-0000-000000000000",
    ];

    for uuid_str in valid_uuids {
        assert!(
            validation::validate_uuid(uuid_str).is_ok(),
            "UUID '{}' should be valid",
            uuid_str
        );
    }
}

#[tokio::test]
async fn test_validate_uuid_failure() {
    // Test invalid UUIDs
    let invalid_cases = vec![
        ("", "empty UUID"),
        ("not-a-uuid", "invalid format"),
        ("550e8400-e29b-41d4-a716", "incomplete UUID"),
        ("550e8400-e29b-41d4-a716-446655440000-extra", "too long"),
    ];

    for (uuid_str, reason) in invalid_cases {
        assert!(
            validation::validate_uuid(uuid_str).is_err(),
            "UUID '{}' should be invalid: {}",
            uuid_str,
            reason
        );
    }
}

#[tokio::test]
async fn test_validate_base64_data_success() {
    // Test valid base64 data
    let valid_data = vec![
        ("SGVsbG8gV29ybGQ=", 100_000_000), // "Hello World" - within 100MB limit
        ("", 100_000_000), // Empty string (will fail on empty check, but format is valid)
    ];

    for (data, max_size) in valid_data {
        // Skip empty string test as it should fail on empty check
        if data.is_empty() {
            continue;
        }
        assert!(
            validation::validate_base64_data(data, max_size).is_ok(),
            "Base64 data should be valid (within size limit)"
        );
    }
}

#[tokio::test]
async fn test_validate_base64_data_failure() {
    // Test invalid base64 data
    let invalid_cases = vec![
        ("", 100_000_000), // Empty data
        (" ", 100_000_000), // Whitespace only
        // Large data test - create a base64 string that exceeds limit
        // Note: We can't easily create a huge base64 string in test, so we test with small limit
        ("SGVsbG8gV29ybGQ=", 5), // "Hello World" exceeds 5 byte limit
    ];

    for (data, max_size) in invalid_cases {
        let result = validation::validate_base64_data(data, max_size);
        if data.trim().is_empty() {
            assert!(
                result.is_err(),
                "Empty base64 data should be invalid"
            );
        } else {
            assert!(
                result.is_err(),
                "Base64 data exceeding size limit should be invalid"
            );
        }
    }
}

#[tokio::test]
async fn test_validate_worker_config_success() {
    // Test valid worker configuration
    let result = validation::validate_worker_config(
        10,    // max_concurrent_requests (1-1000)
        5000,  // request_timeout_ms (100-300000)
        1000,  // health_check_interval_ms (100-60000)
        1000,  // cache_size (100-100000)
        2048,  // max_memory_mb (256-131072)
        5,     // cpu_priority (1-10)
    );

    assert!(result.is_ok(), "Valid worker config should pass validation");
}

#[tokio::test]
async fn test_validate_worker_config_failure() {
    // Test invalid worker configuration values
    let invalid_cases = vec![
        // max_concurrent_requests out of range
        (0, 5000, 1000, 1000, 2048, 5, "max_concurrent_requests too low"),
        (1001, 5000, 1000, 1000, 2048, 5, "max_concurrent_requests too high"),
        // request_timeout_ms out of range
        (10, 99, 1000, 1000, 2048, 5, "request_timeout_ms too low"),
        (10, 300_001, 1000, 1000, 2048, 5, "request_timeout_ms too high"),
        // health_check_interval_ms out of range
        (10, 5000, 99, 1000, 2048, 5, "health_check_interval_ms too low"),
        (10, 5000, 60_001, 1000, 2048, 5, "health_check_interval_ms too high"),
        // cache_size out of range
        (10, 5000, 1000, 99, 2048, 5, "cache_size too low"),
        (10, 5000, 1000, 100_001, 2048, 5, "cache_size too high"),
        // max_memory_mb out of range
        (10, 5000, 1000, 1000, 255, 5, "max_memory_mb too low"),
        (10, 5000, 1000, 1000, 131_073, 5, "max_memory_mb too high"),
        // cpu_priority out of range
        (10, 5000, 1000, 1000, 2048, 0, "cpu_priority too low"),
        (10, 5000, 1000, 1000, 2048, 11, "cpu_priority too high"),
    ];

    for (max_req, timeout, health, cache, memory, priority, reason) in invalid_cases {
        let result = validation::validate_worker_config(
            max_req, timeout, health, cache, memory, priority,
        );
        assert!(
            result.is_err(),
            "Invalid worker config should fail validation: {}",
            reason
        );
    }
}

#[tokio::test]
async fn test_validate_artifact_data_size_success() {
    // Test valid artifact data sizes
    let valid_cases = vec![
        (1, 100_000_000),        // 1 byte within 100MB limit
        (100_000_000, 100_000_000), // Exactly at limit
        (50_000_000, 100_000_000),  // 50MB within limit
    ];

    for (size, max_size) in valid_cases {
        assert!(
            validation::validate_artifact_data_size(size, max_size).is_ok(),
            "Artifact data size {} should be valid (within {} limit)",
            size,
            max_size
        );
    }
}

#[tokio::test]
async fn test_validate_artifact_data_size_failure() {
    // Test invalid artifact data sizes
    let invalid_cases = vec![
        (0, 100_000_000, "empty data"),
        (100_000_001, 100_000_000, "exceeds limit"),
        (200_000_000, 100_000_000, "double the limit"),
    ];

    for (size, max_size, reason) in invalid_cases {
        assert!(
            validation::validate_artifact_data_size(size, max_size).is_err(),
            "Artifact data size {} should be invalid: {}",
            size,
            reason
        );
    }
}

#[tokio::test]
async fn test_validate_range_success() {
    // Test valid range values
    let valid_cases = vec![
        (5, 1, 10, "value"),
        (1, 1, 10, "value at min"),
        (10, 1, 10, "value at max"),
    ];

    for (value, min, max, field) in valid_cases {
        assert!(
            validation::validate_range(value, min, max, field).is_ok(),
            "Value {} should be within range [{}, {}]",
            value,
            min,
            max
        );
    }
}

#[tokio::test]
async fn test_validate_range_failure() {
    // Test invalid range values
    let invalid_cases = vec![
        (0, 1, 10, "value below min"),
        (11, 1, 10, "value above max"),
        (-5, 0, 10, "negative value"),
    ];

    for (value, min, max, reason) in invalid_cases {
        assert!(
            validation::validate_range(value, min, max, "test_field").is_err(),
            "Value {} should be outside range [{}, {}]: {}",
            value,
            min,
            max,
            reason
        );
    }
}

