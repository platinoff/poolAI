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
        return Err(AppError::ConfigError(
            "Worker ID cannot be empty".to_string(),
        ));
    }

    if worker_id.len() > 64 {
        return Err(AppError::ConfigError(
            "Worker ID must be 64 characters or less".to_string(),
        ));
    }

    if !worker_id.chars().next().map(|c| c.is_alphanumeric()).unwrap_or(false) {
        return Err(AppError::ConfigError(
            "Worker ID must start with an alphanumeric character".to_string(),
        ));
    }

    if !worker_id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(AppError::ConfigError(
            "Worker ID can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        ));
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
        return Err(AppError::ConfigError(
            "Artifact name cannot be empty".to_string(),
        ));
    }

    if name.len() > 255 {
        return Err(AppError::ConfigError(
            "Artifact name must be 255 characters or less".to_string(),
        ));
    }

    if !name.chars().next().map(|c| c.is_alphanumeric()).unwrap_or(false) {
        return Err(AppError::ConfigError(
            "Artifact name must start with an alphanumeric character".to_string(),
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err(AppError::ConfigError(
            "Artifact name can only contain alphanumeric characters, hyphens, underscores, and dots"
                .to_string(),
        ));
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
        return Err(AppError::ConfigError(format!(
            "{} must be between {} and {}",
            field_name, min, max
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
        return Err(AppError::ConfigError(
            "Base64 data cannot be empty".to_string(),
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
    validate_range(health_check_interval_ms, 100, 60_000, "health_check_interval_ms")?; // 100ms to 1min
    validate_range(cache_size, 100, 100_000, "cache_size")?;
    validate_range(max_memory_mb, 256, 131_072, "max_memory_mb")?; // 256MB to 128GB
    validate_range(cpu_priority, 1, 10, "cpu_priority")?;

    Ok(())
}

/// Validate artifact data size
pub fn validate_artifact_data_size(data_size: usize, max_size_bytes: usize) -> Result<(), AppError> {
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

