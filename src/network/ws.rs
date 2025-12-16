// network/ws.rs
// WebSocket Security для real-time оновлень (Stage 3)

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    http::{Request, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::network::auth::{Claims, validate_token};

// Структура для WebSocket повідомлень
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

// Структура для метрик в реальному часі
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveMetrics {
    pub active_workers: u32,
    pub total_requests: u64,
    pub avg_response_time: f64,
    pub memory_usage: f64,
    pub gpu_temperature: f64,
    pub timestamp: u64,
}

// Структура для системних подій
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: u64,
}

// Менеджер WebSocket з'єднань
pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<String, WebSocketConnection>>>,
}

pub struct WebSocketConnection {
    pub user_id: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub last_heartbeat: u64,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Додавання нового з'єднання
    pub async fn add_connection(&self, connection_id: String, connection: WebSocketConnection) {
        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection);
    }

    // Видалення з'єднання
    pub async fn remove_connection(&self, connection_id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(connection_id);
    }

    // Broadcast повідомлення до всіх з'єднань
    pub async fn broadcast(&self, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        for (_, connection) in connections.iter() {
            // TODO: Відправка повідомлення через WebSocket
            println!("Broadcasting to user {}: {:?}", connection.user_id, message);
        }
    }

    // Відправка повідомлення конкретному користувачу
    pub async fn send_to_user(&self, user_id: &str, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        if let Some(connection) = connections.get(user_id) {
            println!("Sending to user {}: {:?}", connection.user_id, message);
        }
    }

    // Очищення неактивних з'єднань
    pub async fn cleanup_inactive(&self) {
        let mut connections = self.connections.write().await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        connections.retain(|_, connection| {
            current_time - connection.last_heartbeat < 300 // 5 хвилин
        });
    }
}

// Глобальний екземпляр WebSocket менеджера
lazy_static::lazy_static! {
    static ref WS_MANAGER: WebSocketManager = WebSocketManager::new();
}

// WebSocket upgrade handler з аутентифікацією
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    // Перевіряємо JWT токен з query параметрів або заголовків
    let token = extract_token_from_request(&req);
    
    match token {
        Some(token) => {
            match validate_token(&token) {
                Ok(claims) => {
                    // Аутентифікація успішна, оновлюємо WebSocket
                    ws.on_upgrade(|socket| handle_websocket_connection(socket, claims))
                }
                Err(_) => {
                    // Невірний токен
                    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                        "error": "Invalid authentication token"
                    }))).into_response()
                }
            }
        }
        None => {
            // Токен відсутній
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Authentication token required"
            }))).into_response()
        }
    }
}

// Обробка WebSocket з'єднання
async fn handle_websocket_connection(socket: WebSocket, claims: Claims) {
    let connection_id = format!("ws_{}", claims.sub);
    let connection = WebSocketConnection {
        user_id: claims.sub.clone(),
        role: format!("{:?}", claims.role),
        permissions: claims.permissions.clone(),
        last_heartbeat: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    // Додаємо з'єднання до менеджера
    WS_MANAGER.add_connection(connection_id.clone(), connection).await;

    // Розбиваємо WebSocket на sender та receiver
    let (mut sender, mut receiver) = socket.split();

    // Запускаємо heartbeat
    let heartbeat_handle = tokio::spawn(heartbeat_loop(connection_id.clone()));

    // Основний цикл обробки повідомлень
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&text) {
                    match message.message_type.as_str() {
                        "heartbeat" => {
                            // Оновлюємо heartbeat
                            update_heartbeat(&connection_id).await;
                        }
                        "subscribe_metrics" => {
                            // Підписка на метрики
                            handle_metrics_subscription(&connection_id, &claims).await;
                        }
                        "subscribe_events" => {
                            // Підписка на системні події
                            handle_events_subscription(&connection_id, &claims).await;
                        }
                        _ => {
                            // Невідомий тип повідомлення
                            let error_msg = WebSocketMessage {
                                message_type: "error".to_string(),
                                data: serde_json::json!({
                                    "error": "Unknown message type",
                                    "received": message.message_type
                                }),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            };
                            
                            if let Ok(json) = serde_json::to_string(&error_msg) {
                                let _ = sender.send(Message::Text(json)).await;
                            }
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(Message::Ping(data)) => {
                // Відповідаємо на ping
                if let Err(_) = sender.send(Message::Pong(data)).await {
                    break;
                }
            }
            _ => {}
        }
    }

    // Очищаємо ресурси
    heartbeat_handle.abort();
    WS_MANAGER.remove_connection(&connection_id).await;
}

// Heartbeat цикл для підтримки з'єднань
async fn heartbeat_loop(connection_id: String) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        // Перевіряємо, чи з'єднання все ще активне
        if !WS_MANAGER.connections.read().await.contains_key(&connection_id) {
            break;
        }
        
        // Оновлюємо heartbeat
        update_heartbeat(&connection_id).await;
    }
}

// Оновлення heartbeat для з'єднання
async fn update_heartbeat(connection_id: &str) {
    let mut connections = WS_MANAGER.connections.write().await;
    if let Some(connection) = connections.get_mut(connection_id) {
        connection.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

// Обробка підписки на метрики
async fn handle_metrics_subscription(_connection_id: &str, claims: &Claims) {
    // Перевіряємо права доступу
    if !claims.permissions.contains(&"read:metrics".to_string()) {
        return;
    }

    // TODO: Запуск періодичної відправки метрик
    println!("User {} subscribed to metrics", claims.sub);
}

// Обробка підписки на системні події
async fn handle_events_subscription(_connection_id: &str, claims: &Claims) {
    // Перевіряємо права доступу
    if !claims.permissions.contains(&"read:events".to_string()) {
        return;
    }

    // TODO: Запуск періодичної відправки подій
    println!("User {} subscribed to system events", claims.sub);
}

// Функція для broadcast оновлень (публічне API)
pub async fn broadcast_update(message: &str) {
    let ws_message = WebSocketMessage {
        message_type: "system_update".to_string(),
        data: serde_json::json!({
            "message": message,
            "severity": "info"
        }),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    WS_MANAGER.broadcast(ws_message).await;
}

// Функція для відправки метрик в реальному часі
pub async fn send_live_metrics(metrics: LiveMetrics) {
    let ws_message = WebSocketMessage {
        message_type: "live_metrics".to_string(),
        data: serde_json::to_value(metrics).unwrap(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    WS_MANAGER.broadcast(ws_message).await;
}

// Функція для відправки системних подій
pub async fn send_system_event(event: SystemEvent) {
    let ws_message = WebSocketMessage {
        message_type: "system_event".to_string(),
        data: serde_json::to_value(event).unwrap(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    WS_MANAGER.broadcast(ws_message).await;
}

// Допоміжна функція для витягування токена з запиту
fn extract_token_from_request(req: &Request<axum::body::Body>) -> Option<String> {
    // Спочатку перевіряємо query параметри
    if let Some(query) = req.uri().query() {
        if let Some(token_start) = query.find("token=") {
            let token_part = &query[token_start + 6..];
            if let Some(token_end) = token_part.find('&') {
                return Some(token_part[..token_end].to_string());
            } else {
                return Some(token_part.to_string());
            }
        }
    }

    // Потім перевіряємо заголовки
    req.headers()
        .get("Sec-WebSocket-Protocol")
        .and_then(|h| h.to_str().ok())
        .and_then(|protocol| {
            if protocol.starts_with("token.") {
                Some(protocol[6..].to_string())
            } else {
                None
            }
        })
}