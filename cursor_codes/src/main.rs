use actix_web::{web, App, HttpServer, middleware, Responder};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::CursorCore;
use crate::raid::BurstRaidManager;
use crate::pool::{PoolManager, PoolConfig, PoolStats};
use log::{info, error, LevelFilter};
use env_logger::Builder;
use tokio::signal;
use std::process;
use actix_web::middleware::Logger;
use actix_web::http::header;
use crate::pool::reward_system::{RewardSystem, ActivityType};
use crate::core::{
    error::CursorError,
    lib_manager::{LibraryManager, LibraryStatus},
};
use crate::admin::{
    AdminPanel,
    get_pool_stats,
    get_worker_stats,
    update_pool_config,
    add_worker,
    remove_worker,
    get_reward_stats,
    toggle_maintenance_mode,
};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::env;

// Импорты из новых модулей
use crate::core::state::AppState;
use crate::core::config::AppConfig;
use crate::network::tls::TlsManager;
use crate::platform::model::ModelSystem;
use crate::network::network::NetworkSystem;
use crate::runtime::storage::StorageSystem;
use crate::runtime::cache::CacheSystem;
use crate::runtime::queue::QueueSystem;
use crate::runtime::scheduler::SchedulerSystem;
use crate::monitoring::monitor::MonitorSystem;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::metrics::MetricsSystem;
use crate::monitoring::alert::AlertSystem;
use crate::core::error::ErrorSystem;
use crate::core::config::ConfigSystem;
use crate::core::utils::UtilsSystem;

// ... остальной код без изменений ... 