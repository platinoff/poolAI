//! PoolAI public API (library root)
//!
//! This library provides a comprehensive AI mining pool management system
//! with advanced runtime management, monitoring, and enterprise features.

// Core modules
pub mod core;
pub mod pool;
pub mod monitoring;
pub mod platform;
pub mod version;
pub mod rewards;
pub mod runtime;

// Public modules (exposed for external use)
pub mod network;
pub mod tgbot;
pub mod libs;
pub mod vm;
pub mod raid;
pub mod ui;

// Re-export core types for convenient access
pub use core::model_interface::{ModelInterface, ModelInfo};
pub use core::config::PoolAIConfig;
pub use core::state::AppState;
pub use core::error::AppError;

// Re-export pool types
pub use pool::{Pool, PoolConfig, PoolMetrics};
pub use pool::LoadBalancingStrategy;

// Re-export monitoring types
pub use monitoring::MetricsCollector;
pub use monitoring::{Monitoring, Alert, AlertSeverity, SystemStatus, HistoricalData};

// Re-export runtime types
pub use runtime::{RuntimeManager, RuntimeConfig, RuntimeStatus};
pub use runtime::{
    TaskScheduler, TaskQueue, CacheManager, StorageManager,
    ProcessManager, ResourceOrchestrator, HealthMonitor
};

// Re-export rewards types
pub use rewards::{RewardSystem, RewardType, RewardLevel, Reward, UserProgress};

// Re-export platform types
pub use platform::{GpuInfo, get_gpu_info};

// Re-export library management types
pub use libs::{LibraryManager, LibraryInfo, LibraryStatus, LibraryType};

// Re-export VM types
pub use vm::{VmManager, VmInstance, VmStatus, VmResources, VmIsolation, ResourceLimits, ResourceUsage};

// Re-export RAID types
pub use raid::{RaidManager, RaidConfig, RaidMode, RaidNode, ArtifactRef};

// Re-export network functions
pub use network::start_server;

// Re-export tgbot functions
pub use tgbot::{start_bot, send_notification};

// Re-export version information
pub use version::{APP_VERSION, BUILD_TIME};
