//! Forward Telegram updates to PoolAI coordinator webhook (FM-016++).

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

/// Coordinator connection for the Telegram bot process.
#[derive(Debug, Clone)]
pub struct CoordinatorConfig {
    pub base_url: String,
    pub webhook_secret: Option<String>,
}

impl CoordinatorConfig {
    pub fn from_env() -> Result<Self, String> {
        let base_url = std::env::var("POOLAI_COORDINATOR_URL")
            .map_err(|_| "POOLAI_COORDINATOR_URL is required".to_string())?
            .trim_end_matches('/')
            .to_string();
        let webhook_secret = std::env::var("POOLAI_TELEGRAM_WEBHOOK_SECRET")
            .ok()
            .filter(|s| !s.trim().is_empty());
        Ok(Self {
            base_url,
            webhook_secret,
        })
    }

    pub fn webhook_url(&self) -> String {
        format!("{}/api/v1/virtual-nodes/telegram/webhook", self.base_url)
    }
}

/// Build Telegram Bot API-shaped JSON accepted by `POST .../telegram/webhook`.
pub fn message_to_webhook_payload(update_id: i64, user_id: i64, chat_id: i64, text: &str) -> Value {
    json!({
        "update_id": update_id,
        "message": {
            "from": { "id": user_id },
            "chat": { "id": chat_id },
            "text": text,
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardResult {
    pub ok: bool,
    pub peer_id: Option<String>,
    pub task_type: Option<String>,
    pub detail: Option<String>,
}

/// POST update JSON to coordinator virtual-node webhook.
pub async fn forward_message(
    config: &CoordinatorConfig,
    update_id: i64,
    user_id: i64,
    chat_id: i64,
    text: &str,
) -> Result<ForwardResult, String> {
    let payload = message_to_webhook_payload(update_id, user_id, chat_id, text);
    forward_payload(config, &payload).await
}

pub async fn forward_payload(
    config: &CoordinatorConfig,
    payload: &Value,
) -> Result<ForwardResult, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let mut req = client
        .post(config.webhook_url())
        .header("content-type", "application/json")
        .json(payload);

    if let Some(secret) = &config.webhook_secret {
        req = req.header("x-telegram-webhook-secret", secret);
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("coordinator webhook request failed: {e}"))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("coordinator webhook HTTP {status}: {body}"));
    }

    let v: Value = serde_json::from_str(&body).map_err(|e| format!("parse response: {e}"))?;
    Ok(ForwardResult {
        ok: v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false),
        peer_id: v
            .get("peer_id")
            .and_then(|x| x.as_str())
            .map(str::to_string),
        task_type: v
            .get("task")
            .and_then(|t| t.get("task_type"))
            .and_then(|x| x.as_str())
            .map(str::to_string),
        detail: v.get("detail").and_then(|x| x.as_str()).map(str::to_string),
    })
}

/// User-facing reply after coordinator forward.
pub fn format_reply(result: &Result<ForwardResult, String>) -> String {
    match result {
        Ok(r) if r.ok => {
            let peer = r.peer_id.as_deref().unwrap_or("?");
            let task = r.task_type.as_deref().unwrap_or("task");
            format!("✅ Enqueued `{task}` for worker `{peer}`")
        }
        Ok(r) => r
            .detail
            .clone()
            .unwrap_or_else(|| "Coordinator rejected the update".to_string()),
        Err(e) => format!("❌ {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_payload_shape() {
        let v = message_to_webhook_payload(1, 42, 99, "/status");
        assert_eq!(v["update_id"], 1);
        assert_eq!(v["message"]["from"]["id"], 42);
        assert_eq!(v["message"]["text"], "/status");
    }
}
