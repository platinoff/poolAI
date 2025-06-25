use crate::core::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub input: String,
    pub parameters: ModelParameters,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub output: String,
    pub metrics: ModelMetrics,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    pub temperature: f32,
    pub max_tokens: usize,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub max_tokens: usize,
    pub supported_parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub processing_time_ms: u64,
    pub tokens_generated: usize,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
    pub throughput_tokens_per_sec: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_path: String,
    pub gpu_device: Option<usize>,
    pub max_batch_size: usize,
    pub memory_limit_mb: usize,
    pub enable_caching: bool,
}

pub trait ModelInterface {
    async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError>;
    async fn get_model_info(&self) -> Result<ModelInfo, AppError>;
    async fn update_config(&self, config: ModelConfig) -> Result<(), AppError>;
    async fn get_metrics(&self) -> Result<ModelMetrics, AppError>;
    async fn initialize(&self) -> Result<(), AppError>;
    async fn shutdown(&self) -> Result<(), AppError>;
} 