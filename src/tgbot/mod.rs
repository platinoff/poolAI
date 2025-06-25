pub mod bot;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TGBotConfig {
    pub bot_token: String,
    pub allowed_users: Vec<i64>,
    pub enable_notifications: bool,
    pub notification_interval_minutes: u64,
    pub max_message_length: usize,
    pub enable_admin_commands: bool,
}

#[derive(Debug, Clone)]
pub struct BotCommand {
    pub name: String,
    pub description: String,
    pub handler: String,
    pub requires_admin: bool,
}

#[derive(Debug, Clone)]
pub struct BotMessage {
    pub chat_id: i64,
    pub user_id: i64,
    pub message_id: i32,
    pub text: String,
    pub timestamp: std::time::Instant,
    pub is_command: bool,
}

#[derive(Debug, Clone)]
pub struct BotNotification {
    pub chat_id: i64,
    pub message: String,
    pub notification_type: NotificationType,
    pub timestamp: std::time::Instant,
    pub sent: bool,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    SystemStatus,
    Alert,
    Metrics,
    Error,
    Info,
}

pub struct TGBot {
    config: TGBotConfig,
    commands: Arc<RwLock<HashMap<String, BotCommand>>>,
    messages: Arc<RwLock<Vec<BotMessage>>>,
    notifications: Arc<RwLock<Vec<BotNotification>>>,
    bot_instance: Arc<bot::TelegramBot>,
}

