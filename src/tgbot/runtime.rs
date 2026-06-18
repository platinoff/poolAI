//! Teloxide dispatcher (FM-016++, requires feature `tgbot`).

use crate::tgbot::coordinator::{
    fetch_telegram_seats, format_reply, format_seats_reply, format_unbind_reply, forward_message,
    unbind_telegram, CoordinatorConfig,
};
use std::sync::atomic::{AtomicI64, Ordering};
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tracing::{error, info};

static UPDATE_SEQ: AtomicI64 = AtomicI64::new(1);

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "PoolAI virtual-node bot:")]
enum Command {
    #[command(description = "show help")]
    Help,
    #[command(description = "enqueue status check on your bound worker")]
    Status,
    #[command(description = "ping coordinator binding")]
    Start,
    #[command(description = "bind payout wallet pubkey")]
    Wallet(String),
    #[command(description = "unbind edge worker")]
    Stop,
}

pub async fn run_bot(token: &str, config: CoordinatorConfig) -> Result<(), String> {
    if token.trim().is_empty() {
        return Err("TELEGRAM_BOT_TOKEN is empty".to_string());
    }

    info!("Starting PoolAI Telegram bot → {}", config.webhook_url());

    let bot = Bot::new(token);
    let cfg = std::sync::Arc::new(config);

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(dptree::endpoint(text_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![cfg])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    cfg: std::sync::Arc<CoordinatorConfig>,
) -> ResponseResult<()> {
    let text = match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
            return Ok(());
        }
        Command::Status => {
            let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
            if user_id == 0 {
                bot.send_message(msg.chat.id, "Cannot resolve Telegram user id")
                    .await?;
                return Ok(());
            }
            let result = fetch_telegram_seats(&cfg).await;
            let reply = format_seats_reply(&result);
            bot.send_message(msg.chat.id, reply).await?;
            return Ok(());
        }
        Command::Stop => {
            let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
            if user_id == 0 {
                bot.send_message(msg.chat.id, "Cannot resolve Telegram user id")
                    .await?;
                return Ok(());
            }
            let result = unbind_telegram(&cfg, user_id).await;
            let reply = format_unbind_reply(&result);
            bot.send_message(msg.chat.id, reply).await?;
            return Ok(());
        }
        Command::Start => "/start",
        Command::Wallet(pubkey) => {
            if pubkey.trim().is_empty() {
                bot.send_message(msg.chat.id, "Usage: /wallet <solana_pubkey> (Galaxy §3.2)")
                    .await?;
                return Ok(());
            }
            let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
            let chat_id = msg.chat.id.0;
            if user_id == 0 {
                bot.send_message(msg.chat.id, "Cannot resolve Telegram user id")
                    .await?;
                return Ok(());
            }
            let result =
                crate::tgbot::coordinator::bind_wallet(&cfg, user_id, chat_id, pubkey.trim()).await;
            let reply = crate::tgbot::coordinator::format_wallet_reply(&result);
            bot.send_message(msg.chat.id, reply).await?;
            return Ok(());
        }
    };
    handle_user_text(bot, msg, cfg, text).await
}

async fn text_handler(
    bot: Bot,
    msg: Message,
    cfg: std::sync::Arc<CoordinatorConfig>,
) -> ResponseResult<()> {
    let text = msg.text().map(str::to_string).unwrap_or_default();
    handle_user_text(bot, msg, cfg, &text).await
}

async fn handle_user_text(
    bot: Bot,
    msg: Message,
    cfg: std::sync::Arc<CoordinatorConfig>,
    text: &str,
) -> ResponseResult<()> {
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
    let chat_id = msg.chat.id.0;
    let update_id = UPDATE_SEQ.fetch_add(1, Ordering::Relaxed);

    if user_id == 0 {
        bot.send_message(msg.chat.id, "Cannot resolve Telegram user id")
            .await?;
        return Ok(());
    }

    let result = forward_message(&cfg, update_id, user_id, chat_id, text).await;
    if let Err(ref e) = result {
        error!("coordinator forward failed: {e}");
    }
    let reply = format_reply(&result);
    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}
