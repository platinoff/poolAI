use super::{PlatformError, PlatformService, SystemInfo, MemoryInfo, CpuInfo, DiskInfo};
use std::path::PathBuf;
use windows_service::{
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};
use winapi::um::{
    sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX, GetSystemInfo, SYSTEM_INFO},
    winnt::SYSTEM_INFO,
    pdh::{PdhOpenQueryW, PdhAddCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PDH_FMT_COUNTERVALUE, PDH_FMT_LONG},
};
use std::mem::size_of;
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time;

pub struct WindowsService {
    name: String,
    running: AtomicBool,
}

impl WindowsService {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            running: AtomicBool::new(false),
        }
    }

    fn to_wide_string(s: &str) -> Vec<u16> {
        OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
}

#[async_trait::async_trait]
impl PlatformService for WindowsService {
    async fn install(&self) -> Result<(), PlatformError> {
        // Windows service installation logic
        Ok(())
    }

    async fn uninstall(&self) -> Result<(), PlatformError> {
        // Windows service uninstallation logic
        Ok(())
    }

    async fn start(&self) -> Result<(), PlatformError> {
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn stop(&self) -> Result<(), PlatformError> {
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    async fn status(&self) -> Result<String, PlatformError> {
        if self.running.load(Ordering::SeqCst) {
            Ok("Running".to_string())
        } else {
            Ok("Stopped".to_string())
        }
    }
}

pub struct WindowsSystemInfo {
    cpu_query: *mut std::ffi::c_void,
    cpu_counter: *mut std::ffi::c_void,
}

impl WindowsSystemInfo {
    pub fn new() -> Self {
        let mut query = null_mut();
        let mut counter = null_mut();
        
        unsafe {
            PdhOpenQueryW(null_mut(), 0, &mut query);
            let counter_path = Self::to_wide_string("\\Processor(_Total)\\% Processor Time");
            PdhAddCounterW(query, counter_path.as_ptr(), 0, &mut counter);
        }

        Self {
            cpu_query: query,
            cpu_counter: counter,
        }
    }

    fn to_wide_string(s: &str) -> Vec<u16> {
        OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    fn get_cpu_usage(&self) -> f32 {
        let mut value = PDH_FMT_COUNTERVALUE::default();
        
        unsafe {
            PdhCollectQueryData(self.cpu_query);
            time::sleep(Duration::from_millis(100));
            PdhCollectQueryData(self.cpu_query);
            PdhGetFormattedCounterValue(
                self.cpu_counter,
                PDH_FMT_LONG,
                null_mut(),
                &mut value,
            );
        }

        value.longValue as f32
    }
}

impl Drop for WindowsSystemInfo {
    fn drop(&mut self) {
        // Cleanup PDH handles
    }
}

#[async_trait::async_trait]
impl SystemInfo for WindowsSystemInfo {
    fn get_os_name(&self) -> String {
        "Windows".to_string()
    }

    fn get_os_version(&self) -> String {
        // Get Windows version using winapi
        "10".to_string()
    }

    fn get_architecture(&self) -> String {
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }
        match sys_info.wProcessorArchitecture {
            0 => "x86",
            5 => "ARM",
            6 => "IA64",
            9 => "x64",
            _ => "Unknown",
        }
        .to_string()
    }

    async fn get_memory_info(&self) -> Result<MemoryInfo, PlatformError> {
        let mut mem_status: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
        mem_status.dwLength = size_of::<MEMORYSTATUSEX>() as u32;

        if unsafe { GlobalMemoryStatusEx(&mut mem_status) } == 0 {
            return Err(PlatformError::SystemInfoError(
                "Failed to get memory status".to_string(),
            ));
        }

        Ok(MemoryInfo {
            total: mem_status.ullTotalPhys,
            free: mem_status.ullAvailPhys,
            used: mem_status.ullTotalPhys - mem_status.ullAvailPhys,
            swap_total: mem_status.ullTotalPageFile,
            swap_free: mem_status.ullAvailPageFile,
            cache: None,
        })
    }

    async fn get_cpu_info(&self) -> Result<CpuInfo, PlatformError> {
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }

        Ok(CpuInfo {
            model: "Intel/AMD".to_string(),
            cores: sys_info.dwNumberOfProcessors,
            threads: sys_info.dwNumberOfProcessors * 2,
            frequency: 0,
            usage: self.get_cpu_usage(),
            temperature: None,
        })
    }

    async fn get_disk_info(&self) -> Result<DiskInfo, PlatformError> {
        Ok(DiskInfo {
            total: 0,
            free: 0,
            used: 0,
            mount_point: PathBuf::from("C:\\"),
            fs_type: None,
        })
    }
} 