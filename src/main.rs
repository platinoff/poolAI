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
    raid,
    runtime::{self, RuntimeConfig},
    ui,
    version::{APP_VERSION, BUILD_TIME},
    vm,
    AppState, // Re-exported from core::state
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};

/// Get optimal number of worker threads for tokio runtime
///
/// Uses environment variable TOKIO_WORKER_THREADS if set,
/// otherwise defaults to number of CPU cores (detected at runtime).
fn get_worker_threads() -> usize {
    std::env::var("TOKIO_WORKER_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            // Use available_parallelism() (Rust 1.59+)
            // Fallback to 4 if unavailable
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        })
}

/// Get optimal blocking pool size for tokio runtime
///
/// Uses environment variable TOKIO_BLOCKING_THREADS if set,
/// otherwise defaults to 2 * worker_threads for blocking I/O operations.
fn get_blocking_threads(worker_threads: usize) -> usize {
    std::env::var("TOKIO_BLOCKING_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| worker_threads.max(2).saturating_mul(2))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure tokio runtime with optimal settings for production
    let worker_threads = get_worker_threads();
    let blocking_threads = get_blocking_threads(worker_threads);

    // Build tokio runtime with performance-optimized settings
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .max_blocking_threads(blocking_threads)
        .thread_name("poolai-worker")
        .thread_stack_size(3 * 1024 * 1024) // 3MB stack per thread (default is 2MB)
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

    runtime.block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Log runtime configuration for debugging
    let worker_threads = std::env::var("TOKIO_WORKER_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        });
    let blocking_threads = std::env::var("TOKIO_BLOCKING_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| worker_threads.max(2).saturating_mul(2));

    info!("🚀 Starting PoolAI v{}", APP_VERSION);
    info!("📅 Build time: {}", BUILD_TIME);
    info!(
        "⚙️  Tokio runtime: {} worker threads, {} blocking threads",
        worker_threads, blocking_threads
    );

    // Initialize uptime tracking
    poolai::version::initialize_start_time();

    // Initialize core module
    info!("Initializing core module...");
    core::initialize().await?;
    info!("✅ Core module initialized");

    // Initialize application state
    let app_state = Arc::new(AppState::new());
    app_state.initialize().await?;
    info!("✅ Application state initialized");

    // Initialize monitoring module
    info!("Initializing monitoring module...");
    monitoring::initialize().await?;
    info!("✅ Monitoring module initialized");

    // Initialize pool module
    info!("Initializing pool module...");
    pool::initialize(app_state.discovery.clone()).await?;
    info!("✅ Pool module initialized");

    // Initialize library management module
    info!("Initializing library management module...");
    libs::initialize().await?;
    info!("✅ Library management module initialized");

    // Initialize global model manager (for instance integration)
    core::model_interface::initialize_global_model_manager()
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;
    info!("✅ Model manager initialized");

    // Initialize runtime module
    info!("Initializing runtime module...");
    let runtime_config = RuntimeConfig::default();
    let _runtime_manager = runtime::initialize_runtime(runtime_config).await?;

    // Initialize topology manager first (before instance manager so it can use topology-aware placement)
    pool::topology::initialize_global_topology_manager(Some(app_state.discovery.clone()))
        .map_err(|e| format!("Failed to initialize topology manager: {}", e))?;

    // Initialize instance manager (will use topology-aware placement if topology manager is available)
    runtime::instance::initialize_global_instance_manager()
        .map_err(|e| format!("Failed to initialize instance manager: {}", e))?;

    // Start topology update task (simplified - just trigger periodic updates)
    if let Some(topology_manager) = pool::topology::get_global_topology_manager() {
        let topology_manager_clone = topology_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Ok(()) = topology_manager_clone.read().await.update_topology().await {
                    info!("Topology updated successfully");
                }
            }
        });
        info!("✅ Topology manager started");
    }

    info!("✅ Runtime module initialized (including instance manager and topology manager)");

    // Initialize VM module
    info!("Initializing VM module...");
    vm::initialize().await?;
    info!("✅ VM module initialized");

    // Initialize RAID module
    info!("Initializing RAID module...");
    raid::initialize().await?;
    info!("✅ RAID module initialized");

    // Initialize UI module
    info!("Initializing UI module...");
    ui::initialize().await?;
    info!("✅ UI module initialized");

    // Initialize rewards system (already initialized via lazy_static)
    info!("✅ Rewards system ready");

    // Start network server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🌐 Starting network server on {}", addr);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        network::start_server(addr, app_state).await;
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

    info!("Shutting down UI module...");
    ui::shutdown().await?;

    info!("Shutting down RAID module...");
    raid::shutdown().await?;

    info!("Shutting down VM module...");
    vm::shutdown().await?;

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
