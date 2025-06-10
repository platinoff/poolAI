use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use log::info;
use std::collections::HashMap;
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum ModelConfigError {
    #[error("Invalid endpoint URL: {0}")]
    InvalidEndpoint(String),
    #[error("Endpoint must use HTTPS")]
    NonHttpsEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub version: String,
    pub max_tokens: usize,
    pub min_tokens: usize,
    pub priority: u32,
    pub max_requests_per_minute: u32,
    pub active: bool,
}

impl ModelConfig {
    pub fn validate_endpoint(&self) -> Result<(), ModelConfigError> {
        let url = Url::parse(&self.endpoint)
            .map_err(|e| ModelConfigError::InvalidEndpoint(e.to_string()))?;
        
        if url.scheme() != "https" {
            return Err(ModelConfigError::NonHttpsEndpoint);
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub current_requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub config: ModelConfig,
    pub stats: ModelStats,
}

pub struct LMRouter {
    models: Arc<Mutex<HashMap<String, ModelMetrics>>>,
}

impl LMRouter {
    pub fn new() -> Self {
        Self {
            models: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_model(&self, config: ModelConfig) -> Result<(), String> {
        let mut models = self.models.lock().await;
        
        if models.contains_key(&config.id) {
            return Err(format!("Model '{}' already exists", config.id));
        }

        let metrics = ModelMetrics {
            config,
            stats: ModelStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
                current_requests: 0,
            },
        };

        models.insert(metrics.config.id.clone(), metrics);
        info!("Added new model: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_model(&self, id: &str) -> Result<(), String> {
        let mut models = self.models.lock().await;
        
        if !models.contains_key(id) {
            return Err(format!("Model '{}' not found", id));
        }

        models.remove(id);
        info!("Removed model: {}", id);
        Ok(())
    }

    pub async fn select_model(&self, requirements: &ModelRequirements) -> Result<ModelMetrics, String> {
        let models = self.models.lock().await;
        
        let available_models: Vec<_> = models
            .values()
            .filter(|m| {
                m.config.active
                    && m.config.max_tokens >= requirements.min_tokens
                    && m.config.min_tokens <= requirements.max_tokens
                    && m.config.priority >= requirements.min_priority
                    && m.stats.current_requests < m.config.max_requests_per_minute
            })
            .cloned()
            .collect();

        if available_models.is_empty() {
            return Err("No available models matching requirements".to_string());
        }

        // Select model with highest priority and lowest current requests
        let selected_model = available_models
            .iter()
            .max_by_key(|m| {
                (
                    m.config.priority,
                    -(m.stats.current_requests as i32),
                    -(m.stats.average_response_time as i32),
                )
            })
            .unwrap()
            .clone();

        Ok(selected_model)
    }

    pub async fn process_request(
        &self,
        model_id: &str,
        input: &str,
    ) -> Result<String, String> {
        let mut models = self.models.lock().await;
        
        let model = models
            .get_mut(model_id)
            .ok_or_else(|| format!("Model '{}' not found", model_id))?;

        if !model.config.active {
            return Err("Model is not active".to_string());
        }

        if model.stats.current_requests >= model.config.max_requests_per_minute {
            return Err("Model has reached maximum requests per minute".to_string());
        }

        model.stats.current_requests += 1;
        let start_time = Utc::now();

        // Simulate model processing
        let result = self.execute_model(model, input).await;
        
        let end_time = Utc::now();
        let response_time = (end_time - start_time).num_milliseconds() as f64;

        match result {
            Ok(output) => {
                model.stats.successful_requests += 1;
                model.stats.average_response_time = 
                    (model.stats.average_response_time * (model.stats.total_requests) as f64 + response_time) 
                    / (model.stats.total_requests + 1) as f64;
                model.stats.last_error = None;
                Ok(output)
            }
            Err(e) => {
                model.stats.failed_requests += 1;
                model.stats.last_error = Some(e.clone());
                Err(e)
            }
        }
    }

    async fn execute_model(&self, model: &mut ModelMetrics, input: &str) -> Result<String, String> {
        // Simulate model execution
        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        if tokens.len() < model.config.min_tokens {
            return Err(format!(
                "Input too short ({} tokens, minimum {})",
                tokens.len(),
                model.config.min_tokens
            ));
        }

        if tokens.len() > model.config.max_tokens {
            return Err(format!(
                "Input too long ({} tokens, maximum {})",
                tokens.len(),
                model.config.max_tokens
            ));
        }

        // Simulate processing delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        Ok(format!("Processed by {}: {}", model.config.name, input))
    }

    pub async fn get_model(&self, id: &str) -> Result<ModelMetrics, String> {
        let models = self.models.lock().await;
        
        models
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Model '{}' not found", id))
    }

    pub async fn get_all_models(&self) -> Vec<ModelMetrics> {
        let models = self.models.lock().await;
        models.values().cloned().collect()
    }

    pub async fn get_active_models(&self) -> Vec<ModelMetrics> {
        let models = self.models.lock().await;
        models
            .values()
            .filter(|m| m.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_model_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut models = self.models.lock().await;
        
        let model = models
            .get_mut(id)
            .ok_or_else(|| format!("Model '{}' not found", id))?;

        model.config.active = active;
        info!(
            "Model '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_model_config(&self, id: &str, new_config: ModelConfig) -> Result<(), String> {
        let mut models = self.models.lock().await;
        
        let model = models
            .get_mut(id)
            .ok_or_else(|| format!("Model '{}' not found", id))?;

        model.config = new_config;
        info!("Updated model configuration: {}", id);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequirements {
    pub min_tokens: usize,
    pub max_tokens: usize,
    pub min_priority: u32,
    pub max_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub name: String,
    pub url: String,
    pub api_key: String,
    pub timeout: u64,
    pub max_routes: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub source: String,
    pub destination: String,
    pub priority: u32,
    pub active: bool,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterStats {
    pub total_routes: u64,
    pub active_routes: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterMetrics {
    pub config: RouterConfig,
    pub routes: Vec<Route>,
    pub stats: RouterStats,
}

pub struct RouterManager {
    routers: Arc<Mutex<Vec<RouterMetrics>>>,
}

impl RouterManager {
    pub fn new() -> Self {
        Self {
            routers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_router(&self, config: RouterConfig) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        if routers.iter().any(|r| r.config.name == config.name) {
            return Err(format!("Router '{}' already exists", config.name));
        }

        let metrics = RouterMetrics {
            config,
            routes: Vec::new(),
            stats: RouterStats {
                total_routes: 0,
                active_routes: 0,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
            },
        };

        routers.push(metrics);
        info!("Added new router: {}", metrics.config.name);
        Ok(())
    }

    pub async fn remove_router(&self, name: &str) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        let initial_len = routers.len();
        routers.retain(|r| r.config.name != name);
        
        if routers.len() == initial_len {
            return Err(format!("Router '{}' not found", name));
        }

        info!("Removed router: {}", name);
        Ok(())
    }

    pub async fn add_route(
        &self,
        router_name: &str,
        source: String,
        destination: String,
        priority: u32,
    ) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        let router = routers
            .iter_mut()
            .find(|r| r.config.name == router_name)
            .ok_or_else(|| format!("Router '{}' not found", router_name))?;

        if router.routes.len() >= router.config.max_routes as usize {
            return Err("Maximum number of routes reached".to_string());
        }

        let route = Route {
            source,
            destination,
            priority,
            active: true,
            last_used: None,
        };

        router.routes.push(route);
        router.stats.total_routes += 1;
        router.stats.active_routes += 1;

        info!(
            "Added route {} -> {} to router {}",
            route.source, route.destination, router_name
        );
        Ok(())
    }

    pub async fn remove_route(
        &self,
        router_name: &str,
        source: &str,
        destination: &str,
    ) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        let router = routers
            .iter_mut()
            .find(|r| r.config.name == router_name)
            .ok_or_else(|| format!("Router '{}' not found", router_name))?;

        let initial_len = router.routes.len();
        router.routes.retain(|r| r.source != source || r.destination != destination);
        
        if router.routes.len() == initial_len {
            return Err("Route not found".to_string());
        }

        router.stats.total_routes -= 1;
        router.stats.active_routes -= 1;

        info!(
            "Removed route {} -> {} from router {}",
            source, destination, router_name
        );
        Ok(())
    }

    pub async fn update_router_stats(
        &self,
        name: &str,
        success: bool,
        response_time: f64,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        let router = routers
            .iter_mut()
            .find(|r| r.config.name == name)
            .ok_or_else(|| format!("Router '{}' not found", name))?;

        router.stats.total_requests += 1;
        if success {
            router.stats.successful_requests += 1;
        } else {
            router.stats.failed_requests += 1;
            router.stats.last_error = error;
        }

        let total_time = router.stats.average_response_time * (router.stats.total_requests - 1) as f64;
        router.stats.average_response_time = (total_time + response_time) / router.stats.total_requests as f64;
        
        router.stats.last_request_time = Some(Utc::now());

        Ok(())
    }

    pub async fn get_router(&self, name: &str) -> Result<RouterMetrics, String> {
        let routers = self.routers.lock().await;
        
        routers
            .iter()
            .find(|r| r.config.name == name)
            .cloned()
            .ok_or_else(|| format!("Router '{}' not found", name))
    }

    pub async fn get_all_routers(&self) -> Vec<RouterMetrics> {
        let routers = self.routers.lock().await;
        routers.clone()
    }

    pub async fn get_active_routers(&self) -> Vec<RouterMetrics> {
        let routers = self.routers.lock().await;
        routers
            .iter()
            .filter(|r| r.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_router_active(&self, name: &str, active: bool) -> Result<(), String> {
        let mut routers = self.routers.lock().await;
        
        let router = routers
            .iter_mut()
            .find(|r| r.config.name == name)
            .ok_or_else(|| format!("Router '{}' not found", name))?;

        router.config.active = active;
        info!(
            "Router '{}' {}",
            name,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }
} 