//! Galaxy settlement / payout batch depth classification stub (PH-S773, §8.2).

use crate::grid::galaxy_settlement_metrics::SettlementMetricsSnapshot;
use crate::grid::galaxy_settlement_mode::settlement_on_chain_enabled;
use crate::grid::galaxy_settlement_payout_batch_queue::payout_batch_queue_depth;

/// Settlement / payout batch wire depth (Galaxy §8.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementPayoutDepth {
    None,
    OfflineBatchQueued,
    OfflineBatchLedger,
    OnChainPending,
    FullDepth,
}

/// Classify settlement payout depth from optional metrics snapshot (PH-S773).
pub fn settlement_payout_depth_stub(
    snapshot: Option<&SettlementMetricsSnapshot>,
) -> SettlementPayoutDepth {
    if settlement_on_chain_enabled() {
        return SettlementPayoutDepth::OnChainPending;
    }
    let Some(s) = snapshot else {
        return SettlementPayoutDepth::None;
    };
    let queued = payout_batch_queue_depth() > 0;
    let ledger = s.payout_batch_total > 0;
    if queued && ledger {
        SettlementPayoutDepth::FullDepth
    } else if ledger {
        SettlementPayoutDepth::OfflineBatchLedger
    } else if queued {
        SettlementPayoutDepth::OfflineBatchQueued
    } else {
        SettlementPayoutDepth::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement_metrics::reset_settlement_metrics_for_test;
    use crate::grid::galaxy_settlement_payout_batch_queue::{
        enqueue_offline_payout_batch_on_cleared, reset_payout_batch_queue_for_test,
    };

    #[test]
    fn settlement_payout_depth_stub_ph_s773() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        reset_settlement_metrics_for_test();
        reset_payout_batch_queue_for_test();
        std::env::remove_var(crate::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN);

        assert_eq!(
            settlement_payout_depth_stub(None),
            SettlementPayoutDepth::None
        );

        enqueue_offline_payout_batch_on_cleared("j1");
        let snap = SettlementMetricsSnapshot {
            pending_verification_total: 0,
            cleared_total: 0,
            not_applicable_total: 0,
            resolved_total: 0,
            payout_batch_total: 0,
            human_review_total: 0,
        };
        assert_eq!(
            settlement_payout_depth_stub(Some(&snap)),
            SettlementPayoutDepth::OfflineBatchQueued
        );

        let snap_ledger = SettlementMetricsSnapshot {
            payout_batch_total: 2,
            ..snap
        };
        assert_eq!(
            settlement_payout_depth_stub(Some(&snap_ledger)),
            SettlementPayoutDepth::FullDepth
        );

        reset_payout_batch_queue_for_test();
        assert_eq!(
            settlement_payout_depth_stub(Some(&snap_ledger)),
            SettlementPayoutDepth::OfflineBatchLedger
        );

        std::env::set_var(
            crate::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN,
            "1",
        );
        assert_eq!(
            settlement_payout_depth_stub(Some(&snap_ledger)),
            SettlementPayoutDepth::OnChainPending
        );
        std::env::remove_var(crate::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN);

        reset_settlement_metrics_for_test();
        reset_payout_batch_queue_for_test();
    }
}
