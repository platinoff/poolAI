use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcieDevice {
    pub id: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub vendor_name: String,
    pub device_name: String,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub class: u8,
    pub subclass: u8,
    pub programming_interface: u8,
    pub revision: u8,
    pub subsystem_vendor_id: Option<u16>,
    pub subsystem_id: Option<u16>,
    pub driver: Option<String>,
    pub numa_node: Option<u8>,
    pub iommu_group: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciePassthrough {
    pub device: PcieDevice,
    pub auto_attach: bool,
    pub hotplug: bool,
    pub iommu_group: Option<u32>,
    pub vfio_driver: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PcieSpeed {
    Gen1,
    Gen2,
    Gen3,
    Gen4,
    Gen5,
}

impl fmt::Display for PcieSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PcieSpeed::Gen1 => write!(f, "PCIe Gen 1 (2.5 GT/s)"),
            PcieSpeed::Gen2 => write!(f, "PCIe Gen 2 (5.0 GT/s)"),
            PcieSpeed::Gen3 => write!(f, "PCIe Gen 3 (8.0 GT/s)"),
            PcieSpeed::Gen4 => write!(f, "PCIe Gen 4 (16.0 GT/s)"),
            PcieSpeed::Gen5 => write!(f, "PCIe Gen 5 (32.0 GT/s)"),
        }
    }
}

pub trait PcieManager {
    fn list_devices(&self) -> Result<Vec<PcieDevice>, String>;
    fn attach_device(&self, device: &PcieDevice) -> Result<(), String>;
    fn detach_device(&self, device: &PcieDevice) -> Result<(), String>;
    fn is_device_attached(&self, device: &PcieDevice) -> bool;
    fn get_iommu_groups(&self) -> Result<Vec<u32>, String>;
    fn bind_to_vfio(&self, device: &PcieDevice) -> Result<(), String>;
    fn unbind_from_vfio(&self, device: &PcieDevice) -> Result<(), String>;
}

#[cfg(target_os = "windows")]
pub struct WindowsPcieManager;

#[cfg(target_os = "windows")]
impl PcieManager for WindowsPcieManager {
    fn list_devices(&self) -> Result<Vec<PcieDevice>, String> {
        // TODO: Implement Windows PCIe device listing
        Ok(Vec::new())
    }

    fn attach_device(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Windows PCIe device attachment
        Ok(())
    }

    fn detach_device(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Windows PCIe device detachment
        Ok(())
    }

    fn is_device_attached(&self, device: &PcieDevice) -> bool {
        // TODO: Implement Windows PCIe device attachment check
        false
    }

    fn get_iommu_groups(&self) -> Result<Vec<u32>, String> {
        // TODO: Implement Windows IOMMU group listing
        Ok(Vec::new())
    }

    fn bind_to_vfio(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Windows VFIO binding
        Ok(())
    }

    fn unbind_from_vfio(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Windows VFIO unbinding
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub struct UnixPcieManager;

#[cfg(not(target_os = "windows"))]
impl PcieManager for UnixPcieManager {
    fn list_devices(&self) -> Result<Vec<PcieDevice>, String> {
        // TODO: Implement Unix PCIe device listing
        Ok(Vec::new())
    }

    fn attach_device(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Unix PCIe device attachment
        Ok(())
    }

    fn detach_device(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Unix PCIe device detachment
        Ok(())
    }

    fn is_device_attached(&self, device: &PcieDevice) -> bool {
        // TODO: Implement Unix PCIe device attachment check
        false
    }

    fn get_iommu_groups(&self) -> Result<Vec<u32>, String> {
        // TODO: Implement Unix IOMMU group listing
        Ok(Vec::new())
    }

    fn bind_to_vfio(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Unix VFIO binding
        Ok(())
    }

    fn unbind_from_vfio(&self, device: &PcieDevice) -> Result<(), String> {
        // TODO: Implement Unix VFIO unbinding
        Ok(())
    }
}

pub fn create_pcie_manager() -> Box<dyn PcieManager> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsPcieManager)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Box::new(UnixPcieManager)
    }
} 