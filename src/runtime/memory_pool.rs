//! Memory Pool Optimization for Frequently Allocated Structures
//!
//! Provides object pooling for frequently allocated structures to reduce
//! memory allocation overhead and improve performance.
//!
//! # Features
//!
//! - **Object Pooling**: Reuse `ModelRequest` and `ModelResponse` instances
//! - **String Pooling**: Reuse frequently allocated strings (cache keys, session IDs)
//! - **Vec Pooling**: Reuse `Vec<String>` for errors and stop sequences
//!
//! # Example
//!
//! ```no_run
//! use poolai::runtime::memory_pool::{MemoryPool, ModelRequestPool, ModelResponsePool};
//! use poolai::core::model_interface::{ModelRequest, ModelResponse};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let pool = MemoryPool::new();
//! pool.initialize().await?;
//!
//! // Acquire a request from the pool
//! let mut request = pool.acquire_request().await;
//! request.input = "Hello".to_string();
//!
//! // Release it back to the pool when done
//! pool.release_request(request).await;
//!
//! pool.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::model_interface::{
    ModelMetrics, ModelParameters, ModelRequest, ModelResponse, ResponseStatus,
};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// Maximum pool size for each object type
const MAX_POOL_SIZE: usize = 100;

/// Memory pool manager
pub struct MemoryPool {
    request_pool: Arc<Mutex<VecDeque<ModelRequest>>>,
    response_pool: Arc<Mutex<VecDeque<ModelResponse>>>,
    string_pool: Arc<Mutex<VecDeque<String>>>,
    vec_string_pool: Arc<Mutex<VecDeque<Vec<String>>>>,
    initialized: Arc<Mutex<bool>>,
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new() -> Self {
        Self {
            request_pool: Arc::new(Mutex::new(VecDeque::new())),
            response_pool: Arc::new(Mutex::new(VecDeque::new())),
            string_pool: Arc::new(Mutex::new(VecDeque::new())),
            vec_string_pool: Arc::new(Mutex::new(VecDeque::new())),
            initialized: Arc::new(Mutex::new(false)),
        }
    }

    /// Initialize the memory pool with pre-allocated objects
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut initialized = self.initialized.lock().await;
        if *initialized {
            return Ok(());
        }

        info!("Initializing memory pool with pre-allocated objects");

        // Pre-allocate some requests
        let mut request_pool = self.request_pool.lock().await;
        for _ in 0..10 {
            request_pool.push_back(Self::create_default_request());
        }
        drop(request_pool);

        // Pre-allocate some responses
        let mut response_pool = self.response_pool.lock().await;
        for _ in 0..10 {
            response_pool.push_back(Self::create_default_response());
        }
        drop(response_pool);

        // Pre-allocate some strings (cache keys, session IDs)
        let mut string_pool = self.string_pool.lock().await;
        for _ in 0..20 {
            string_pool.push_back(String::with_capacity(64));
        }
        drop(string_pool);

        // Pre-allocate some Vec<String> (for errors, stop sequences)
        let mut vec_string_pool = self.vec_string_pool.lock().await;
        for _ in 0..10 {
            vec_string_pool.push_back(Vec::with_capacity(4));
        }
        drop(vec_string_pool);

