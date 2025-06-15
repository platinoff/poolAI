mod error;
mod device;
mod usb;
mod pcie;

pub use error::VmError;
pub use device::{Device, DeviceType};
pub use usb::{UsbDevice, UsbPassthrough};
pub use pcie::{PcieDevice, PciePassthrough};

use std::collections::HashMap;
use async_trait::async_trait;

pub mod endorphin;
pub mod tuning;

pub use endorphin::*;
pub use tuning::*;

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub name: String,
    pub memory: u64,
    pub cpus: u32,
    pub devices: Vec<Device>,
    pub usb_passthrough: Vec<UsbPassthrough>,
    pub pcie_passthrough: Vec<PciePassthrough>,
}

#[async_trait]
pub trait VmManager: Send + Sync {
    async fn create_vm(&self, config: VmConfig) -> Result<(), VmError>;
    async fn start_vm(&self, name: &str) -> Result<(), VmError>;
    async fn stop_vm(&self, name: &str) -> Result<(), VmError>;
    async fn delete_vm(&self, name: &str) -> Result<(), VmError>;
    async fn list_vms(&self) -> Result<Vec<String>, VmError>;
    async fn get_vm_status(&self, name: &str) -> Result<VmStatus, VmError>;
    async fn attach_device(&self, name: &str, device: Device) -> Result<(), VmError>;
    async fn detach_device(&self, name: &str, device_id: &str) -> Result<(), VmError>;
    async fn attach_usb(&self, name: &str, usb: UsbPassthrough) -> Result<(), VmError>;
    async fn detach_usb(&self, name: &str, usb_id: &str) -> Result<(), VmError>;
    async fn attach_pcie(&self, name: &str, pcie: PciePassthrough) -> Result<(), VmError>;
    async fn detach_pcie(&self, name: &str, pcie_id: &str) -> Result<(), VmError>;
}

#[derive(Debug, Clone)]
pub struct VmStatus {
    pub name: String,
    pub state: VmState,
    pub memory_usage: u64,
    pub cpu_usage: f32,
    pub attached_devices: Vec<Device>,
    pub attached_usb: Vec<UsbDevice>,
    pub attached_pcie: Vec<PcieDevice>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VmState {
    Running,
    Stopped,
    Paused,
    Error(String),
}

pub fn create_vm_manager() -> Box<dyn VmManager> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsVmManager::new())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Box::new(UnixVmManager::new())
    }
} 