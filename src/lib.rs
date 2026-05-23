//! PoolAI public API (library root)
//!
//! This library provides a comprehensive AI mining pool management system
//! with advanced runtime management, monitoring, and enterprise features.
//!
//! # Quick Start
//!
//! ## Starting the server
//!
//! ```no_run
//! use poolai::network::start_server;
//! use poolai::AppState;
//! use std::sync::Arc;
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Start the server with default configuration
//! let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//! let app_state = Arc::new(AppState::new());
//! start_server(addr, app_state).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Creating a VM instance
//!
//! ```no_run
//! use poolai::vm::{VmManager, VmResources, VmIsolation};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = VmManager::new();
//!
//! let instance = manager.create_instance(
//!     "my-vm".to_string(),
//!     VmResources::default(),
//!     VmIsolation::ProcessSandbox,
//! ).await?;
//!
//! println!("Created VM: {:?}", instance.id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Storing an artifact in RAID
//!
//! ```no_run
//! use poolai::raid::{RaidManager, RaidConfig, RaidMode};
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = RaidConfig {
//!     mode: RaidMode::Local,
//!     base_path: PathBuf::from("./data/raid"),
//!     quota_bytes: Some(10 * 1024 * 1024 * 1024),
//!     retention_days: Some(30),
//!     gc_on_startup: true,
//! };
//!
//! let manager = RaidManager::new(config);
//! let artifact_id = manager.put_artifact("my-artifact", b"data").await?;
//! println!("Stored artifact: {:?}", artifact_id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Using Enterprise Features
//!
//! ```rust,ignore
//! // Note: Requires the "enterprise" feature to be enabled
//! # #[cfg(feature = "enterprise")]
//! use poolai::enterprise::{EnterpriseManager, EnterpriseConfig};
//!
//! # #[cfg(feature = "enterprise")]
//! # async fn example() -> Result<(), poolai::AppError> {
//! # let config = EnterpriseConfig::default();
//! # let manager = EnterpriseManager::new(config);
//! # manager.initialize().await?;
//! // Use enterprise features...
//! # manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using Cloud Integration
//!
//! ```rust,ignore
//! // Note: Requires the "cloud" feature to be enabled
//! # #[cfg(feature = "cloud")]
//! use poolai::cloud::{CloudManager, CloudConfig};
//!
//! # #[cfg(feature = "cloud")]
//! # async fn example() -> Result<(), poolai::AppError> {
//! # let config = CloudConfig {
//! #     kubernetes_enabled: true,
//! #     kubernetes_namespace: "poolai".to_string(),
//! #     ..Default::default()
//! # };
//! # let manager = CloudManager::new(config);
//! # manager.initialize().await?;
//! # manager.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Modules
//!
//! - [`core`] - Core functionality (config, error handling, model interface)
//! - [`services`] - Service layer orchestration above domains (RAID, VM, libraries, …; expanding)
//! - [`pool`] - Worker pool management
//! - [`monitoring`] - System monitoring and metrics
//! - [`network`] - REST API and WebSocket server
//! - [`observability`] - HTTP tracing + optional OpenTelemetry (FM-038)
//! - [`vm`] - Virtual machine instance management
//! - [`raid`] - Distributed artifact storage
//! - [`grid`] - Grid envelope v1 (Job/Result/MemoryShard/PeerStatus wire types)
//! - [`job`] - Job layer wire types (JobSpec, JobStatus)
//! - [`memory`] - Memory shard references (RAID / Grid)
//! - [`ui`] - Web dashboard interface
//! - [`runtime`] - Advanced runtime management (Stage 4.1)
//! - [`platform`] - Cross-platform GPU and system information
//! - [`tgbot`] - Telegram bot integration (planned)
//! - [`enterprise`] - Enterprise features (multi-tenancy, audit, security, monitoring)
//! - [`ml`] - Stage 4.4 AI/ML (Model Optimization, AutoML, Federated Learning; optional, feature `ml`)
//! - [`cloud`] - Cloud integration (Kubernetes, AWS, Azure, GCP)

// Core modules
pub mod core;
pub mod grid;
pub mod job;
pub mod memory;
pub mod monitoring;
pub mod observability;
pub mod platform;
pub mod pool;
pub mod rewards;
pub mod runtime;
pub mod services;
pub mod version;
pub mod workers;

// Public modules (exposed for external use)
pub mod libs;
pub mod network;
pub mod raid;
pub mod tgbot;
pub mod ui;
pub mod vm;

#[cfg(feature = "enterprise")]
pub mod enterprise;

#[cfg(feature = "ml")]
pub mod ml;

#[cfg(feature = "cloud")]
pub mod cloud;

// Re-export core types for convenient access
pub use core::config::PoolAIConfig;
pub use core::error::AppError;
pub use core::model_interface::{ModelInfo, ModelInterface};
pub use core::state::AppState;

// Re-export pool types
pub use pool::LoadBalancingStrategy;
pub use pool::{Pool as PoolType, PoolConfig, PoolMetrics};

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
pub use raid::{
    admin::RaidAdmin, ArtifactRef, RaidConfig, RaidManager, RaidMode, RaidNode, StrategyStatus,
};

// Re-export network functions
pub use network::start_server;

// Re-export tgbot functions
pub use tgbot::{send_notification, start_bot};

// Re-export version information
pub use version::{APP_VERSION, BUILD_TIME};

// Re-export enterprise types (if feature enabled)
// Re-export cloud types
#[cfg(feature = "cloud")]
pub use cloud::{CloudConfig, CloudManager};

#[cfg(feature = "enterprise")]
pub use enterprise::{
    audit::{AuditEvent, AuditLevel, AuditLogger, AuditQueryFilters},
    monitoring::MonitoringManager,
    multi_tenancy::{Tenant, TenantConfig, TenantManager},
    security::SecurityManager,
    EnterpriseManager,
};
