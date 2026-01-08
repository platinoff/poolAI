//! Model interface module
//!
//! Provides abstractions for AI model integration, including request/response
//! handling, model lifecycle management, and metrics collection.
//!
//! # Features
//!
//! - **Model Interface Trait**: Unified interface for different model backends
//! - **Model Manager**: Centralized model registration and management
//! - **Request/Response**: Structured request and response types
//! - **Metrics**: Performance metrics and monitoring
//!
//! # Example
//!
//! ```no_run
//! use poolai::core::model_interface::{ModelInterface, ModelRequest, ModelParameters};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! // Create a model request
//! let request = ModelRequest {
//!     input: "Hello, world!".to_string(),
//!     parameters: ModelParameters::default(),
//!     session_id: None,
//!     priority: 5,
//!     timeout: Some(30),
//! };
//!
//! // Process request (assuming model implements ModelInterface)
//! // let response = model.process_request(request).await?;
//! // println!("Generated: {}", response.output);
//! # Ok(())
//! # }
//! ```

use crate::core::config::ModelConfig as ConfigModelConfig;
use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    /// Input (prompt)
    pub input: String,
    /// Generation parameters
    pub parameters: ModelParameters,
    /// Session ID for caching
    pub session_id: Option<String>,
    /// Request priority (1-10, 10 is the highest)
    pub priority: u8,
    /// Request timeout (seconds)
    pub timeout: Option<u64>,
}

/// Model response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    /// Generated text
    pub output: String,
    /// Processing metrics
    pub metrics: ModelMetrics,
    /// Session ID
    pub session_id: Option<String>,
    /// Processing status
    pub status: ResponseStatus,
    /// Errors (if any)
    pub errors: Vec<String>,
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Partial,
    Error,
    Timeout,
}

/// Model parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Sampling temperature (0.0-2.0)
    pub temperature: f32,
    /// Maximum tokens
    pub max_tokens: usize,
    /// Top-p sampling (0.0-1.0)
    pub top_p: f32,
    /// Frequency penalty (-2.0-2.0)
    pub frequency_penalty: f32,
    /// Presence penalty (-2.0-2.0)
    pub presence_penalty: f32,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    /// Seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 100,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: vec![],
            seed: None,
        }
    }
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model capabilities
    pub capabilities: Vec<String>,
    /// Maximum tokens
    pub max_tokens: usize,
    /// Supported parameters
    pub supported_parameters: Vec<String>,
    /// Model size (MB)
    pub model_size_mb: u64,
    /// Supported languages
    pub supported_languages: Vec<String>,
    /// GPU requirements
    pub gpu_requirements: GpuRequirements,
}

/// GPU requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRequirements {
    /// Minimum GPU memory (MB)
    pub min_memory_mb: u64,
    /// Recommended GPU memory (MB)
    pub recommended_memory_mb: u64,
    /// Supported GPU architectures
    pub supported_architectures: Vec<String>,
    /// Requires CUDA
    pub requires_cuda: bool,
}

/// Model metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Processing time (ms)
    pub processing_time_ms: u64,
    /// Number of generated tokens
    pub tokens_generated: usize,
    /// GPU utilization (%)
    pub gpu_utilization: f32,
    /// Memory usage (MB)
    pub memory_usage_mb: f32,
    /// Throughput (tokens/sec)
    pub throughput_tokens_per_sec: f32,
    /// CPU utilization (%)
    pub cpu_utilization: f32,
    /// GPU temperature (°C)
    pub gpu_temperature: f32,
    /// GPU power (Watts)
    pub gpu_power_watts: f32,
    /// Number of requests in queue
    pub queue_length: usize,
    /// Average latency (ms)
    pub average_latency_ms: f32,
}

/// Model state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelState {
    /// Model status
    pub status: ModelStatus,
    /// Active requests count
    pub active_requests: usize,
    /// Total processed requests
    pub total_requests: u64,
    /// Last activity time
    pub last_activity: chrono::DateTime<chrono::Utc>,
    /// Errors (if any)
    pub errors: Vec<String>,
    /// Performance metrics
    pub metrics: ModelMetrics,
}

/// Model status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Initializing,
    Ready,
    Busy,
    Error,
    Shutdown,
}

/// Model configuration (for the interface)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model path
    pub model_path: String,
    /// GPU device
    pub gpu_device: Option<usize>,
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Memory limit (MB)
    pub memory_limit_mb: usize,
    /// Enable caching
    pub enable_caching: bool,
    /// Cache size (MB)
    pub cache_size_mb: usize,
    /// Default parameters
    pub default_parameters: ModelParameters,
    /// Performance settings
    pub performance_settings: PerformanceSettings,
}

/// Performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Number of threads
    pub num_threads: usize,
    /// Use GPU
    pub use_gpu: bool,
    /// Memory optimization
    pub memory_optimization: bool,
    /// Parallel processing
    pub parallel_processing: bool,
}

