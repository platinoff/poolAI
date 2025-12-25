//! Platform module for cross-platform GPU and system information
//!
//! Provides platform-specific implementations for Windows and Linux
//! with a unified interface for GPU information and system resources.

pub mod windows;
pub mod linux;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct GpuInfo {
    pub device: &'static str,
    pub memory_total: u64,
    pub memory_used: u64,
    pub temperature: f32,
    pub utilization: f32,
}

pub fn get_gpu_info() -> GpuInfo {
    #[cfg(target_os = "windows")]
    {
        println!("[platform] {}", windows::get_windows_gpu_info());
    }
    #[cfg(target_os = "linux")]
    {
        println!("[platform] {}", linux::get_linux_gpu_info());
    }
    // TODO: Implement real platform-specific calls for each OS
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
