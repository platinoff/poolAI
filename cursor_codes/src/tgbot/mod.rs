mod commands;
mod handlers;
mod bot;

pub use bot::TelegramBot;
pub use commands::Command;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::command::BotCommands,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};

use crate::{
    workers::WorkerManager,
    vm::VMManager,
    reward_system::RewardSystem,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Stop mining")]
    Stop,
    #[command(description = "Check mining status")]
    Status,
    #[command(description = "Configure worker settings")]
    Config,
    #[command(description = "View mining statistics")]
    Stats,
    #[command(description = "Show this help message")]
    Help,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub admin_chat_id: i64,
    pub allowed_users: Vec<i64>,
    pub vm_manager: Box<dyn VmManager>,
}

impl BotConfig {
    pub fn new(token: String, vm_manager: Box<dyn VmManager>) -> Self {
        Self {
            token,
            admin_chat_id: 0,
            allowed_users: Vec::new(),
            vm_manager,
        }
    }
}

pub async fn run_bot(config: BotConfig) {
pub struct MiningBot {
    bot: Bot,
    config: BotConfig,
    worker_manager: Arc<WorkerManager>,
    vm_manager: Arc<Mutex<VMManager>>,
    reward_system: Arc<RewardSystem>,
}

impl MiningBot {
    pub fn new(
        config: BotConfig,
        worker_manager: Arc<WorkerManager>,
        vm_manager: Arc<Mutex<VMManager>>,
        reward_system: Arc<RewardSystem>,
    ) -> Self {
        Self {
            bot: Bot::new(&config.token),
            config,
            worker_manager,
            vm_manager,
            reward_system,
        }
    }

    pub async fn run(&self) {
        let handler = Update::filter_message()
            .filter_command::<Command>()
            .endpoint(answer);

        Dispatcher::builder(self.bot.clone(), handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to the Mining Bot! Use /help to see available commands.")
                .await?;
        }
        Command::Stop => {
            // Stop mining operations
            bot.send_message(msg.chat.id, "Stopping mining operations...")
                .await?;
        }
        Command::Status => {
            // Get current mining status
            let status = "Mining Status:\nActive: Yes\nWorkers: 2\nHashrate: 100 MH/s";
            bot.send_message(msg.chat.id, status).await?;
        }
        Command::Config => {
            // Show configuration options
            let keyboard = make_config_keyboard();
            bot.send_message(msg.chat.id, "Select configuration option:")
                .reply_markup(keyboard)
                .await?;
        }
        Command::Stats => {
            // Show mining statistics
            let stats = "Mining Statistics:\nTotal Rewards: 100 SOL\nUptime: 24h\nSuccess Rate: 99%";
            bot.send_message(msg.chat.id, stats).await?;
        }
        Command::Help => {
            let help_text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, help_text).await?;
        }
    }

    Ok(())
}

fn make_config_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let row1 = vec![
        InlineKeyboardButton::callback("Set Memory", "config_memory"),
        InlineKeyboardButton::callback("Set GPU", "config_gpu"),
    ];
    let row2 = vec![
        InlineKeyboardButton::callback("Set Workers", "config_workers"),
        InlineKeyboardButton::callback("Set Address", "config_address"),
    ];

    keyboard.push(row1);
    keyboard.push(row2);

    InlineKeyboardMarkup::new(keyboard)
}

fn format_status(status: &str) -> String {
    format!("📊 Mining Status:\n{}", status)
}

fn format_stats(stats: &str) -> String {
    format!("📈 Mining Statistics:\n{}", stats)
} 