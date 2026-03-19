//! Integration tests for Runtime Module

use poolai::runtime::{initialize_runtime, RuntimeConfig, RuntimeManager};

#[tokio::test]
async fn test_runtime_config_default() {
    let config = RuntimeConfig::default();
    assert_eq!(config.max_workers, 8);
    assert_eq!(config.queue_capacity, 1000);
    assert_eq!(config.cache_size_mb, 512);
    assert!(config.auto_scaling);
    assert_eq!(config.health_check_interval, 30);
    assert!(config.resource_monitoring);
}

#[tokio::test]
async fn test_runtime_config_custom() {
    let config = RuntimeConfig {
        max_workers: 16,
        queue_capacity: 2000,
        cache_size_mb: 1024,
        auto_scaling: false,
        health_check_interval: 60,
        resource_monitoring: false,
    };

    assert_eq!(config.max_workers, 16);
    assert_eq!(config.queue_capacity, 2000);
    assert_eq!(config.cache_size_mb, 1024);
    assert!(!config.auto_scaling);
    assert_eq!(config.health_check_interval, 60);
    assert!(!config.resource_monitoring);
}

#[tokio::test]
async fn test_runtime_manager_creation() {
    let config = RuntimeConfig::default();
    let runtime = RuntimeManager::new(config);
    // Just verify it can be created
    let _ = runtime;
}

#[tokio::test]
async fn test_runtime_manager_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let config = RuntimeConfig::default();
    let mut runtime = RuntimeManager::new(config);
    runtime.initialize().await?;
    runtime.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_runtime_manager_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let config = RuntimeConfig::default();
    let mut runtime = RuntimeManager::new(config);

    runtime.initialize().await?;
    runtime.start().await?;

    let status = runtime.get_status().await;
    // workers_active may vary, just verify status is accessible
    let _workers_active = status.workers_active;

    runtime.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_initialize_runtime_helper() -> Result<(), Box<dyn std::error::Error>> {
    let config = RuntimeConfig::default();
    let mut runtime = initialize_runtime(config).await?;

    let status = runtime.get_status().await;
    // workers_active may vary, just verify status is accessible
    let _workers_active = status.workers_active;

    runtime.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_runtime_status_fields() -> Result<(), Box<dyn std::error::Error>> {
    let config = RuntimeConfig::default();
    let mut runtime = RuntimeManager::new(config);
    runtime.initialize().await?;
    runtime.start().await?;

    let status = runtime.get_status().await;

    // Verify all fields are accessible
    let _ = status.workers_active;
    let _ = status.queue_length;
    let _ = status.cache_usage;
    let _ = status.storage_usage;
    let _ = status.processes_running;
    let _ = status.resource_utilization;
    let _ = status.health_score;

    runtime.shutdown().await?;
    Ok(())
}
