pub mod burstraid;
pub mod smallworld;
pub mod lib;
pub mod admin;
pub mod vm;
pub mod worker_interface;
pub mod config;
pub mod network;
pub mod storage;
pub mod worker;
pub mod mount;

pub use burstraid::*;
pub use smallworld::*;
pub use lib::*;
pub use admin::*;
pub use vm::*;
pub use worker_interface::*;
pub use config::*;
pub use network::*;
pub use storage::*;
pub use worker::*;
pub use mount::*;

use std::error::Error;

/// Инициализация raid модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing raid module");
    Ok(())
}

/// Остановка raid модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down raid module");
    Ok(())
}

/// Проверка здоровья raid модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("RAID module health check passed");
    Ok(())
} 