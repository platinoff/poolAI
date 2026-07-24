//! Network module for REST API and WebSocket server
//!
//! This module provides:
//! - REST API endpoints (67+ endpoints)
//! - WebSocket connections for real-time updates
//! - Authentication and authorization (JWT, RBAC)
//! - HTTPS/TLS support with certificate management

pub mod api;
pub mod auth;
pub mod discovery;
pub mod json_errors;
pub mod raid_distributed_handlers;
pub mod rate_limit;
pub mod security_headers;
pub mod tls_config;
pub mod validation;
pub mod ws;

#[cfg(feature = "enterprise")]
pub mod enterprise_api;

use crate::core::state::ApiContext;
use crate::observability;
use crate::ui;
use axum::middleware;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tracing::info;

/// Start the network server (HTTP or HTTPS)
///
/// # Arguments
/// * `addr` - Socket address to bind the server to
/// * `app_state` - Shared application context used by API handlers
///
/// # Note
/// HTTPS support requires feature "https" and valid certificates.
/// Configuration is read from PoolAIConfig.
/// Discovery service is automatically started if enabled.
pub async fn start_server(addr: SocketAddr, app_state: ApiContext) {
    use crate::network::discovery::{DiscoveryConfig, DiscoveryService};
    use crate::security::secret_rotation::{
        init_default_rotation_hooks, spawn_jwt_env_poll_if_configured,
    };
    use std::sync::Arc;

    init_default_rotation_hooks();
    spawn_jwt_env_poll_if_configured();

    let discovery_config = DiscoveryConfig::default();
    if discovery_config.enabled {
        let instance_manager = app_state.instance_manager.get().cloned();
        let service = Arc::new(DiscoveryService::new(
            discovery_config,
            addr,
            instance_manager,
        ));
        {
            let mut slot = app_state.discovery.write().await;
            *slot =
                Some(service.clone() as Arc<dyn crate::core::discovery_handle::DiscoveryHandle>);
        }
        if let Err(e) = service.start().await {
            tracing::warn!("Failed to start discovery service: {}", e);
        } else {
            let hydrated = service.hydrate_persisted_network_profiles().await;
            if hydrated > 0 {
                info!(
                    "Hydrated network_profile metadata for {} discovery peers",
                    hydrated
                );
            }
            info!("Discovery service started successfully");
        }
    }
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
        let router = router.nest(
            "/api/enterprise",
            enterprise_api::create_enterprise_api_routes(),
        );

        #[cfg(feature = "prometheus")]
        let router = {
            observability::init_prometheus();
            router.route("/metrics", get(observability::metrics_handler))
        };

        #[cfg(feature = "prometheus")]
        let router = observability::apply_prometheus_http_layer(router);

        observability::apply_http_trace(router.with_state(app_state))
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

        let tls_policy = match tls_config::TlsConfig::from_https_config(&https_config) {
            Ok(policy) => policy,
            Err(e) => {
                warn!("Invalid TLS policy: {}. Falling back to HTTP.", e);
                let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
                axum::serve(listener, app).await.unwrap();
                return;
            }
        };

        let cert_paths = tls_config::TlsConfig::resolve_cert_paths(&https_config);

        match tls_config::TlsServeContext::from_pem_files(cert_paths.clone(), tls_policy).await {
            Ok(ctx) => {
                let tls_ctx = ctx.clone();
                // Sync rotation hook → block_in_place + await reload (PH-SVC43 / rustdoc jwt,https).
                crate::security::secret_rotation::register_tls_reload_hook(move || {
                    tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current()
                            .block_on(tls_ctx.reload_certificates())
                            .map_err(|e| e.to_string())
                    })
                });
                info!(
                    "Starting HTTPS server on {} (TLS {}..{})",
                    addr, ctx.policy.min_version, ctx.policy.max_version
                );
                tls_config::spawn_cert_reload_if_configured(ctx.clone());
                axum_server::bind_rustls(addr, ctx.rustls)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
            Err(e) => {
                warn!(
                    "Failed to load TLS certificates (cert={}, key={}): {}. Falling back to HTTP.",
                    cert_paths.cert, cert_paths.key, e
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
