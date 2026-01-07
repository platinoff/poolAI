//! Validation utilities for API endpoints
//!
//! Provides comprehensive validation functions for all write operations
//! following Rust best practices: type safety, error handling, and clear error messages.

use crate::core::error::AppError;

/// Validate worker ID format
///
/// Rules:
/// - Must be 1-64 characters
/// - Can contain alphanumeric characters, hyphens, and underscores
/// - Must start with alphanumeric character
pub fn validate_worker_id(worker_id: &str) -> Result<(), AppError> {
    if worker_id.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Worker ID cannot be empty. Suggestion: Provide a non-empty worker ID (1-64 characters, alphanumeric with hyphens/underscores).".to_string(),
        ));
    }

    if worker_id.len() > 64 {
        return Err(AppError::ValidationError(format!(
            "Worker ID must be 64 characters or less (got {}). Suggestion: Shorten the worker ID to 64 characters or less.",
            worker_id.len()
        )));
    }

    if !worker_id
        .chars()
        .next()
        .map(|c| c.is_alphanumeric())
        .unwrap_or(false)
    {
        return Err(AppError::ValidationError(format!(
            "Worker ID must start with an alphanumeric character (got '{}'). Suggestion: Start the worker ID with a letter or number.",
            worker_id.chars().next().unwrap_or('?')
        )));
    }

    if !worker_id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        let invalid_chars: Vec<char> = worker_id
            .chars()
            .filter(|c| !c.is_alphanumeric() && *c != '-' && *c != '_')
            .collect();
        return Err(AppError::ValidationError(format!(
            "Worker ID can only contain alphanumeric characters, hyphens, and underscores (found invalid characters: {:?}). Suggestion: Remove or replace invalid characters.",
            invalid_chars
        )));
    }

    Ok(())
}

/// Validate artifact name format
///
/// Rules:
/// - Must be 1-255 characters
/// - Can contain alphanumeric characters, hyphens, underscores, dots
/// - Must start with alphanumeric character
pub fn validate_artifact_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Artifact name cannot be empty. Suggestion: Provide a non-empty artifact name (1-255 characters, alphanumeric with hyphens/underscores/dots).".to_string(),
        ));
    }

    if name.len() > 255 {
        return Err(AppError::ValidationError(format!(
            "Artifact name must be 255 characters or less (got {}). Suggestion: Shorten the artifact name to 255 characters or less.",
            name.len()
        )));
    }

    if !name
        .chars()
        .next()
        .map(|c| c.is_alphanumeric())
        .unwrap_or(false)
    {
        return Err(AppError::ValidationError(format!(
            "Artifact name must start with an alphanumeric character (got '{}'). Suggestion: Start the artifact name with a letter or number.",
            name.chars().next().unwrap_or('?')
        )));
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        let invalid_chars: Vec<char> = name
            .chars()
            .filter(|c| !c.is_alphanumeric() && *c != '-' && *c != '_' && *c != '.')
            .collect();
        return Err(AppError::ValidationError(format!(
            "Artifact name can only contain alphanumeric characters, hyphens, underscores, and dots (found invalid characters: {:?}). Suggestion: Remove or replace invalid characters.",
            invalid_chars
        )));
    }

    Ok(())
}

/// Validate numeric value is within range
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    field_name: &str,
) -> Result<(), AppError> {
    if value < min || value > max {
        return Err(AppError::ValidationError(format!(
            "{} must be between {} and {} (got {}). Suggestion: Adjust the value to be within the valid range.",
            field_name, min, max, value
        )));
    }
    Ok(())
}

