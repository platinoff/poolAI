//! Integration tests for VM Resource Monitoring enhancements
//!
//! Tests:
//! - Resource usage history tracking
//! - Resource usage statistics (aggregation)
//! - Resource alert thresholds
//! - History limits (FIFO)

use poolai::vm::{ResourceAlertThresholds, ResourceUsage, ResourceUsageHistoryEntry, VmManager, VmResources, VmIsolation};

#[tokio::test]
async fn test_record_resource_usage() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let usage = ResourceUsage {
        cpu_percent: 50.0,
        memory_mb: 1024,
        gpu_utilization: Some(75.0),
    };

    manager
        .record_resource_usage(instance.id, usage.clone())
        .await
        .unwrap();

    let history = manager
        .get_resource_usage_history(instance.id, None)
        .await
        .unwrap();

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].usage.cpu_percent, 50.0);
    assert_eq!(history[0].usage.memory_mb, 1024);
    assert_eq!(history[0].usage.gpu_utilization, Some(75.0));
}

#[tokio::test]
async fn test_resource_usage_history_multiple_entries() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record multiple entries
    for i in 0..10 {
        let usage = ResourceUsage {
            cpu_percent: i as f32 * 10.0,
            memory_mb: i * 100,
            gpu_utilization: Some(i as f32 * 5.0),
        };
        manager
            .record_resource_usage(instance.id, usage)
            .await
            .unwrap();
    }

    let history = manager
        .get_resource_usage_history(instance.id, None)
        .await
        .unwrap();

    assert_eq!(history.len(), 10);
    assert_eq!(history[0].usage.cpu_percent, 0.0);
    assert_eq!(history[9].usage.cpu_percent, 90.0);
}

#[tokio::test]
async fn test_resource_usage_history_limit() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record 10 entries
    for i in 0..10 {
        let usage = ResourceUsage {
            cpu_percent: i as f32 * 10.0,
            memory_mb: i * 100,
            gpu_utilization: None,
        };
        manager
            .record_resource_usage(instance.id, usage)
            .await
            .unwrap();
    }

    // Get last 5 entries
    let history = manager
        .get_resource_usage_history(instance.id, Some(5))
        .await
        .unwrap();

    assert_eq!(history.len(), 5);
    assert_eq!(history[0].usage.cpu_percent, 50.0);
    assert_eq!(history[4].usage.cpu_percent, 90.0);
}

#[tokio::test]
async fn test_resource_usage_history_fifo_limit() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record 1001 entries (more than the 1000 limit)
    for i in 0..1001 {
        let usage = ResourceUsage {
            cpu_percent: i as f32,
            memory_mb: i as u32,
            gpu_utilization: None,
        };
        manager
            .record_resource_usage(instance.id, usage)
            .await
            .unwrap();
    }

    let history = manager
        .get_resource_usage_history(instance.id, None)
        .await
        .unwrap();

    // Should be limited to 1000 entries
    assert_eq!(history.len(), 1000);
    // First entry should be 1 (not 0, as 0 was removed)
    assert_eq!(history[0].usage.cpu_percent, 1.0);
    // Last entry should be 1000
    assert_eq!(history[999].usage.cpu_percent, 1000.0);
}

#[tokio::test]
async fn test_resource_usage_stats() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record entries with known values
    let usages = vec![
        ResourceUsage {
            cpu_percent: 10.0,
            memory_mb: 100,
            gpu_utilization: Some(20.0),
        },
        ResourceUsage {
            cpu_percent: 20.0,
            memory_mb: 200,
            gpu_utilization: Some(40.0),
        },
        ResourceUsage {
            cpu_percent: 30.0,
            memory_mb: 300,
            gpu_utilization: Some(60.0),
        },
    ];

    for usage in usages {
        manager
            .record_resource_usage(instance.id, usage)
            .await
            .unwrap();
    }

    let stats = manager
        .get_resource_usage_stats(instance.id, None)
        .await
        .unwrap();

    assert_eq!(stats.cpu_percent_min, 10.0);
    assert_eq!(stats.cpu_percent_max, 30.0);
    assert_eq!(stats.cpu_percent_avg, 20.0);
    assert_eq!(stats.memory_mb_min, 100);
    assert_eq!(stats.memory_mb_max, 300);
    assert_eq!(stats.memory_mb_avg, 200.0);
    assert_eq!(stats.gpu_utilization_min, Some(20.0));
    assert_eq!(stats.gpu_utilization_max, Some(60.0));
    assert_eq!(stats.gpu_utilization_avg, Some(40.0));
    assert_eq!(stats.sample_count, 3);
}

