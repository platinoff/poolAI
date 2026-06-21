//! Settlement mode toggle for payout batch / on-chain stub (PH-S550, Galaxy §8.2).

/// Env: when `1`, Cleared settlement uses on-chain mode stub.
pub const ENV_SETTLEMENT_ON_CHAIN: &str = "POOLAI_SETTLEMENT_ON_CHAIN";

/// Whether on-chain settlement mode is enabled.
pub fn settlement_on_chain_enabled() -> bool {
    matches!(
        std::env::var(ENV_SETTLEMENT_ON_CHAIN)
            .ok()
            .map(|v| v.trim().to_ascii_lowercase())
            .as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}

/// Wire settlement mode label for payout-batch API.
pub fn current_settlement_mode() -> &'static str {
    if settlement_on_chain_enabled() {
        "on_chain"
    } else {
        "offline_batch"
    }
}

/// Whether Cleared ledger row is pending on-chain submit.
pub fn settlement_on_chain_pending() -> bool {
    settlement_on_chain_enabled()
}

/// Whether offline batch payout gate is active (default Galaxy §8.2 path).
pub fn offline_batch_payout_enabled() -> bool {
    !settlement_on_chain_enabled()
}

/// Settlement mode gate label for docs / admin stubs (PH-S774).
pub fn settlement_mode_gate_label() -> &'static str {
    if settlement_on_chain_enabled() {
        "on_chain_submit_pending"
    } else {
        "offline_batch_queue"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_on_chain_toggle_ph_s550() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
        assert_eq!(current_settlement_mode(), "offline_batch");
        assert!(!settlement_on_chain_pending());
        std::env::set_var(ENV_SETTLEMENT_ON_CHAIN, "1");
        assert_eq!(current_settlement_mode(), "on_chain");
        assert!(settlement_on_chain_pending());
        std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
    }

    #[test]
    fn settlement_mode_offline_vs_on_chain_gate_ph_s774() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
        assert!(offline_batch_payout_enabled());
        assert_eq!(settlement_mode_gate_label(), "offline_batch_queue");
        assert_eq!(current_settlement_mode(), "offline_batch");
        std::env::set_var(ENV_SETTLEMENT_ON_CHAIN, "true");
        assert!(!offline_batch_payout_enabled());
        assert_eq!(settlement_mode_gate_label(), "on_chain_submit_pending");
        assert_eq!(current_settlement_mode(), "on_chain");
        std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
    }
}
