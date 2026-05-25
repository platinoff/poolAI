//! Integration tests for Network WebSocket Module

use chrono::Utc;
use poolai::network::ws::{
    LiveMetrics, SystemEvent, TopologyLiveUpdate, WebSocketManager, WebSocketMessage,
};
use poolai::pool::topology::{NodeResources, Topology};
use serde_json::json;
use std::collections::HashMap;
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
    let _connection_id = "test_connection".to_string();

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

#[test]
fn test_topology_live_update_from_topology() {
    let topology = Topology {
        latency_matrix: HashMap::from([("a:b".to_string(), 1.5)]),
        bandwidth_matrix: HashMap::new(),
        node_resources: HashMap::from([(
            "a".to_string(),
            NodeResources {
                node_id: "a".to_string(),
                available_gpu_memory_mb: 1000,
                total_gpu_memory_mb: 2000,
                available_cpu_cores: 4,
                total_cpu_cores: 8,
                available_memory_mb: 8000,
                total_memory_mb: 16_000,
                current_load: 0.25,
            },
        )]),
        last_updated: Utc::now(),
    };

    let update = TopologyLiveUpdate::from_topology(&topology);
    assert_eq!(update.node_count, 1);
    assert_eq!(update.latency_measurements, 1);
    assert_eq!(update.node_ids, vec!["a".to_string()]);
    assert!(update.nodes.contains_key("a"));

    let json = serde_json::to_string(&update).expect("serialize topology live update");
    assert!(json.contains("latency_matrix"));
    assert!(json.contains("node_count"));
}
