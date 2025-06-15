use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Telegram error: {0}")]
    TelegramError(String),
    #[error("Worker error: {0}")]
    WorkerError(String),
    #[error("Hardware error: {0}")]
    HardwareError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_cores: u32,
    pub total_memory: u64,
    pub gpu_model: Option<String>,
    pub gpu_memory: Option<u64>,
    pub storage_space: u64,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Mobile,
    TV,
    Console,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInterface {
    pub worker_id: String,
    pub hardware_info: HardwareInfo,
    pub allocated_cores: u32,
    pub allocated_memory: u64,
    pub allocated_storage: u64,
    pub seeds: Vec<String>,
    pub metrics: WorkerMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub gpu_usage: Option<f32>,
    pub storage_usage: f32,
    pub network_usage: f32,
    pub uptime: u64,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Worker interface commands:")]
pub enum Command {
    #[command(description = "Show worker status and metrics")]
    Status,
    #[command(description = "Configure hardware allocation")]
    Configure { cores: u32, memory: u64, storage: u64 },
    #[command(description = "Show available seeds")]
    Seeds,
    #[command(description = "Start/stop worker")]
    Toggle,
    #[command(description = "Show hardware information")]
    Hardware,
}

pub struct WorkerInterfaceManager {
    interfaces: Arc<Mutex<HashMap<String, WorkerInterface>>>,
    bot: Bot,
}

impl WorkerInterfaceManager {
    pub fn new(bot_token: String) -> Self {
        Self {
            interfaces: Arc::new(Mutex::new(HashMap::new())),
            bot: Bot::new(bot_token),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        // Initialize bot commands
        self.bot.set_my_commands(Command::bot_commands()).await
            .map_err(|e| Error::TelegramError(e.to_string()))?;
        Ok(())
    }

    pub async fn register_worker(&self, worker_id: String, hardware_info: HardwareInfo) -> Result<(), Error> {
        let mut interfaces = self.interfaces.lock().await;
        
        if interfaces.contains_key(&worker_id) {
            return Err(Error::WorkerError(format!("Worker {} already registered", worker_id)));
        }

        let interface = WorkerInterface {
            worker_id: worker_id.clone(),
            hardware_info,
            allocated_cores: 0,
            allocated_memory: 0,
            allocated_storage: 0,
            seeds: Vec::new(),
            metrics: WorkerMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                gpu_usage: None,
                storage_usage: 0.0,
                network_usage: 0.0,
                uptime: 0,
                last_update: chrono::Utc::now(),
            },
        };

        interfaces.insert(worker_id, interface);
        Ok(())
    }

    pub async fn update_metrics(&self, worker_id: &str, metrics: WorkerMetrics) -> Result<(), Error> {
        let mut interfaces = self.interfaces.lock().await;
        
        if let Some(interface) = interfaces.get_mut(worker_id) {
            interface.metrics = metrics;
            Ok(())
        } else {
            Err(Error::WorkerError(format!("Worker {} not found", worker_id)))
        }
    }

    pub async fn configure_hardware(&self, worker_id: &str, cores: u32, memory: u64, storage: u64) -> Result<(), Error> {
        let mut interfaces = self.interfaces.lock().await;
        
        if let Some(interface) = interfaces.get_mut(worker_id) {
            if cores > interface.hardware_info.cpu_cores {
                return Err(Error::HardwareError("Requested cores exceed available cores".to_string()));
            }
            if memory > interface.hardware_info.total_memory {
                return Err(Error::HardwareError("Requested memory exceeds available memory".to_string()));
            }
            if storage > interface.hardware_info.storage_space {
                return Err(Error::HardwareError("Requested storage exceeds available storage".to_string()));
            }

            interface.allocated_cores = cores;
            interface.allocated_memory = memory;
            interface.allocated_storage = storage;
            Ok(())
        } else {
            Err(Error::WorkerError(format!("Worker {} not found", worker_id)))
        }
    }

    pub async fn add_seed(&self, worker_id: &str, seed: String) -> Result<(), Error> {
        let mut interfaces = self.interfaces.lock().await;
        
        if let Some(interface) = interfaces.get_mut(worker_id) {
            interface.seeds.push(seed);
            Ok(())
        } else {
            Err(Error::WorkerError(format!("Worker {} not found", worker_id)))
        }
    }

    pub async fn handle_command(&self, msg: Message, cmd: Command) -> Result<(), Error> {
        match cmd {
            Command::Status => {
                let worker_id = msg.from().unwrap().id.to_string();
                let interfaces = self.interfaces.lock().await;
                
                if let Some(interface) = interfaces.get(&worker_id) {
                    let status = format!(
                        "Worker Status:\n\
                         CPU Usage: {:.1}%\n\
                         Memory Usage: {:.1}%\n\
                         Storage Usage: {:.1}%\n\
                         Uptime: {} seconds\n\
                         Allocated Cores: {}\n\
                         Allocated Memory: {} MB\n\
                         Allocated Storage: {} GB",
                        interface.metrics.cpu_usage * 100.0,
                        interface.metrics.memory_usage * 100.0,
                        interface.metrics.storage_usage * 100.0,
                        interface.metrics.uptime,
                        interface.allocated_cores,
                        interface.allocated_memory / 1024 / 1024,
                        interface.allocated_storage / 1024 / 1024 / 1024
                    );
                    
                    self.bot.send_message(msg.chat.id, status).await
                        .map_err(|e| Error::TelegramError(e.to_string()))?;
                }
            }
            Command::Configure { cores, memory, storage } => {
                let worker_id = msg.from().unwrap().id.to_string();
                self.configure_hardware(&worker_id, cores, memory, storage).await?;
                
                self.bot.send_message(msg.chat.id, "Hardware configuration updated").await
                    .map_err(|e| Error::TelegramError(e.to_string()))?;
            }
            Command::Seeds => {
                let worker_id = msg.from().unwrap().id.to_string();
                let interfaces = self.interfaces.lock().await;
                
                if let Some(interface) = interfaces.get(&worker_id) {
                    let seeds = interface.seeds.join("\n");
                    self.bot.send_message(msg.chat.id, format!("Available seeds:\n{}", seeds)).await
                        .map_err(|e| Error::TelegramError(e.to_string()))?;
                }
            }
            Command::Hardware => {
                let worker_id = msg.from().unwrap().id.to_string();
                let interfaces = self.interfaces.lock().await;
                
                if let Some(interface) = interfaces.get(&worker_id) {
                    let hardware = format!(
                        "Hardware Information:\n\
                         Device Type: {:?}\n\
                         CPU Cores: {}\n\
                         Total Memory: {} GB\n\
                         GPU Model: {:?}\n\
                         GPU Memory: {:?} GB\n\
                         Storage Space: {} GB",
                        interface.hardware_info.device_type,
                        interface.hardware_info.cpu_cores,
                        interface.hardware_info.total_memory / 1024 / 1024 / 1024,
                        interface.hardware_info.gpu_model,
                        interface.hardware_info.gpu_memory.map(|m| m / 1024 / 1024 / 1024),
                        interface.hardware_info.storage_space / 1024 / 1024 / 1024
                    );
                    
                    self.bot.send_message(msg.chat.id, hardware).await
                        .map_err(|e| Error::TelegramError(e.to_string()))?;
                }
            }
            Command::Toggle => {
                // Implement worker start/stop logic
            }
        }
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Cleanup and close all connections
        Ok(())
    }
} 