//! Forward Telegram updates to PoolAI coordinator webhook (FM-016++).

use reqwest::StatusCode;
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

    pub fn wallet_url(&self) -> String {
        format!("{}/api/v1/virtual-nodes/telegram/wallet", self.base_url)
    }

    pub fn telegram_seats_url(&self) -> String {
        format!("{}/api/v1/grid/telegram-seats", self.base_url)
    }

    pub fn telegram_unbind_url(&self, telegram_user_id: i64) -> String {
        format!(
            "{}/api/v1/virtual-nodes/telegram/bindings/{telegram_user_id}",
            self.base_url
        )
    }
}

/// Coordinator Telegram seat snapshot (`GET /api/v1/grid/telegram-seats`, PH-S514).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelegramSeatsSnapshot {
    pub seat_policy: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_max_seats: Option<u32>,
    pub bound_wallets_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seat_limit: Option<u32>,
    pub active_telegram_edge_workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindResult {
    pub ok: bool,
    pub detail: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBindResult {
    pub ok: bool,
    pub payout_pubkey: Option<String>,
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

/// POST wallet bind to coordinator (PH-S509, Galaxy §3.2).
pub async fn bind_wallet(
    config: &CoordinatorConfig,
    user_id: i64,
    chat_id: i64,
    payout_pubkey: &str,
) -> Result<WalletBindResult, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let payload = json!({
        "telegram_user_id": user_id.to_string(),
        "chat_id": chat_id.to_string(),
        "payout_pubkey": payout_pubkey,
        "chain": "solana",
    });

    let response = client
        .post(config.wallet_url())
        .header("content-type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("wallet bind request failed: {e}"))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        let v: Value = serde_json::from_str(&body).map_err(|e| format!("parse response: {e}"))?;
        return Ok(WalletBindResult {
            ok: true,
            payout_pubkey: v
                .get("wallet")
                .and_then(|w| w.get("payout_pubkey"))
                .and_then(|x| x.as_str())
                .map(str::to_string),
            detail: None,
        });
    }

    let detail = serde_json::from_str::<Value>(&body)
        .ok()
        .and_then(|v| {
            v.get("message")
                .and_then(|m| m.as_str())
                .map(str::to_string)
        })
        .unwrap_or(body);
    Ok(WalletBindResult {
        ok: false,
        payout_pubkey: None,
        detail: Some(format!("HTTP {status}: {detail}")),
    })
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

/// Fetch coordinator Telegram seat snapshot (PH-S514, Galaxy §3.1).
pub async fn fetch_telegram_seats(
    config: &CoordinatorConfig,
) -> Result<TelegramSeatsSnapshot, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(config.telegram_seats_url())
        .send()
        .await
        .map_err(|e| format!("telegram-seats request failed: {e}"))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!("telegram-seats HTTP {status}: {body}"));
    }
    serde_json::from_str(&body).map_err(|e| format!("parse telegram-seats: {e}"))
}

/// DELETE Telegram edge binding (PH-S515, Galaxy §3.2 `/stop`).
pub async fn unbind_telegram(
    config: &CoordinatorConfig,
    user_id: i64,
) -> Result<UnbindResult, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .delete(config.telegram_unbind_url(user_id))
        .send()
        .await
        .map_err(|e| format!("unbind request failed: {e}"))?;

    match response.status() {
        s if s.is_success() => Ok(UnbindResult {
            ok: true,
            detail: None,
        }),
        StatusCode::NOT_FOUND => Ok(UnbindResult {
            ok: false,
            detail: Some("No binding found for this Telegram user".into()),
        }),
        s => {
            let body = response.text().await.unwrap_or_default();
            Ok(UnbindResult {
                ok: false,
                detail: Some(format!("HTTP {s}: {body}")),
            })
        }
    }
}

/// User-facing reply after `/status` seat snapshot (PH-S514).
pub fn format_seats_reply(result: &Result<TelegramSeatsSnapshot, String>) -> String {
    match result {
        Ok(s) => {
            let limit = s
                .seat_limit
                .map(|n| n.to_string())
                .unwrap_or_else(|| "∞".into());
            format!(
                "📊 Telegram seats\npolicy: `{}`\nactive: {}/{}\nbound wallets: {}",
                s.seat_policy, s.active_telegram_edge_workers, limit, s.bound_wallets_count
            )
        }
        Err(e) => format!("❌ {e}"),
    }
}

/// User-facing reply after `/stop` unbind (PH-S515).
pub fn format_unbind_reply(result: &Result<UnbindResult, String>) -> String {
    match result {
        Ok(r) if r.ok => "✅ Edge worker unbound".to_string(),
        Ok(r) => r
            .detail
            .clone()
            .unwrap_or_else(|| "Unbind rejected".to_string()),
        Err(e) => format!("❌ {e}"),
    }
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

/// User-facing reply after wallet bind (PH-S509).
pub fn format_wallet_reply(result: &Result<WalletBindResult, String>) -> String {
    match result {
        Ok(r) if r.ok => {
            let pk = r.payout_pubkey.as_deref().unwrap_or("?");
            format!("✅ Wallet bound: `{pk}`")
        }
        Ok(r) => r
            .detail
            .clone()
            .unwrap_or_else(|| "Wallet bind rejected".to_string()),
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

    #[test]
    fn wallet_bind_payload_fields_ph_s509() {
        let cfg = CoordinatorConfig {
            base_url: "http://127.0.0.1:8080".into(),
            webhook_secret: None,
        };
        assert!(cfg
            .wallet_url()
            .ends_with("/api/v1/virtual-nodes/telegram/wallet"));
    }
}