#[tokio::test]
async fn test_resource_usage_stats_no_gpu() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record entries without GPU
    let usages = vec![
        ResourceUsage {
            cpu_percent: 10.0,
            memory_mb: 100,
            gpu_utilization: None,
        },
        ResourceUsage {
            cpu_percent: 20.0,
            memory_mb: 200,
            gpu_utilization: None,
        },
    ];

    for usage in usages {
        manager
            .record_resource_usage(instance.id, usage)
            .await
            .unwrap();
    }

    let stats = manager
        .get_resource_usage_stats(instance.id, None)
        .await
        .unwrap();

    assert_eq!(stats.gpu_utilization_min, None);
    assert_eq!(stats.gpu_utilization_max, None);
    assert_eq!(stats.gpu_utilization_avg, None);
}

#[tokio::test]
async fn test_resource_usage_stats_empty_history() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Try to get stats without any history
    let result = manager.get_resource_usage_stats(instance.id, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_alert_thresholds_default() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let thresholds = manager
        .get_resource_alert_thresholds(instance.id)
        .await
        .unwrap();

    assert_eq!(thresholds.cpu_percent_threshold, Some(90.0));
    assert_eq!(thresholds.memory_mb_threshold, None);
    assert_eq!(thresholds.gpu_utilization_threshold, Some(95.0));
}

#[tokio::test]
async fn test_set_resource_alert_thresholds() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let thresholds = ResourceAlertThresholds {
        cpu_percent_threshold: Some(80.0),
        memory_mb_threshold: Some(2048),
        gpu_utilization_threshold: Some(90.0),
    };

    manager
        .set_resource_alert_thresholds(instance.id, thresholds.clone())
        .await
        .unwrap();

    let retrieved = manager
        .get_resource_alert_thresholds(instance.id)
        .await
        .unwrap();

    assert_eq!(retrieved.cpu_percent_threshold, Some(80.0));
    assert_eq!(retrieved.memory_mb_threshold, Some(2048));
    assert_eq!(retrieved.gpu_utilization_threshold, Some(90.0));
}

#[tokio::test]
async fn test_resource_alerts_triggered() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Set thresholds
    let thresholds = ResourceAlertThresholds {
        cpu_percent_threshold: Some(50.0),
        memory_mb_threshold: Some(1000),
        gpu_utilization_threshold: Some(80.0),
    };

    manager
        .set_resource_alert_thresholds(instance.id, thresholds)
        .await
        .unwrap();

    // Record usage that exceeds thresholds
    let usage = ResourceUsage {
        cpu_percent: 75.0, // Exceeds 50.0
        memory_mb: 1500,   // Exceeds 1000
        gpu_utilization: Some(90.0), // Exceeds 80.0
    };

    // This should trigger alerts (logged as warnings)
    manager
        .record_resource_usage(instance.id, usage)
        .await
        .unwrap();

    // Verify the usage was recorded
    let history = manager
        .get_resource_usage_history(instance.id, None)
        .await
        .unwrap();
    assert_eq!(history.len(), 1);
}

#[tokio::test]
async fn test_delete_instance_cleans_up_resource_data() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "test-instance".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    // Record some usage
    let usage = ResourceUsage {
        cpu_percent: 50.0,
        memory_mb: 1024,
        gpu_utilization: None,
    };
    manager
        .record_resource_usage(instance.id, usage)
        .await
        .unwrap();

    // Set alert thresholds
    let thresholds = ResourceAlertThresholds::default();
    manager
        .set_resource_alert_thresholds(instance.id, thresholds)
        .await
        .unwrap();

    // Delete instance
    manager.delete_instance(instance.id).await.unwrap();

    // Try to get history (should fail - instance not found)
    let result = manager
        .get_resource_usage_history(instance.id, None)
        .await;
    assert!(result.is_err());

    // Try to get thresholds (should fail - instance not found)
    let result = manager
        .get_resource_alert_thresholds(instance.id)
        .await;
    assert!(result.is_err());
}

