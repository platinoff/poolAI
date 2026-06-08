//! Telegram payout wallet bindings (PH-S131 stub, Galaxy §3.2).
//!
//! Links `telegram_user_id` → `payout_pubkey` for settlement identity.
//! No live Solana on-chain verification wire.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::virtual_node_store;

/// Supported payout chain on the stub path (concept default).
pub const DEFAULT_WALLET_CHAIN: &str = "solana";

/// Solana pubkey length bounds for base58 stub validation.
pub const SOLANA_PUBKEY_MIN_LEN: usize = 32;
pub const SOLANA_PUBKEY_MAX_LEN: usize = 44;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelegramWalletBinding {
    pub telegram_user_id: String,
    pub chat_id: String,
    pub payout_pubkey: String,
    pub chain: String,
    /// Stub flag — always `true` on bind until on-chain verify wire (future sprint).
    pub verified: bool,
    pub bound_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletBindError {
    EmptyTelegramUserId,
    EmptyChatId,
    EmptyPayoutPubkey,
    UnsupportedChain(String),
    InvalidSolanaPubkey,
}

impl WalletBindError {
    pub fn as_status_message(&self) -> &'static str {
        match self {
            Self::EmptyTelegramUserId => "telegram_user_id is required",
            Self::EmptyChatId => "chat_id is required",
            Self::EmptyPayoutPubkey => "payout_pubkey is required",
            Self::UnsupportedChain(_) => "unsupported chain (stub supports solana only)",
            Self::InvalidSolanaPubkey => "invalid solana payout_pubkey",
        }
    }
}

fn wallets() -> &'static Mutex<HashMap<String, TelegramWalletBinding>> {
    static W: OnceLock<Mutex<HashMap<String, TelegramWalletBinding>>> = OnceLock::new();
    W.get_or_init(|| {
        let mut map = HashMap::new();
        if let Ok(loaded) = virtual_node_store::load_json("telegram_wallets.json") {
            if let Ok(entries) = serde_json::from_value::<Vec<TelegramWalletBinding>>(loaded) {
                for w in entries {
                    map.insert(w.telegram_user_id.clone(), w);
                }
            }
        }
        Mutex::new(map)
    })
}

fn persist_all(guard: &HashMap<String, TelegramWalletBinding>) {
    let entries: Vec<_> = guard.values().cloned().collect();
    if let Err(e) = virtual_node_store::save_json("telegram_wallets.json", &entries) {
        tracing::warn!("virtual node telegram wallets persist failed: {e}");
    }
}

/// Base58 charset stub check for Solana pubkeys (no on-chain verify).
pub fn is_valid_solana_pubkey_stub(pubkey: &str) -> bool {
    let s = pubkey.trim();
    let len = s.len();
    if !(SOLANA_PUBKEY_MIN_LEN..=SOLANA_PUBKEY_MAX_LEN).contains(&len) {
        return false;
    }
    s.chars().all(|c| {
        matches!(
            c,
            '1'..='9'
                | 'A'..='H'
                | 'J'..='N'
                | 'P'..='Z'
                | 'a'..='k'
                | 'm'..='z'
        )
    })
}

pub struct VirtualNodeTelegramWalletService;

impl VirtualNodeTelegramWalletService {
    pub fn bind(
        telegram_user_id: &str,
        chat_id: &str,
        payout_pubkey: &str,
        chain: Option<&str>,
    ) -> Result<TelegramWalletBinding, WalletBindError> {
        let telegram_user_id = telegram_user_id.trim();
        let chat_id = chat_id.trim();
        let payout_pubkey = payout_pubkey.trim();
        if telegram_user_id.is_empty() {
            return Err(WalletBindError::EmptyTelegramUserId);
        }
        if chat_id.is_empty() {
            return Err(WalletBindError::EmptyChatId);
        }
        if payout_pubkey.is_empty() {
            return Err(WalletBindError::EmptyPayoutPubkey);
        }
        let chain = chain
            .unwrap_or(DEFAULT_WALLET_CHAIN)
            .trim()
            .to_ascii_lowercase();
        if chain != DEFAULT_WALLET_CHAIN {
            return Err(WalletBindError::UnsupportedChain(chain));
        }
        if !is_valid_solana_pubkey_stub(payout_pubkey) {
            return Err(WalletBindError::InvalidSolanaPubkey);
        }
        let binding = TelegramWalletBinding {
            telegram_user_id: telegram_user_id.to_string(),
            chat_id: chat_id.to_string(),
            payout_pubkey: payout_pubkey.to_string(),
            chain,
            verified: true,
            bound_at: Utc::now(),
        };
        let mut guard = wallets().lock().expect("telegram wallets lock");
        guard.insert(telegram_user_id.to_string(), binding.clone());
        persist_all(&guard);
        Ok(binding)
    }

    pub fn lookup(telegram_user_id: &str) -> Option<TelegramWalletBinding> {
        let guard = wallets().lock().expect("telegram wallets lock");
        guard.get(telegram_user_id).cloned()
    }

    pub fn list() -> Vec<TelegramWalletBinding> {
        let guard = wallets().lock().expect("telegram wallets lock");
        let mut v: Vec<_> = guard.values().cloned().collect();
        v.sort_by(|a, b| a.telegram_user_id.cmp(&b.telegram_user_id));
        v
    }

    /// Test helper — reset in-memory wallets.
    pub fn clear_all() {
        let mut guard = wallets().lock().expect("telegram wallets lock");
        guard.clear();
        persist_all(&guard);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_wallet_roundtrip() {
        VirtualNodeTelegramWalletService::clear_all();
        let pubkey = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";
        let w =
            VirtualNodeTelegramWalletService::bind("9001", "-100123", pubkey, None).expect("bind");
        assert_eq!(w.chain, "solana");
        assert!(w.verified);
        let row = VirtualNodeTelegramWalletService::lookup("9001").expect("lookup");
        assert_eq!(row.payout_pubkey, pubkey);
        VirtualNodeTelegramWalletService::clear_all();
    }

    #[test]
    fn reject_invalid_pubkey() {
        VirtualNodeTelegramWalletService::clear_all();
        let err = VirtualNodeTelegramWalletService::bind("9001", "-100123", "bad!", None)
            .expect_err("reject");
        assert_eq!(err, WalletBindError::InvalidSolanaPubkey);
    }

    #[test]
    fn reject_unsupported_chain() {
        VirtualNodeTelegramWalletService::clear_all();
        let pubkey = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";
        let err =
            VirtualNodeTelegramWalletService::bind("9001", "-100123", pubkey, Some("ethereum"))
                .expect_err("reject");
        assert!(matches!(err, WalletBindError::UnsupportedChain(_)));
    }
}
