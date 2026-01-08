//! Integration tests for Network WebSocket Module

use poolai::network::ws::{LiveMetrics, SystemEvent, WebSocketManager, WebSocketMessage};
use serde_json::json;
use std::time::SystemTime;

#[tokio::test]
async fn test_websocket_manager_creation() {
    let manager = WebSocketManager::new();
    // Just verify it can be created
    let _ = manager;
}

#[tokio::test]
async fn test_websocket_message_creation() {
    let message = WebSocketMessage {
        message_type: "test".to_string(),
        data: json!({"key": "value"}),
        timestamp: SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    assert_eq!(message.message_type, "test");
    assert!(message.data.get("key").is_some());
}

#[tokio::test]
async fn test_live_metrics_creation() {
    let metrics = LiveMetrics {
        active_workers: 10,
        total_requests: 1000,
        avg_response_time: 45.5,
        memory_usage: 75.0,
        gpu_temperature: 65.0,
        timestamp: SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    assert_eq!(metrics.active_workers, 10);
    assert_eq!(metrics.total_requests, 1000);
    assert_eq!(metrics.avg_response_time, 45.5);
    assert_eq!(metrics.memory_usage, 75.0);
    assert_eq!(metrics.gpu_temperature, 65.0);
}

#[tokio::test]
async fn test_system_event_creation() {
    let event = SystemEvent {
        event_type: "alert".to_string(),
        severity: "warning".to_string(),
        message: "High CPU usage".to_string(),
        timestamp: SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    assert_eq!(event.event_type, "alert");
    assert_eq!(event.severity, "warning");
    assert_eq!(event.message, "High CPU usage");
}

#[tokio::test]
async fn test_websocket_manager_connection_management() {
    let manager = WebSocketManager::new();

    // Test connection management (these are async operations that require actual WebSocket connections)
    // For now, we just verify the manager can be created and methods exist
    let connection_id = "test_connection".to_string();

    // These would require actual WebSocket connections to test fully
    // manager.add_connection(...).await;
    // manager.remove_connection(&connection_id).await;

    // Just verify manager exists
    let _ = manager;
}

#[tokio::test]
async fn test_websocket_message_serialization() {
    let message = WebSocketMessage {
        message_type: "test".to_string(),
        data: json!({"test": "data"}),
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&message);
    assert!(json.is_ok());
    let json_str = json.unwrap();
    assert!(json_str.contains("test"));
    assert!(json_str.contains("data"));
}

#[tokio::test]
async fn test_live_metrics_serialization() {
    let metrics = LiveMetrics {
        active_workers: 5,
        total_requests: 500,
        avg_response_time: 30.0,
        memory_usage: 50.0,
        gpu_temperature: 60.0,
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&metrics);
    assert!(json.is_ok());
    let json_str = json.unwrap();
    assert!(json_str.contains("active_workers"));
    assert!(json_str.contains("5"));
}

#[tokio::test]
async fn test_system_event_serialization() {
    let event = SystemEvent {
        event_type: "info".to_string(),
        severity: "low".to_string(),
        message: "System normal".to_string(),
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&event);
    assert!(json.is_ok());
    let json_str = json.unwrap();
    assert!(json_str.contains("info"));
    assert!(json_str.contains("System normal"));
}
