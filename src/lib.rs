//! PoolAI public API (library root)

mod core;
mod pool;
mod monitoring;
pub mod network;
pub mod tgbot;
mod platform;
mod version;
mod rewards;
mod runtime;

// Публичные интерфейсы (экспортируем только то, что нужно)

pub use core::model_interface::{ModelInterface, ModelInfo};
pub use core::config::PoolAIConfig;
pub use core::state::AppState;
pub use pool::{Pool, PoolConfig};
pub use pool::LoadBalancingStrategy;
pub use monitoring::MetricsCollector;
pub use network::start_server;
pub use platform::*;
pub use tgbot::*;
pub use version::{APP_VERSION, BUILD_TIME};
