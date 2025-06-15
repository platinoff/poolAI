mod error;
mod windows;
mod unix;

pub use error::PlatformError;
pub use windows::WindowsService;
pub use windows::WindowsSystemInfo;
pub use unix::UnixService;
pub use unix::UnixSystemInfo;

use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait PlatformService: Send + Sync {
    async fn install(&self) -> Result<(), PlatformError>;
    async fn uninstall(&self) -> Result<(), PlatformError>;
    async fn start(&self) -> Result<(), PlatformError>;
    async fn stop(&self) -> Result<(), PlatformError>;
    async fn status(&self) -> Result<String, PlatformError>;
}

#[async_trait]
pub trait SystemInfo: Send + Sync {
    fn get_os_name(&self) -> String;
    fn get_os_version(&self) -> String;
    fn get_architecture(&self) -> String;
    async fn get_memory_info(&self) -> Result<MemoryInfo, PlatformError>;
    async fn get_cpu_info(&self) -> Result<CpuInfo, PlatformError>;
    async fn get_disk_info(&self) -> Result<DiskInfo, PlatformError>;
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub cache: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
    pub frequency: u64,
    pub usage: f32,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub mount_point: PathBuf,
    pub fs_type: Option<String>,
}

pub fn create_service() -> Box<dyn PlatformService> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsService::new("CursorService"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Box::new(UnixService::new("cursor-service"))
    }
}

pub fn create_system_info() -> Box<dyn SystemInfo> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsSystemInfo::new())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Box::new(UnixSystemInfo::new())
    }
} 