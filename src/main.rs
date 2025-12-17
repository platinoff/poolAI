//! PoolAI main application entry point
//!
//! This module initializes all system components and starts the HTTP/HTTPS server
//! according to the PoolAI architecture concept.

use poolai::{
    core,
    libs,
    monitoring,
    network,
    pool,
    runtime::{self, RuntimeConfig},
    version::{APP_VERSION, BUILD_TIME},
    AppState,  // Re-exported from core::state
};
use std::net::SocketAddr;
use tracing::{info, error};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("🚀 Starting PoolAI v{}", APP_VERSION);
    info!("📅 Build time: {}", BUILD_TIME);

    // Initialize core module
    info!("Initializing core module...");
    core::initialize().await?;
    info!("✅ Core module initialized");

    // Initialize application state
    let app_state = AppState::new();
    app_state.initialize().await?;
    info!("✅ Application state initialized");

    // Initialize monitoring module
    info!("Initializing monitoring module...");
    monitoring::initialize().await?;
    info!("✅ Monitoring module initialized");

    // Initialize pool module
    info!("Initializing pool module...");
    pool::initialize().await?;
    info!("✅ Pool module initialized");

    // Initialize library management module
    info!("Initializing library management module...");
    libs::initialize().await?;
    info!("✅ Library management module initialized");

    // Initialize runtime module
    info!("Initializing runtime module...");
    let runtime_config = RuntimeConfig::default();
    let _runtime_manager = runtime::initialize_runtime(runtime_config).await?;
    info!("✅ Runtime module initialized");

    // Initialize rewards system (already initialized via lazy_static)
    info!("✅ Rewards system ready");

    // Start network server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🌐 Starting network server on {}", addr);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        network::start_server(addr).await;
    });

    info!("✅ PoolAI started successfully!");
    info!("📊 System ready to accept requests");
    info!("🔗 API available at http://{}", addr);

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("🛑 Shutdown signal received");
        }
        Err(err) => {
            error!("Failed to listen for shutdown signal: {}", err);
        }
    }

    // Graceful shutdown
    info!("🔄 Shutting down gracefully...");

    // Shutdown modules in reverse order
    info!("Shutting down runtime module...");
    // Runtime manager is dropped automatically

    info!("Shutting down pool module...");
    pool::shutdown().await?;

    info!("Shutting down library management module...");
    libs::shutdown().await?;

    info!("Shutting down monitoring module...");
    monitoring::shutdown().await?;

    info!("Shutting down core module...");
    core::shutdown().await?;

    // Cancel server task
    server_handle.abort();

    info!("✅ PoolAI shutdown complete");
    Ok(())
}