/// Validate base64 data size
///
/// Rules:
/// - Must not be empty
/// - Must be within size limit (default: 100MB)
pub fn validate_base64_data(data: &str, max_size_bytes: usize) -> Result<(), AppError> {
    if data.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Base64 data cannot be empty. Suggestion: Provide valid base64-encoded data.".to_string(),
        ));
    }

    // Base64 encoding increases size by ~33%, so we check decoded size
    // Approximate check: base64 string length * 3/4
    let estimated_size = (data.len() * 3) / 4;
    if estimated_size > max_size_bytes {
        return Err(AppError::ConfigError(format!(
            "Artifact data size ({}) exceeds maximum allowed size ({})",
            estimated_size, max_size_bytes
        )));
    }

    Ok(())
}

/// Validate UUID format
pub fn validate_uuid(uuid_str: &str) -> Result<(), AppError> {
    use uuid::Uuid;
    Uuid::parse_str(uuid_str)
        .map(|_| ())
        .map_err(|e| AppError::ConfigError(format!("Invalid UUID format: {}", e)))
}

/// Validate worker configuration values
pub fn validate_worker_config(
    max_concurrent_requests: usize,
    request_timeout_ms: u64,
    health_check_interval_ms: u64,
    cache_size: usize,
    max_memory_mb: usize,
    cpu_priority: u8,
) -> Result<(), AppError> {
    validate_range(max_concurrent_requests, 1, 1000, "max_concurrent_requests")?;
    validate_range(request_timeout_ms, 100, 300_000, "request_timeout_ms")?; // 100ms to 5min
    validate_range(
        health_check_interval_ms,
        100,
        60_000,
        "health_check_interval_ms",
    )?; // 100ms to 1min
    validate_range(cache_size, 100, 100_000, "cache_size")?;
    validate_range(max_memory_mb, 256, 131_072, "max_memory_mb")?; // 256MB to 128GB
    validate_range(cpu_priority, 1, 10, "cpu_priority")?;

    Ok(())
}

