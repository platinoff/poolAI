use poolai::{initialize_system, shutdown_system, health_check, get_system_info};
use poolai::core::get_system_config;
use std::process;
use tokio;
use tracing::{info, error, warn};
use clap::Parser;

#[derive(Parser)]
#[command(name = "poolai")]
#[command(about = "PoolAI - AI Mining Pool Management System")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    /// Initialize with default configuration
    #[arg(long)]
    init_default: bool,
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize tracing with specified log level
    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting PoolAI - AI Mining Pool Management System");
    
    let info = get_system_info();
    info!("Version: {}", info.version);
    info!("Build: {}", info.name);
    info!("Description: {}", info.description);
    info!("MVP Modules: {:?}", info.mvp_modules);

    // Load configuration
    if cli.init_default {
        info!("🔧 Initializing with default configuration...");
        if let Err(e) = initialize_system().await {
            error!("❌ Failed to initialize PoolAI with default config: {}", e);
            process::exit(1);
        }
    } else {
        info!("🔧 Loading configuration from: {}", cli.config);
        match poolai::core::config::PoolAIConfig::from_file(&cli.config) {
            Ok(config) => {
                info!("✅ Configuration loaded successfully");
                if let Err(e) = poolai::core::initialize_with_config(config).await {
                    error!("❌ Failed to initialize PoolAI with config: {}", e);
                    process::exit(1);
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to load configuration file: {}", e);
                info!("🔧 Falling back to default configuration...");
                if let Err(e) = initialize_system().await {
                    error!("❌ Failed to initialize PoolAI with default config: {}", e);
                    process::exit(1);
                }
            }
        }
    }

    info!("✅ PoolAI MVP system initialized successfully!");

    // Health check
    info!("📈 Running system health check...");
    if let Err(e) = health_check().await {
        error!("❌ Health check failed: {}", e);
        process::exit(1);
    } else {
        info!("📈 System health check passed.");
    }

    // Display system configuration
    if let Ok(config) = get_system_config() {
        info!("🔧 System Configuration:");
        info!("  - System: {} v{}", config.system.name, config.system.version);
        info!("  - GPU: {} ({} MB memory)", 
            if config.gpu.enabled { "Enabled" } else { "Disabled" }, 
            config.gpu.memory_limit);
        info!("  - Pool: {} workers, {} queue size", 
            config.pool.max_workers, config.pool.queue_size);
        info!("  - Monitoring: {}s interval, {} threshold", 
            config.monitoring.metrics_interval, config.monitoring.alert_threshold);
        info!("  - Models: {} configured", config.models.len());
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
