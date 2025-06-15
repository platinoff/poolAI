use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum VmError {
    DeviceError(String),
    UsbError(String),
    PcieError(String),
    IoError(std::io::Error),
    WindowsError(String),
    UnixError(String),
    ConfigurationError(String),
    ResourceError(String),
    PermissionError(String),
    NotFoundError(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::DeviceError(msg) => write!(f, "Device error: {}", msg),
            VmError::UsbError(msg) => write!(f, "USB error: {}", msg),
            VmError::PcieError(msg) => write!(f, "PCIe error: {}", msg),
            VmError::IoError(err) => write!(f, "IO error: {}", err),
            VmError::WindowsError(msg) => write!(f, "Windows error: {}", msg),
            VmError::UnixError(msg) => write!(f, "Unix error: {}", msg),
            VmError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            VmError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
            VmError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
            VmError::NotFoundError(msg) => write!(f, "Not found error: {}", msg),
        }
    }
}

impl Error for VmError {}

impl From<std::io::Error> for VmError {
    fn from(err: std::io::Error) -> Self {
        VmError::IoError(err)
    }
}

impl From<String> for VmError {
    fn from(err: String) -> Self {
        VmError::DeviceError(err)
    }
}

impl From<&str> for VmError {
    fn from(err: &str) -> Self {
        VmError::DeviceError(err.to_string())
    }
} 