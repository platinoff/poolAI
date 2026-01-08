//! Integration tests for Runtime Process Module

use poolai::runtime::process::{ProcessManager, ProcessConfig, ProcessStatus};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn test_process_manager_creation() {
    let manager = ProcessManager::new();
    // Just verify it can be created
    let _ = manager;
}

#[tokio::test]
async fn test_process_manager_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ProcessManager::new();
    manager.initialize().await?;
    manager.start().await?;
    manager.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_list_processes_empty() {
    let manager = ProcessManager::new();
    let processes = manager.list_processes().await;
    assert_eq!(processes.len(), 0);
}

#[tokio::test]
async fn test_process_not_found_error() {
    let manager = ProcessManager::new();
    let fake_id = Uuid::new_v4();
    
    // Should return error for non-existent process
    let status_result = manager.get_process_status(fake_id).await;
    assert!(status_result.is_err());
    
    let logs_result = manager.get_process_logs(fake_id).await;
    assert!(logs_result.is_err());
    
    let pid_result = manager.get_process_pid(fake_id).await;
    assert!(pid_result.is_err());
}

#[tokio::test]
async fn test_process_config_creation() {
    let config = ProcessConfig {
        command: "echo".to_string(),
        args: vec!["test".to_string()],
        working_dir: None,
        env: HashMap::new(),
        timeout_seconds: Some(30),
        cpu_limit_percent: Some(50),
        memory_limit_mb: Some(1024),
        capture_logs: true,
    };
    
    assert_eq!(config.command, "echo");
    assert_eq!(config.args.len(), 1);
    assert_eq!(config.timeout_seconds, Some(30));
    assert!(config.capture_logs);
}

#[tokio::test]
async fn test_process_logs_default() {
    let logs = poolai::runtime::process::ProcessLogs::default();
    
    assert_eq!(logs.stdout.len(), 0);
    assert_eq!(logs.stderr.len(), 0);
    assert_eq!(logs.max_lines, 1000);
}

#[tokio::test]
async fn test_process_status_variants() {
    let statuses = vec![
        ProcessStatus::Starting,
        ProcessStatus::Running,
        ProcessStatus::Stopping,
        ProcessStatus::Stopped,
        ProcessStatus::Failed("test error".to_string()),
        ProcessStatus::Timeout,
    ];
    
    for status in statuses {
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }
}
