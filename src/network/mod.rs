//! Network module for REST API and WebSocket server
//!
//! This module provides:
//! - REST API endpoints (67+ endpoints)
//! - WebSocket connections for real-time updates
//! - Authentication and authorization (JWT, RBAC)
//! - HTTPS/TLS support with certificate management

pub mod api;
pub mod auth;
pub mod raid_distributed_handlers;
pub mod ws;

use crate::ui;
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
/// HTTPS support is currently disabled due to build toolchain issues.
/// TODO: Read HTTPS configuration from config file
pub async fn start_server(addr: SocketAddr) {
    let app = Router::new()
        // Trailing-slash compat for UI entrypoint.
        .route("/ui/", get(|| async { Redirect::permanent("/ui") }))
        .nest("/api/v1", api::create_api_routes())
        .nest("/ui", ui::create_ui_routes());

    // TODO: Read from configuration file
    // HTTPS support is optional and requires feature "https"
    // For production, use: cargo build --features https
    // Note: Requires native toolchain (gcc/dlltool on Windows GNU)

    #[cfg(feature = "https")]
    {
        // HTTPS mode - requires certificates
        // TODO: Read cert paths from config
        use tracing::warn;
        let cert_path =
            std::env::var("HTTPS_CERT_PATH").unwrap_or_else(|_| "certs/cert.pem".to_string());
        let key_path =
            std::env::var("HTTPS_KEY_PATH").unwrap_or_else(|_| "certs/key.pem".to_string());

        match RustlsConfig::from_pem_file(&cert_path, &key_path).await {
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
