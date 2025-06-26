use poolai::{initialize_system, shutdown_system, health_check, get_system_info};
use std::process;
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting PoolAI - AI Mining Pool Management System");
    
    let info = get_system_info();
    info!("Version: {}", info.version);
    info!("Build: {}", info.name);
    info!("Description: {}", info.description);
    info!("MVP Modules: {:?}", info.mvp_modules);

    // Initialize system
    info!("🔧 Initializing PoolAI MVP system...");
    if let Err(e) = initialize_system().await {
        error!("❌ Failed to initialize PoolAI: {}", e);
        process::exit(1);
    }
    info!("✅ PoolAI MVP system initialized successfully!");

    // Health check
    if let Err(e) = health_check().await {
        error!("❌ Health check failed: {}", e);
        process::exit(1);
    } else {
        info!("📈 System health check passed.");
    }

    // Wait for shutdown signal
    info!("🔄 PoolAI MVP is running. Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    info!("🛑 Received shutdown signal...");

    // Graceful shutdown
    info!("🔧 Shutting down PoolAI MVP system...");
    if let Err(e) = shutdown_system().await {
        error!("❌ Error during shutdown: {}", e);
        process::exit(1);
    }
    info!("✅ PoolAI MVP system shutdown completed successfully!");
}
