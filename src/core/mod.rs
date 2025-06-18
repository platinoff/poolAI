pub mod main;
pub mod lib;
pub mod state;
pub mod config;
pub mod error;
pub mod utils;
pub mod model_interface;

pub use main::*;
pub use lib::*;
pub use state::*;
pub use config::*;
pub use error::*;
pub use utils::*;
pub use model_interface::*;

use std::error::Error;

/// Инициализация core модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing core module");
    Ok(())
}

/// Остановка core модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down core module");
    Ok(())
}

/// Проверка здоровья core модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Core module health check passed");
    Ok(())
} 