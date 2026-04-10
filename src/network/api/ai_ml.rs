//! API routes for Stage 4.4 AI/ML (v0.3.0+)
//!
//! Serves `/api/enterprise/ai-ml` when both `enterprise` and `ml` features are enabled.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, routing::post, Json, Router};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::ml::automl::AutomlConfig;
use crate::ml::federated::FederatedConfig;
use crate::ml::optimization::{
    apply_quantization, profile_model, suggest_hyperparams, ModelProfile, OptimizationProfile,
    QuantizationResult, TuningConfig, TuningResult,
};
use crate::ml::pipeline::{MLPipeline, PipelineStep, StepType};
use crate::ml::AiMlStatus;
use crate::network::api::common::HttpAppError;

/// Request body for `POST .../pipeline`.
#[derive(Debug, Deserialize)]
pub struct CreateMlPipelineRequest {
    pub name: String,
    pub steps: Vec<PipelineStep>,
}

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
/// - `GET /pipeline` — list pipelines (`MLPipelineManager`)
/// - `POST /pipeline` — create pipeline (JSON body)
/// - `GET /pipeline/{id}` — get pipeline by id
/// - `POST /pipeline/{id}/execute` — run pipeline
/// - `GET /pipeline/demo` — quick demo (Profiling); uses shared `AppState` manager
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
        .route(
            "/pipeline",
            get(list_ml_pipelines_handler).post(create_ml_pipeline_handler),
        )
        .route("/pipeline/{id}", get(get_ml_pipeline_handler))
        .route("/pipeline/{id}/execute", post(execute_ml_pipeline_handler))
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

async fn list_ml_pipelines_handler(State(ctx): State<ApiContext>) -> Json<Vec<MLPipeline>> {
    Json(ctx.ml_pipeline_manager.list_pipelines().await)
}

async fn create_ml_pipeline_handler(
    State(ctx): State<ApiContext>,
    Json(body): Json<CreateMlPipelineRequest>,
) -> Result<Json<MLPipeline>, HttpAppError> {
    let p = ctx
        .ml_pipeline_manager
        .create_pipeline(&body.name, body.steps)
        .await
        .map_err(|e| {
            HttpAppError::new(e)
                .with_context(ErrorContext::new("create_ml_pipeline"))
                .with_status(StatusCode::BAD_REQUEST)
        })?;
    Ok(Json(p))
}

async fn get_ml_pipeline_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<MLPipeline>, HttpAppError> {
    let p = ctx
        .ml_pipeline_manager
        .get_pipeline(&id)
        .await
        .map_err(|e| {
            HttpAppError::new(e)
                .with_context(
                    ErrorContext::new("get_ml_pipeline").with_resource("pipeline", id.clone()),
                )
                .with_status(StatusCode::NOT_FOUND)
        })?;
    Ok(Json(p))
}

async fn execute_ml_pipeline_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<StatusCode, HttpAppError> {
    ctx.ml_pipeline_manager
        .execute_pipeline(&id)
        .await
        .map_err(|e| {
            HttpAppError::new(e).with_context(
                ErrorContext::new("execute_ml_pipeline").with_resource("pipeline", id.clone()),
            )
        })?;
    Ok(StatusCode::NO_CONTENT)
}

/// Демо: один крок Profiling на спільному `AppState.ml_pipeline_manager`.
async fn ai_ml_pipeline_demo_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<MLPipeline>, HttpAppError> {
    let short = Uuid::new_v4().to_string();
    let short = &short[..8];
    let name = format!("api-demo-{short}");
    let steps = vec![PipelineStep {
        id: "profile".to_string(),
        step_type: StepType::Profiling,
        config: HashMap::new(),
        dependencies: vec![],
    }];
    let pipeline = ctx
        .ml_pipeline_manager
        .create_pipeline(&name, steps)
        .await
        .map_err(|e| {
            HttpAppError::new(e).with_context(
                ErrorContext::new("ai_ml_pipeline_demo").with_details("create_pipeline"),
            )
        })?;
    ctx.ml_pipeline_manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .map_err(|e| {
            HttpAppError::new(e).with_context(
                ErrorContext::new("ai_ml_pipeline_demo").with_details("execute_pipeline"),
            )
        })?;
    let got = ctx
        .ml_pipeline_manager
        .get_pipeline(pipeline.id.as_str())
        .await
        .map_err(|e| {
            HttpAppError::new(e)
                .with_context(ErrorContext::new("ai_ml_pipeline_demo").with_details("get_pipeline"))
        })?;
    Ok(Json(got))
}
