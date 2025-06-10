use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{Message, InlineKeyboardMarkup, InlineKeyboardButton, ParseMode, CallbackQuery};
use teloxide::utils::command::BotCommands;
use crate::state::AppState;
use log::info;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступні команди:")]
enum Command {
    #[command(description = "Показати статус воркера")]
    Status,
    #[command(description = "Почати майнінг")]
    Mine,
    #[command(description = "Показати статистику пулу")]
    Stats,
    #[command(description = "Налаштування воркера")]
    Config,
    #[command(description = "Допомога")]
    Help,
    #[command(description = "Управління VM")]
    Vm,
}

pub async fn start(app_state: Arc<AppState>) {
    info!("Starting Telegram bot");
    
    let bot = Bot::from_env();
    
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command_handler)
        )
        .branch(
            dptree::filter(|msg: Message| msg.text().is_some())
                .endpoint(message_handler)
        )
        .branch(
            dptree::entry()
                .filter_callback_query()
                .endpoint(callback_handler)
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![app_state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    app_state: Arc<AppState>,
) -> ResponseResult<()> {
    match cmd {
        Command::Status => {
            let status = get_worker_status(&app_state, msg.chat.id).await;
            bot.send_message(msg.chat.id, status)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Mine => {
            let response = start_mining(&app_state, msg.chat.id).await;
            bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Stats => {
            let stats = get_pool_stats(&app_state).await;
            bot.send_message(msg.chat.id, stats)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Config => {
            let keyboard = create_config_keyboard();
            bot.send_message(msg.chat.id, "Налаштування воркера:")
                .reply_markup(keyboard)
                .await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Vm => {
            let keyboard = create_vm_keyboard();
            bot.send_message(msg.chat.id, "Управління віртуальною машиною:")
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}

fn create_vm_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    
    // Main menu buttons
    keyboard.push(vec![
        InlineKeyboardButton::callback("Створити VM", "create_vm"),
        InlineKeyboardButton::callback("Список VM", "list_vms"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("Запустити VM", "start_vm"),
        InlineKeyboardButton::callback("Зупинити VM", "stop_vm"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("Налаштування портів", "port_config"),
        InlineKeyboardButton::callback("Статистика", "vm_stats"),
    ]);
    
    InlineKeyboardMarkup::new(keyboard)
}

fn create_config_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("Ім'я воркера", "set_name"),
        InlineKeyboardButton::callback("Адреса Solana", "set_solana"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("CPU ядра", "set_cpu"),
        InlineKeyboardButton::callback("GPU ядра", "set_gpu"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("Пам'ять", "set_memory"),
        InlineKeyboardButton::callback("Сховище", "set_storage"),
    ]);
    
    InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(bot: Bot, msg: Message, app_state: Arc<AppState>) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        let response = process_config_message(&app_state, msg.chat.id, text).await;
        bot.send_message(msg.chat.id, response)
            .parse_mode(ParseMode::Html)
            .await?;
    }
    Ok(())
}

async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    app_state: Arc<AppState>,
) -> ResponseResult<()> {
    if let Some(data) = q.data {
        let response = match data.as_str() {
            "create_vm" => create_vm_dialog(&app_state, q.from.id).await,
            "list_vms" => list_vms(&app_state, q.from.id).await,
            "start_vm" => start_vm_dialog(&app_state, q.from.id).await,
            "stop_vm" => stop_vm_dialog(&app_state, q.from.id).await,
            "port_config" => port_config_dialog(&app_state, q.from.id).await,
            "vm_stats" => get_vm_stats(&app_state, q.from.id).await,
            _ => "Невідома команда".to_string(),
        };

        if let Some(msg) = q.message {
            bot.edit_message_text(msg.chat.id, msg.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}

async fn create_vm_dialog(app_state: &Arc<AppState>, user_id: i64) -> String {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Налаштувати", "vm_config"),
        InlineKeyboardButton::callback("Скасувати", "cancel"),
    ]]);

    "Створення нової VM:\n\nВведіть параметри:\n1. Ім'я VM\n2. Кількість CPU ядер\n3. Кількість GPU ядер\n4. Обсяг пам'яті (GB)\n5. Обсяг сховища (GB)\n\nПісля введення натисніть 'Налаштувати'"
        .to_string()
}

async fn list_vms(app_state: &Arc<AppState>, user_id: i64) -> String {
    let vms = app_state.vm_manager.list_vms().await;
    if vms.is_empty() {
        "Немає активних VM".to_string()
    } else {
        let mut response = "Список VM:\n\n".to_string();
        for vm in vms {
            response.push_str(&format!(
                "VM: {}\nСтатус: {}\nCPU: {} ядер\nGPU: {} ядер\nПам'ять: {} GB\n\n",
                vm.name, if vm.is_running { "Запущено" } else { "Зупинено" },
                vm.cpu_cores, vm.gpu_cores, vm.memory_gb
            ));
        }
        response
    }
}

async fn start_vm_dialog(app_state: &Arc<AppState>, user_id: i64) -> String {
    let vms = app_state.vm_manager.list_vms().await;
    if vms.is_empty() {
        "Немає доступних VM для запуску".to_string()
    } else {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        for vm in vms {
            if !vm.is_running {
                keyboard.push(vec![InlineKeyboardButton::callback(
                    format!("Запустити {}", vm.name),
                    format!("start_vm_{}", vm.name)
                )]);
            }
        }
        keyboard.push(vec![InlineKeyboardButton::callback("Назад", "vm_menu")]);
        
        "Виберіть VM для запуску:".to_string()
    }
}

async fn stop_vm_dialog(app_state: &Arc<AppState>, user_id: i64) -> String {
    let vms = app_state.vm_manager.list_vms().await;
    if vms.is_empty() {
        "Немає запущених VM".to_string()
    } else {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        for vm in vms {
            if vm.is_running {
                keyboard.push(vec![InlineKeyboardButton::callback(
                    format!("Зупинити {}", vm.name),
                    format!("stop_vm_{}", vm.name)
                )]);
            }
        }
        keyboard.push(vec![InlineKeyboardButton::callback("Назад", "vm_menu")]);
        
        "Виберіть VM для зупинки:".to_string()
    }
}

async fn port_config_dialog(app_state: &Arc<AppState>, user_id: i64) -> String {
    let vms = app_state.vm_manager.list_vms().await;
    if vms.is_empty() {
        "Немає доступних VM для налаштування портів".to_string()
    } else {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        for vm in vms {
            keyboard.push(vec![InlineKeyboardButton::callback(
                format!("Налаштувати порти для {}", vm.name),
                format!("config_ports_{}", vm.name)
            )]);
        }
        keyboard.push(vec![InlineKeyboardButton::callback("Назад", "vm_menu")]);
        
        "Виберіть VM для налаштування портів:".to_string()
    }
}

async fn get_vm_stats(app_state: &Arc<AppState>, user_id: i64) -> String {
    let vms = app_state.vm_manager.list_vms().await;
    if vms.is_empty() {
        "Немає активних VM для відображення статистики".to_string()
    } else {
        let mut response = "Статистика VM:\n\n".to_string();
        for vm in vms {
            let stats = app_state.vm_manager.get_vm_status(&vm.name).await;
            response.push_str(&format!(
                "VM: {}\nCPU використання: {}%\nGPU використання: {}%\nПам'ять: {}%\nПорти: {}\n\n",
                vm.name,
                stats.cpu_usage,
                stats.gpu_usage,
                stats.memory_usage,
                stats.forwarded_ports.join(", ")
            ));
        }
        response
    }
}

async fn get_worker_status(app_state: &Arc<AppState>, chat_id: i64) -> String {
    // TODO: Implement actual worker status
    format!("Статус воркера {}:\nCPU: 0%\nGPU: 0%\nПам'ять: 0%", chat_id)
}

async fn start_mining(app_state: &Arc<AppState>, chat_id: i64) -> String {
    // TODO: Implement actual mining start
    format!("Майнінг запущено для воркера {}", chat_id)
}

async fn get_pool_stats(app_state: &Arc<AppState>) -> String {
    // TODO: Implement actual pool stats
    "Статистика пулу:\nАктивних воркерів: 0\nЗагальна потужність: 0".to_string()
}

async fn process_config_message(app_state: &Arc<AppState>, chat_id: i64, text: &str) -> String {
    // TODO: Implement actual config processing
    format!("Налаштування оновлено для воркера {}: {}", chat_id, text)
} 