pub mod linux;
pub mod windows;
pub mod unix;
pub mod model;
pub mod soladdr;
pub mod lmrouter;
pub mod lib;
pub mod error;

pub use linux::*;
pub use windows::*;
pub use unix::*;
pub use model::*;
pub use soladdr::*;
pub use lmrouter::*;
pub use lib::*;
pub use error::*;

use std::path::PathBuf;
use thiserror::Error;
use std::sync::Arc;
use parking_lot::RwLock;
use std::error::Error;

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Failed to create service: {0}")]
    ServiceCreationError(String),
    #[error("Failed to start service: {0}")]
    ServiceStartError(String),
    #[error("Failed to stop service: {0}")]
    ServiceStopError(String),
    #[error("Failed to get service status: {0}")]
    ServiceStatusError(String),
    #[error("Failed to create daemon: {0}")]
    DaemonCreationError(String),
    #[error("Failed to get system info: {0}")]
    SystemInfoError(String),
}

#[async_trait::async_trait]
pub trait PlatformService: Send + Sync {
    async fn install(&self) -> Result<(), PlatformError>;
    async fn uninstall(&self) -> Result<(), PlatformError>;
    async fn start(&self) -> Result<(), PlatformError>;
    async fn stop(&self) -> Result<(), PlatformError>;
    async fn status(&self) -> Result<String, PlatformError>;
}

#[async_trait::async_trait]
pub trait SystemInfo: Send + Sync {
    fn get_os_name(&self) -> String;
    fn get_os_version(&self) -> String;
    fn get_architecture(&self) -> String;
    async fn get_memory_info(&self) -> Result<MemoryInfo, PlatformError>;
    async fn get_cpu_info(&self) -> Result<CpuInfo, PlatformError>;
    async fn get_disk_info(&self) -> Result<DiskInfo, PlatformError>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    #[serde(skip)]
    pub cache: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
    pub frequency: u64,
    pub usage: f32,
    #[serde(skip)]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub mount_point: PathBuf,
    #[serde(skip)]
    pub fs_type: Option<String>,
}

#[cfg(windows)]
pub use windows::{WindowsService, WindowsSystemInfo};

#[cfg(unix)]
pub use unix::{UnixService, UnixSystemInfo};

pub struct PlatformManager {
    service: Arc<RwLock<Box<dyn PlatformService>>>,
    system_info: Arc<RwLock<Box<dyn SystemInfo>>>,
}

impl PlatformManager {
    pub fn new() -> Self {
        Self {
            service: Arc::new(RwLock::new(create_service("cursor-service"))),
            system_info: Arc::new(RwLock::new(create_system_info())),
        }
    }

    pub async fn get_service_status(&self) -> Result<String, PlatformError> {
        self.service.read().status().await
    }

    pub async fn get_memory_info(&self) -> Result<MemoryInfo, PlatformError> {
        self.system_info.read().get_memory_info().await
    }

    pub async fn get_cpu_info(&self) -> Result<CpuInfo, PlatformError> {
        self.system_info.read().get_cpu_info().await
    }

    pub async fn get_disk_info(&self) -> Result<DiskInfo, PlatformError> {
        self.system_info.read().get_disk_info().await
    }
}

pub fn create_service(name: &str) -> Box<dyn PlatformService> {
    #[cfg(windows)]
    {
        Box::new(WindowsService::new(name))
    }
    #[cfg(unix)]
    {
        Box::new(UnixService::new(name))
    }
}

pub fn create_system_info() -> Box<dyn SystemInfo> {
    #[cfg(windows)]
    {
        Box::new(WindowsSystemInfo::new())
    }
    #[cfg(unix)]
    {
        Box::new(UnixSystemInfo::new())
    }
}

/// Инициализация platform модуля
pub async fn initialize() -> Result<(), Box<dyn Error>> {
    log::info!("Initializing platform module");
    Ok(())
}

/// Остановка platform модуля
pub async fn shutdown() -> Result<(), Box<dyn Error>> {
    log::info!("Shutting down platform module");
    Ok(())
}

/// Проверка здоровья platform модуля
pub async fn health_check() -> Result<(), Box<dyn Error>> {
    log::debug!("Platform module health check passed");
    Ok(())
} 