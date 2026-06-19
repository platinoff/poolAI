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
}
