//! Integration tests for Enterprise Audit Logger
//!
//! Tests:
//! - Audit event logging
//! - Audit event querying with filters
//! - Query filtering (user_id, tenant_id, action, level, time range)

#[cfg(feature = "enterprise")]
use chrono::Utc;
#[cfg(feature = "enterprise")]
use poolai::enterprise::audit::{
    AuditConfig, AuditEvent, AuditLevel, AuditLogger, AuditQueryFilters,
};
#[cfg(feature = "enterprise")]
use tempfile::TempDir;

#[cfg(feature = "enterprise")]
fn create_test_logger() -> (AuditLogger, TempDir) {
    let temp_dir = TempDir::new().expect("temp dir for audit test");
    let config = AuditConfig {
        log_directory: temp_dir.path().to_path_buf(),
        max_file_size: 1024 * 1024,
        max_files: 5,
        enable_compression: false,
        immediate_flush: true,
    };
    (AuditLogger::with_config(config), temp_dir)
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_audit_logger_initialization() {
    let (logger, _tmp) = create_test_logger();
    assert!(logger.initialize().await.is_ok());
    assert!(logger.shutdown().await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_log_audit_event() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    let event = AuditEvent::new(
        AuditLevel::Info,
        "test_action".to_string(),
        "test_resource".to_string(),
        "success".to_string(),
    )
    .with_user_id("user123".to_string())
    .with_tenant_id("tenant-abc".to_string());

    assert!(logger.log_event(event).await.is_ok());
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_all() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log some test events
    let event1 = AuditEvent::new(
        AuditLevel::Info,
        "create_instance".to_string(),
        "vm_instance".to_string(),
        "success".to_string(),
    )
    .with_user_id("user123".to_string())
    .with_tenant_id("tenant-abc".to_string());

    let event2 = AuditEvent::new(
        AuditLevel::Warning,
        "delete_worker".to_string(),
        "worker".to_string(),
        "success".to_string(),
    )
    .with_user_id("user456".to_string());

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();

    // Query all events
    let filters = AuditQueryFilters::default();
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events.len() >= 2);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_by_user_id() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log test events
    let event1 = AuditEvent::new(
        AuditLevel::Info,
        "test_action1".to_string(),
        "test_resource".to_string(),
        "success".to_string(),
    )
    .with_user_id("user123".to_string());

    let event2 = AuditEvent::new(
        AuditLevel::Info,
        "test_action2".to_string(),
        "test_resource".to_string(),
        "success".to_string(),
    )
    .with_user_id("user456".to_string());

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();

    // Query by user_id
    let filters = AuditQueryFilters {
        user_id: Some("user123".to_string()),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events
        .iter()
        .all(|e| e.user_id.as_ref() == Some(&"user123".to_string())));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_by_action() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log test events
    let event1 = AuditEvent::new(
        AuditLevel::Info,
        "create_instance".to_string(),
        "vm_instance".to_string(),
        "success".to_string(),
    );

    let event2 = AuditEvent::new(
        AuditLevel::Info,
        "delete_worker".to_string(),
        "worker".to_string(),
        "success".to_string(),
    );

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();

    // Query by action
    let filters = AuditQueryFilters {
        action: Some("create_instance".to_string()),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events.iter().all(|e| e.action == "create_instance"));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_by_level() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log test events with different levels
    let event1 = AuditEvent::new(
        AuditLevel::Info,
        "test_action1".to_string(),
        "test_resource".to_string(),
        "success".to_string(),
    );

    let event2 = AuditEvent::new(
        AuditLevel::Warning,
        "test_action2".to_string(),
        "test_resource".to_string(),
        "warning".to_string(),
    );

    let event3 = AuditEvent::new(
        AuditLevel::Error,
        "test_action3".to_string(),
        "test_resource".to_string(),
        "error".to_string(),
    );

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();
    logger.log_event(event3).await.unwrap();

    // Query by minimum level (Warning and above)
    let filters = AuditQueryFilters {
        min_level: Some(AuditLevel::Warning),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events.iter().all(|e| {
        matches!(
            e.level,
            AuditLevel::Warning | AuditLevel::Error | AuditLevel::Critical
        )
    }));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_by_time_range() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    let now = Utc::now();
    let start_time = now - chrono::Duration::hours(1);
    let end_time = now + chrono::Duration::hours(1);

    // Log test event
    let event = AuditEvent::new(
        AuditLevel::Info,
        "test_action".to_string(),
        "test_resource".to_string(),
        "success".to_string(),
    );
    logger.log_event(event).await.unwrap();

    // Query by time range
    let filters = AuditQueryFilters {
        start_time: Some(start_time),
        end_time: Some(end_time),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events
        .iter()
        .all(|e| { e.timestamp >= start_time && e.timestamp <= end_time }));
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_with_limit() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log multiple events
    for i in 0..10 {
        let event = AuditEvent::new(
            AuditLevel::Info,
            format!("test_action_{}", i),
            "test_resource".to_string(),
            "success".to_string(),
        );
        logger.log_event(event).await.unwrap();
    }

    // Query with limit
    let filters = AuditQueryFilters {
        limit: Some(5),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events.len() <= 5);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn test_query_audit_events_combined_filters() {
    let (logger, _tmp) = create_test_logger();
    logger.initialize().await.unwrap();

    // Log test events
    let event1 = AuditEvent::new(
        AuditLevel::Info,
        "create_instance".to_string(),
        "vm_instance".to_string(),
        "success".to_string(),
    )
    .with_user_id("user123".to_string())
    .with_tenant_id("tenant-abc".to_string());

    let event2 = AuditEvent::new(
        AuditLevel::Info,
        "create_instance".to_string(),
        "vm_instance".to_string(),
        "success".to_string(),
    )
    .with_user_id("user456".to_string());

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();

    // Query with combined filters
    let filters = AuditQueryFilters {
        user_id: Some("user123".to_string()),
        action: Some("create_instance".to_string()),
        tenant_id: Some("tenant-abc".to_string()),
        ..Default::default()
    };
    let events = logger.query_events(&filters).await.unwrap();
    assert!(events.iter().all(|e| {
        e.user_id.as_ref() == Some(&"user123".to_string())
            && e.action == "create_instance"
            && e.tenant_id.as_ref() == Some(&"tenant-abc".to_string())
    }));
}
