//! TLS Configuration Module
//!
//! Provides abstraction for TLS configuration supporting TLS 1.3 and TLS 2.0.
//! Prepared for TLS 2.0 when it becomes available.

use crate::core::error::AppError;

/// TLS version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    /// TLS 1.3 (current)
    Tls1_3,
    /// TLS 2.0 (target, when available)
    Tls2_0,
}

impl TlsVersion {
    /// Parse TLS version from string
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        match s {
            "1.3" => Ok(TlsVersion::Tls1_3),
            "2.0" => Ok(TlsVersion::Tls2_0),
            _ => Err(AppError::ConfigError(format!(
                "Unsupported TLS version: {}. Supported: 1.3, 2.0",
                s
            ))),
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            TlsVersion::Tls1_3 => "1.3".to_string(),
            TlsVersion::Tls2_0 => "2.0".to_string(),
        }
    }
}

/// TLS configuration structure
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Minimum TLS version
    pub min_version: TlsVersion,
    /// Maximum TLS version (target: TLS 2.0)
    pub max_version: TlsVersion,
    /// Enable HSTS
    pub hsts_enabled: bool,
    /// HSTS max age in seconds
    pub hsts_max_age: u64,
    /// HSTS include subdomains
    pub hsts_include_subdomains: bool,
    /// Enable certificate transparency
    pub certificate_transparency: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            min_version: TlsVersion::Tls1_3,
            max_version: TlsVersion::Tls1_3, // Will be updated to Tls2_0 when available
            hsts_enabled: true,
            hsts_max_age: 31536000, // 1 year
            hsts_include_subdomains: true,
            certificate_transparency: true,
        }
    }
}

impl TlsConfig {
    /// Create TLS configuration for TLS 2.0 (when available)
    /// Note: TLS 2.0 feature flag will be added when TLS 2.0 becomes available
    pub fn for_tls_2_0() -> Self {
        Self {
            min_version: TlsVersion::Tls2_0,
            max_version: TlsVersion::Tls2_0,
            hsts_enabled: true,
            hsts_max_age: 31536000,
            hsts_include_subdomains: true,
            certificate_transparency: true,
        }
    }

    /// Create TLS configuration with custom settings
    pub fn new(min_version: TlsVersion, max_version: TlsVersion, hsts_enabled: bool) -> Self {
        Self {
            min_version,
            max_version,
            hsts_enabled,
            hsts_max_age: 31536000,
            hsts_include_subdomains: true,
            certificate_transparency: true,
        }
    }

    /// Check if TLS 2.0 is enabled
    pub fn is_tls_2_0(&self) -> bool {
        matches!(self.max_version, TlsVersion::Tls2_0)
    }

    /// Get HSTS header value
    pub fn hsts_header(&self) -> Option<String> {
        if !self.hsts_enabled {
            return None;
        }

        let mut header = format!("max-age={}", self.hsts_max_age);
        if self.hsts_include_subdomains {
            header.push_str("; includeSubDomains");
        }
        Some(header)
    }
}
