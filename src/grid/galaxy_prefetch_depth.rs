//! Galaxy prefetch live pull depth classification stub (PH-S754, §5.5).

use crate::grid::galaxy_prefetch_metrics::PrefetchMetricsSnapshot;

/// Prefetch live pull telemetry depth (Galaxy §5.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchDepth {
    None,
    PlanOnly,
    LivePull,
    Backpressure,
    FullDepth,
}

/// Classify prefetch depth from optional metrics snapshot (PH-S754).
pub fn prefetch_depth_stub(snapshot: Option<&PrefetchMetricsSnapshot>) -> PrefetchDepth {
    let Some(s) = snapshot else {
        return PrefetchDepth::None;
    };
    let has_pull = s.pull_bytes_total > 0;
    let has_backpressure = s.backpressure_total > 0;
    let has_peer_fetch = s.peer_fetch_total > 0;
    let has_plan = s.plan_total > 0 || s.enqueue_total > 0;

    if has_pull && (has_backpressure || has_peer_fetch) {
        PrefetchDepth::FullDepth
    } else if has_backpressure {
        PrefetchDepth::Backpressure
    } else if has_pull {
        PrefetchDepth::LivePull
    } else if has_plan {
        PrefetchDepth::PlanOnly
    } else {
        PrefetchDepth::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefetch_depth_stub_ph_s754() {
        assert_eq!(prefetch_depth_stub(None), PrefetchDepth::None);
        assert_eq!(
            prefetch_depth_stub(Some(&PrefetchMetricsSnapshot {
                pull_bytes_total: 0,
                backpressure_total: 0,
                plan_total: 2,
                enqueue_total: 0,
                peer_fetch_total: 0,
            })),
            PrefetchDepth::PlanOnly
        );
        assert_eq!(
            prefetch_depth_stub(Some(&PrefetchMetricsSnapshot {
                pull_bytes_total: 4_194_304,
                backpressure_total: 0,
                plan_total: 1,
                enqueue_total: 1,
                peer_fetch_total: 0,
            })),
            PrefetchDepth::LivePull
        );
        assert_eq!(
            prefetch_depth_stub(Some(&PrefetchMetricsSnapshot {
                pull_bytes_total: 0,
                backpressure_total: 3,
                plan_total: 0,
                enqueue_total: 0,
                peer_fetch_total: 0,
            })),
            PrefetchDepth::Backpressure
        );
        assert_eq!(
            prefetch_depth_stub(Some(&PrefetchMetricsSnapshot {
                pull_bytes_total: 8_388_608,
                backpressure_total: 1,
                plan_total: 2,
                enqueue_total: 1,
                peer_fetch_total: 2,
            })),
            PrefetchDepth::FullDepth
        );
    }
}