/// Validate artifact data size
pub fn validate_artifact_data_size(
    data_size: usize,
    max_size_bytes: usize,
) -> Result<(), AppError> {
    if data_size == 0 {
        return Err(AppError::ConfigError(
            "Artifact data cannot be empty".to_string(),
        ));
    }

    if data_size > max_size_bytes {
        return Err(AppError::ConfigError(format!(
            "Artifact data size ({}) exceeds maximum allowed size ({})",
            data_size, max_size_bytes
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_worker_id_success() {
        let long_id = "a".repeat(64);
        let valid_ids = vec![
            "worker1",
            "worker-123",
            "worker_456",
            "w1",
            long_id.as_str(),
        ];

        for id in valid_ids {
            assert!(
                validate_worker_id(id).is_ok(),
                "Worker ID '{}' should be valid",
                id
            );
        }
    }

    #[test]
    fn test_validate_worker_id_failure() {
        let too_long = "a".repeat(65);
        let invalid_cases = vec![
            ("", "empty"),
            (" ", "whitespace only"),
            ("-invalid", "starts with hyphen"),
            ("_invalid", "starts with underscore"),
            ("invalid@id", "contains invalid character"),
            ("invalid id", "contains space"),
            (too_long.as_str(), "too long"),
        ];

        for (id, reason) in invalid_cases {
            assert!(
                validate_worker_id(id).is_err(),
                "Worker ID '{}' should be invalid: {}",
                id,
                reason
            );
        }
    }

    #[test]
    fn test_validate_artifact_name_success() {
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
                validate_artifact_name(name).is_ok(),
                "Artifact name '{}' should be valid",
                name
            );
        }
    }

    #[test]
    fn test_validate_artifact_name_failure() {
        let too_long = "a".repeat(256);
        let invalid_cases = vec![
            ("", "empty"),
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
                validate_artifact_name(name).is_err(),
                "Artifact name '{}' should be invalid: {}",
                name,
                reason
            );
        }
    }

    #[test]
    fn test_validate_range_success() {
        assert!(validate_range(5, 1, 10, "value").is_ok());
        assert!(validate_range(1, 1, 10, "value").is_ok());
        assert!(validate_range(10, 1, 10, "value").is_ok());
        assert!(validate_range(5.5, 1.0, 10.0, "value").is_ok());
    }

    #[test]
    fn test_validate_range_failure() {
        assert!(validate_range(0, 1, 10, "value").is_err());
        assert!(validate_range(11, 1, 10, "value").is_err());
        assert!(validate_range(0.5, 1.0, 10.0, "value").is_err());
        assert!(validate_range(10.5, 1.0, 10.0, "value").is_err());
    }

    #[test]
    fn test_validate_base64_data_success() {
        let valid_data = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
        assert!(validate_base64_data(valid_data, 1000).is_ok());
        assert!(validate_base64_data("dGVzdA==", 1000).is_ok()); // "test" in base64
    }

    #[test]
    fn test_validate_base64_data_failure() {
        assert!(validate_base64_data("", 1000).is_err());
        assert!(validate_base64_data(" ", 1000).is_err());
        // Test with data that exceeds size limit
        let large_data = "A".repeat(1_000_000);
        assert!(validate_base64_data(&large_data, 1000).is_err());
    }

    #[test]
    fn test_validate_uuid_success() {
        let valid_uuids = vec![
            "550e8400-e29b-41d4-a716-446655440000",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            "00000000-0000-0000-0000-000000000000",
        ];

        for uuid_str in valid_uuids {
            assert!(
                validate_uuid(uuid_str).is_ok(),
                "UUID '{}' should be valid",
                uuid_str
            );
        }
    }

    #[test]
    fn test_validate_uuid_failure() {
        let invalid_uuids = vec![
            "",
            "not-a-uuid",
            "550e8400-e29b-41d4-a716",
            "550e8400-e29b-41d4-a716-446655440000-extra",
        ];

        for uuid_str in invalid_uuids {
            assert!(
                validate_uuid(uuid_str).is_err(),
                "UUID '{}' should be invalid",
                uuid_str
            );
        }
    }

    #[test]
    fn test_validate_worker_config_success() {
        assert!(validate_worker_config(
            100,      // max_concurrent_requests
            5000,     // request_timeout_ms
            1000,     // health_check_interval_ms
            1000,     // cache_size
            4096,     // max_memory_mb
            5         // cpu_priority
        )
        .is_ok());
    }

    #[test]
    fn test_validate_worker_config_failure() {
        // Test each invalid parameter
        assert!(validate_worker_config(0, 5000, 1000, 1000, 4096, 5).is_err()); // max_concurrent_requests too low
        assert!(validate_worker_config(1001, 5000, 1000, 1000, 4096, 5).is_err()); // max_concurrent_requests too high
        assert!(validate_worker_config(100, 50, 1000, 1000, 4096, 5).is_err()); // request_timeout_ms too low
        assert!(validate_worker_config(100, 5000, 50, 1000, 4096, 5).is_err()); // health_check_interval_ms too low
        assert!(validate_worker_config(100, 5000, 1000, 50, 4096, 5).is_err()); // cache_size too low
        assert!(validate_worker_config(100, 5000, 1000, 1000, 128, 5).is_err()); // max_memory_mb too low
        assert!(validate_worker_config(100, 5000, 1000, 1000, 4096, 0).is_err()); // cpu_priority too low
        assert!(validate_worker_config(100, 5000, 1000, 1000, 4096, 11).is_err()); // cpu_priority too high
    }

    #[test]
    fn test_validate_artifact_data_size_success() {
        assert!(validate_artifact_data_size(100, 1000).is_ok());
        assert!(validate_artifact_data_size(1000, 1000).is_ok());
        assert!(validate_artifact_data_size(1, 1000).is_ok());
    }

    #[test]
    fn test_validate_artifact_data_size_failure() {
        assert!(validate_artifact_data_size(0, 1000).is_err());
        assert!(validate_artifact_data_size(1001, 1000).is_err());
    }
}
