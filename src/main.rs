use poolai::{
    PoolAIConfig,
    AppState,
    Pool,
    PoolConfig,
    LoadBalancingStrategy,
    MetricsCollector,
};
#[cfg(feature = "stage2")]
use poolai::network::start_server;
#[cfg(feature = "stage2")]
use poolai::tgbot::start_bot;
use std::net::SocketAddr;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting PoolAI...");

    // Load configuration
    let config = PoolAIConfig::from_file("config.toml")?;
    info!("Configuration loaded successfully");

    // Initialize global state
    let state = AppState::new();
    info!("Application state initialized");

    // Initialize pool
    let pool_config = PoolConfig {
        max_workers: config.pool.max_workers,
        max_queue_size: config.pool.queue_size,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        auto_scaling: config.pool.auto_scaling,
        scaling_threshold: config.pool.scaling_threshold,
        request_timeout: config.pool.request_timeout,
    };
    let pool = Pool::new(pool_config);
    info!("Worker pool initialized with {} workers", config.pool.max_workers);

    // Initialize monitoring
    let metrics = MetricsCollector::new();
    info!("Metrics collector initialized");

    // Stage 2: Network server
    #[cfg(feature = "stage2")]
    {
        let addr: SocketAddr = "127.0.0.1:8080".parse()?;
        info!("Starting network server on {}", addr);
        start_server(addr).await;
    }

    // Stage 2: Telegram bot
    #[cfg(feature = "stage2")]
    {
        let bot_token = "your_bot_token_here"; // TODO: Load from config
        info!("Starting Telegram bot");
        start_bot(bot_token).await;
    }

    // Start monitoring loop
    info!("Starting monitoring loop");
    loop {
        let _ = metrics.collect().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}
