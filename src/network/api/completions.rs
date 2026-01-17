//! OpenAI-compatible chat completions API
//!
//! Provides `/v1/chat/completions` endpoint compatible with OpenAI API format.
//! Supports both streaming and non-streaming responses.

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{sse::Event, IntoResponse, Sse},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::{self as stream, Stream, StreamExt};

use crate::core::model_interface::{ModelParameters, ModelRequest};
use crate::network::auth::Claims;

/// OpenAI-compatible chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role (system, user, assistant)
    pub role: String,
    /// Message content
    pub content: String,
}

/// OpenAI-compatible chat completion request
#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    /// Model name/ID
    pub model: String,
    /// Chat messages
    pub messages: Vec<ChatMessage>,
    /// Temperature (0.0-2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Max tokens
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    /// Stream response
    #[serde(default)]
    pub stream: bool,
    /// Top-p sampling
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    /// Frequency penalty
    #[serde(default)]
    pub frequency_penalty: f32,
    /// Presence penalty
    #[serde(default)]
    pub presence_penalty: f32,
    /// Stop sequences
    #[serde(default)]
    pub stop: Option<Vec<String>>,
}

fn default_temperature() -> f32 {
    1.0
}

fn default_max_tokens() -> usize {
    2048
}

fn default_top_p() -> f32 {
    1.0
}

/// OpenAI-compatible chat completion response
#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    /// Response ID
    pub id: String,
    /// Object type
    pub object: String,
    /// Created timestamp
    pub created: i64,
    /// Model used
    pub model: String,
    /// Choices
    pub choices: Vec<ChatCompletionChoice>,
    /// Usage statistics
    pub usage: ChatUsage,
}

/// Chat completion choice
#[derive(Debug, Serialize)]
pub struct ChatCompletionChoice {
    /// Index
    pub index: usize,
    /// Message
    pub message: ChatMessage,
    /// Finish reason
    pub finish_reason: String,
}

/// Usage statistics
#[derive(Debug, Serialize)]
pub struct ChatUsage {
    /// Prompt tokens
    pub prompt_tokens: usize,
    /// Completion tokens
    pub completion_tokens: usize,
    /// Total tokens
    pub total_tokens: usize,
}

/// Streaming chat completion chunk
#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    /// Response ID
    pub id: String,
    /// Object type
    pub object: String,
    /// Created timestamp
    pub created: i64,
    /// Model used
    pub model: String,
    /// Choices
    pub choices: Vec<ChatCompletionChunkChoice>,
}

/// Chat completion chunk choice
#[derive(Debug, Serialize)]
pub struct ChatCompletionChunkChoice {
    /// Index
    pub index: usize,
    /// Delta (content change)
    pub delta: ChatMessageDelta,
    /// Finish reason (if finished)
    pub finish_reason: Option<String>,
}

/// Chat message delta (for streaming)
#[derive(Debug, Serialize)]
pub struct ChatMessageDelta {
    /// Role (only in first chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Content delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Create completions routes
pub fn create_completions_routes() -> Router {
    Router::new().route("/v1/chat/completions", post(chat_completions_handler))
}

/// Handler for POST /v1/chat/completions
/// OpenAI-compatible chat completions endpoint
async fn chat_completions_handler(
    Extension(_claims): Extension<Claims>,
    Json(request): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    // Convert OpenAI request to internal ModelRequest
    let prompt = request
        .messages
        .iter()
        .map(|m| format!("{}: {}", m.role, m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let model_params = ModelParameters {
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        top_p: request.top_p,
        frequency_penalty: request.frequency_penalty,
        presence_penalty: request.presence_penalty,
        stop_sequences: request.stop.unwrap_or_default(),
        seed: None,
    };

    let model_request = ModelRequest {
        input: prompt,
        parameters: model_params,
        session_id: None,
        priority: 5,
        timeout: Some(30),
    };

    // Process request (simplified - in real implementation would use instance manager)
    if request.stream {
        // Streaming response
        let stream = create_streaming_response(&request.model, model_request);
        Sse::new(stream).into_response()
    } else {
        // Non-streaming response
        let response = process_chat_completion(&request.model, model_request).await;

        match response {
            Ok(completion) => (StatusCode::OK, Json(completion)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": {"message": e.to_string(), "type": "internal_error"}})),
            )
                .into_response(),
        }
    }
}

/// Process chat completion (non-streaming)
async fn process_chat_completion(
    model: &str,
    request: ModelRequest,
) -> Result<ChatCompletionResponse, crate::core::error::AppError> {
    // Simplified processing - in real implementation would use instance manager
    // For now, simulate processing
    tokio::time::sleep(Duration::from_millis(100)).await;

    let response_text = format!("Processed: {}", request.input);

    let completion = ChatCompletionResponse {
        id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: model.to_string(),
        choices: vec![ChatCompletionChoice {
            index: 0,
            message: ChatMessage {
                role: "assistant".to_string(),
                content: response_text,
            },
            finish_reason: "stop".to_string(),
        }],
        usage: ChatUsage {
            prompt_tokens: request.input.len() / 4, // Rough estimate
            completion_tokens: response_text.len() / 4,
            total_tokens: (request.input.len() + response_text.len()) / 4,
        },
    };

    Ok(completion)
}

/// Create streaming response
fn create_streaming_response(
    model: &str,
    request: ModelRequest,
) -> impl Stream<Item = Result<Event, Infallible>> {
    let model = model.to_string();
    let response_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = chrono::Utc::now().timestamp();

    // Simulate streaming by chunking the response
    let response_text = format!("Processed: {}", request.input);
    let chunks: Vec<String> = response_text
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect())
        .collect();

    let chunks_len = chunks.len();
    let model_clone = model.clone();
    let response_id_clone = response_id.clone();

    stream::iter(chunks.into_iter().enumerate().map(move |(index, chunk)| {
        let is_first = index == 0;
        let is_last = index == chunks_len - 1;

        let chunk_data = ChatCompletionChunk {
            id: response_id_clone.clone(),
            object: "chat.completion.chunk".to_string(),
            created,
            model: model_clone.clone(),
            choices: vec![ChatCompletionChunkChoice {
                index: 0,
                delta: ChatMessageDelta {
                    role: if is_first {
                        Some("assistant".to_string())
                    } else {
                        None
                    },
                    content: Some(chunk),
                },
                finish_reason: if is_last {
                    Some("stop".to_string())
                } else {
                    None
                },
            }],
        };

        let json = serde_json::to_string(&chunk_data).unwrap_or_default();
        Ok(Event::default().data(json))
    }))
    .chain(stream::iter(vec![Ok(Event::default().data("[DONE]"))]))
}