impl TGBot {
    pub fn new(config: TGBotConfig) -> Result<Self, AppError> {
        let bot_instance = Arc::new(bot::TelegramBot::new(config.clone())?);
        
        Ok(Self {
            config,
            commands: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(Vec::new())),
            notifications: Arc::new(RwLock::new(Vec::new())),
            bot_instance,
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Инициализация бота
        self.bot_instance.initialize().await?;
        
        // Регистрация команд
        self.register_commands().await?;
        
        // Запуск обработки сообщений
        self.start_message_processing().await?;
        
        // Запуск уведомлений если включены
        if self.config.enable_notifications {
            self.start_notification_service().await?;
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Выключение бота
        self.bot_instance.shutdown().await?;
        
        Ok(())
    }

    pub async fn send_message(&self, chat_id: i64, message: &str) -> Result<(), AppError> {
        // Отправка сообщения через бота
        self.bot_instance.send_message(chat_id, message).await?;
        
        Ok(())
    }

    pub async fn send_notification(&self, chat_id: i64, message: &str, notification_type: NotificationType) -> Result<(), AppError> {
        // Создание уведомления
        let notification = BotNotification {
            chat_id,
            message: message.to_string(),
            notification_type,
            timestamp: std::time::Instant::now(),
            sent: false,
        };
        
        // Добавление в очередь уведомлений
        {
            let mut notifications = self.notifications.write().await;
            notifications.push(notification);
        }
        
        // Отправка уведомления
        self.send_message(chat_id, message).await?;
        
        // Отметка как отправленное
        {
            let mut notifications = self.notifications.write().await;
            if let Some(notification) = notifications.iter_mut().last() {
                notification.sent = true;
            }
        }
        
        Ok(())
    }

    pub async fn broadcast_message(&self, message: &str) -> Result<(), AppError> {
        // Отправка сообщения всем разрешенным пользователям
        for user_id in &self.config.allowed_users {
            self.send_message(*user_id, message).await?;
        }
        
        Ok(())
    }

    pub async fn get_system_status(&self) -> Result<String, AppError> {
        // Получение статуса системы для отправки в Telegram
        // В реальной реализации здесь будет интеграция с мониторингом
        
        let status = format!(
            "🤖 PoolAI System Status\n\n\
            📊 Overall Health: 95%\n\
            🔧 Active Workers: 8\n\
            💾 Memory Usage: 6.2GB / 16GB\n\
            🎮 GPU Utilization: 75%\n\
            ⚡ CPU Usage: 45%\n\
            🌐 Network: 125 Mbps\n\
            ⏰ Uptime: 2d 15h 30m\n\
            📈 Total Requests: 15,432\n\
            ✅ Success Rate: 98.5%"
        );
        
        Ok(status)
    }

    pub async fn get_metrics_summary(&self) -> Result<String, AppError> {
        // Получение краткой сводки метрик
        let metrics = format!(
            "📈 PoolAI Metrics Summary\n\n\
            🚀 Requests/sec: 45.2\n\
            ⏱️ Avg Response Time: 250ms\n\
            🎯 GPU Utilization: 75.5%\n\
            💾 Memory Usage: 6.2GB\n\
            🔥 Temperature: 65°C\n\
            ⚡ Power: 350W\n\
            📊 Error Rate: 0.02%"
        );
        
        Ok(metrics)
    }

    async fn register_commands(&self) -> Result<(), AppError> {
        let mut commands = self.commands.write().await;
        
        // Базовые команды
        commands.insert("/start".to_string(), BotCommand {
            name: "start".to_string(),
            description: "Start the bot".to_string(),
            handler: "handle_start".to_string(),
            requires_admin: false,
        });
        
        commands.insert("/help".to_string(), BotCommand {
            name: "help".to_string(),
            description: "Show available commands".to_string(),
            handler: "handle_help".to_string(),
            requires_admin: false,
        });
        
        commands.insert("/status".to_string(), BotCommand {
            name: "status".to_string(),
            description: "Get system status".to_string(),
            handler: "handle_status".to_string(),
            requires_admin: false,
        });
        
        commands.insert("/metrics".to_string(), BotCommand {
            name: "metrics".to_string(),
            description: "Get system metrics".to_string(),
            handler: "handle_metrics".to_string(),
            requires_admin: false,
        });
        
        // Админские команды
        if self.config.enable_admin_commands {
            commands.insert("/restart".to_string(), BotCommand {
                name: "restart".to_string(),
                description: "Restart the system".to_string(),
                handler: "handle_restart".to_string(),
                requires_admin: true,
            });
            
            commands.insert("/shutdown".to_string(), BotCommand {
                name: "shutdown".to_string(),
                description: "Shutdown the system".to_string(),
                handler: "handle_shutdown".to_string(),
                requires_admin: true,
            });
            
            commands.insert("/scale".to_string(), BotCommand {
                name: "scale".to_string(),
                description: "Scale workers".to_string(),
                handler: "handle_scale".to_string(),
                requires_admin: true,
            });
        }
        
        Ok(())
    }

    async fn start_message_processing(&self) -> Result<(), AppError> {
        let bot_instance = self.bot_instance.clone();
        let commands = self.commands.clone();
        let messages = self.messages.clone();
        let allowed_users = self.config.allowed_users.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                
                // Получение новых сообщений
                if let Ok(new_messages) = bot_instance.get_updates().await {
                    for message in new_messages {
                        // Проверка разрешенных пользователей
                        if !allowed_users.contains(&message.user_id) {
                            continue;
                        }
                        
                        // Сохранение сообщения
                        {
                            let mut messages_write = messages.write().await;
                            messages_write.push(message.clone());
                            
                            // Ограничение размера истории
                            if messages_write.len() > 1000 {
                                messages_write.drain(0..100);
                            }
                        }
                        
                        // Обработка команды
                        if message.is_command {
                            Self::process_command(&bot_instance, &commands, &message).await;
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    async fn process_command(
        bot_instance: &Arc<bot::TelegramBot>,
        commands: &Arc<RwLock<HashMap<String, BotCommand>>>,
        message: &BotMessage,
    ) {
        let command_parts: Vec<&str> = message.text.split_whitespace().collect();
        if command_parts.is_empty() {
            return;
        }
        
        let command_name = command_parts[0];
        let commands_read = commands.read().await;
        
        if let Some(command) = commands_read.get(command_name) {
            // Проверка прав администратора
            if command.requires_admin && !Self::is_admin(message.user_id) {
                let _ = bot_instance.send_message(message.chat_id, "❌ Access denied. Admin privileges required.").await;
                return;
            }
            
            // Выполнение команды
            match command.handler.as_str() {
                "handle_start" => {
                    let response = "🤖 Welcome to PoolAI Bot!\n\nUse /help to see available commands.";
                    let _ = bot_instance.send_message(message.chat_id, response).await;
                }
                "handle_help" => {
                    let response = Self::generate_help_message(&commands_read);
                    let _ = bot_instance.send_message(message.chat_id, &response).await;
                }
                "handle_status" => {
                    let status = Self::get_system_status_static().await;
                    let _ = bot_instance.send_message(message.chat_id, &status).await;
                }
                "handle_metrics" => {
                    let metrics = Self::get_metrics_summary_static().await;
                    let _ = bot_instance.send_message(message.chat_id, &metrics).await;
                }
                "handle_restart" => {
                    let _ = bot_instance.send_message(message.chat_id, "🔄 Restarting system...").await;
                    // В реальной реализации здесь будет вызов API для перезапуска
                }
                "handle_shutdown" => {
                    let _ = bot_instance.send_message(message.chat_id, "🛑 Shutting down system...").await;
                    // В реальной реализации здесь будет вызов API для выключения
                }
                "handle_scale" => {
                    let _ = bot_instance.send_message(message.chat_id, "📈 Scaling workers...").await;
                    // В реальной реализации здесь будет вызов API для масштабирования
                }
                _ => {
                    let _ = bot_instance.send_message(message.chat_id, "❌ Unknown command.").await;
                }
            }
        } else {
            let _ = bot_instance.send_message(message.chat_id, "❌ Unknown command. Use /help for available commands.").await;
        }
    }

    async fn start_notification_service(&self) -> Result<(), AppError> {
        let bot_instance = self.bot_instance.clone();
        let notifications = self.notifications.clone();
        let allowed_users = self.config.allowed_users.clone();
        let interval_minutes = self.config.notification_interval_minutes;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_minutes * 60));
            
            loop {
                interval.tick().await;
                
                // Отправка периодических уведомлений
                for user_id in &allowed_users {
                    let status = Self::get_system_status_static().await;
                    let _ = bot_instance.send_message(*user_id, &status).await;
                }
            }
        });
        
        Ok(())
    }

    fn generate_help_message(commands: &HashMap<String, BotCommand>) -> String {
        let mut help_text = "🤖 PoolAI Bot Commands:\n\n".to_string();
        
        for (command_name, command) in commands.iter() {
            let admin_marker = if command.requires_admin { " (Admin)" } else { "" };
            help_text.push_str(&format!("{} - {}{}\n", command_name, command.description, admin_marker));
        }
        
        help_text.push_str("\n💡 Use any command to get more information.");
        help_text
    }

    async fn get_system_status_static() -> String {
        // Статическая версия для использования в async контексте
        "🤖 PoolAI System Status\n\n\
        📊 Overall Health: 95%\n\
        🔧 Active Workers: 8\n\
        💾 Memory Usage: 6.2GB / 16GB\n\
        🎮 GPU Utilization: 75%\n\
        ⚡ CPU Usage: 45%\n\
        🌐 Network: 125 Mbps\n\
        ⏰ Uptime: 2d 15h 30m\n\
        📈 Total Requests: 15,432\n\
        ✅ Success Rate: 98.5%".to_string()
    }

    async fn get_metrics_summary_static() -> String {
        // Статическая версия для использования в async контексте
        "📈 PoolAI Metrics Summary\n\n\
        🚀 Requests/sec: 45.2\n\
        ⏱️ Avg Response Time: 250ms\n\
        🎯 GPU Utilization: 75.5%\n\
        💾 Memory Usage: 6.2GB\n\
        🔥 Temperature: 65°C\n\
        ⚡ Power: 350W\n\
        📊 Error Rate: 0.02%".to_string()
    }

    fn is_admin(user_id: i64) -> bool {
        // Заглушка для проверки прав администратора
        // В реальной реализации здесь будет проверка списка админов
        user_id == 123456789 // Пример ID администратора
    }
} 