pub mod lib_manager;
pub mod tokenizer;
pub mod file_mirror;
pub mod gpu;
pub mod model;
pub mod tuning;

pub use lib_manager::*;
pub use tokenizer::*;
pub use file_mirror::*;
pub use gpu::*;
pub use model::*;
pub use tuning::*;

use std::error::Error;

/// Инициализация libs модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing libs module");
    Ok(())
}

/// Остановка libs модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down libs module");
    Ok(())
}

/// Проверка здоровья libs модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Libs module health check passed");
    Ok(())
} 