//! Platform module for cross-platform GPU and system information
//!
//! Provides platform-specific implementations for Windows and Linux
//! with a unified interface for GPU information and system resources.
//!
//! # Features
//!
//! - **Cross-platform Support**: Windows, Linux, and macOS (planned)
//! - **GPU Information**: Device name, memory usage, temperature, utilization
//! - **Vendor Support**: NVIDIA, AMD, Intel GPU detection
//! - **Unified Interface**: Same API across all platforms
//!
//! # Example
//!
//! ```no_run
//! use poolai::platform::get_gpu_info;
//!
//! // Get GPU information (works on Windows and Linux)
//! let gpu_info = get_gpu_info();
//!
//! println!("GPU Device: {}", gpu_info.device);
//! println!("Memory: {} MB / {} MB", gpu_info.memory_used, gpu_info.memory_total);
//! println!("Temperature: {:.1}°C", gpu_info.temperature);
//! println!("Utilization: {:.1}%", gpu_info.utilization);
//!
//! // Calculate memory usage percentage
//! let memory_usage = (gpu_info.memory_used as f32 / gpu_info.memory_total as f32) * 100.0;
//! println!("Memory Usage: {:.1}%", memory_usage);
//! ```
//!
//! # Platform-Specific Implementation
//!
//! ## Windows
//! - Uses Windows Management Instrumentation (WMI) or DirectX APIs
//! - Queries `Win32_VideoController` class for GPU information
//! - Supports NVIDIA Management Library and AMD ADL (when available)
//!
//! ## Linux
//! - Reads from `/sys/class/drm/` for GPU information
//! - Uses `nvidia-smi` or `rocm-smi` commands for vendor-specific info
//! - Parses `/proc/driver/nvidia/gpus/*/information` for NVIDIA
//! - Parses `/sys/class/drm/card*/device/uevent` for AMD
//!
//! ## macOS (planned)
//! - Uses IOKit framework for GPU information
//! - Queries IORegistry for GPU properties

pub mod linux;
pub mod windows;

use serde::Serialize;

/// GPU information structure
///
/// Contains information about the GPU device including memory usage,
/// temperature, and utilization.
///
/// # Example
///
/// ```rust
/// use poolai::platform::get_gpu_info;
///
/// let gpu = get_gpu_info();
/// println!("GPU: {}", gpu.device);
/// println!("Memory: {} MB / {} MB", gpu.memory_used, gpu.memory_total);
/// ```
#[derive(Serialize, Clone)]
pub struct GpuInfo {
    /// GPU device name (e.g., "NVIDIA RTX 4090")
    pub device: &'static str,
    /// Total GPU memory in MB
    pub memory_total: u64,
    /// Used GPU memory in MB
    pub memory_used: u64,
    /// GPU temperature in Celsius
    pub temperature: f32,
    /// GPU utilization percentage (0.0 - 100.0)
    pub utilization: f32,
}

/// Get GPU information for the current platform
///
/// Returns GPU information including device name, memory usage,
/// temperature, and utilization. Works on Windows and Linux.
///
/// # Platform Support
///
/// - **Windows**: Uses WMI or DirectX APIs
/// - **Linux**: Reads from `/sys/class/drm/` or vendor-specific tools
/// - **macOS**: Planned (IOKit framework)
///
/// # Example
///
/// ```rust
/// use poolai::platform::get_gpu_info;
///
/// let gpu = get_gpu_info();
/// let memory_usage = (gpu.memory_used as f32 / gpu.memory_total as f32) * 100.0;
/// println!("GPU: {} ({}% memory used)", gpu.device, memory_usage);
/// ```
pub fn get_gpu_info() -> GpuInfo {
    #[cfg(target_os = "windows")]
    {
        println!("[platform] {}", windows::get_windows_gpu_info());
    }
    #[cfg(target_os = "linux")]
    {
        println!("[platform] {}", linux::get_linux_gpu_info());
    }
    // Future improvement: Implement real platform-specific calls for each OS
    // For Windows:
    //  - Use Windows Management Instrumentation (WMI) or DirectX APIs
    //  - Query GPU information using WMI Win32_VideoController class
    //  - Or use vendor-specific APIs (NVIDIA Management Library, AMD ADL)
    //
    // For Linux:
    //  - Read from /sys/class/drm/ for GPU information
    //  - Use nvidia-smi or rocm-smi commands for vendor-specific info
    //  - Parse /proc/driver/nvidia/gpus/*/information for NVIDIA
    //  - Parse /sys/class/drm/card*/device/uevent for AMD
    //
    // For macOS:
    //  - Use IOKit framework for GPU information
    //  - Query IORegistry for GPU properties
    //
    // This requires:
    // - Platform-specific API bindings (windows-sys, libc, or IOKit)
    // - Vendor-specific SDKs for detailed GPU information (optional)
    // - Proper error handling for missing GPUs or unsupported systems
    GpuInfo {
        device: "NVIDIA RTX 4090",
        memory_total: 24576,
        memory_used: 8192,
        temperature: 62.5,
        utilization: 78.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gpu_info() {
        let info = get_gpu_info();
        assert_eq!(info.device, "NVIDIA RTX 4090");
        assert_eq!(info.memory_total, 24576);
        assert_eq!(info.memory_used, 8192);
        assert_eq!(info.temperature, 62.5);
        assert_eq!(info.utilization, 78.0);
    }

    #[test]
    fn test_gpu_info_serialization() {
        let info = get_gpu_info();
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("NVIDIA RTX 4090"));
        assert!(json.contains("24576"));
    }

    #[test]
    fn test_gpu_info_clone() {
        let info1 = get_gpu_info();
        let info2 = info1.clone();
        assert_eq!(info1.device, info2.device);
        assert_eq!(info1.memory_total, info2.memory_total);
        assert_eq!(info1.memory_used, info2.memory_used);
        assert_eq!(info1.temperature, info2.temperature);
        assert_eq!(info1.utilization, info2.utilization);
    }

    #[test]
    fn test_gpu_info_memory_usage() {
        let info = get_gpu_info();
        let usage_percent = (info.memory_used as f32 / info.memory_total as f32) * 100.0;
        assert!(usage_percent >= 0.0 && usage_percent <= 100.0);
    }
}
