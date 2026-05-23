//! TLS policy and rustls server configuration (PH-S08 / FM-044).
//!
//! - Policy: [`TlsConfig`] from [`HttpsConfig`](crate::core::config::HttpsConfig) (min/max TLS, HSTS).
//! - Runtime (`feature = "https"`): PEM load, TLS 1.3-first rustls [`ServerConfig`], hot reload via
//!   [`RustlsConfig::reload_from_config`](axum_server::tls_rustls::RustlsConfig::reload_from_config).

use crate::core::config::HttpsConfig;
use crate::core::error::AppError;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
/// TLS version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TlsVersion {
    /// TLS 1.2 (optional backward compatibility when `min_version` allows it)
    Tls1_2,
    /// TLS 1.3 (recommended default)
    Tls1_3,
    /// TLS 2.0 (reserved — not available in rustls yet)
    Tls2_0,
}

impl FromStr for TlsVersion {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1.2" => Ok(TlsVersion::Tls1_2),
            "1.3" => Ok(TlsVersion::Tls1_3),
            "2.0" => Ok(TlsVersion::Tls2_0),
            _ => Err(AppError::ConfigError(format!(
                "Unsupported TLS version: {s}. Supported: 1.2, 1.3, 2.0 (2.0 not yet in rustls)"
            ))),
        }
    }
}

impl fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsVersion::Tls1_2 => f.write_str("1.2"),
            TlsVersion::Tls1_3 => f.write_str("1.3"),
            TlsVersion::Tls2_0 => f.write_str("2.0"),
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
    /// Enable certificate transparency (policy flag; CT logs are operator-managed)
    pub certificate_transparency: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            min_version: TlsVersion::Tls1_3,
            max_version: TlsVersion::Tls1_3,
            hsts_enabled: true,
            hsts_max_age: 31536000,
            hsts_include_subdomains: true,
            certificate_transparency: true,
        }
    }
}

impl TlsConfig {
    /// Reserved for TLS 2.0 when rustls supports it.
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

    pub fn is_tls_2_0(&self) -> bool {
        matches!(self.max_version, TlsVersion::Tls2_0)
            || matches!(self.min_version, TlsVersion::Tls2_0)
    }

    /// Build policy from application [`HttpsConfig`].
    pub fn from_https_config(https: &HttpsConfig) -> Result<Self, AppError> {
        let min = parse_tls_version_field(
            https
                .tls_min_version
                .as_deref()
                .or(https.tls_version.as_deref()),
            TlsVersion::Tls1_3,
        )?;
        let max = parse_tls_version_field(
            https
                .tls_max_version
                .as_deref()
                .or(https.tls_version.as_deref()),
            TlsVersion::Tls1_3,
        )?;
        if min > max {
            return Err(AppError::ConfigError(format!(
                "https.tls_min_version ({min}) must be <= tls_max_version ({max})"
            )));
        }
        if min == TlsVersion::Tls2_0 || max == TlsVersion::Tls2_0 {
            return Err(AppError::ConfigError(
                "TLS 2.0 is not available in rustls yet; use min/max 1.2 or 1.3".into(),
            ));
        }
        Ok(Self {
            min_version: min,
            max_version: max,
            hsts_enabled: https.hsts_enabled.unwrap_or(true),
            hsts_max_age: https.hsts_max_age.unwrap_or(31536000),
            hsts_include_subdomains: true,
            certificate_transparency: true,
        })
    }

    /// Get HSTS header value for `Strict-Transport-Security`.
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

    /// Resolve certificate and key paths (config file, then env, then defaults).
    pub fn resolve_cert_paths(https: &HttpsConfig) -> CertificatePaths {
        CertificatePaths {
            cert: https
                .cert_path
                .clone()
                .or_else(|| std::env::var("HTTPS_CERT_PATH").ok())
                .unwrap_or_else(|| "certs/cert.pem".to_string()),
            key: https
                .key_path
                .clone()
                .or_else(|| std::env::var("HTTPS_KEY_PATH").ok())
                .unwrap_or_else(|| "certs/key.pem".to_string()),
        }
    }
}

fn parse_tls_version_field(
    value: Option<&str>,
    default: TlsVersion,
) -> Result<TlsVersion, AppError> {
    match value {
        Some(s) => TlsVersion::from_str(s),
        None => Ok(default),
    }
}

