use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use log::{info, error};
use thiserror::Error;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::fs;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;

#[derive(Error, Debug)]
pub enum TlsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TLS error: {0}")]
    TlsError(String),
    #[error("Certificate error: {0}")]
    CertError(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone)]
pub struct TLSConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_path: Option<PathBuf>,
    pub enabled: bool,
}

pub struct TLSManager {
    config: Arc<Mutex<TLSConfig>>,
    server_config: Arc<Mutex<Option<ServerConfig>>>,
}

impl TLSManager {
    pub fn new(config: TLSConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            server_config: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn load_certificates(&self) -> Result<(), TlsError> {
        let config = self.config.lock().await;
        
        if !config.enabled {
            return Ok(());
        }

        // Load certificate
        let cert_file = fs::read(&config.cert_path)
            .map_err(|e| TlsError::CertError(format!("Failed to read certificate: {}", e)))?;
        
        let certs = certs(&mut &cert_file[..])
            .map_err(|e| TlsError::CertError(format!("Failed to parse certificate: {}", e)))?
            .into_iter()
            .map(Certificate)
            .collect();

        // Load private key
        let key_file = fs::read(&config.key_path)
            .map_err(|e| TlsError::CertError(format!("Failed to read private key: {}", e)))?;
        
        let keys = pkcs8_private_keys(&mut &key_file[..])
            .map_err(|e| TlsError::CertError(format!("Failed to parse private key: {}", e)))?;

        let key = keys
            .first()
            .ok_or_else(|| TlsError::CertError("No private key found".to_string()))?;

        // Create server config
        let mut server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, PrivateKey(key.clone()))
            .map_err(|e| TlsError::TlsError(e.to_string()))?;

        // Load CA certificate if provided
        if let Some(ca_path) = &config.ca_path {
            let ca_file = fs::read(ca_path)
                .map_err(|e| TlsError::CertError(format!("Failed to read CA certificate: {}", e)))?;
            
            let ca_certs = certs(&mut &ca_file[..])
                .map_err(|e| TlsError::CertError(format!("Failed to parse CA certificate: {}", e)))?;

            server_config
                .root_store
                .add_server_trust_anchors(ca_certs.into_iter().map(|cert| {
                    rustls::OwnedTrustAnchor::from_cert(Certificate(cert))
                }));
        }

        // Update server config
        let mut current_config = self.server_config.lock().await;
        *current_config = Some(server_config);

        info!("TLS certificates loaded successfully");
        Ok(())
    }

    pub async fn get_server_config(&self) -> Option<ServerConfig> {
        let config = self.server_config.lock().await;
        config.clone()
    }

    pub async fn update_config(&self, new_config: TLSConfig) -> Result<(), TlsError> {
        let mut config = self.config.lock().await;
        *config = new_config;

        if config.enabled {
            self.load_certificates().await?;
        } else {
            let mut server_config = self.server_config.lock().await;
            *server_config = None;
        }

        info!("TLS configuration updated");
        Ok(())
    }

    pub async fn is_enabled(&self) -> bool {
        let config = self.config.lock().await;
        config.enabled
    }

    pub async fn validate_certificates(&self) -> Result<(), TlsError> {
        let config = self.config.lock().await;
        
        if !config.enabled {
            return Ok(());
        }

        // Check certificate file
        if !config.cert_path.exists() {
            return Err(TlsError::CertError("Certificate file not found".to_string()));
        }

        // Check private key file
        if !config.key_path.exists() {
            return Err(TlsError::CertError("Private key file not found".to_string()));
        }

        // Check CA certificate if provided
        if let Some(ca_path) = &config.ca_path {
            if !ca_path.exists() {
                return Err(TlsError::CertError("CA certificate file not found".to_string()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_tls_manager_creation() {
        let cert_path = PathBuf::from("test_cert.pem");
        let key_path = PathBuf::from("test_key.pem");
        
        // Create test certificates
        fs::write(&cert_path, "-----BEGIN CERTIFICATE-----\nMIIB...\n-----END CERTIFICATE-----").unwrap();
        fs::write(&key_path, "-----BEGIN PRIVATE KEY-----\nMIIB...\n-----END PRIVATE KEY-----").unwrap();

        let result = TLSManager::new(TLSConfig {
            cert_path: cert_path.clone(),
            key_path: key_path.clone(),
            ca_path: None,
            enabled: true,
        });
        assert!(result.is_err()); // Should fail with invalid certificates

        // Cleanup
        fs::remove_file(cert_path).unwrap();
        fs::remove_file(key_path).unwrap();
    }

    #[test]
    fn test_certificate_reload() {
        let cert_path = PathBuf::from("test_cert.pem");
        let key_path = PathBuf::from("test_key.pem");
        
        // Create test certificates
        fs::write(&cert_path, "-----BEGIN CERTIFICATE-----\nMIIB...\n-----END CERTIFICATE-----").unwrap();
        fs::write(&key_path, "-----BEGIN PRIVATE KEY-----\nMIIB...\n-----END PRIVATE KEY-----").unwrap();

        let mut manager = TLSManager::new(TLSConfig {
            cert_path: cert_path.clone(),
            key_path: key_path.clone(),
            ca_path: None,
            enabled: true,
        });
        
        // Try to reload too quickly
        let result = manager.update_config(TLSConfig {
            cert_path: cert_path.clone(),
            key_path: key_path.clone(),
            ca_path: None,
            enabled: true,
        });
        assert!(result.is_err());

        // Cleanup
        fs::remove_file(cert_path).unwrap();
        fs::remove_file(key_path).unwrap();
    }
} 