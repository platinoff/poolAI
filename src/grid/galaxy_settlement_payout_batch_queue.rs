//! Offline payout batch queue stub on cleared settlement (PH-S770, Galaxy §8.2).
//!
//! Enqueues ledger job ids pending offline batch submit; mirrored on `GET /metrics`.

use std::collections::VecDeque;
use std::sync::Mutex;

/// Prometheus gauge: pending offline payout batch queue depth (PH-S770).
pub const METRIC_SETTLEMENT_PAYOUT_BATCH_QUEUE_DEPTH: &str =
    "galaxy_settlement_payout_batch_queue_depth";

static PAYOUT_BATCH_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// Enqueue one cleared job for offline payout batch processing (PH-S770).
pub fn enqueue_offline_payout_batch_on_cleared(job_id: &str) {
    if let Ok(mut q) = PAYOUT_BATCH_QUEUE.lock() {
        q.push_back(job_id.to_string());
        while q.len() > 256 {
            q.pop_front();
        }
    }
}

/// Current offline payout batch queue depth.
pub fn payout_batch_queue_depth() -> u64 {
    PAYOUT_BATCH_QUEUE
        .lock()
        .ok()
        .map(|q| q.len() as u64)
        .unwrap_or(0)
}

/// Peek queued job ids (newest last) for diagnostics / tests.
pub fn payout_batch_queue_snapshot(limit: usize) -> Vec<String> {
    let cap = limit.clamp(1, 32);
    PAYOUT_BATCH_QUEUE
        .lock()
        .ok()
        .map(|q| {
            let start = q.len().saturating_sub(cap);
            q.range(start..).cloned().collect()
        })
        .unwrap_or_default()
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_payout_batch_queue_for_test() {
    if let Ok(mut q) = PAYOUT_BATCH_QUEUE.lock() {
        q.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_offline_payout_batch_on_cleared_ph_s770() {
        reset_payout_batch_queue_for_test();
        assert_eq!(payout_batch_queue_depth(), 0);
        enqueue_offline_payout_batch_on_cleared("job-a");
        enqueue_offline_payout_batch_on_cleared("job-b");
        assert_eq!(payout_batch_queue_depth(), 2);
        let snap = payout_batch_queue_snapshot(10);
        assert_eq!(snap, vec!["job-a".to_string(), "job-b".to_string()]);
        reset_payout_batch_queue_for_test();
    }
}
