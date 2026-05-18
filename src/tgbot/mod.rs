//! Telegram Bot module for PoolAI (FM-016++).
//!
//! - **`coordinator`**: forwards messages to `POST /api/v1/virtual-nodes/telegram/webhook`
//! - **`runtime`** (feature `tgbot`): teloxide long-poll bot (`poolai-telegram-bot` binary)
//!
//! # Environment
//!
//! | Variable | Required | Description |
//! |----------|----------|-------------|
//! | `TELEGRAM_BOT_TOKEN` | bot process | From @BotFather |
//! | `POOLAI_COORDINATOR_URL` | bot process | Coordinator base URL |
//! | `POOLAI_TELEGRAM_WEBHOOK_SECRET` | optional | Same as coordinator |

pub mod coordinator;

#[cfg(feature = "tgbot")]
mod runtime;

pub use coordinator::{
    forward_message, forward_payload, message_to_webhook_payload, CoordinatorConfig, ForwardResult,
};

#[cfg(feature = "tgbot")]
pub use runtime::run_bot;

/// Legacy entry: logs in tests / without `tgbot` feature; with feature + non-empty token runs bot.
pub async fn start_bot(token: &str) {
    if token.trim().is_empty() || cfg!(test) {
        tracing::info!("[tgbot] dry-run (empty token or test build)");
        return;
    }

    #[cfg(feature = "tgbot")]
    {
        match CoordinatorConfig::from_env() {
            Ok(cfg) => {
                if let Err(e) = runtime::run_bot(token, cfg).await {
                    tracing::error!("[tgbot] bot exited: {e}");
                }
            }
            Err(e) => tracing::error!("[tgbot] config error: {e}"),
        }
    }

    #[cfg(not(feature = "tgbot"))]
    {
        tracing::info!(
            "[tgbot] feature `tgbot` disabled — rebuild with `--features tgbot` for teloxide runtime"
        );
    }
}

/// Send a notification to a Telegram chat (requires `tgbot` + `TELEGRAM_BOT_TOKEN`).
pub async fn send_notification(chat_id: &str, message: &str) {
    if chat_id.trim().is_empty() || message.is_empty() {
        tracing::warn!("[tgbot] send_notification skipped: empty chat_id or message");
        return;
    }

    #[cfg(feature = "tgbot")]
    {
        let Ok(token) = std::env::var("TELEGRAM_BOT_TOKEN") else {
            tracing::warn!("[tgbot] TELEGRAM_BOT_TOKEN not set");
            return;
        };
        use teloxide::prelude::*;
        let bot = Bot::new(token);
        let chat: ChatId = match chat_id.parse::<i64>() {
            Ok(id) => ChatId(id),
            Err(_) => {
                tracing::warn!("[tgbot] invalid chat_id (expected numeric): {chat_id}");
                return;
            }
        };
        if let Err(e) = bot.send_message(chat, message).await {
            tracing::error!("[tgbot] send failed: {e}");
        }
        return;
    }

    #[cfg(not(feature = "tgbot"))]
    tracing::info!("[tgbot] notification to {chat_id}: {message} (feature `tgbot` disabled)");
}
