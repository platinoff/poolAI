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
        // Initialize bot
        self.bot_instance.initialize().await?;
        
        // Register commands
        self.register_commands().await?;
        
        // Start message processing
        self.start_message_processing().await?;
        
        // Start notifications if enabled
        if self.config.enable_notifications {
            self.start_notification_service().await?;
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Shutdown bot
        self.bot_instance.shutdown().await?;
        
        Ok(())
    }

    pub async fn send_message(&self, chat_id: i64, message: &str) -> Result<(), AppError> {
        // Send message through bot
        self.bot_instance.send_message(chat_id, message).await?;
        
        Ok(())
    }

    pub async fn send_notification(&self, chat_id: i64, message: &str, notification_type: NotificationType) -> Result<(), AppError> {
        // Create notification
        let notification = BotNotification {
            chat_id,
            message: message.to_string(),
            notification_type,
            timestamp: std::time::Instant::now(),
            sent: false,
        };
        
        // Add to notification queue
        {
            let mut notifications = self.notifications.write().await;
            notifications.push(notification);
        }
        
        // Send notification
        self.send_message(chat_id, message).await?;
        
        // Mark as sent
        {
            let mut notifications = self.notifications.write().await;
            if let Some(notification) = notifications.iter_mut().last() {
                notification.sent = true;
            }
        }
        
        Ok(())
    }

    pub async fn broadcast_message(&self, message: &str) -> Result<(), AppError> {
        // Send message to all allowed users
        for user_id in &self.config.allowed_users {
            self.send_message(*user_id, message).await?;
        }
        
        Ok(())
    }

    pub async fn get_system_status(&self) -> Result<String, AppError> {
        // Get system status for Telegram
        // In real implementation, this would integrate with monitoring
        
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
        // Get metrics summary
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
        
        // Basic commands
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
            description: "Show system status".to_string(),
            handler: "handle_status".to_string(),
            requires_admin: false,
        });
        
        commands.insert("/metrics".to_string(), BotCommand {
            name: "metrics".to_string(),
            description: "Show system metrics".to_string(),
            handler: "handle_metrics".to_string(),
            requires_admin: false,
        });
        
        // Admin commands
        if self.config.enable_admin_commands {
            commands.insert("/restart".to_string(), BotCommand {
                name: "restart".to_string(),
                description: "Restart system".to_string(),
                handler: "handle_restart".to_string(),
                requires_admin: true,
            });
            
            commands.insert("/shutdown".to_string(), BotCommand {
                name: "shutdown".to_string(),
                description: "Shutdown system".to_string(),
                handler: "handle_shutdown".to_string(),
                requires_admin: true,
            });
        }
        
        Ok(())
    }

    async fn start_message_processing(&self) -> Result<(), AppError> {
        let bot_instance = self.bot_instance.clone();
        let commands = self.commands.clone();
        
        tokio::spawn(async move {
            // Start message processing loop
            loop {
                // In real implementation, this would poll for new messages
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                
                // Process incoming messages
                // This is a stub - in real implementation, this would handle actual Telegram messages
            }
        });
        
        Ok(())
    }

    async fn process_command(
        bot_instance: &Arc<bot::TelegramBot>,
        commands: &Arc<RwLock<HashMap<String, BotCommand>>>,
        message: &BotMessage,
    ) {
        let commands = commands.read().await;
        
        if let Some(command) = commands.get(&message.text) {
            let response = match command.handler.as_str() {
                "handle_start" => "🤖 Welcome to PoolAI Bot!\nUse /help to see available commands.".to_string(),
                "handle_help" => Self::generate_help_message(&commands),
                "handle_status" => Self::get_system_status_static().await,
                "handle_metrics" => Self::get_metrics_summary_static().await,
                "handle_restart" => {
                    if Self::is_admin(message.user_id) {
                        "🔄 Restarting system...".to_string()
                    } else {
                        "❌ Access denied. Admin privileges required.".to_string()
                    }
                }
                "handle_shutdown" => {
                    if Self::is_admin(message.user_id) {
                        "🛑 Shutting down system...".to_string()
                    } else {
                        "❌ Access denied. Admin privileges required.".to_string()
                    }
                }
                _ => "❓ Unknown command. Use /help for available commands.".to_string(),
            };
            
            if let Err(e) = bot_instance.send_message(message.chat_id, &response).await {
                log::error!("Failed to send response: {}", e);
            }
        }
    }

    async fn start_notification_service(&self) -> Result<(), AppError> {
        let bot_instance = self.bot_instance.clone();
        let allowed_users = self.config.allowed_users.clone();
        let interval = self.config.notification_interval_minutes;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(tokio::time::Duration::from_secs(interval * 60));
            
            loop {
                interval_timer.tick().await;
                
                // Send periodic status updates
                let status = Self::get_system_status_static().await;
                
                for user_id in &allowed_users {
                    if let Err(e) = bot_instance.send_message(*user_id, &status).await {
                        log::error!("Failed to send notification to {}: {}", user_id, e);
                    }
                }
            }
        });
        
        Ok(())
    }

    fn generate_help_message(commands: &HashMap<String, BotCommand>) -> String {
        let mut help_text = "📋 Available Commands:\n\n".to_string();
        
        for (command, info) in commands.iter() {
            let admin_marker = if info.requires_admin { " (Admin)" } else { "" };
            help_text.push_str(&format!("{} - {}{}\n", command, info.description, admin_marker));
        }
        
        help_text
    }

    async fn get_system_status_static() -> String {
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
        // In real implementation, this would check against admin user list
        user_id == 123456789 // Stub admin ID
    }
} 