/// PEM file locations for the HTTPS listener.
#[derive(Debug, Clone)]
pub struct CertificatePaths {
    pub cert: String,
    pub key: String,
}

impl CertificatePaths {
    pub fn cert_path(&self) -> &Path {
        Path::new(&self.cert)
    }

    pub fn key_path(&self) -> &Path {
        Path::new(&self.key)
    }
}

#[cfg(feature = "https")]
mod rustls_impl {
    use super::{CertificatePaths, TlsConfig, TlsVersion};
    use crate::core::error::AppError;
    use axum_server::tls_rustls::RustlsConfig;
    use rustls::pki_types::pem::PemObject;
    use rustls::pki_types::{CertificateDer, PrivateKeyDer};
    use rustls::version::{TLS12, TLS13};
    use rustls::ServerConfig;
    use std::io;
    use std::sync::Arc;
    use std::time::Duration;
    use tracing::{info, warn};

    /// Live TLS listener state (reloadable certificates).
    #[derive(Clone)]
    pub struct TlsServeContext {
        pub rustls: RustlsConfig,
        pub policy: TlsConfig,
        pub paths: CertificatePaths,
    }

    impl TlsServeContext {
        pub async fn from_pem_files(
            paths: CertificatePaths,
            policy: TlsConfig,
        ) -> Result<Self, AppError> {
            let server = load_server_config_from_pem_files(&paths, &policy).await?;
            Ok(Self {
                rustls: RustlsConfig::from_config(server),
                policy,
                paths,
            })
        }

        /// Reload PEM files from disk into the active rustls config (cert rotation).
        pub async fn reload_certificates(&self) -> Result<(), AppError> {
            let server = load_server_config_from_pem_files(&self.paths, &self.policy).await?;
            self.rustls.reload_from_config(server);
            info!(
                cert = %self.paths.cert,
                key = %self.paths.key,
                "TLS certificates reloaded"
            );
            Ok(())
        }
    }

    /// Background task: periodic certificate reload (`HTTPS_CERT_RELOAD_SECS`).
    pub async fn cert_reload_loop(ctx: TlsServeContext, interval_secs: u64) {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        interval.tick().await; // skip immediate tick after startup
        loop {
            interval.tick().await;
            if let Err(e) = ctx.reload_certificates().await {
                warn!(
                    error = %e,
                    "TLS certificate reload failed; keeping previous certificates"
                );
            }
        }
    }

    pub async fn load_server_config_from_pem_files(
        paths: &CertificatePaths,
        policy: &TlsConfig,
    ) -> Result<Arc<ServerConfig>, AppError> {
        let cert_pem = tokio::fs::read(paths.cert_path())
            .await
            .map_err(|e| pem_io_error("certificate", paths.cert.clone(), e))?;
        let key_pem = tokio::fs::read(paths.key_path())
            .await
            .map_err(|e| pem_io_error("private key", paths.key.clone(), e))?;

        let policy = policy.clone();
        tokio::task::spawn_blocking(move || {
            build_server_config_from_pem(&cert_pem, &key_pem, &policy)
        })
        .await
        .map_err(|e| AppError::ConfigError(format!("TLS build task join: {e}")))?
    }

    fn pem_io_error(kind: &str, path: String, err: io::Error) -> AppError {
        AppError::ConfigError(format!("Failed to read TLS {kind} at {path}: {err}"))
    }

    fn ensure_rustls_crypto_provider() {
        static INSTALLED: std::sync::Once = std::sync::Once::new();
        INSTALLED.call_once(|| {
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("rustls ring crypto provider");
        });
    }

    fn build_server_config_from_pem(
        cert_pem: &[u8],
        key_pem: &[u8],
        policy: &TlsConfig,
    ) -> Result<Arc<ServerConfig>, AppError> {
        ensure_rustls_crypto_provider();
        let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_slice_iter(cert_pem)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| AppError::ConfigError("failed to parse TLS certificate PEM".into()))?;

