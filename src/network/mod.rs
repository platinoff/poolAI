//! Network module for REST API and WebSocket server
//!
//! This module provides:
//! - REST API endpoints (67+ endpoints)
//! - WebSocket connections for real-time updates
//! - Authentication and authorization (JWT, RBAC)
//! - HTTPS/TLS support with certificate management

pub mod api;
pub mod api_legacy;
pub mod auth;
pub mod raid_distributed_handlers;
pub mod security_headers;
pub mod tls_config;
pub mod validation;
pub mod ws;

#[cfg(feature = "enterprise")]
pub mod enterprise_api;

use crate::ui;
use axum::middleware;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tracing::info;

#[cfg(feature = "https")]
use axum_server::tls_rustls::RustlsConfig;

/// Start the network server (HTTP or HTTPS)
///
/// # Arguments
/// * `addr` - Socket address to bind the server to
///
/// # Note
/// HTTPS support requires feature "https" and valid certificates.
/// Configuration is read from PoolAIConfig.
pub async fn start_server(addr: SocketAddr) {
    let app = {
        let router = Router::new()
            // Trailing-slash compat for UI entrypoint.
            .route("/ui/", get(|| async { Redirect::permanent("/ui") }))
            .nest("/api/v1", api::create_api_routes())
            .nest("/ui", ui::create_ui_routes())
            // Add security headers middleware to all responses
            .layer(middleware::from_fn(
                security_headers::security_headers_middleware,
            ));

        // Add enterprise API routes if feature is enabled
        #[cfg(feature = "enterprise")]
        {
            router.nest(
                "/api/enterprise",
                enterprise_api::create_enterprise_api_routes(),
            )
        }
        #[cfg(not(feature = "enterprise"))]
        {
            router
        }
    };

    // Read HTTPS configuration from config file
    // HTTPS support is optional and requires feature "https"
    // For production, use: cargo build --features https
    // Note: Requires native toolchain (gcc/dlltool on Windows GNU)

    #[cfg(feature = "https")]
    {
        // HTTPS mode - read configuration from config file
        use crate::core::config::get_config;
        use tracing::warn;

        let https_config = get_config()
            .map(|config| config.https.clone())
            .unwrap_or_default();

        if !https_config.enabled {
            info!("HTTPS is disabled in configuration, starting HTTP server");
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            info!("Server listening on {}", addr);
            axum::serve(listener, app).await.unwrap();
            return;
        }

        // Get certificate paths from config or environment variables
        let cert_path = https_config
            .cert_path
            .or_else(|| std::env::var("HTTPS_CERT_PATH").ok())
            .unwrap_or_else(|| "certs/cert.pem".to_string());
        let key_path = https_config
            .key_path
            .or_else(|| std::env::var("HTTPS_KEY_PATH").ok())
            .unwrap_or_else(|| "certs/key.pem".to_string());

        match RustlsConfig::from_pem_file(cert_path.clone(), key_path.clone()).await {
            Ok(config) => {
                info!("Starting HTTPS server on {}", addr);
                axum_server::bind_rustls(addr, config)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
            Err(e) => {
                warn!(
                    "Failed to load HTTPS certificates ({}): {}. Falling back to HTTP.",
                    cert_path, e
                );
                info!("Starting HTTP server on {}", addr);
                let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
                info!("Server listening on {}", addr);
                axum::serve(listener, app).await.unwrap();
            }
        }
    }

    #[cfg(not(feature = "https"))]
    {
        // HTTP mode (default)
        info!("Starting HTTP server on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        info!("Server listening on {}", addr);
        axum::serve(listener, app).await.unwrap();
    }
}