        *initialized = true;
        debug!("Memory pool initialized with pre-allocated objects");
        Ok(())
    }

    /// Acquire a ModelRequest from the pool, or create a new one if pool is empty
    pub async fn acquire_request(&self) -> ModelRequest {
        let mut pool = self.request_pool.lock().await;
        pool.pop_front()
            .unwrap_or_else(Self::create_default_request)
    }

    /// Release a ModelRequest back to the pool (resets its fields)
    pub async fn release_request(&self, mut request: ModelRequest) {
        let mut pool = self.request_pool.lock().await;
        if pool.len() < MAX_POOL_SIZE {
            // Reset the request for reuse
            request.input.clear();
            request.parameters = ModelParameters::default();
            request.session_id = None;
            request.priority = 5;
            request.timeout = None;
            pool.push_back(request);
        }
    }

    /// Acquire a ModelResponse from the pool, or create a new one if pool is empty
    pub async fn acquire_response(&self) -> ModelResponse {
        let mut pool = self.response_pool.lock().await;
        pool.pop_front()
            .unwrap_or_else(Self::create_default_response)
    }

    /// Release a ModelResponse back to the pool (resets its fields)
    pub async fn release_response(&self, mut response: ModelResponse) {
        let mut pool = self.response_pool.lock().await;
        if pool.len() < MAX_POOL_SIZE {
            // Reset the response for reuse
            response.output.clear();
            response.metrics = ModelMetrics {
                processing_time_ms: 0,
                tokens_generated: 0,
                gpu_utilization: 0.0,
                memory_usage_mb: 0.0,
                throughput_tokens_per_sec: 0.0,
                cpu_utilization: 0.0,
                gpu_temperature: 0.0,
                gpu_power_watts: 0.0,
                queue_length: 0,
                average_latency_ms: 0.0,
            };
            response.session_id = None;
            response.status = ResponseStatus::Success;
            response.errors.clear();
            pool.push_back(response);
        }
    }

    /// Acquire a String from the pool, or create a new one if pool is empty
    pub async fn acquire_string(&self) -> String {
        let mut pool = self.string_pool.lock().await;
        pool.pop_front()
            .unwrap_or_else(|| String::with_capacity(64))
    }

    /// Release a String back to the pool (clears it for reuse)
    pub async fn release_string(&self, mut s: String) {
        let mut pool = self.string_pool.lock().await;
        if pool.len() < MAX_POOL_SIZE {
            s.clear();
            pool.push_back(s);
        }
    }

    /// Acquire a Vec<String> from the pool, or create a new one if pool is empty
    pub async fn acquire_vec_string(&self) -> Vec<String> {
        let mut pool = self.vec_string_pool.lock().await;
        pool.pop_front().unwrap_or_else(|| Vec::with_capacity(4))
    }

    /// Release a Vec<String> back to the pool (clears it for reuse)
    pub async fn release_vec_string(&self, mut v: Vec<String>) {
        let mut pool = self.vec_string_pool.lock().await;
        if pool.len() < MAX_POOL_SIZE {
            v.clear();
            pool.push_back(v);
        }
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats {
        let request_pool = self.request_pool.lock().await;
        let response_pool = self.response_pool.lock().await;
        let string_pool = self.string_pool.lock().await;
        let vec_string_pool = self.vec_string_pool.lock().await;

        PoolStats {
            request_pool_size: request_pool.len(),
            response_pool_size: response_pool.len(),
            string_pool_size: string_pool.len(),
            vec_string_pool_size: vec_string_pool.len(),
            max_pool_size: MAX_POOL_SIZE,
        }
    }

    /// Shutdown the memory pool and clear all objects
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut initialized = self.initialized.lock().await;
        if !*initialized {
            return Ok(());
        }

        info!("Shutting down memory pool");

        let mut request_pool = self.request_pool.lock().await;
        request_pool.clear();
        drop(request_pool);

        let mut response_pool = self.response_pool.lock().await;
        response_pool.clear();
        drop(response_pool);

        let mut string_pool = self.string_pool.lock().await;
        string_pool.clear();
        drop(string_pool);

        let mut vec_string_pool = self.vec_string_pool.lock().await;
        vec_string_pool.clear();
        drop(vec_string_pool);

        *initialized = false;
        debug!("Memory pool shut down");
        Ok(())
    }

    /// Create a default ModelRequest
    fn create_default_request() -> ModelRequest {
        ModelRequest {
            input: String::with_capacity(256),
            parameters: ModelParameters::default(),
            session_id: None,
            priority: 5,
            timeout: None,
        }
    }

    /// Create a default ModelResponse
    fn create_default_response() -> ModelResponse {
        ModelResponse {
            output: String::with_capacity(512),
            metrics: ModelMetrics {
                processing_time_ms: 0,
                tokens_generated: 0,
                gpu_utilization: 0.0,
                memory_usage_mb: 0.0,
                throughput_tokens_per_sec: 0.0,
                cpu_utilization: 0.0,
                gpu_temperature: 0.0,
                gpu_power_watts: 0.0,
                queue_length: 0,
                average_latency_ms: 0.0,
            },
            session_id: None,
            status: ResponseStatus::Success,
            errors: Vec::with_capacity(4),
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub request_pool_size: usize,
    pub response_pool_size: usize,
    pub string_pool_size: usize,
    pub vec_string_pool_size: usize,
    pub max_pool_size: usize,
}

/// Convenience type aliases
pub type ModelRequestPool = MemoryPool;
pub type ModelResponsePool = MemoryPool;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_pool_initialization() {
        let pool = MemoryPool::new();
        assert!(pool.initialize().await.is_ok());

        let stats = pool.get_stats().await;
        assert!(stats.request_pool_size > 0);
        assert!(stats.response_pool_size > 0);

        assert!(pool.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_request_pool_acquire_release() {
        let pool = MemoryPool::new();
        pool.initialize().await.unwrap();

        let request = pool.acquire_request().await;
        assert_eq!(request.priority, 5);

        pool.release_request(request).await;

        let stats = pool.get_stats().await;
        assert!(stats.request_pool_size >= 1);

        pool.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_response_pool_acquire_release() {
        let pool = MemoryPool::new();
        pool.initialize().await.unwrap();

        let response = pool.acquire_response().await;
        assert_eq!(response.output.capacity(), 512);

        pool.release_response(response).await;

        let stats = pool.get_stats().await;
        assert!(stats.response_pool_size >= 1);

        pool.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_string_pool() {
        let pool = MemoryPool::new();
        pool.initialize().await.unwrap();

        let s = pool.acquire_string().await;
        assert_eq!(s.capacity(), 64);

        pool.release_string(s).await;

        let stats = pool.get_stats().await;
        assert!(stats.string_pool_size >= 1);

        pool.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_vec_string_pool() {
        let pool = MemoryPool::new();
        pool.initialize().await.unwrap();

        let v = pool.acquire_vec_string().await;
        assert_eq!(v.capacity(), 4);

        pool.release_vec_string(v).await;

        let stats = pool.get_stats().await;
        assert!(stats.vec_string_pool_size >= 1);

        pool.shutdown().await.unwrap();
    }
}
