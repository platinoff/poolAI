//! Cleared on-chain settlement depth classification (PH-S870, Solana band 22).

use crate::grid::galaxy_settlement_mode::settlement_on_chain_enabled;
use crate::grid::galaxy_settlement_onchain::{
    last_onchain_rpc_signature_len, onchain_events_dir_configured, onchain_submit_total,
};

/// On-chain cleared settlement wire depth (Galaxy §7 / FM-010).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementOnchainDepth {
    None,
    NdjsonSink,
    MockRpcPending,
    FullDepth,
}

/// Classify on-chain cleared depth from env + submit counters (PH-S870).
pub fn settlement_onchain_depth_stub(
    events_dir: bool,
    on_chain: bool,
    submit_total: u64,
    last_sig_len: u64,
) -> SettlementOnchainDepth {
    if on_chain && events_dir && submit_total > 0 && last_sig_len > 0 {
        SettlementOnchainDepth::FullDepth
    } else if on_chain && events_dir {
        SettlementOnchainDepth::MockRpcPending
    } else if events_dir {
        SettlementOnchainDepth::NdjsonSink
    } else {
        SettlementOnchainDepth::None
    }
}

/// Wire label for payout-batch / stand smoke (PH-S870).
pub fn settlement_onchain_depth_wire_label(depth: SettlementOnchainDepth) -> &'static str {
    match depth {
        SettlementOnchainDepth::None => "none",
        SettlementOnchainDepth::NdjsonSink => "ndjson_sink",
        SettlementOnchainDepth::MockRpcPending => "mock_rpc_pending",
        SettlementOnchainDepth::FullDepth => "full_depth",
    }
}

/// Runtime on-chain depth from process env + counters.
pub fn current_settlement_onchain_depth() -> SettlementOnchainDepth {
    settlement_onchain_depth_stub(
        onchain_events_dir_configured(),
        settlement_on_chain_enabled(),
        onchain_submit_total(),
        last_onchain_rpc_signature_len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_onchain_depth_stub_ph_s870() {
        assert_eq!(
            settlement_onchain_depth_stub(false, false, 0, 0),
            SettlementOnchainDepth::None
        );
        assert_eq!(
            settlement_onchain_depth_stub(true, false, 0, 0),
            SettlementOnchainDepth::NdjsonSink
        );
        assert_eq!(
            settlement_onchain_depth_stub(true, true, 0, 0),
            SettlementOnchainDepth::MockRpcPending
        );
        assert_eq!(
            settlement_onchain_depth_stub(true, true, 2, 24),
            SettlementOnchainDepth::FullDepth
        );
        assert_eq!(
            settlement_onchain_depth_wire_label(SettlementOnchainDepth::FullDepth),
            "full_depth"
        );
    }
}
