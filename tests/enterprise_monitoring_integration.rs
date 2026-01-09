//! Integration tests for Enterprise Monitoring Manager
//!
//! Tests:
//! - Alert rule creation and management
//! - Active alerts retrieval and filtering
//! - Alert acknowledgment
//! - Dashboard creation and listing
//! - Metric history retrieval

#[cfg(feature = "enterprise")]
use poolai::enterprise::monitoring::{
    get_global_monitoring_manager, Alert, AlertRule, AlertSeverity, Dashboard, MetricDataPoint,
    MonitoringManager,
};
#[cfg(feature = "enterprise")]
use poolai::core::error::AppError;
#[cfg(feature = "enterprise")]
use chrono::Utc;
#[cfg(feature = "enterprise")]
use uuid::Uuid;

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_monitoring_manager_initialization() {
    let manager = get_global_monitoring_manager();
    assert!(manager.initialize().await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_create_alert_rule() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    let rule = AlertRule {
        name: "test-high-cpu".to_string(),
        metric: "cpu_usage".to_string(),
        threshold: 90.0,
        operator: ">".to_string(),
        severity: AlertSeverity::Warning,
        enabled: true,
        tenant_id: None,
    };

    assert!(manager.create_alert_rule(rule).await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_get_active_alerts() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    // Create alert rule
    let rule = AlertRule {
        name: "test-high-cpu".to_string(),
        metric: "cpu_usage".to_string(),
        threshold: 90.0,
        operator: ">".to_string(),
        severity: AlertSeverity::Warning,
        enabled: true,
        tenant_id: None,
    };
    manager.create_alert_rule(rule).await.unwrap();

    // Record metric that triggers alert
    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "cpu_usage".to_string(),
        value: 95.0,
        tags: std::collections::HashMap::new(),
        tenant_id: None,
    };
    manager.record_metric(data_point).await.unwrap();

    // Get active alerts
    let alerts = manager.get_active_alerts(None, None, None).await.unwrap();
    assert!(!alerts.is_empty());
    assert_eq!(alerts[0].metric, "cpu_usage");
    assert_eq!(alerts[0].current_value, 95.0);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_get_active_alerts_with_filters() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    // Create alert rules with different severities
    let rule1 = AlertRule {
        name: "test-warning".to_string(),
        metric: "cpu_usage".to_string(),
        threshold: 80.0,
        operator: ">".to_string(),
        severity: AlertSeverity::Warning,
        enabled: true,
        tenant_id: None,
    };

    let rule2 = AlertRule {
        name: "test-error".to_string(),
        metric: "memory_usage".to_string(),
        threshold: 95.0,
        operator: ">".to_string(),
        severity: AlertSeverity::Error,
        enabled: true,
        tenant_id: None,
    };

    manager.create_alert_rule(rule1).await.unwrap();
    manager.create_alert_rule(rule2).await.unwrap();

    // Record metrics that trigger alerts
    let data_point1 = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "cpu_usage".to_string(),
        value: 85.0,
        tags: std::collections::HashMap::new(),
        tenant_id: None,
    };
    manager.record_metric(data_point1).await.unwrap();

    let data_point2 = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "memory_usage".to_string(),
        value: 96.0,
        tags: std::collections::HashMap::new(),
        tenant_id: None,
    };
    manager.record_metric(data_point2).await.unwrap();

    // Get alerts filtered by severity
    let alerts = manager
        .get_active_alerts(Some(AlertSeverity::Warning), None, None)
        .await
        .unwrap();
    assert!(alerts.iter().all(|a| a.severity == AlertSeverity::Warning));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_acknowledge_alert() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    // Create alert rule and trigger alert
    let rule = AlertRule {
        name: "test-acknowledge".to_string(),
        metric: "cpu_usage".to_string(),
        threshold: 90.0,
        operator: ">".to_string(),
        severity: AlertSeverity::Warning,
        enabled: true,
        tenant_id: None,
    };
    manager.create_alert_rule(rule).await.unwrap();

    let data_point = MetricDataPoint {
        timestamp: Utc::now(),
        metric: "cpu_usage".to_string(),
        value: 95.0,
        tags: std::collections::HashMap::new(),
        tenant_id: None,
    };
    manager.record_metric(data_point).await.unwrap();

    // Get alert ID
    let alerts = manager.get_active_alerts(None, None, None).await.unwrap();
    assert!(!alerts.is_empty());
    let alert_id = alerts[0].id;

    // Acknowledge alert
    assert!(manager.acknowledge_alert(alert_id).await.is_ok());

    // Verify alert is acknowledged
    let alerts = manager
        .get_active_alerts(None, None, Some(true))
        .await
        .unwrap();
    assert!(alerts.iter().any(|a| a.id == alert_id && a.acknowledged));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_create_dashboard() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    let dashboard = Dashboard {
        id: Uuid::new_v4(),
        name: "test-dashboard".to_string(),
        description: "Test dashboard".to_string(),
        metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
        layout: "{}".to_string(),
        is_public: false,
        tenant_id: None,
        created_at: Utc::now(),
    };

    assert!(manager.create_dashboard(dashboard.clone()).await.is_ok());

    // Verify dashboard exists
    let dashboards = manager.list_dashboards(None).await.unwrap();
    assert!(dashboards.iter().any(|d| d.id == dashboard.id));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_list_dashboards_with_tenant_filter() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    let tenant_id = Uuid::new_v4();

    // Create dashboards with and without tenant
    let dashboard1 = Dashboard {
        id: Uuid::new_v4(),
        name: "tenant-dashboard".to_string(),
        description: "Tenant dashboard".to_string(),
        metrics: vec!["cpu_usage".to_string()],
        layout: "{}".to_string(),
        is_public: false,
        tenant_id: Some(tenant_id),
        created_at: Utc::now(),
    };

    let dashboard2 = Dashboard {
        id: Uuid::new_v4(),
        name: "public-dashboard".to_string(),
        description: "Public dashboard".to_string(),
        metrics: vec!["memory_usage".to_string()],
        layout: "{}".to_string(),
        is_public: true,
        tenant_id: None,
        created_at: Utc::now(),
    };

    manager.create_dashboard(dashboard1).await.unwrap();
    manager.create_dashboard(dashboard2).await.unwrap();

    // List dashboards filtered by tenant
    let dashboards = manager.list_dashboards(Some(tenant_id)).await.unwrap();
    assert!(dashboards.iter().all(|d| d.tenant_id == Some(tenant_id)));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_get_metric_history() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    // Record multiple metrics
    for i in 0..10 {
        let data_point = MetricDataPoint {
            timestamp: Utc::now() - chrono::Duration::minutes(i as i64),
            metric: "cpu_usage".to_string(),
            value: 50.0 + (i as f64),
            tags: std::collections::HashMap::new(),
            tenant_id: None,
        };
        manager.record_metric(data_point).await.unwrap();
    }

    // Get metric history
    let metrics = manager
        .get_metric_history(Some("cpu_usage"), None, None, None, None)
        .await
        .unwrap();
    assert!(!metrics.is_empty());
    assert!(metrics.iter().all(|m| m.metric == "cpu_usage"));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_get_metric_history_with_time_range() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    let now = Utc::now();
    let start_time = now - chrono::Duration::hours(1);
    let end_time = now;

    // Record metrics
    for i in 0..5 {
        let data_point = MetricDataPoint {
            timestamp: now - chrono::Duration::minutes(i as i64 * 10),
            metric: "cpu_usage".to_string(),
            value: 50.0 + (i as f64),
            tags: std::collections::HashMap::new(),
            tenant_id: None,
        };
        manager.record_metric(data_point).await.unwrap();
    }

    // Get metric history with time range
    let metrics = manager
        .get_metric_history(
            Some("cpu_usage"),
            Some(start_time),
            Some(end_time),
            None,
            None,
        )
        .await
        .unwrap();
    assert!(metrics.iter().all(|m| {
        m.timestamp >= start_time && m.timestamp <= end_time
    }));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_get_metric_history_with_limit() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    // Record multiple metrics
    for i in 0..20 {
        let data_point = MetricDataPoint {
            timestamp: Utc::now() - chrono::Duration::minutes(i as i64),
            metric: "cpu_usage".to_string(),
            value: 50.0 + (i as f64),
            tags: std::collections::HashMap::new(),
            tenant_id: None,
        };
        manager.record_metric(data_point).await.unwrap();
    }

    // Get metric history with limit
    let metrics = manager
        .get_metric_history(Some("cpu_usage"), None, None, None, Some(10))
        .await
        .unwrap();
    assert!(metrics.len() <= 10);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_acknowledge_nonexistent_alert() {
    let manager = get_global_monitoring_manager();
    manager.initialize().await.unwrap();

    let nonexistent_id = Uuid::new_v4();
    let result = manager.acknowledge_alert(nonexistent_id).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("not found"));
    }
}
