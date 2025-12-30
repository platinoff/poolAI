//! PoolAI public API (library root)
//!
//! This library provides a comprehensive AI mining pool management system
//! with advanced runtime management, monitoring, and enterprise features.

// Core modules
pub mod core;
pub mod monitoring;
pub mod platform;
pub mod pool;
pub mod rewards;
pub mod runtime;
pub mod version;

// Public modules (exposed for external use)
pub mod libs;
pub mod network;
pub mod raid;
pub mod tgbot;
pub mod ui;
pub mod vm;

// Re-export core types for convenient access
pub use core::config::PoolAIConfig;
pub use core::error::AppError;
pub use core::model_interface::{ModelInfo, ModelInterface};
pub use core::state::AppState;

// Re-export pool types
pub use pool::LoadBalancingStrategy;
pub use pool::{Pool, PoolConfig, PoolMetrics};

// Re-export monitoring types
pub use monitoring::MetricsCollector;
pub use monitoring::{Alert, AlertSeverity, HistoricalData, Monitoring, SystemStatus};

// Re-export runtime types
pub use runtime::{
    CacheManager, HealthMonitor, ProcessManager, ResourceOrchestrator, StorageManager, TaskQueue,
    TaskScheduler,
};
pub use runtime::{RuntimeConfig, RuntimeManager, RuntimeStatus};

// Re-export rewards types
pub use rewards::{Reward, RewardLevel, RewardSystem, RewardType, UserProgress};

// Re-export platform types
pub use platform::{get_gpu_info, GpuInfo};

// Re-export library management types
pub use libs::{LibraryInfo, LibraryManager, LibraryStatus, LibraryType};

// Re-export VM types
pub use vm::{
    AutoRecoveryConfig, FilesystemIsolationConfig, FilesystemIsolator, NetworkIsolationConfig,
    NetworkIsolator, PlatformFilesystemIsolator, PlatformNetworkIsolator, ResourceAlertThresholds,
    ResourceLimits, ResourceUsage, ResourceUsageHistoryEntry, ResourceUsageStats, VmInstance,
    VmIsolation, VmManager, VmResources, VmStatus,
};

// Re-export RAID types
pub use raid::{ArtifactRef, RaidConfig, RaidManager, RaidMode, RaidNode};

// Re-export network functions
pub use network::start_server;

// Re-export tgbot functions
pub use tgbot::{send_notification, start_bot};

// Re-export version information
pub use version::{APP_VERSION, BUILD_TIME};