        let mut key_result: Result<PrivateKeyDer<'static>, AppError> = Err(AppError::ConfigError(
            "TLS private key PEM contained no key".into(),
        ));

        for item in PrivateKeyDer::pem_slice_iter(key_pem) {
            let key = item
                .map_err(|_| AppError::ConfigError("failed to parse TLS private key PEM".into()))?;
            if key_result.is_ok() {
                return Err(AppError::ConfigError(
                    "TLS private key PEM must contain exactly one key".into(),
                ));
            }
            key_result = Ok(key);
        }
        let key = key_result?;

        let versions = rustls_protocol_versions(policy)?;
        let mut config = ServerConfig::builder_with_protocol_versions(&versions)
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| AppError::ConfigError(format!("invalid TLS certificate/key: {e}")))?;

        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        Ok(Arc::new(config))
    }

    fn rustls_protocol_versions(
        policy: &TlsConfig,
    ) -> Result<Vec<&'static rustls::SupportedProtocolVersion>, AppError> {
        if policy.is_tls_2_0() {
            return Err(AppError::ConfigError(
                "TLS 2.0 is not supported by rustls yet".into(),
            ));
        }
        let mut versions: Vec<&'static rustls::SupportedProtocolVersion> = Vec::new();
        if policy.max_version >= TlsVersion::Tls1_3 {
            versions.push(&TLS13);
        }
        if policy.min_version <= TlsVersion::Tls1_2 && policy.max_version >= TlsVersion::Tls1_2 {
            versions.push(&TLS12);
        }
        if versions.is_empty() {
            versions.push(&TLS13);
        }
        Ok(versions)
    }

    pub fn spawn_cert_reload_if_configured(ctx: TlsServeContext) {
        let Ok(raw) = std::env::var("HTTPS_CERT_RELOAD_SECS") else {
            return;
        };
        let Ok(secs) = raw.parse::<u64>() else {
            warn!(
                HTTPS_CERT_RELOAD_SECS = %raw,
                "invalid HTTPS_CERT_RELOAD_SECS; expected positive integer"
            );
            return;
        };
        if secs == 0 {
            return;
        }
        info!(interval_secs = secs, "TLS certificate auto-reload enabled");
        tokio::spawn(cert_reload_loop(ctx, secs));
    }
}

#[cfg(feature = "https")]
pub use rustls_impl::{
    cert_reload_loop, load_server_config_from_pem_files, spawn_cert_reload_if_configured,
    TlsServeContext,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::HttpsConfig;

    #[test]
    fn parses_tls_versions() {
        assert_eq!("1.2".parse::<TlsVersion>().unwrap(), TlsVersion::Tls1_2);
        assert_eq!("1.3".parse::<TlsVersion>().unwrap(), TlsVersion::Tls1_3);
    }

    #[test]
    fn from_https_defaults_to_tls13() {
        let https = HttpsConfig::default();
        let tls = TlsConfig::from_https_config(&https).unwrap();
        assert_eq!(tls.min_version, TlsVersion::Tls1_3);
        assert_eq!(tls.max_version, TlsVersion::Tls1_3);
        assert!(tls.hsts_enabled);
    }

    #[test]
    fn from_https_allows_tls12_when_configured() {
        let https = HttpsConfig {
            tls_min_version: Some("1.2".into()),
            tls_max_version: Some("1.3".into()),
            ..HttpsConfig::default()
        };
        let tls = TlsConfig::from_https_config(&https).unwrap();
        assert_eq!(tls.min_version, TlsVersion::Tls1_2);
        assert_eq!(tls.max_version, TlsVersion::Tls1_3);
    }

    #[test]
    fn rejects_tls2_in_https_config() {
        let https = HttpsConfig {
            tls_max_version: Some("2.0".into()),
            ..HttpsConfig::default()
        };
        assert!(TlsConfig::from_https_config(&https).is_err());
    }

    #[test]
    fn hsts_header_omitted_when_disabled() {
        let tls = TlsConfig {
            hsts_enabled: false,
            ..TlsConfig::default()
        };
        assert!(tls.hsts_header().is_none());
    }

    #[cfg(feature = "https")]
    #[tokio::test]
    async fn loads_dev_pem_with_tls13_policy() {
        let paths = CertificatePaths {
            cert: "certs/cert.pem".into(),
            key: "certs/key.pem".into(),
        };
        if !paths.cert_path().exists() {
            return;
        }
        let policy = TlsConfig::default();
        let cfg = load_server_config_from_pem_files(&paths, &policy)
            .await
            .expect("dev certs");
        assert!(!cfg.alpn_protocols.is_empty());
    }
}
