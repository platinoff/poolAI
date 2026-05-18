//! PoolAI Telegram bot — forwards user messages to coordinator virtual-node webhook.
//!
//! Build: `cargo build --bin poolai-telegram-bot --features tgbot`
//! Run: `TELEGRAM_BOT_TOKEN=... POOLAI_COORDINATOR_URL=http://127.0.0.1:8080 poolai-telegram-bot`

use poolai::tgbot::{run_bot, CoordinatorConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let token =
        std::env::var("TELEGRAM_BOT_TOKEN").map_err(|_| "TELEGRAM_BOT_TOKEN must be set")?;
    let config = CoordinatorConfig::from_env()?;
    info!("poolai-telegram-bot → coordinator {}", config.webhook_url());
    run_bot(&token, config).await?;
    Ok(())
}
