//! Network module for REST API and WebSocket server
//!
//! This module provides:
//! - REST API endpoints (67+ endpoints)
//! - WebSocket connections for real-time updates
//! - Authentication and authorization (JWT, RBAC)
//! - HTTPS/TLS support with certificate management

pub mod api;
pub mod ws;
pub mod auth;

use axum::Router;
use axum::routing::get;
use axum::response::Redirect;
use std::net::SocketAddr;
use tracing::info;
use crate::ui;
// use axum_server::Server;  // Temporarily disabled - requires ring/gcc

// #[cfg(feature = "https")]
// use axum_server::tls_rustls::RustlsConfig;  // Temporarily disabled

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
    // HTTPS temporarily disabled - requires gcc for ring crate
    // let _enable_https = true; // Temporary for testing
    // let _cert_path = "certs/cert.pem";
    // let _key_path = "certs/key.pem";

    // Temporarily disabled HTTPS due to gcc.exe issue
    // if enable_https {
    //     #[cfg(feature = "https")]
    //     {
    //         let config = RustlsConfig::from_pem_file(_cert_path, _key_path).await.expect("Failed to load certs");
    //         info!("Starting HTTPS server on {}", addr);
    //         axum_server::bind_rustls(addr, config)
    //             .serve(app.into_make_service())
    //             .await
    //             .unwrap();
    //     }
    //     #[cfg(not(feature = "https"))]
    //     {
    //         panic!("HTTPS feature not enabled. Rebuild with --features https");
    //     }
    // } else {
        info!("Starting HTTP server on {}", addr);
        // Use axum's built-in server instead of axum-server
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        info!("Server listening on {}", addr);
        axum::serve(listener, app).await.unwrap();
    // }
} 