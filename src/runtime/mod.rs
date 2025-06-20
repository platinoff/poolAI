pub mod worker;
pub mod scheduler;
pub mod queue;
pub mod cache;
pub mod storage;
pub mod instance;

pub use worker::*;
pub use scheduler::*;
pub use queue::*;
pub use cache::*;
pub use storage::*;
pub use instance::*;

use std::error::Error;

/// Инициализация runtime модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing runtime module");
    Ok(())
}

/// Остановка runtime модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down runtime module");
    Ok(())
}

/// Проверка здоровья runtime модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Runtime module health check passed");
    Ok(())
} 