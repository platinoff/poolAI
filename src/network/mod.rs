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
        // Create and start API server
        let api_server = api::ApiServer::new(
            self.config.clone(),
            self.connections.clone(),
            self.rate_limits.clone(),
        ).await?;
        
        self.api_server = Some(api_server);
        
        // Start background tasks
        self.start_background_tasks().await?;
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), AppError> {
        if let Some(api_server) = &self.api_server {
            api_server.shutdown().await?;
        }
        
        // Close all connections
        self.connections.write().await.clear();
        self.rate_limits.write().await.clear();
        
        Ok(())
    }

    pub async fn process_request(&self, request: ModelRequest, client_ip: &str) -> Result<ModelResponse, AppError> {
        // Check rate limit
        self.check_rate_limit(client_ip).await?;
        
        // Check authentication
        self.check_authentication(client_ip).await?;
        
        // Update connection activity
        self.update_connection_activity(client_ip).await?;
        
        // Process request through API
        if let Some(api_server) = &self.api_server {
            api_server.process_model_request(request).await
        } else {
            Err(AppError::Network("Service unavailable".to_string()))
        }
    }

    async fn check_rate_limit(&self, client_ip: &str) -> Result<(), AppError> {
        let mut rate_limits = self.rate_limits.write().await;
        let now = Instant::now();
        
        if let Some(rate_limit) = rate_limits.get_mut(client_ip) {
            // Check time window
            if now.duration_since(rate_limit.window_start) > Duration::from_secs(60) {
                // Reset counter for new window
                rate_limit.requests_in_window = 1;
                rate_limit.window_start = now;
            } else {
                // Increment counter
                rate_limit.requests_in_window += 1;
                
                // Check limit
                if rate_limit.requests_in_window > self.config.rate_limit_requests_per_minute {
                    return Err(AppError::Network("Rate limit exceeded".to_string()));
                }
            }
        } else {
            // Create new entry
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
                return Err(AppError::Network("Authentication required".to_string()));
            }
        } else {
            return Err(AppError::Network("Connection not found".to_string()));
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
        
        // Check connection limit
        if connections.len() >= self.config.max_connections {
            return Err(AppError::Network("Connection limit exceeded".to_string()));
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
        // Stub for authentication
        // In real implementation, token verification would happen here
        
        let mut connections = self.connections.write().await;
        
        if let Some(connection) = connections.get_mut(connection_id) {
            // Simple token check
            if token == "valid_token" {
                connection.is_authenticated = true;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(AppError::Network("Connection not found".to_string()))
        }
    }

    async fn start_background_tasks(&self) -> Result<(), AppError> {
        // Start connection cleanup task
        let connections = self.connections.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let mut connections = connections.write().await;
                let now = Instant::now();
                let timeout = Duration::from_millis(config.connection_timeout_ms);
                
                connections.retain(|_, connection| {
                    now.duration_since(connection.last_activity) < timeout
                });
            }
        });
        
        Ok(())
    }
} 