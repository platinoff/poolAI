//! Core Error Handling Tests
//!
//! Tests for error handling, error types, and error propagation.

use poolai::core::error::AppError;

#[test]
fn test_app_error_variants() {
    // Test that all error variants exist
    let _ = AppError::ModelError("model error".to_string());
    let _ = AppError::ConfigError("config error".to_string());
    let _ = AppError::PoolError("pool error".to_string());
    let _ = AppError::MonitoringError("monitoring error".to_string());
    let _ = AppError::ResourceError("resource error".to_string());
    let _ = AppError::NetworkError("network error".to_string());
    let _ = AppError::GpuError("gpu error".to_string());
    let _ = AppError::MemoryError("memory error".to_string());
    let _ = AppError::TimeoutError("timeout error".to_string());
    let _ = AppError::ValidationError("validation error".to_string());
    let _ = AppError::InitializationError("init error".to_string());
    let _ = AppError::ShutdownError("shutdown error".to_string());
    let _ = AppError::Unknown;
}

#[test]
fn test_app_error_display() {
    let err = AppError::ConfigError("Invalid configuration".to_string());
    let display = format!("{}", err);
    assert!(display.contains("Invalid configuration"));
}

#[test]
fn test_app_error_debug() {
    let err = AppError::PoolError("Pool error".to_string());
    let debug = format!("{:?}", err);
    assert!(debug.contains("PoolError"));
    assert!(debug.contains("Pool error"));
}

#[test]
fn test_app_error_error_code() {
    let err = AppError::ConfigError("Invalid config".to_string());
    assert_eq!(err.error_code(), "CONFIG_ERROR");
    
    let err2 = AppError::NetworkError("Network error".to_string());
    assert_eq!(err2.error_code(), "NETWORK_ERROR");
    
    let err3 = AppError::TimeoutError("Timeout".to_string());
    assert_eq!(err3.error_code(), "TIMEOUT_ERROR");
}

#[test]
fn test_app_error_is_recoverable() {
    let recoverable = AppError::TimeoutError("Request timeout".to_string());
    assert!(recoverable.is_recoverable());
    
    let recoverable2 = AppError::ResourceError("Resource error".to_string());
    assert!(recoverable2.is_recoverable());
    
    let non_recoverable = AppError::ValidationError("Invalid input".to_string());
    assert!(!non_recoverable.is_recoverable());
    
    let non_recoverable2 = AppError::Unknown;
    assert!(!non_recoverable2.is_recoverable());
}

#[test]
fn test_app_error_recover() {
    let err = AppError::TimeoutError("Request timeout".to_string());
    let result = err.recover();
    assert!(result.is_ok());
}

#[test]
fn test_app_error_equality() {
    let err1 = AppError::ConfigError("test".to_string());
    let err2 = AppError::ConfigError("test".to_string());
    // AppError doesn't implement Eq, but we can check display
    assert_eq!(format!("{}", err1), format!("{}", err2));
}

#[test]
fn test_app_error_inequality() {
    let err1 = AppError::ConfigError("test1".to_string());
    let err2 = AppError::ModelError("test2".to_string());
    // Different error codes
    assert_ne!(err1.error_code(), err2.error_code());
}

#[test]
fn test_error_from_io_error() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let app_err: AppError = AppError::IoError(io_err);
    assert_eq!(app_err.error_code(), "IO_ERROR");
}

#[test]
fn test_error_from_string_error() {
    // AppError doesn't implement From<String>, but we can create it directly
    let app_err = AppError::ModelError("String error".to_string());
    let display = format!("{}", app_err);
    assert!(display.contains("String error"));
}

#[test]
fn test_error_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<AppError>();
    assert_sync::<AppError>();
}
