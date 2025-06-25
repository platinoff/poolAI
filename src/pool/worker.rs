use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse, ModelMetrics, ModelInterface};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub worker_id: String,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub enable_caching: bool,
    pub cache_size: usize,
}

#[derive(Debug, Clone)]
pub struct WorkerStatus {
    pub is_healthy: bool,
    pub active_connections: usize,
    pub queue_size: usize,
    pub last_health_check: std::time::Instant,
    pub total_requests_processed: u64,
    pub average_response_time_ms: f64,
}

pub struct Worker {
    config: WorkerConfig,
    model_interface: Arc<dyn ModelInterface + Send + Sync>,
    status: Arc<RwLock<WorkerStatus>>,
    request_queue: Arc<RwLock<VecDeque<ModelRequest>>>,
    cache: Arc<RwLock<std::collections::HashMap<String, ModelResponse>>>,
}

impl Worker {
    pub fn new(
        config: WorkerConfig,
        model_interface: Arc<dyn ModelInterface + Send + Sync>,
    ) -> Self {
        Self {
            config,
            model_interface,
            status: Arc::new(RwLock::new(WorkerStatus {
                is_healthy: true,
                active_connections: 0,
                queue_size: 0,
                last_health_check: std::time::Instant::now(),
                total_requests_processed: 0,
                average_response_time_ms: 0.0,
            })),
            request_queue: Arc::new(RwLock::new(VecDeque::new())),
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Проверка кэша
        if self.config.enable_caching {
            if let Some(cached_response) = self.check_cache(&request).await {
                return Ok(cached_response);
            }
        }

        // Обновление статуса
        {
            let mut status = self.status.write().await;
            status.active_connections += 1;
            status.queue_size += 1;
        }

        // Обработка запроса
        let start_time = std::time::Instant::now();
        let response = self.model_interface.process_request(request.clone()).await?;
        let processing_time = start_time.elapsed();

        // Кэширование результата
        if self.config.enable_caching {
            self.cache_response(&request, &response).await;
        }

        // Обновление метрик
        self.update_metrics(processing_time).await;

        // Обновление статуса
        {
            let mut status = self.status.write().await;
            status.active_connections -= 1;
            status.queue_size -= 1;
            status.total_requests_processed += 1;
        }

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
        
        // Проверка размера кэша
        if cache.len() >= self.config.cache_size {
            // Удаление самого старого элемента
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
        let new_avg = (current_avg * (total_requests - 1.0) + processing_time.as_millis() as f64) / total_requests;
        status.average_response_time_ms = new_avg;
    }

    pub async fn health_check(&self) -> Result<bool, AppError> {
        let start_time = std::time::Instant::now();
        
        // Простая проверка здоровья - получение информации о модели
        match self.model_interface.get_model_info().await {
            Ok(_) => {
                let mut status = self.status.write().await;
                status.is_healthy = true;
                status.last_health_check = start_time;
                Ok(true)
            }
            Err(_) => {
                let mut status = self.status.write().await;
                status.is_healthy = false;
                Ok(false)
            }
        }
    }

    pub fn get_active_connections(&self) -> usize {
        // Блокирующий вызов для простоты
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.status.read().await.active_connections
            })
        })
    }

    pub fn get_health_score(&self) -> f64 {
        // Блокирующий вызов для простоты
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let status = self.status.read().await;
                if !status.is_healthy {
                    return 0.0;
                }
                
                // Простая оценка здоровья на основе метрик
                let connection_score = 1.0 - (status.active_connections as f64 / self.config.max_concurrent_requests as f64);
                let response_time_score = if status.average_response_time_ms < 1000.0 { 1.0 } else { 0.5 };
                
                connection_score * response_time_score
            })
        })
    }

    pub async fn get_status(&self) -> WorkerStatus {
        self.status.read().await.clone()
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Очистка ресурсов
        self.request_queue.write().await.clear();
        self.cache.write().await.clear();
        
        // Выключение модели
        self.model_interface.shutdown().await?;
        
        Ok(())
    }
} 