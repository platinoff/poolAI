// network/ws.rs
// WebSocket Security для real-time оновлень (Stage 3)

use crate::network::auth::{validate_token, Claims};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

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
    // Channel для відправки повідомлень до з'єднань
    senders: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    // Підписки на метрики
    metrics_subscriptions: Arc<RwLock<HashMap<String, bool>>>,
    // Підписки на події
    events_subscriptions: Arc<RwLock<HashMap<String, bool>>>,
}

pub struct WebSocketConnection {
    pub user_id: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub last_heartbeat: u64,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let manager = Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            senders: Arc::new(RwLock::new(HashMap::new())),
            metrics_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            events_subscriptions: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Запускаємо періодичну відправку метрик та подій
        let metrics_subs = manager.metrics_subscriptions.clone();
        let events_subs = manager.events_subscriptions.clone();
        let senders_clone = manager.senders.clone();
        
        tokio::spawn(async move {
            let mut metrics_interval = tokio::time::interval(std::time::Duration::from_secs(5));
            loop {
                metrics_interval.tick().await;
                
                // Відправка метрик підписаним користувачам
                let subs = metrics_subs.read().await;
                let senders = senders_clone.read().await;
                for (connection_id, _) in subs.iter() {
                    if let Some(sender) = senders.get(connection_id) {
                        let metrics = get_current_metrics().await;
                        let ws_message = WebSocketMessage {
                            message_type: "live_metrics".to_string(),
                            data: serde_json::to_value(metrics).unwrap_or_default(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };
                        if let Ok(json) = serde_json::to_string(&ws_message) {
                            let _ = sender.send(Message::Text(json));
                        }
                    }
                }
            }
        });
        
        tokio::spawn(async move {
            let mut events_interval = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                events_interval.tick().await;
                
                // Відправка подій підписаним користувачам
                let subs = events_subs.read().await;
                let senders = senders_clone.read().await;
                for (connection_id, _) in subs.iter() {
                    if let Some(sender) = senders.get(connection_id) {
                        // В реальній реалізації тут були б нові події з event store
                        // Зараз просто heartbeat для підтримки підписки
                    }
                }
            }
        });
        
        manager
    }
    
    /// Register sender for a connection
    pub async fn register_sender(&self, connection_id: String, sender: mpsc::UnboundedSender<Message>) {
        let mut senders = self.senders.write().await;
        senders.insert(connection_id, sender);
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
        let mut senders = self.senders.write().await;
        senders.remove(connection_id);
        let mut metrics_subs = self.metrics_subscriptions.write().await;
        metrics_subs.remove(connection_id);
        let mut events_subs = self.events_subscriptions.write().await;
        events_subs.remove(connection_id);
    }
    
    /// Subscribe connection to metrics updates
    pub async fn subscribe_metrics(&self, connection_id: &str) {
        let mut subs = self.metrics_subscriptions.write().await;
        subs.insert(connection_id.to_string(), true);
    }
    
    /// Subscribe connection to events updates
    pub async fn subscribe_events(&self, connection_id: &str) {
        let mut subs = self.events_subscriptions.write().await;
        subs.insert(connection_id.to_string(), true);
    }

    // Broadcast повідомлення до всіх з'єднань
    pub async fn broadcast(&self, message: WebSocketMessage) {
        let senders = self.senders.read().await;
        let message_json = match serde_json::to_string(&message) {
            Ok(json) => json,
            Err(e) => {
                tracing::warn!("Failed to serialize WebSocket message: {}", e);
                return;
            }
        };
        
        for (connection_id, sender) in senders.iter() {
            if let Err(e) = sender.send(Message::Text(message_json.clone())) {
                tracing::warn!("Failed to send message to connection {}: {}", connection_id, e);
            }
        }
    }

    // Відправка повідомлення конкретному користувачу
    pub async fn send_to_user(&self, user_id: &str, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        let senders = self.senders.read().await;
        
        if let Some(connection) = connections.get(user_id) {
            let message_json = match serde_json::to_string(&message) {
                Ok(json) => json,
                Err(e) => {
                    tracing::warn!("Failed to serialize WebSocket message: {}", e);
                    return;
                }
            };
            
            if let Some(sender) = senders.get(user_id) {
                if let Err(e) = sender.send(Message::Text(message_json)) {
                    tracing::warn!("Failed to send message to user {}: {}", connection.user_id, e);
                }
            }
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
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({
                            "error": "Invalid authentication token"
                        })),
                    )
                        .into_response()
                }
            }
        }
        None => {
            // Токен відсутній
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Authentication token required"
                })),
            )
                .into_response()
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
    WS_MANAGER
        .add_connection(connection_id.clone(), connection)
        .await;

    // Розбиваємо WebSocket на sender та receiver
    let (mut sender, mut receiver) = socket.split();
    
    // Створюємо channel для відправки повідомлень
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    WS_MANAGER.register_sender(connection_id.clone(), tx).await;
    
    // Запускаємо task для відправки повідомлень через sender
    let connection_id_send = connection_id.clone();
    let sender_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
        WS_MANAGER.remove_connection(&connection_id_send).await;
    });

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
                                // Send error message through channel
                                let tx_clone = WS_MANAGER.senders.read().await.get(&connection_id).cloned();
                                if let Some(tx) = tx_clone {
                                    let _ = tx.send(Message::Text(json));
                                }
                            }
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(Message::Ping(data)) => {
                // Відповідаємо на ping через channel
                let tx_clone = WS_MANAGER.senders.read().await.get(&connection_id).cloned();
                if let Some(tx) = tx_clone {
                    if tx.send(Message::Pong(data)).is_err() {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    // Очищаємо ресурси
    heartbeat_handle.abort();
    sender_task.abort();
    WS_MANAGER.remove_connection(&connection_id).await;
}

// Heartbeat цикл для підтримки з'єднань
async fn heartbeat_loop(connection_id: String) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));

    loop {
        interval.tick().await;

        // Перевіряємо, чи з'єднання все ще активне
        if !WS_MANAGER
            .connections
            .read()
            .await
            .contains_key(&connection_id)
        {
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
async fn handle_metrics_subscription(connection_id: &str, claims: &Claims) {
    // Перевіряємо права доступу
    if !claims.permissions.contains(&"read:metrics".to_string()) {
        return;
    }

    // Підписуємо з'єднання на метрики
    WS_MANAGER.subscribe_metrics(connection_id).await;
    tracing::info!("User {} subscribed to metrics", claims.sub);
}

// Обробка підписки на системні події
async fn handle_events_subscription(connection_id: &str, claims: &Claims) {
    // Перевіряємо права доступу
    if !claims.permissions.contains(&"read:events".to_string()) {
        return;
    }

    // Підписуємо з'єднання на події
    WS_MANAGER.subscribe_events(connection_id).await;
    tracing::info!("User {} subscribed to system events", claims.sub);
}

// Отримання поточних метрик системи
async fn get_current_metrics() -> LiveMetrics {
    let active_workers = if let Some(pool) = crate::pool::get_global_pool() {
        let pool_guard = pool.read().await;
        pool_guard.get_worker_count().await as u32
    } else {
        0
    };
    
    // В реальній реалізації тут були б реальні метрики з monitoring модуля
    LiveMetrics {
        active_workers,
        total_requests: 0,
        avg_response_time: 0.0,
        memory_usage: 0.0,
        gpu_temperature: 0.0,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
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
