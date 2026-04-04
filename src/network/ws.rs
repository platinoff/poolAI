//! WebSocket module for real-time updates (Stage 3)
//!
//! Handlers use [`crate::core::state::ApiContext`] for the shared [`WebSocketManager`].
//! Types and connection hub live in [`crate::core::ws_manager`].

pub use crate::core::ws_manager::{
    LiveMetrics, SystemEvent, WebSocketConnection, WebSocketManager, WebSocketMessage,
};

use crate::core::state::ApiContext;
use crate::network::auth::{validate_token, Claims};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

/// WebSocket upgrade handler with authentication
pub async fn websocket_handler(
    State(ctx): State<ApiContext>,
    ws: WebSocketUpgrade,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    let token = extract_token_from_request(&req);

    match token {
        Some(token) => match validate_token(&token) {
            Ok(claims) => {
                let mgr = ctx.ws_manager.clone();
                ws.on_upgrade(move |socket| handle_websocket_connection(socket, claims, mgr))
            }
            Err(_) => (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid authentication token"
                })),
            )
                .into_response(),
        },
        None => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Authentication token required"
            })),
        )
            .into_response(),
    }
}

async fn handle_websocket_connection(
    socket: WebSocket,
    claims: Claims,
    mgr: Arc<WebSocketManager>,
) {
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

    mgr.add_connection(connection_id.clone(), connection).await;

    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    mgr.register_sender(connection_id.clone(), tx).await;

    let connection_id_send = connection_id.clone();
    let mgr_send = mgr.clone();
    let sender_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
        mgr_send.remove_connection(&connection_id_send).await;
    });

    let mgr_hb = mgr.clone();
    let heartbeat_handle = tokio::spawn(heartbeat_loop(connection_id.clone(), mgr_hb));

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let text_str = text.to_string();
                if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&text_str) {
                    match message.message_type.as_str() {
                        "heartbeat" => {
                            update_heartbeat(&mgr, &connection_id).await;
                        }
                        "subscribe_metrics" => {
                            handle_metrics_subscription(&mgr, &connection_id, &claims).await;
                        }
                        "subscribe_events" => {
                            handle_events_subscription(&mgr, &connection_id, &claims).await;
                        }
                        _ => {
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
                                let tx_clone =
                                    mgr.senders.read().await.get(&connection_id).cloned();
                                if let Some(tx) = tx_clone {
                                    let _ = tx.send(Message::Text(json.into()));
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
                let tx_clone = mgr.senders.read().await.get(&connection_id).cloned();
                if let Some(tx) = tx_clone {
                    if tx.send(Message::Pong(data)).is_err() {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    heartbeat_handle.abort();
    sender_task.abort();
    mgr.remove_connection(&connection_id).await;
}

async fn heartbeat_loop(connection_id: String, mgr: Arc<WebSocketManager>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));

    loop {
        interval.tick().await;

        if !mgr.connections.read().await.contains_key(&connection_id) {
            break;
        }

        update_heartbeat(&mgr, &connection_id).await;
    }
}

async fn update_heartbeat(mgr: &WebSocketManager, connection_id: &str) {
    let mut connections = mgr.connections.write().await;
    if let Some(connection) = connections.get_mut(connection_id) {
        connection.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

async fn handle_metrics_subscription(mgr: &WebSocketManager, connection_id: &str, claims: &Claims) {
    if !claims.permissions.contains(&"read:metrics".to_string()) {
        return;
    }

    mgr.subscribe_metrics(connection_id).await;
    tracing::info!("User {} subscribed to metrics", claims.sub);
}

async fn handle_events_subscription(mgr: &WebSocketManager, connection_id: &str, claims: &Claims) {
    if !claims.permissions.contains(&"read:events".to_string()) {
        return;
    }

    mgr.subscribe_events(connection_id).await;
    tracing::info!("User {} subscribed to system events", claims.sub);
}

/// Broadcast system update to all connected WebSocket clients (uses shared manager).
pub async fn broadcast_update(manager: &WebSocketManager, message: &str) {
    manager.broadcast_system_update(message).await;
}

/// Send live metrics to all subscribed WebSocket clients.
pub async fn send_live_metrics(manager: &WebSocketManager, metrics: LiveMetrics) {
    manager.broadcast_live_metrics_payload(metrics).await;
}

/// Send system event to all subscribed WebSocket clients.
pub async fn send_system_event(manager: &WebSocketManager, event: SystemEvent) {
    manager.broadcast_system_event_payload(event).await;
}

fn extract_token_from_request(req: &Request<axum::body::Body>) -> Option<String> {
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
