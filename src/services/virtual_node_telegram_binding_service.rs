//! Telegram user/chat ↔ virtual-node (`peer_id`) bindings (FM-016+).
//!
//! Coordinator resolves Telegram updates to a registered worker via `telegram_user_id`.
//! Optional persistence: set `POOLAI_VIRTUAL_NODE_DATA_DIR` (see `virtual_node_store`).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::virtual_node_store;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelegramBinding {
    pub telegram_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    pub peer_id: String,
    pub bound_at: DateTime<Utc>,
}

fn bindings() -> &'static Mutex<HashMap<String, TelegramBinding>> {
    static B: OnceLock<Mutex<HashMap<String, TelegramBinding>>> = OnceLock::new();
    B.get_or_init(|| {
        let mut map = HashMap::new();
        if let Ok(loaded) = virtual_node_store::load_json("telegram_bindings.json") {
            if let Ok(entries) = serde_json::from_value::<Vec<TelegramBinding>>(loaded) {
                for b in entries {
                    map.insert(b.telegram_user_id.clone(), b);
                }
            }
        }
        Mutex::new(map)
    })
}

fn persist_all(guard: &HashMap<String, TelegramBinding>) {
    let entries: Vec<_> = guard.values().cloned().collect();
    if let Err(e) = virtual_node_store::save_json("telegram_bindings.json", &entries) {
        tracing::warn!("virtual node telegram bindings persist failed: {e}");
    }
}

pub struct VirtualNodeTelegramBindingService;

impl VirtualNodeTelegramBindingService {
    pub fn bind(telegram_user_id: &str, chat_id: Option<String>, peer_id: &str) -> TelegramBinding {
        let binding = TelegramBinding {
            telegram_user_id: telegram_user_id.to_string(),
            chat_id,
            peer_id: peer_id.to_string(),
            bound_at: Utc::now(),
        };
        let mut guard = bindings().lock().expect("telegram bindings lock");
        guard.insert(telegram_user_id.to_string(), binding.clone());
        persist_all(&guard);
        binding
    }

    pub fn lookup(telegram_user_id: &str) -> Option<TelegramBinding> {
        let guard = bindings().lock().expect("telegram bindings lock");
        guard.get(telegram_user_id).cloned()
    }

    pub fn lookup_by_peer(peer_id: &str) -> Option<TelegramBinding> {
        let guard = bindings().lock().expect("telegram bindings lock");
        guard.values().find(|b| b.peer_id == peer_id).cloned()
    }

    pub fn list() -> Vec<TelegramBinding> {
        let guard = bindings().lock().expect("telegram bindings lock");
        let mut v: Vec<_> = guard.values().cloned().collect();
        v.sort_by(|a, b| a.telegram_user_id.cmp(&b.telegram_user_id));
        v
    }

    pub fn unbind(telegram_user_id: &str) -> bool {
        let mut guard = bindings().lock().expect("telegram bindings lock");
        let removed = guard.remove(telegram_user_id).is_some();
        if removed {
            persist_all(&guard);
        }
        removed
    }

    /// Test helper — reset in-memory bindings (does not delete on-disk store unless dir set).
    pub fn clear_all() {
        let mut guard = bindings().lock().expect("telegram bindings lock");
        guard.clear();
        persist_all(&guard);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_and_lookup_roundtrip() {
        VirtualNodeTelegramBindingService::clear_all();
        VirtualNodeTelegramBindingService::bind("tg-42", Some("chat-99".into()), "worker-a");
        let b = VirtualNodeTelegramBindingService::lookup("tg-42").expect("binding");
        assert_eq!(b.peer_id, "worker-a");
        assert_eq!(b.chat_id.as_deref(), Some("chat-99"));
        VirtualNodeTelegramBindingService::clear_all();
    }
}
