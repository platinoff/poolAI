// network/mod.rs
pub mod api;
pub mod ws;
pub mod auth;

use axum::Router;
use std::net::SocketAddr;
use tracing::info;
use axum_server::Server;

#[cfg(feature = "https")]
use axum_server::tls_rustls::RustlsConfig;

pub async fn start_server(addr: SocketAddr) {
    let app = Router::new()
        .nest("/api/v1", api::create_api_routes());

    // TODO: Заменить на чтение из конфига
    let enable_https = true; // временно для теста
    let cert_path = "certs/cert.pem";
    let key_path = "certs/key.pem";

    if enable_https {
        #[cfg(feature = "https")]
        {
            let config = RustlsConfig::from_pem_file(cert_path, key_path).await.expect("Failed to load certs");
            info!("Starting HTTPS server on {}", addr);
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        #[cfg(not(feature = "https"))]
        {
            panic!("HTTPS feature not enabled. Rebuild with --features https");
        }
    } else {
        info!("Starting HTTP server on {}", addr);
        Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
} 