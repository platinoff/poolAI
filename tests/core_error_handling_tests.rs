//! Core Error Handling Tests
//!
//! Tests for error handling, error types, and error propagation.

use poolai::core::error::{AppError, ErrorKind};

#[test]
fn test_error_kind_variants() {
    // Test that all error kinds exist
    let _ = ErrorKind::NotFound;
    let _ = ErrorKind::InvalidInput;
    let _ = ErrorKind::PermissionDenied;
    let _ = ErrorKind::InternalError;
    let _ = ErrorKind::NetworkError;
    let _ = ErrorKind::StorageError;
    let _ = ErrorKind::ConfigurationError;
    let _ = ErrorKind::Timeout;
    let _ = ErrorKind::ResourceExhausted;
    let _ = ErrorKind::Unavailable;
}

#[test]
fn test_app_error_from_string() {
    let err = AppError::new(ErrorKind::InvalidInput, "test error");
    assert_eq!(err.message(), "test error");
    assert_eq!(err.kind(), &ErrorKind::InvalidInput);
}

#[test]
fn test_app_error_display() {
    let err = AppError::new(ErrorKind::NotFound, "Resource not found");
    let display = format!("{}", err);
    assert!(display.contains("Resource not found"));
}

#[test]
fn test_app_error_debug() {
    let err = AppError::new(ErrorKind::InternalError, "Internal error");
    let debug = format!("{:?}", err);
    assert!(debug.contains("InternalError"));
    assert!(debug.contains("Internal error"));
}

#[test]
fn test_error_kind_display() {
    let kind = ErrorKind::PermissionDenied;
    let display = format!("{}", kind);
    assert!(!display.is_empty());
}

#[test]
fn test_error_kind_debug() {
    let kind = ErrorKind::NetworkError;
    let debug = format!("{:?}", kind);
    assert!(debug.contains("NetworkError"));
}

#[test]
fn test_error_with_context() {
    let err =
        AppError::new(ErrorKind::InvalidInput, "Invalid input").with_context("field", "username");
    assert_eq!(err.message(), "Invalid input");
}

#[test]
fn test_error_chain() {
    let err1 = AppError::new(ErrorKind::StorageError, "Storage error");
    let err2 = AppError::new(ErrorKind::InternalError, "Internal error").with_cause(err1);
    assert_eq!(err2.message(), "Internal error");
}

#[test]
fn test_error_kind_equality() {
    let kind1 = ErrorKind::NotFound;
    let kind2 = ErrorKind::NotFound;
    assert_eq!(kind1, kind2);
}

#[test]
fn test_error_kind_inequality() {
    let kind1 = ErrorKind::NotFound;
    let kind2 = ErrorKind::InvalidInput;
    assert_ne!(kind1, kind2);
}

#[test]
fn test_error_from_io_error() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let app_err: AppError = io_err.into();
    assert_eq!(app_err.kind(), &ErrorKind::NotFound);
}

#[test]
fn test_error_from_string_error() {
    let str_err = "String error".to_string();
    let app_err = AppError::from(str_err);
    assert_eq!(app_err.message(), "String error");
}

#[test]
fn test_error_from_static_str() {
    let app_err = AppError::from("Static error");
    assert_eq!(app_err.message(), "Static error");
}

#[test]
fn test_error_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<AppError>();
    assert_sync::<AppError>();
}
