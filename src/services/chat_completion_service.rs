//! OpenAI-compatible chat completion orchestration (non-stream + SSE helpers).

use crate::core::error::AppError;
use crate::core::model_interface::{ModelParameters, ModelRequest};
use crate::core::state::ApiContext;
use crate::runtime::instance::InstanceManager;
use axum::response::sse::Event;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock as TokioRwLock;
use tokio_stream::{self as stream, Stream, StreamExt};

/// OpenAI-compatible chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// OpenAI-compatible chat completion request
#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default)]
    pub stream: bool,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default)]
    pub frequency_penalty: f32,
    #[serde(default)]
    pub presence_penalty: f32,
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

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatUsage,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct ChatUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunkChoice {
    pub index: usize,
    pub delta: ChatMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessageDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

pub struct ChatCompletionService;

impl ChatCompletionService {
    pub fn build_model_request(request: &ChatCompletionRequest) -> ModelRequest {
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
            stop_sequences: request.stop.clone().unwrap_or_default(),
            seed: None,
        };

        ModelRequest {
            input: prompt,
            parameters: model_params,
            session_id: None,
            priority: 5,
            timeout: Some(30),
        }
    }

    /// Returns `(manager Arc, instance_id)` when an instance serves `model_id`.
    pub async fn resolve_instance_for_model(
        ctx: &ApiContext,
        model_id: &str,
    ) -> Option<(Arc<TokioRwLock<InstanceManager>>, String)> {
        let mgr = ctx.instance_manager.get().cloned()?;
        let instance_id = {
            let m = mgr.read().await;
            m.get_instance_by_model_id(model_id)
                .await
                .map(|i| i.instance_id.clone())
        }?;
        Some((mgr, instance_id))
    }

    pub async fn complete_via_instance(
        instance_id: &str,
        model: &str,
        request: ModelRequest,
        manager_arc: Arc<TokioRwLock<InstanceManager>>,
    ) -> Result<ChatCompletionResponse, AppError> {
        let manager = manager_arc.read().await;
        let model_response = manager
            .process_request_via_instance(instance_id, request.clone())
            .await?;

        Ok(ChatCompletionResponse {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp(),
            model: model.to_string(),
            choices: vec![ChatCompletionChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: model_response.output,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: ChatUsage {
                prompt_tokens: model_response.metrics.tokens_generated,
                completion_tokens: model_response.metrics.tokens_generated,
                total_tokens: model_response.metrics.tokens_generated * 2,
            },
        })
    }

    pub async fn complete_fallback(
        model: &str,
        request: ModelRequest,
    ) -> Result<ChatCompletionResponse, AppError> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let response_text = format!("Processed: {}", request.input);

        Ok(ChatCompletionResponse {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp(),
            model: model.to_string(),
            choices: vec![ChatCompletionChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: response_text.clone(),
                },
                finish_reason: "stop".to_string(),
            }],
            usage: ChatUsage {
                prompt_tokens: request.input.len() / 4,
                completion_tokens: response_text.len() / 4,
                total_tokens: (request.input.len() + response_text.len()) / 4,
            },
        })
    }

    pub fn stream_from_instance(
        instance_id: &str,
        model: &str,
        request: ModelRequest,
        manager_opt: Option<Arc<TokioRwLock<InstanceManager>>>,
    ) -> impl Stream<Item = Result<Event, Infallible>> {
        use tokio::sync::mpsc;

        let instance_id = instance_id.to_string();
        let model = model.to_string();
        let response_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
        let created = chrono::Utc::now().timestamp();

        if let Some(manager_arc) = manager_opt {
            let (tx, rx) = mpsc::unbounded_channel();
            let manager_clone = manager_arc.clone();
            let request_clone = request.clone();
            let instance_id_clone = instance_id.clone();
            let model_clone = model.clone();
            let response_id_clone = response_id.clone();

            let request_input = request_clone.input.clone();
            tokio::spawn(async move {
                let manager = manager_clone.read().await;
                match manager
                    .process_request_via_instance(&instance_id_clone, request_clone)
                    .await
                {
                    Ok(model_response) => {
                        stream_chunks_to_channel(
                            &tx,
                            &response_id_clone,
                            created,
                            &model_clone,
                            &model_response.output,
                        );
                    }
                    Err(_) => {
                        let response_text = format!("Processed: {}", request_input);
                        stream_chunks_to_channel(
                            &tx,
                            &response_id_clone,
                            created,
                            &model_clone,
                            &response_text,
                        );
                    }
                }
            });

            return tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let model_clone = model.clone();
        let request_input = request.input.clone();
        let response_id_clone = response_id.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let response_text = format!("Processed: {}", request_input);
            stream_chunks_to_channel(
                &tx,
                &response_id_clone,
                created,
                &model_clone,
                &response_text,
            );
        });

        tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
    }

    pub fn stream_fallback(model: &str, request: ModelRequest) -> impl Stream<Item = Result<Event, Infallible>> {
        let model = model.to_string();
        let response_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
        let created = chrono::Utc::now().timestamp();
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
            chunk_event(
                &response_id_clone,
                created,
                &model_clone,
                index,
                chunks_len,
                chunk,
            )
        }))
        .chain(stream::iter(vec![Ok(Event::default().data("[DONE]"))]))
    }
}

fn stream_chunks_to_channel(
    tx: &tokio::sync::mpsc::UnboundedSender<Result<Event, Infallible>>,
    response_id: &str,
    created: i64,
    model: &str,
    response_text: &str,
) {
    let chunks: Vec<String> = response_text
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect())
        .collect();
    let chunks_len = chunks.len();
    for (index, chunk) in chunks.into_iter().enumerate() {
        let _ = tx.send(chunk_event(
            response_id,
            created,
            model,
            index,
            chunks_len,
            chunk,
        ));
    }
    let _ = tx.send(Ok(Event::default().data("[DONE]")));
}

fn chunk_event(
    response_id: &str,
    created: i64,
    model: &str,
    index: usize,
    chunks_len: usize,
    chunk: String,
) -> Result<Event, Infallible> {
    let is_first = index == 0;
    let is_last = index == chunks_len - 1;
    let chunk_data = ChatCompletionChunk {
        id: response_id.to_string(),
        object: "chat.completion.chunk".to_string(),
        created,
        model: model.to_string(),
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
}
