//! Integration tests for core::model_interface module
//!
//! Tests model interface traits, model manager, and model lifecycle operations.

use poolai::core::model_interface::{
    ModelConfig, ModelInfo, ModelInterface, ModelManager, ModelMetrics, ModelParameters,
    ModelRequest, ModelResponse, ModelState, ModelStatus,
};
use poolai::core::config::PoolAIConfig;
use poolai::core::error::AppError;
use std::collections::HashMap;

// Mock implementation of ModelInterface for testing
struct MockModel {
    name: String,
    initialized: bool,
}

#[async_trait::async_trait]
impl ModelInterface for MockModel {
    async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        Ok(ModelResponse {
            output: format!("Mock response for: {}", request.input),
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
            session_id: request.session_id,
            status: poolai::core::model_interface::ResponseStatus::Success,
            errors: vec![],
        })
    }

    async fn get_model_info(&self) -> Result<ModelInfo, AppError> {
        Ok(ModelInfo {
            name: self.name.clone(),
            version: "1.0.0".to_string(),
            capabilities: vec!["text-generation".to_string()],
            max_tokens: 2048,
            supported_parameters: vec!["temperature".to_string(), "max_tokens".to_string()],
            model_size_mb: 1000,
            supported_languages: vec!["en".to_string()],
            gpu_requirements: poolai::core::model_interface::GpuRequirements {
                min_memory_mb: 4096,
                recommended_memory_mb: 8192,
                supported_architectures: vec!["CUDA".to_string()],
                requires_cuda: true,
            },
        })
    }

    async fn update_config(&self, _config: ModelConfig) -> Result<(), AppError> {
        Ok(())
    }

    async fn get_metrics(&self) -> Result<ModelMetrics, AppError> {
        Ok(ModelMetrics {
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
        })
    }

    async fn get_state(&self) -> Result<ModelState, AppError> {
        Ok(ModelState {
            status: if self.initialized {
                ModelStatus::Ready
            } else {
                ModelStatus::Initializing
            },
            active_requests: 0,
            total_requests: 0,
            last_activity: chrono::Utc::now(),
            errors: vec![],
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
        })
    }

    async fn initialize(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<(), AppError> {
        if self.initialized {
            Ok(())
        } else {
            Err(AppError::ModelError("Model not initialized".to_string()))
        }
    }

    async fn clear_cache(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn get_statistics(&self) -> Result<HashMap<String, f64>, AppError> {
        Ok(HashMap::new())
    }
}

#[tokio::test]
async fn test_model_manager_creation() {
    let config = PoolAIConfig::default();
    let manager = ModelManager::new(config.model);
    // Manager should be created successfully
    assert_eq!(manager.get_all_models().len(), 0);
}

#[tokio::test]
async fn test_register_model() {
    let config = PoolAIConfig::default();
    let mut manager = ModelManager::new(config.model);

    let model = Box::new(MockModel {
        name: "test-model".to_string(),
        initialized: true,
    });

    manager
        .register_model("test-model".to_string(), model)
        .await
        .expect("Should register model");

    assert_eq!(manager.get_all_models().len(), 1);
    assert!(manager.get_model("test-model").is_some());
}

#[tokio::test]
async fn test_unregister_model() {
    let config = PoolAIConfig::default();
    let mut manager = ModelManager::new(config.model);

    let model = Box::new(MockModel {
        name: "test-model".to_string(),
        initialized: true,
    });

    manager
        .register_model("test-model".to_string(), model)
        .await
        .expect("Should register model");

    manager
        .unregister_model("test-model")
        .await
        .expect("Should unregister model");

    assert_eq!(manager.get_all_models().len(), 0);
    assert!(manager.get_model("test-model").is_none());
}

#[tokio::test]
async fn test_process_request() {
    let config = PoolAIConfig::default();
    let mut manager = ModelManager::new(config.model);

    let model = Box::new(MockModel {
        name: "test-model".to_string(),
        initialized: true,
    });

    manager
        .register_model("test-model".to_string(), model)
        .await
        .expect("Should register model");

    let request = ModelRequest {
        input: "Hello, world!".to_string(),
        parameters: ModelParameters::default(),
        session_id: None,
        priority: 5,
        timeout: Some(30),
    };

    let response = manager
        .process_request("test-model", request)
        .await
        .expect("Should process request");

    assert!(response.output.contains("Hello, world!"));
    assert!(matches!(
        response.status,
        poolai::core::model_interface::ResponseStatus::Success
    ));
}

#[tokio::test]
async fn test_process_request_nonexistent_model() {
    let config = PoolAIConfig::default();
    let manager = ModelManager::new(config.model);

    let request = ModelRequest {
        input: "Hello, world!".to_string(),
        parameters: ModelParameters::default(),
        session_id: None,
        priority: 5,
        timeout: Some(30),
    };

    let result = manager.process_request("nonexistent", request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_all_metrics() {
    let config = PoolAIConfig::default();
    let mut manager = ModelManager::new(config.model);

    for i in 1..=3 {
        let model = Box::new(MockModel {
            name: format!("model-{}", i),
            initialized: true,
        });
        manager
            .register_model(format!("model-{}", i), model)
            .await
            .expect("Should register model");
    }

    let all_metrics = manager
        .get_all_metrics()
        .await
        .expect("Should get all metrics");

    assert_eq!(all_metrics.len(), 3);
    assert!(all_metrics.contains_key("model-1"));
    assert!(all_metrics.contains_key("model-2"));
    assert!(all_metrics.contains_key("model-3"));
}

#[tokio::test]
async fn test_get_all_states() {
    let config = PoolAIConfig::default();
    let mut manager = ModelManager::new(config.model);

    for i in 1..=3 {
        let model = Box::new(MockModel {
            name: format!("model-{}", i),
            initialized: true,
        });
        manager
            .register_model(format!("model-{}", i), model)
            .await
            .expect("Should register model");
    }

    let all_states = manager
        .get_all_states()
        .await
        .expect("Should get all states");

    assert_eq!(all_states.len(), 3);
    for (name, state) in all_states {
        assert!(name.starts_with("model-"));
        assert!(matches!(state.status, ModelStatus::Ready));
    }
}

#[tokio::test]
async fn test_model_parameters_default() {
    let params = ModelParameters::default();
    assert_eq!(params.temperature, 0.7);
    assert_eq!(params.max_tokens, 100);
    assert_eq!(params.top_p, 1.0);
    assert_eq!(params.frequency_penalty, 0.0);
    assert_eq!(params.presence_penalty, 0.0);
    assert!(params.stop_sequences.is_empty());
    assert!(params.seed.is_none());
}

#[tokio::test]
async fn test_model_metrics_structure() {
    let metrics = ModelMetrics {
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
    assert_eq!(metrics.processing_time_ms, 0);
    assert_eq!(metrics.tokens_generated, 0);
    assert_eq!(metrics.gpu_utilization, 0.0);
    assert_eq!(metrics.memory_usage_mb, 0.0);
    assert_eq!(metrics.throughput_tokens_per_sec, 0.0);
    assert_eq!(metrics.cpu_utilization, 0.0);
    assert_eq!(metrics.gpu_temperature, 0.0);
    assert_eq!(metrics.gpu_power_watts, 0.0);
    assert_eq!(metrics.queue_length, 0);
    assert_eq!(metrics.average_latency_ms, 0.0);
}
