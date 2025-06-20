pub mod network;
pub mod bridges;
pub mod loadbalancer;
pub mod tls;
pub mod api;
pub mod pool_cok;
pub mod smallworld;

pub use network::*;
pub use bridges::*;
pub use loadbalancer::*;
pub use tls::*;
pub use api::*;
pub use pool_cok::*;
pub use smallworld::*;

use std::error::Error;

/// Инициализация network модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing network module");
    Ok(())
}

/// Остановка network модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down network module");
    Ok(())
}

/// Проверка здоровья network модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Network module health check passed");
    Ok(())
} 