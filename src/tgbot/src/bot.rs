use teloxide::{prelude::*, utils::command::BotCommands};
use crate::tgbot::{Command, BotConfig};
use crate::tgbot::handlers::answer;

pub struct TelegramBot {
    bot: Bot,
    config: BotConfig,
}

impl TelegramBot {
    pub fn new(config: BotConfig) -> Self {
        let bot = Bot::new(config.token.clone());
        Self { bot, config }
    }

    pub async fn run(&self) {
        log::info!("Starting Telegram bot...");

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