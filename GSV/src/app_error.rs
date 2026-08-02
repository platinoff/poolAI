//! GSV `AppError` — shared error type across the server and boxes.
//!
//! Mirrors the PoolAI canon (`src/core/error.rs`): `Display + std::error::Error`,
//! `?`-friendly `From` conversions, no `unwrap()`/`expect()` in product code.

use std::fmt;

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Application-level error carrying a human-readable message.
#[derive(Debug, Clone)]
pub struct AppError {
    msg: String,
}

impl AppError {
    /// Build a new `AppError` from a message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }

    /// Borrow the underlying message.
    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::new(format!("io: {e}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(format!("json: {e}"))
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        Self::new(format!("toml: {e}"))
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = json!({
            "error": {
                "code": "GSV_ERROR",
                "message": self.message(),
            }
        });
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_display_and_message() {
        let err = AppError::new("boom");
        assert_eq!(err.message(), "boom");
        assert_eq!(err.to_string(), "boom");
    }

    #[test]
    fn app_error_from_string_and_str() {
        let a: AppError = "static".into();
        assert_eq!(a.message(), "static");
        let b: AppError = String::from("owned").into();
        assert_eq!(b.message(), "owned");
    }
}
