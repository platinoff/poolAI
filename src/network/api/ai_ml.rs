//! API routes for Stage 4.4 AI/ML (v0.3.0+)
//!
//! Serves `/api/enterprise/ai-ml` when both `enterprise` and `ml` features are enabled.

use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use std::collections::HashMap;

use crate::core::state::ApiContext;
use crate::ml::automl::AutomlConfig;
use crate::ml::federated::FederatedConfig;
use crate::ml::optimization::{
    apply_quantization, profile_model, suggest_hyperparams, ModelProfile, OptimizationProfile,
    QuantizationResult, TuningConfig, TuningResult,
};
use crate::ml::pipeline::{MLPipeline, MLPipelineManager, PipelineStep, StepType};
use crate::ml::AiMlStatus;

/// Create AI/ML API routes.
///
/// - `GET /` — status
/// - `GET /status` — status
/// - `GET /optimization` — ML.1 profile
/// - `GET /optimization/profile` — ML.1 profiling stub
/// - `GET /optimization/tuning` — ML.1 tuning stub
/// - `GET /optimization/quantization-result` — ML.1 quantization stub
/// - `GET /automl` — ML.2
/// - `GET /federated` — ML.3
/// - `GET /pipeline/demo` — ML.6 demo (single Profiling step; ephemeral manager per request)
pub fn create_ai_ml_routes() -> Router<ApiContext> {
    Router::new()
        .route("/", get(ai_ml_status_handler))
        .route("/status", get(ai_ml_status_handler))
        .route("/optimization", get(ai_ml_optimization_handler))
        .route(
            "/optimization/profile",
            get(ai_ml_optimization_profile_handler),
        )
        .route(
            "/optimization/tuning",
            get(ai_ml_optimization_tuning_handler),
        )
        .route(
            "/optimization/quantization-result",
            get(ai_ml_optimization_quantization_handler),
        )
        .route("/automl", get(ai_ml_automl_handler))
        .route("/federated", get(ai_ml_federated_handler))
        .route("/pipeline/demo", get(ai_ml_pipeline_demo_handler))
}

async fn ai_ml_status_handler() -> Json<AiMlStatus> {
    Json(AiMlStatus::default())
}

async fn ai_ml_optimization_handler() -> Json<OptimizationProfile> {
    Json(OptimizationProfile::default_balanced())
}

async fn ai_ml_optimization_profile_handler() -> Json<ModelProfile> {
    Json(profile_model())
}

async fn ai_ml_optimization_tuning_handler() -> Json<TuningResult> {
    let cfg = TuningConfig::default_config();
    Json(suggest_hyperparams(&cfg))
}

async fn ai_ml_optimization_quantization_handler() -> Json<QuantizationResult> {
    let profile = OptimizationProfile::default_balanced();
    Json(apply_quantization(&profile))
}

async fn ai_ml_automl_handler() -> Json<AutomlConfig> {
    Json(AutomlConfig::default_config())
}

async fn ai_ml_federated_handler() -> Json<FederatedConfig> {
    Json(FederatedConfig::default_config())
}

/// Повертає результат виконання демо-pipeline (один крок Profiling). Окремий менеджер на запит.
async fn ai_ml_pipeline_demo_handler() -> Result<Json<MLPipeline>, StatusCode> {
    let manager = MLPipelineManager::new();
    let steps = vec![PipelineStep {
        id: "profile".to_string(),
        step_type: StepType::Profiling,
        config: HashMap::new(),
        dependencies: vec![],
    }];
    let pipeline = manager
        .create_pipeline("api-demo", steps)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let got = manager
        .get_pipeline(pipeline.id.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(got))
}
