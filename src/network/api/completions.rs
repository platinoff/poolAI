//! OpenAI-compatible chat completions API.
//!
//! Handlers stay thin; orchestration lives in [`crate::services::chat_completion_service`].

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Sse},
    routing::post,
    Json, Router,
};

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::network::auth::Claims;
use crate::services::chat_completion_service::ChatCompletionService;

pub use crate::services::chat_completion_service::{
    ChatCompletionChoice, ChatCompletionChunk, ChatCompletionChunkChoice, ChatCompletionRequest,
    ChatCompletionResponse, ChatMessage, ChatMessageDelta, ChatUsage,
};

/// Create completions routes
pub fn create_completions_routes() -> Router<ApiContext> {
    Router::new().route("/v1/chat/completions", post(chat_completions_handler))
}

async fn chat_completions_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    let model_request = ChatCompletionService::build_model_request(&request);

    if let Some((manager_arc, instance_id)) =
        ChatCompletionService::resolve_instance_for_model(&ctx, &request.model).await
    {
        if request.stream {
            let stream = ChatCompletionService::stream_from_instance(
                &instance_id,
                &request.model,
                model_request.clone(),
                Some(manager_arc),
            );
            Sse::new(stream).into_response()
        } else {
            match ChatCompletionService::complete_via_instance(
                &instance_id,
                &request.model,
                model_request,
                manager_arc,
            )
            .await
            {
                Ok(completion) => (StatusCode::OK, Json(completion)).into_response(),
                Err(e) => {
                    let (s, j) = api_json_error(
                        "internal_error",
                        e.to_string(),
                        Some(ErrorContext::new("chat_completion")),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    );
                    (s, j).into_response()
                }
            }
        }
    } else if request.stream {
        let stream = ChatCompletionService::stream_fallback(&request.model, model_request);
        Sse::new(stream).into_response()
    } else {
        match ChatCompletionService::complete_fallback(&request.model, model_request).await {
            Ok(completion) => (StatusCode::OK, Json(completion)).into_response(),
            Err(e) => {
                let (s, j) = api_json_error(
                    "internal_error",
                    e.to_string(),
                    Some(ErrorContext::new("chat_completion")),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                (s, j).into_response()
            }
        }
    }
}
