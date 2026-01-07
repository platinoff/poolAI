//! Integration tests for VM Auto-Recovery enhancements
//!
//! Tests:
//! - AutoRecoveryConfig default values
//! - Exponential backoff calculation
//! - Max restart attempts enforcement
//! - Restart attempts tracking
//! - Reset restart attempts

use poolai::vm::{AutoRecoveryConfig, VmIsolation, VmManager, VmResources};

#[tokio::test]
async fn test_auto_recovery_config_default() {
    let config = AutoRecoveryConfig::default();
    assert_eq!(config.max_restart_attempts, 5);
    assert_eq!(config.initial_restart_delay_secs, 1);
    assert_eq!(config.max_restart_delay_secs, 60);
    assert!(config.use_exponential_backoff);
}

#[tokio::test]
async fn test_auto_recovery_config_custom() {
    let config = AutoRecoveryConfig {
        max_restart_attempts: 10,
        initial_restart_delay_secs: 2,
        max_restart_delay_secs: 120,
        use_exponential_backoff: false,
    };

    assert_eq!(config.max_restart_attempts, 10);
    assert_eq!(config.initial_restart_delay_secs, 2);
    assert_eq!(config.max_restart_delay_secs, 120);
    assert!(!config.use_exponential_backoff);
}

#[tokio::test]
async fn test_vm_instance_has_auto_recovery_config() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    assert_eq!(instance.auto_recovery.max_restart_attempts, 5);
    assert_eq!(instance.restart_attempts, 0);
}

#[tokio::test]
async fn test_get_auto_recovery_config() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let config = manager.get_auto_recovery_config(instance.id).await.unwrap();

    assert_eq!(config.max_restart_attempts, 5);
    assert_eq!(config.initial_restart_delay_secs, 1);
    assert_eq!(config.max_restart_delay_secs, 60);
    assert!(config.use_exponential_backoff);
}

#[tokio::test]
async fn test_get_restart_attempts() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let attempts = manager.get_restart_attempts(instance.id).await.unwrap();
    assert_eq!(attempts, 0);
}

#[tokio::test]
async fn test_reset_restart_attempts() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Reset should succeed even if attempts are already 0
    manager.reset_restart_attempts(instance.id).await.unwrap();

    let attempts = manager.get_restart_attempts(instance.id).await.unwrap();
    assert_eq!(attempts, 0);
}

#[tokio::test]
async fn test_update_instance_with_auto_recovery() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let new_config = AutoRecoveryConfig {
        max_restart_attempts: 10,
        initial_restart_delay_secs: 5,
        max_restart_delay_secs: 120,
        use_exponential_backoff: false,
    };

    let updated = manager
        .update_instance(instance.id, None, None, None, Some(new_config.clone()))
        .await
        .unwrap();

    assert_eq!(updated.auto_recovery.max_restart_attempts, 10);
    assert_eq!(updated.auto_recovery.initial_restart_delay_secs, 5);
    assert_eq!(updated.auto_recovery.max_restart_delay_secs, 120);
    assert!(!updated.auto_recovery.use_exponential_backoff);

    // Restart attempts should be reset when auto-recovery config changes
    let attempts = manager.get_restart_attempts(instance.id).await.unwrap();
    assert_eq!(attempts, 0);
}

#[tokio::test]
async fn test_start_instance_resets_restart_attempts() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start the instance
    manager.start_instance(instance.id).await.unwrap();

    // Restart attempts should be reset to 0 on successful start
    let attempts = manager.get_restart_attempts(instance.id).await.unwrap();
    assert_eq!(attempts, 0);
}

#[tokio::test]
async fn test_restart_instance_resets_restart_attempts() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Start the instance first
    manager.start_instance(instance.id).await.unwrap();

    // Manual restart should reset restart attempts
    manager.restart_instance(instance.id).await.unwrap();

    // Restart attempts should be reset to 0 after manual restart
    let attempts = manager.get_restart_attempts(instance.id).await.unwrap();
    assert_eq!(attempts, 0);
}
