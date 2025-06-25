use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse, ModelConfig};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct InstanceConfig {
    pub instance_id: String,
    pub model_config: ModelConfig,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
    pub enable_caching: bool,
    pub cache_size: usize,
}

#[derive(Debug, Clone)]
pub struct InstanceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub current_load: f32,
    pub memory_usage_mb: f32,
    pub gpu_utilization: f32,
}

#[derive(Debug, Clone)]
pub enum InstanceStatus {
    Initializing,
    Ready,
    Busy,
    Error,
    ShuttingDown,
}

pub struct Instance {
    config: InstanceConfig,
    status: Arc<RwLock<InstanceStatus>>,
    metrics: Arc<RwLock<InstanceMetrics>>,
    active_requests: Arc<RwLock<usize>>,
    cache: Arc<RwLock<std::collections::HashMap<String, ModelResponse>>>,
    created_at: Instant,
}

impl Instance {
    pub async fn new(instance_id: String, model_config: ModelConfig) -> Result<Self, AppError> {
        let config = InstanceConfig {
            instance_id,
            model_config,
            max_concurrent_requests: 10,
            request_timeout_ms: 30000,
            enable_caching: true,
            cache_size: 1000,
        };
        
        let instance = Self {
            config,
            status: Arc::new(RwLock::new(InstanceStatus::Initializing)),
            metrics: Arc::new(RwLock::new(InstanceMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                current_load: 0.0,
                memory_usage_mb: 0.0,
                gpu_utilization: 0.0,
            })),
            active_requests: Arc::new(RwLock::new(0)),
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            created_at: Instant::now(),
        };
        
        // Initialize model
        instance.initialize_model().await?;
        
        Ok(instance)
    }

    async fn initialize_model(&self) -> Result<(), AppError> {
        // Stub for model initialization
        // In real implementation, model loading would happen here
        
        // Simulate loading
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Update status
        {
            let mut status = self.status.write().await;
            *status = InstanceStatus::Ready;
        }
        
        Ok(())
    }

    pub async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Check status
        {
            let status = self.status.read().await;
            match *status {
                InstanceStatus::Ready => {}
                InstanceStatus::Busy => {
                    if *self.active_requests.read().await >= self.config.max_concurrent_requests {
                        return Err(AppError::Resource("Instance overloaded".to_string()));
                    }
                }
                InstanceStatus::Error | InstanceStatus::ShuttingDown => {
                    return Err(AppError::Model("Instance unavailable".to_string()));
                }
                InstanceStatus::Initializing => {
                    return Err(AppError::Model("Instance not ready".to_string()));
                }
            }
        }
        
        // Increment active requests counter
        {
            let mut active_requests = self.active_requests.write().await;
            *active_requests += 1;
            
            if *active_requests >= self.config.max_concurrent_requests {
                let mut status = self.status.write().await;
                *status = InstanceStatus::Busy;
            }
        }
        
        // Check cache
        if self.config.enable_caching {
            if let Some(cached_response) = self.check_cache(&request).await {
                self.decrement_active_requests().await;
                return Ok(cached_response);
            }
        }
        
        // Process request
        let start_time = Instant::now();
        let response = self.process_model_request(request.clone()).await?;
        let processing_time = start_time.elapsed();
        
        // Cache result
        if self.config.enable_caching {
            self.cache_response(&request, &response).await;
        }
        
        // Update metrics
        self.update_metrics(processing_time, true).await;
        
        // Decrement active requests counter
        self.decrement_active_requests().await;
        
        Ok(response)
    }

    async fn process_model_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Stub for model request processing
        // In real implementation, model call would happen here
        
        // Simulate processing
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        // Create response
        let response = ModelResponse {
            output: format!("Response to: {}", request.input),
            metrics: crate::core::model_interface::ModelMetrics {
                processing_time_ms: 50,
                tokens_generated: request.input.len(),
                gpu_utilization: 75.5,
                memory_usage_mb: 2048.0,
                throughput_tokens_per_sec: 1000.0,
            },
            session_id: request.session_id,
        };
        
        Ok(response)
    }

    async fn check_cache(&self, request: &ModelRequest) -> Option<ModelResponse> {
        let cache_key = self.generate_cache_key(request);
        let cache = self.cache.read().await;
        cache.get(&cache_key).cloned()
    }

    async fn cache_response(&self, request: &ModelRequest, response: &ModelResponse) {
        let cache_key = self.generate_cache_key(request);
        let mut cache = self.cache.write().await;
        
        // Check cache size
        if cache.len() >= self.config.cache_size {
            // Remove oldest item
            if let Some((oldest_key, _)) = cache.iter().next() {
                let oldest_key = oldest_key.clone();
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(cache_key, response.clone());
    }

    fn generate_cache_key(&self, request: &ModelRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        request.input.hash(&mut hasher);
        request.parameters.temperature.to_bits().hash(&mut hasher);
        request.parameters.max_tokens.hash(&mut hasher);
        request.parameters.top_p.to_bits().hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }

    async fn update_metrics(&self, processing_time: std::time::Duration, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }
        
        let total_requests = metrics.total_requests as f64;
        let current_avg = metrics.average_response_time_ms;
        let new_avg = (current_avg * (total_requests - 1.0) + processing_time.as_millis() as f64) / total_requests;
        metrics.average_response_time_ms = new_avg;
        
        // Update current load
        let active_requests = *self.active_requests.read().await;
        metrics.current_load = active_requests as f32 / self.config.max_concurrent_requests as f32;
    }

    async fn decrement_active_requests(&self) {
        let mut active_requests = self.active_requests.write().await;
        *active_requests = active_requests.saturating_sub(1);
        
        if *active_requests < self.config.max_concurrent_requests {
            let mut status = self.status.write().await;
            if matches!(*status, InstanceStatus::Busy) {
                *status = InstanceStatus::Ready;
            }
        }
    }

    pub async fn health_check(&self) -> Result<bool, AppError> {
        let status = self.status.read().await;
        match *status {
            InstanceStatus::Ready | InstanceStatus::Busy => Ok(true),
            InstanceStatus::Error | InstanceStatus::ShuttingDown | InstanceStatus::Initializing => Ok(false),
        }
    }

    pub async fn is_healthy(&self) -> bool {
        self.health_check().await.unwrap_or(false)
    }

    pub async fn get_metrics(&self) -> InstanceMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn optimize_resources(&self) -> Result<(), AppError> {
        // Resource optimization logic
        log::info!("Optimizing resources for instance {}", self.config.instance_id);
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Update status
        {
            let mut status = self.status.write().await;
            *status = InstanceStatus::ShuttingDown;
        }
        
        // Wait for active requests to complete
        let mut attempts = 0;
        while *self.active_requests.read().await > 0 && attempts < 10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            attempts += 1;
        }
        
        // Clear cache
        self.cache.write().await.clear();
        
        log::info!("Instance {} shutdown complete", self.config.instance_id);
        Ok(())
    }

    pub fn get_instance_id(&self) -> &str {
        &self.config.instance_id
    }

    pub async fn get_status(&self) -> InstanceStatus {
        self.status.read().await.clone()
    }
} 