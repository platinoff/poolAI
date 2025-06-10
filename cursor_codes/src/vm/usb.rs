use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub id: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: Option<String>,
    pub bus_number: u8,
    pub device_number: u8,
    pub speed: UsbSpeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbPassthrough {
    pub device: UsbDevice,
    pub auto_attach: bool,
    pub hotplug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UsbSpeed {
    Low,
    Full,
    High,
    Super,
    SuperPlus,
    SuperPlus2,
}

impl fmt::Display for UsbSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsbSpeed::Low => write!(f, "Low Speed (1.5 Mbps)"),
            UsbSpeed::Full => write!(f, "Full Speed (12 Mbps)"),
            UsbSpeed::High => write!(f, "High Speed (480 Mbps)"),
            UsbSpeed::Super => write!(f, "Super Speed (5 Gbps)"),
            UsbSpeed::SuperPlus => write!(f, "Super Speed+ (10 Gbps)"),
            UsbSpeed::SuperPlus2 => write!(f, "Super Speed+ 2 (20 Gbps)"),
        }
    }
}

pub trait UsbManager {
    fn list_devices(&self) -> Result<Vec<UsbDevice>, String>;
    fn attach_device(&self, device: &UsbDevice) -> Result<(), String>;
    fn detach_device(&self, device: &UsbDevice) -> Result<(), String>;
    fn is_device_attached(&self, device: &UsbDevice) -> bool;
}

#[cfg(target_os = "windows")]
pub struct WindowsUsbManager;

#[cfg(target_os = "windows")]
impl UsbManager for WindowsUsbManager {
    fn list_devices(&self) -> Result<Vec<UsbDevice>, String> {
        // TODO: Implement Windows USB device listing
        Ok(Vec::new())
    }

    fn attach_device(&self, device: &UsbDevice) -> Result<(), String> {
        // TODO: Implement Windows USB device attachment
        Ok(())
    }

    fn detach_device(&self, device: &UsbDevice) -> Result<(), String> {
        // TODO: Implement Windows USB device detachment
        Ok(())
    }

    fn is_device_attached(&self, device: &UsbDevice) -> bool {
        // TODO: Implement Windows USB device attachment check
        false
    }
}

#[cfg(not(target_os = "windows"))]
pub struct UnixUsbManager;

#[cfg(not(target_os = "windows"))]
impl UsbManager for UnixUsbManager {
    fn list_devices(&self) -> Result<Vec<UsbDevice>, String> {
        // TODO: Implement Unix USB device listing
        Ok(Vec::new())
    }

    fn attach_device(&self, device: &UsbDevice) -> Result<(), String> {
        // TODO: Implement Unix USB device attachment
        Ok(())
    }

    fn detach_device(&self, device: &UsbDevice) -> Result<(), String> {
        // TODO: Implement Unix USB device detachment
        Ok(())
    }

    fn is_device_attached(&self, device: &UsbDevice) -> bool {
        // TODO: Implement Unix USB device attachment check
        false
    }
}

pub fn create_usb_manager() -> Box<dyn UsbManager> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsUsbManager)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Box::new(UnixUsbManager)
    }
} 