/// Primary model interface according to the MVP concept
///
/// Defines the standard interface that all AI models must implement
/// to integrate with PoolAI. Provides methods for request processing,
/// lifecycle management, and monitoring.
///
/// # Example
///
/// ```no_run
/// use poolai::core::model_interface::{ModelInterface, ModelRequest, ModelResponse};
///
/// // Example implementation (simplified)
/// struct MyModel;
///
/// #[async_trait::async_trait]
/// impl ModelInterface for MyModel {
///     async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, poolai::core::error::AppError> {
///         // Implementation here
///         # todo!()
///     }
///     // ... other required methods
///     # async fn get_model_info(&self) -> Result<poolai::core::model_interface::ModelInfo, poolai::core::error::AppError> { todo!() }
///     # async fn update_config(&self, _config: poolai::core::model_interface::ModelConfig) -> Result<(), poolai::core::error::AppError> { todo!() }
///     # async fn get_metrics(&self) -> Result<poolai::core::model_interface::ModelMetrics, poolai::core::error::AppError> { todo!() }
///     # async fn get_state(&self) -> Result<poolai::core::model_interface::ModelState, poolai::core::error::AppError> { todo!() }
///     # async fn initialize(&self) -> Result<(), poolai::core::error::AppError> { todo!() }
///     # async fn shutdown(&self) -> Result<(), poolai::core::error::AppError> { todo!() }
///     # async fn health_check(&self) -> Result<(), poolai::core::error::AppError> { todo!() }
///     # async fn clear_cache(&self) -> Result<(), poolai::core::error::AppError> { todo!() }
///     # async fn get_statistics(&self) -> Result<std::collections::HashMap<String, f64>, poolai::core::error::AppError> { todo!() }
/// }
/// ```
#[async_trait::async_trait]
pub trait ModelInterface {
    /// Process a request with the model
    async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError>;

    /// Get model information
    async fn get_model_info(&self) -> Result<ModelInfo, AppError>;

    /// Update model configuration
    async fn update_config(&self, config: ModelConfig) -> Result<(), AppError>;

    /// Get model metrics
    async fn get_metrics(&self) -> Result<ModelMetrics, AppError>;

    /// Get model state
    async fn get_state(&self) -> Result<ModelState, AppError>;

    /// Initialize the model
    async fn initialize(&self) -> Result<(), AppError>;

    /// Shutdown the model
    async fn shutdown(&self) -> Result<(), AppError>;

    /// Health check for the model
    async fn health_check(&self) -> Result<(), AppError>;

    /// Clear cache
    async fn clear_cache(&self) -> Result<(), AppError>;

    /// Get statistics
    async fn get_statistics(&self) -> Result<HashMap<String, f64>, AppError>;
}

/// Model manager for MVP
///
/// Centralized manager for registering, managing, and routing requests
/// to different AI models. Supports multiple models simultaneously.
///
/// # Example
///
/// ```no_run
/// use poolai::core::model_interface::{ModelManager, ModelRequest};
/// use poolai::core::config::ModelConfig as ConfigModelConfig;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let mut manager = ModelManager::new(ConfigModelConfig::default());
///
/// // Register a model (assuming model implements ModelInterface)
/// // manager.register_model("my-model".to_string(), Box::new(my_model)).await?;
///
/// // Process a request
/// // let request = ModelRequest { ... };
/// // let response = manager.process_request("my-model", request).await?;
///
/// // Get metrics for all models
/// // let all_metrics = manager.get_all_metrics().await?;
/// # Ok(())
/// # }
/// ```
pub struct ModelManager {
    models: HashMap<String, Box<dyn ModelInterface + Send + Sync>>,
    #[allow(dead_code)] // Will be used for model configuration in future
    config: ConfigModelConfig,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new(config: ConfigModelConfig) -> Self {
        Self {
            models: HashMap::new(),
            config,
        }
    }

    /// Register a model
    pub async fn register_model(
        &mut self,
        name: String,
        model: Box<dyn ModelInterface + Send + Sync>,
    ) -> Result<(), AppError> {
        // Model initialization
        model.initialize().await?;

        // Health check
        model.health_check().await?;

        self.models.insert(name, model);
        Ok(())
    }

    /// Unregister a model
    pub async fn unregister_model(&mut self, name: &str) -> Result<(), AppError> {
        if let Some(model) = self.models.remove(name) {
            model.shutdown().await?;
        }
        Ok(())
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<&Box<dyn ModelInterface + Send + Sync>> {
        self.models.get(name)
    }

    /// Get all models
    pub fn get_all_models(&self) -> &HashMap<String, Box<dyn ModelInterface + Send + Sync>> {
        &self.models
    }

    /// Process a request via a specific model
    pub async fn process_request(
        &self,
        model_name: &str,
        request: ModelRequest,
    ) -> Result<ModelResponse, AppError> {
        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| AppError::ModelError(format!("Model {} not found", model_name)))?;

        model.process_request(request).await
    }

    /// Get metrics for all models
    pub async fn get_all_metrics(&self) -> Result<HashMap<String, ModelMetrics>, AppError> {
        let mut metrics = HashMap::new();

        for (name, model) in &self.models {
            let model_metrics = model.get_metrics().await?;
            metrics.insert(name.clone(), model_metrics);
        }

        Ok(metrics)
    }

    /// Get states for all models
    pub async fn get_all_states(&self) -> Result<HashMap<String, ModelState>, AppError> {
        let mut states = HashMap::new();

        for (name, model) in &self.models {
            let model_state = model.get_state().await?;
            states.insert(name.clone(), model_state);
        }

        Ok(states)
    }
}
