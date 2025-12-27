use crate::core::error::AppError;
use crate::core::model_interface::{ModelMetrics, ModelRequest, ModelResponse};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
// use tracing::info; // Unused import

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub worker_id: String,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub enable_caching: bool,
    pub cache_size: usize,
    // Stage 4.1: Runtime capabilities
    pub max_memory_mb: usize,
    pub cpu_priority: u8,
    pub gpu_device: Option<usize>,
    pub auto_restart: bool,
    pub resource_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub is_healthy: bool,
    pub active_connections: usize,
    pub queue_size: usize,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    pub total_requests_processed: u64,
    pub average_response_time_ms: f64,
    // Stage 4.1: Enhanced metrics
    pub cpu_usage: f32,
    pub memory_usage_mb: f32,
    pub gpu_usage: Option<f32>,
    pub process_id: Option<u32>,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct Worker {
    config: WorkerConfig,
    status: Arc<RwLock<WorkerStatus>>,
    request_queue: Arc<RwLock<VecDeque<ModelRequest>>>,
    cache: Arc<RwLock<std::collections::HashMap<String, ModelResponse>>>,
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(WorkerStatus {
                is_healthy: true,
                active_connections: 0,
                queue_size: 0,
                last_health_check: chrono::Utc::now(),
                total_requests_processed: 0,
                average_response_time_ms: 0.0,
                // Stage 4.1: Initialize enhanced metrics
                cpu_usage: 0.0,
                memory_usage_mb: 0.0,
                gpu_usage: None,
                process_id: None,
                uptime_seconds: 0,
            })),
            request_queue: Arc::new(RwLock::new(VecDeque::new())),
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Check cache
        if self.config.enable_caching {
            if let Some(cached_response) = self.check_cache(&request).await {
                return Ok(cached_response);
            }
        }

        // Update status
        {
            let mut status = self.status.write().await;
            status.active_connections += 1;
            status.queue_size += 1;
        }

        // Process request (simulated)
        let start_time = std::time::Instant::now();
        let response = self.simulate_request_processing(&request).await?;
        let processing_time = start_time.elapsed();

        // Cache response
        if self.config.enable_caching {
            self.cache_response(&request, &response).await;
        }

        // Update metrics
        self.update_metrics(processing_time).await;

        // Update status
        {
            let mut status = self.status.write().await;
            status.active_connections -= 1;
            status.queue_size -= 1;
            status.total_requests_processed += 1;
        }

        Ok(response)
    }

    async fn simulate_request_processing(
        &self,
        request: &ModelRequest,
    ) -> Result<ModelResponse, AppError> {
        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(ModelResponse {
            output: format!("Processed: {}", request.input),
            metrics: ModelMetrics {
                processing_time_ms: 100,
                tokens_generated: request.input.len(),
                gpu_utilization: 0.5,
                memory_usage_mb: 512.0,
                throughput_tokens_per_sec: 1000.0,
                cpu_utilization: 0.3,
                gpu_temperature: 65.0,
                gpu_power_watts: 150.0,
                queue_length: 0,
                average_latency_ms: 100.0,
            },
            session_id: request.session_id.clone(),
            status: crate::core::model_interface::ResponseStatus::Success,
            errors: vec![],
        })
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

    async fn update_metrics(&self, processing_time: std::time::Duration) {
        let mut status = self.status.write().await;
        let total_requests = status.total_requests_processed as f64;
        let current_avg = status.average_response_time_ms;
        let new_avg = (current_avg * (total_requests - 1.0) + processing_time.as_millis() as f64)
            / total_requests;
        status.average_response_time_ms = new_avg;
    }

    pub async fn health_check(&self) -> Result<bool, AppError> {
        let start_time = chrono::Utc::now();

        // Simple health check
        let mut status = self.status.write().await;
        status.is_healthy = true;
        status.last_health_check = start_time;
        Ok(true)
    }

    pub fn get_active_connections(&self) -> usize {
        // Blocking call for simplicity
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.status.read().await.active_connections })
        })
    }

    pub fn get_health_score(&self) -> f64 {
        // Blocking call for simplicity
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let status = self.status.read().await;
                if !status.is_healthy {
                    return 0.0;
                }

                // Simple health score based on metrics
                let connection_score = 1.0
                    - (status.active_connections as f64
                        / self.config.max_concurrent_requests as f64);
                let response_time_score = if status.average_response_time_ms < 1000.0 {
                    1.0
                } else {
                    0.5
                };

                connection_score * response_time_score
            })
        })
    }

    pub async fn get_status(&self) -> WorkerStatus {
        self.status.read().await.clone()
    }

    pub async fn optimize_resources(&self) -> Result<(), AppError> {
        // Resource optimization logic
        tracing::info!("Optimizing resources for worker {}", self.config.worker_id);
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Cleanup resources
        self.request_queue.write().await.clear();
        self.cache.write().await.clear();

        tracing::info!("Worker {} shutdown complete", self.config.worker_id);
        Ok(())
    }
}
