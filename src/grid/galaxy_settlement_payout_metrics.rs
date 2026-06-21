//! Galaxy payout batch metrics snapshot for HTTP wire (PH-S770, §8.2).

use crate::grid::galaxy_settlement_metrics::settlement_payout_batch_total;
use crate::grid::galaxy_settlement_mode::current_settlement_mode;
use crate::grid::galaxy_settlement_payout_batch_queue::payout_batch_queue_depth;

/// Read-only payout batch counters snapshot for `GET /api/v1/grid/payout-batch-metrics` (PH-S770).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PayoutBatchMetricsSnapshot {
    pub payout_batch_total: u64,
    pub payout_batch_queue_depth: u64,
    pub settlement_mode: &'static str,
}

/// Coordinator payout batch metrics snapshot (PH-S770).
pub fn payout_batch_metrics_snapshot() -> PayoutBatchMetricsSnapshot {
    PayoutBatchMetricsSnapshot {
        payout_batch_total: settlement_payout_batch_total(),
        payout_batch_queue_depth: payout_batch_queue_depth(),
        settlement_mode: current_settlement_mode(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement_metrics::{
        record_settlement_payout_batch, reset_settlement_metrics_for_test,
    };
    use crate::grid::galaxy_settlement_payout_batch_queue::{
        enqueue_offline_payout_batch_on_cleared, reset_payout_batch_queue_for_test,
    };

    #[test]
    fn payout_batch_metrics_snapshot_ph_s770() {
        reset_settlement_metrics_for_test();
        reset_payout_batch_queue_for_test();
        record_settlement_payout_batch();
        enqueue_offline_payout_batch_on_cleared("job-x");
        let snap = payout_batch_metrics_snapshot();
        assert_eq!(snap.payout_batch_total, 1);
        assert_eq!(snap.payout_batch_queue_depth, 1);
        assert_eq!(snap.settlement_mode, "offline_batch");
        reset_settlement_metrics_for_test();
        reset_payout_batch_queue_for_test();
    }
}
