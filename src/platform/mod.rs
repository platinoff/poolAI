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