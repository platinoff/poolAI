//! Telegram edge seat cap admission stub (PH-S475, Galaxy §3.1).
//! Seat policy `bound_wallet_session` (PH-S486).

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use crate::services::virtual_node_telegram_binding_service::VirtualNodeTelegramBindingService;

/// Env: max concurrent active `telegram_edge` workers (Galaxy §3.1 seat cap).
pub const ENV_TELEGRAM_SEAT_LIMIT: &str = "POOLAI_TELEGRAM_SEAT_LIMIT";

/// Env: `flat` (default) | `bound_wallet_session` seat policy (PH-S486).
pub const ENV_TELEGRAM_SEAT_POLICY: &str = "POOLAI_TELEGRAM_SEAT_POLICY";

static ACTIVE_TELEGRAM_EDGE: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Whether register-remote metadata marks a `telegram_edge` worker.
pub fn is_telegram_edge_metadata(metadata: &std::collections::HashMap<String, String>) -> bool {
    metadata
        .get("origin")
        .map(|v| v.trim().eq_ignore_ascii_case("telegram_edge"))
        .unwrap_or(false)
        || metadata
            .get("role")
            .map(|v| v.trim().eq_ignore_ascii_case("telegram_edge"))
            .unwrap_or(false)
}

/// Configured seat limit from env; `None` when unset (no cap).
pub fn telegram_seat_limit_from_env() -> Option<u32> {
    std::env::var(ENV_TELEGRAM_SEAT_LIMIT)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
}

/// Seat policy from env (PH-S486).
pub fn telegram_seat_policy_from_env() -> TelegramSeatPolicy {
    match std::env::var(ENV_TELEGRAM_SEAT_POLICY)
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "bound_wallet_session" | "bound-wallet-session" => TelegramSeatPolicy::BoundWalletSession,
        _ => TelegramSeatPolicy::Flat,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelegramSeatPolicy {
    Flat,
    BoundWalletSession,
}

/// Effective seat cap: `min(admin_max, bound_wallets)` under bound_wallet_session (PH-S486).
pub fn compute_seat_limit(admin_max: u32, bound_wallets: usize) -> u32 {
    match telegram_seat_policy_from_env() {
        TelegramSeatPolicy::Flat => admin_max,
        TelegramSeatPolicy::BoundWalletSession => admin_max.min(bound_wallets.max(1) as u32),
    }
}

fn effective_seat_limit() -> Option<u32> {
    let admin_max = telegram_seat_limit_from_env()?;
    let bound = VirtualNodeTelegramBindingService::list().len();
    Some(compute_seat_limit(admin_max, bound))
}

/// Admit one `telegram_edge` peer; returns `Err(())` when seat cap exhausted.
pub fn try_admit_telegram_edge(peer_id: &str) -> Result<(), ()> {
    let Some(limit) = effective_seat_limit() else {
        return Ok(());
    };
    let mut active = ACTIVE_TELEGRAM_EDGE
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if active.contains(peer_id) {
        return Ok(());
    }
    if active.len() >= limit as usize {
        return Err(());
    }
    active.insert(peer_id.to_string());
    Ok(())
}

/// Active telegram_edge seat count (in-process stub).
pub fn active_telegram_edge_seats() -> usize {
    ACTIVE_TELEGRAM_EDGE.lock().map(|g| g.len()).unwrap_or(0)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_telegram_seats_for_test() {
    if let Ok(mut active) = ACTIVE_TELEGRAM_EDGE.lock() {
        active.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn try_admit_telegram_edge_ph_s475() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_telegram_seats_for_test();
        std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "1");
        assert!(try_admit_telegram_edge("peer-a").is_ok());
        assert_eq!(active_telegram_edge_seats(), 1);
        assert!(try_admit_telegram_edge("peer-b").is_err());
        assert!(try_admit_telegram_edge("peer-a").is_ok());
        std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
        reset_telegram_seats_for_test();
    }

    #[test]
    fn is_telegram_edge_metadata_ph_s475() {
        let mut meta = HashMap::new();
        meta.insert("origin".into(), "telegram_edge".into());
        assert!(is_telegram_edge_metadata(&meta));
        meta.insert("origin".into(), "local_srv".into());
        assert!(!is_telegram_edge_metadata(&meta));
    }

    #[test]
    fn compute_seat_limit_bound_wallet_session_ph_s486() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var(ENV_TELEGRAM_SEAT_POLICY, "bound_wallet_session");
        assert_eq!(compute_seat_limit(10, 3), 3);
        assert_eq!(compute_seat_limit(2, 5), 2);
        std::env::remove_var(ENV_TELEGRAM_SEAT_POLICY);
        assert_eq!(compute_seat_limit(10, 3), 10);
    }
}
