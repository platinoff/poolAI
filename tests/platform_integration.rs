//! Integration tests for Platform Module

use poolai::platform::get_gpu_info;

#[test]
fn test_get_gpu_info() {
    let gpu = get_gpu_info();
    
    assert_eq!(gpu.device, "NVIDIA RTX 4090");
    assert_eq!(gpu.memory_total, 24576);
    assert_eq!(gpu.memory_used, 8192);
    assert_eq!(gpu.temperature, 62.5);
    assert_eq!(gpu.utilization, 78.0);
}

#[test]
fn test_gpu_info_memory_usage_calculation() {
    let gpu = get_gpu_info();
    let memory_usage = (gpu.memory_used as f32 / gpu.memory_total as f32) * 100.0;
    
    assert!(memory_usage >= 0.0);
    assert!(memory_usage <= 100.0);
    assert!((memory_usage - 33.33).abs() < 1.0); // Approximately 33.33%
}

#[test]
fn test_gpu_info_utilization_range() {
    let gpu = get_gpu_info();
    
    assert!(gpu.utilization >= 0.0);
    assert!(gpu.utilization <= 100.0);
}

#[test]
fn test_gpu_info_temperature_range() {
    let gpu = get_gpu_info();
    
    // Temperature should be reasonable (0-150°C for GPUs)
    assert!(gpu.temperature >= 0.0);
    assert!(gpu.temperature <= 150.0);
}

#[test]
fn test_gpu_info_serialization() {
    let gpu = get_gpu_info();
    let json = serde_json::to_string(&gpu).unwrap();
    
    assert!(json.contains("NVIDIA RTX 4090"));
    assert!(json.contains("24576"));
    assert!(json.contains("8192"));
    assert!(json.contains("62.5"));
    assert!(json.contains("78.0"));
}

#[test]
fn test_gpu_info_clone() {
    let gpu1 = get_gpu_info();
    let gpu2 = gpu1.clone();
    
    assert_eq!(gpu1.device, gpu2.device);
    assert_eq!(gpu1.memory_total, gpu2.memory_total);
    assert_eq!(gpu1.memory_used, gpu2.memory_used);
    assert_eq!(gpu1.temperature, gpu2.temperature);
    assert_eq!(gpu1.utilization, gpu2.utilization);
}

#[test]
fn test_gpu_info_memory_used_less_than_total() {
    let gpu = get_gpu_info();
    assert!(gpu.memory_used <= gpu.memory_total);
}
