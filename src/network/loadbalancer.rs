use std::sync::Arc;
use parking_lot::RwLock;
use log::info;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use thiserror::Error;
use crate::lmrouter::{ModelConfig, ModelStats};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use rand;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Error, Debug)]
pub enum LoadBalancerError {
    #[error("Failed to acquire model: {0}")]
    AcquisitionError(String),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Failed to acquire permit")]
    AcquireError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub url: String,
    pub weight: u32,
    pub max_connections: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub current_connections: u32,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub config: NodeConfig,
    pub stats: NodeStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub algorithm: String,
    pub health_check_interval: u64,
    pub max_retries: u32,
    pub timeout: u64,
}

pub struct LoadBalancer {
    config: Arc<Mutex<LoadBalancerConfig>>,
    nodes: Arc<Mutex<HashMap<String, NodeMetrics>>>,
}

impl LoadBalancer {
    pub fn new(config: LoadBalancerConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, config: NodeConfig) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        if nodes.contains_key(&config.id) {
            return Err(format!("Node '{}' already exists", config.id));
        }

        let metrics = NodeMetrics {
            config,
            stats: NodeStats {
                current_connections: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
            },
        };

        nodes.insert(metrics.config.id.clone(), metrics);
        info!("Added new node: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_node(&self, id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        if !nodes.contains_key(id) {
            return Err(format!("Node '{}' not found", id));
        }

        nodes.remove(id);
        info!("Removed node: {}", id);
        Ok(())
    }

    pub async fn select_node(&self) -> Result<NodeMetrics, String> {
        let nodes = self.nodes.lock().await;
        let config = self.config.lock().await;

        let active_nodes: Vec<_> = nodes
            .values()
            .filter(|n| n.config.active && n.stats.current_connections < n.config.max_connections)
            .cloned()
            .collect();

        if active_nodes.is_empty() {
            return Err("No available nodes".to_string());
        }

        match config.algorithm.as_str() {
            "round_robin" => {
                // Simple round-robin selection
                let node = active_nodes.first().unwrap();
                Ok(node.clone())
            }
            "least_connections" => {
                // Select node with least current connections
                let node = active_nodes
                    .iter()
                    .min_by_key(|n| n.stats.current_connections)
                    .unwrap();
                Ok(node.clone())
            }
            "weighted" => {
                // Weighted selection based on node weights
                let total_weight: u32 = active_nodes.iter().map(|n| n.config.weight).sum();
                let mut rng = rand::thread_rng();
                let mut random = rand::Rng::gen_range(&mut rng, 0..total_weight);
                
                for node in active_nodes {
                    if random < node.config.weight {
                        return Ok(node);
                    }
                    random -= node.config.weight;
                }
                
                // Fallback to first node if something goes wrong
                Ok(active_nodes.first().unwrap().clone())
            }
            _ => Err("Invalid load balancing algorithm".to_string()),
        }
    }

    pub async fn update_node_stats(
        &self,
        id: &str,
        success: bool,
        response_time: f64,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node = nodes
            .get_mut(id)
            .ok_or_else(|| format!("Node '{}' not found", id))?;

        if success {
            node.stats.successful_requests += 1;
        } else {
            node.stats.failed_requests += 1;
            node.stats.last_error = error;
        }

        node.stats.total_requests += 1;
        let total_time = node.stats.average_response_time * (node.stats.total_requests - 1) as f64;
        node.stats.average_response_time = (total_time + response_time) / node.stats.total_requests as f64;
        
        node.stats.last_request_time = Some(Utc::now());

        Ok(())
    }

    pub async fn increment_connections(&self, id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node = nodes
            .get_mut(id)
            .ok_or_else(|| format!("Node '{}' not found", id))?;

        if node.stats.current_connections >= node.config.max_connections {
            return Err("Maximum connections reached".to_string());
        }

        node.stats.current_connections += 1;
        Ok(())
    }

    pub async fn decrement_connections(&self, id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node = nodes
            .get_mut(id)
            .ok_or_else(|| format!("Node '{}' not found", id))?;

        if node.stats.current_connections == 0 {
            return Err("No active connections".to_string());
        }

        node.stats.current_connections -= 1;
        Ok(())
    }

    pub async fn get_node(&self, id: &str) -> Result<NodeMetrics, String> {
        let nodes = self.nodes.lock().await;
        
        nodes
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Node '{}' not found", id))
    }

    pub async fn get_all_nodes(&self) -> Vec<NodeMetrics> {
        let nodes = self.nodes.lock().await;
        nodes.values().cloned().collect()
    }

    pub async fn get_active_nodes(&self) -> Vec<NodeMetrics> {
        let nodes = self.nodes.lock().await;
        nodes
            .values()
            .filter(|n| n.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_node_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node = nodes
            .get_mut(id)
            .ok_or_else(|| format!("Node '{}' not found", id))?;

        node.config.active = active;
        info!(
            "Node '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_config(&self, new_config: LoadBalancerConfig) -> Result<(), String> {
        let mut config = self.config.lock().await;
        *config = new_config;
        info!("Updated load balancer configuration");
        Ok(())
    }

    pub async fn get_config(&self) -> LoadBalancerConfig {
        self.config.lock().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_registration() {
        let balancer = LoadBalancer::new(3, 1000, 60);
        let config = ModelConfig {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: "https://test.com".to_string(),
            max_tokens: 1000,
            max_requests_per_minute: 60,
            priority: 1,
        };
        
        assert!(balancer.register_model("test_model".to_string(), config).await.is_ok());
    }

    #[tokio::test]
    async fn test_model_acquisition() {
        let balancer = LoadBalancer::new(3, 1000, 60);
        let config = ModelConfig {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: "https://test.com".to_string(),
            max_tokens: 1000,
            max_requests_per_minute: 60,
            priority: 1,
        };
        
        balancer.register_model("test_model".to_string(), config).await.unwrap();
        
        let requirements = crate::lmrouter::ModelRequirements {
            min_tokens: 500,
            min_priority: 1,
        };
        
        assert!(balancer.get_available_model(&requirements).await.is_ok());
    }
} 