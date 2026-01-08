//! Integration tests for Runtime Health Module

use poolai::runtime::health::HealthMonitor;

#[tokio::test]
async fn test_health_monitor_creation() {
    let monitor = HealthMonitor::new(30);
    // Just verify it can be created
    let _ = monitor;
}

#[tokio::test]
async fn test_health_monitor_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = HealthMonitor::new(30);
    monitor.initialize().await?;
    Ok(())
}

#[tokio::test]
async fn test_health_monitor_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = HealthMonitor::new(30);
    monitor.initialize().await?;
    monitor.start().await?;
    monitor.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_health_monitor_interval() {
    let monitor = HealthMonitor::new(60);
    // Just verify it can be created with different interval
    let _ = monitor;
}
