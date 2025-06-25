pub mod api;

use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout_ms: u64,
    pub enable_ssl: bool,
    pub rate_limit_requests_per_minute: usize,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub client_ip: String,
    pub user_agent: String,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub request_count: u64,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub client_ip: String,
    pub requests_in_window: usize,
    pub window_start: Instant,
}

pub struct Network {
    config: NetworkConfig,
    connections: Arc<RwLock<HashMap<String, ConnectionInfo>>>,
    rate_limits: Arc<RwLock<HashMap<String, RateLimitInfo>>>,
    api_server: Option<api::ApiServer>,
}

impl Network {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            api_server: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), AppError> {
        // Создание и запуск API сервера
        let api_server = api::ApiServer::new(
            self.config.clone(),
            self.connections.clone(),
            self.rate_limits.clone(),
        ).await?;
        
        self.api_server = Some(api_server);
        
        // Запуск фоновых задач
        self.start_background_tasks().await?;
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), AppError> {
        if let Some(api_server) = &self.api_server {
            api_server.shutdown().await?;
        }
        
        // Закрытие всех соединений
        self.connections.write().await.clear();
        self.rate_limits.write().await.clear();
        
        Ok(())
    }

    pub async fn process_request(&self, request: ModelRequest, client_ip: &str) -> Result<ModelResponse, AppError> {
        // Проверка rate limit
        self.check_rate_limit(client_ip).await?;
        
        // Проверка аутентификации
        self.check_authentication(client_ip).await?;
        
        // Обновление активности соединения
        self.update_connection_activity(client_ip).await?;
        
        // Обработка запроса через API
        if let Some(api_server) = &self.api_server {
            api_server.process_model_request(request).await
        } else {
            Err(AppError::ServiceUnavailable)
        }
    }

    async fn check_rate_limit(&self, client_ip: &str) -> Result<(), AppError> {
        let mut rate_limits = self.rate_limits.write().await;
        let now = Instant::now();
        
        if let Some(rate_limit) = rate_limits.get_mut(client_ip) {
            // Проверка окна времени
            if now.duration_since(rate_limit.window_start) > Duration::from_secs(60) {
                // Сброс счетчика для нового окна
                rate_limit.requests_in_window = 1;
                rate_limit.window_start = now;
            } else {
                // Увеличение счетчика
                rate_limit.requests_in_window += 1;
                
                // Проверка лимита
                if rate_limit.requests_in_window > self.config.rate_limit_requests_per_minute {
                    return Err(AppError::RateLimitExceeded);
                }
            }
        } else {
            // Создание новой записи
            rate_limits.insert(client_ip.to_string(), RateLimitInfo {
                client_ip: client_ip.to_string(),
                requests_in_window: 1,
                window_start: now,
            });
        }
        
        Ok(())
    }

    async fn check_authentication(&self, client_ip: &str) -> Result<(), AppError> {
        let connections = self.connections.read().await;
        
        if let Some(connection) = connections.get(client_ip) {
            if !connection.is_authenticated {
                return Err(AppError::AuthenticationRequired);
            }
        } else {
            return Err(AppError::ConnectionNotFound);
        }
        
        Ok(())
    }

    async fn update_connection_activity(&self, client_ip: &str) -> Result<(), AppError> {
        let mut connections = self.connections.write().await;
        
        if let Some(connection) = connections.get_mut(client_ip) {
            connection.last_activity = Instant::now();
            connection.request_count += 1;
        }
        
        Ok(())
    }

    pub async fn add_connection(&self, connection_info: ConnectionInfo) -> Result<(), AppError> {
        let mut connections = self.connections.write().await;
        
        // Проверка лимита соединений
        if connections.len() >= self.config.max_connections {
            return Err(AppError::ConnectionLimitExceeded);
        }
        
        connections.insert(connection_info.connection_id.clone(), connection_info);
        
        Ok(())
    }

    pub async fn remove_connection(&self, connection_id: &str) -> Result<(), AppError> {
        let mut connections = self.connections.write().await;
        connections.remove(connection_id);
        
        Ok(())
    }

    pub async fn get_connection_info(&self, connection_id: &str) -> Option<ConnectionInfo> {
        let connections = self.connections.read().await;
        connections.get(connection_id).cloned()
    }

    pub async fn list_connections(&self) -> Vec<ConnectionInfo> {
        let connections = self.connections.read().await;
        connections.values().cloned().collect()
    }

    pub async fn authenticate_connection(&self, connection_id: &str, token: &str) -> Result<bool, AppError> {
        // Заглушка для аутентификации
        // В реальной реализации здесь будет проверка токена
        
        let mut connections = self.connections.write().await;
        
        if let Some(connection) = connections.get_mut(connection_id) {
            // Простая проверка токена
            if token == "valid_token" {
                connection.is_authenticated = true;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(AppError::ConnectionNotFound)
        }
    }

    async fn start_background_tasks(&self) -> Result<(), AppError> {
        let connections = self.connections.clone();
        let rate_limits = self.rate_limits.clone();
        let connection_timeout = Duration::from_millis(self.config.connection_timeout_ms);
        
        // Задача очистки неактивных соединений
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let mut connections_to_remove = Vec::new();
                let mut rate_limits_to_remove = Vec::new();
                
                // Очистка неактивных соединений
                {
                    let mut conns = connections.write().await;
                    for (connection_id, connection) in conns.iter() {
                        if now.duration_since(connection.last_activity) > connection_timeout {
                            connections_to_remove.push(connection_id.clone());
                        }
                    }
                    
                    for connection_id in connections_to_remove {
                        conns.remove(&connection_id);
                    }
                }
                
                // Очистка устаревших rate limits
                {
                    let mut rate_lims = rate_limits.write().await;
                    for (client_ip, rate_limit) in rate_lims.iter() {
                        if now.duration_since(rate_limit.window_start) > Duration::from_secs(120) {
                            rate_limits_to_remove.push(client_ip.clone());
                        }
                    }
                    
                    for client_ip in rate_limits_to_remove {
                        rate_lims.remove(&client_ip);
                    }
                }
            }
        });
        
        Ok(())
    }
} 