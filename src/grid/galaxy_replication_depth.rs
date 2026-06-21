//! Galaxy replication production depth classification (PH-S894, §6.4).

use crate::grid::galaxy_replication_metrics::{
    replication_max_per_hour_from_env, replication_metrics_snapshot, ReplicationMetricsSnapshot,
};

/// Replication production wire depth (Galaxy §6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationDepth {
    None,
    StrictIngest,
    ExecutorEnqueue,
    QuorumGate,
    RateCap,
    FullProduction,
}

/// Classify replication depth from metrics snapshot + hourly cap (PH-S894).
pub fn replication_depth_stub(
    snapshot: Option<&ReplicationMetricsSnapshot>,
    rate_cap_per_hour: u64,
) -> ReplicationDepth {
    let Some(s) = snapshot else {
        return ReplicationDepth::None;
    };
    let has_strict = s.strict_total > 0;
    let has_executor = s.executor_enqueue_total > 0;
    let has_quorum_gate = has_strict && has_executor && s.enqueue_total > s.strict_total;
    let has_rate_cap = s.rate_limited_total > 0;
    let cap_active = rate_cap_per_hour > 0 && rate_cap_per_hour < u64::MAX;

    if has_strict && has_executor && has_quorum_gate && has_rate_cap && cap_active {
        ReplicationDepth::FullProduction
    } else if has_rate_cap {
        ReplicationDepth::RateCap
    } else if has_quorum_gate {
        ReplicationDepth::QuorumGate
    } else if has_executor {
        ReplicationDepth::ExecutorEnqueue
    } else if has_strict {
        ReplicationDepth::StrictIngest
    } else {
        ReplicationDepth::None
    }
}

/// Wire label for replication-metrics / stand smoke (PH-S894).
pub fn replication_depth_wire_label(depth: ReplicationDepth) -> &'static str {
    match depth {
        ReplicationDepth::None => "none",
        ReplicationDepth::StrictIngest => "strict_ingest",
        ReplicationDepth::ExecutorEnqueue => "executor_enqueue",
        ReplicationDepth::QuorumGate => "quorum_gate",
        ReplicationDepth::RateCap => "rate_cap",
        ReplicationDepth::FullProduction => "full_production",
    }
}

/// Runtime replication depth from in-process counters.
pub fn current_replication_depth() -> ReplicationDepth {
    let cap = replication_max_per_hour_from_env();
    replication_depth_stub(Some(&replication_metrics_snapshot()), cap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_replication_metrics::reset_replication_strict_metrics_for_test;

    #[test]
    fn replication_depth_stub_ph_s894() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        reset_replication_strict_metrics_for_test();

        let empty = ReplicationMetricsSnapshot {
            strict_total: 0,
            enqueue_total: 0,
            executor_enqueue_total: 0,
            rate_limited_total: 0,
        };
        assert_eq!(replication_depth_stub(None, 1000), ReplicationDepth::None);
        assert_eq!(
            replication_depth_stub(Some(&empty), 1000),
            ReplicationDepth::None
        );

        let strict_only = ReplicationMetricsSnapshot {
            strict_total: 1,
            ..empty
        };
        assert_eq!(
            replication_depth_stub(Some(&strict_only), 1000),
            ReplicationDepth::StrictIngest
        );

        let executor = ReplicationMetricsSnapshot {
            strict_total: 1,
            enqueue_total: 1,
            executor_enqueue_total: 2,
            ..empty
        };
        assert_eq!(
            replication_depth_stub(Some(&executor), 1000),
            ReplicationDepth::ExecutorEnqueue
        );

        let quorum = ReplicationMetricsSnapshot {
            strict_total: 2,
            enqueue_total: 3,
            executor_enqueue_total: 2,
            ..empty
        };
        assert_eq!(
            replication_depth_stub(Some(&quorum), 1000),
            ReplicationDepth::QuorumGate
        );

        let rate_cap = ReplicationMetricsSnapshot {
            strict_total: 1,
            enqueue_total: 1,
            executor_enqueue_total: 1,
            rate_limited_total: 1,
        };
        assert_eq!(
            replication_depth_stub(Some(&rate_cap), 100),
            ReplicationDepth::RateCap
        );

        let full = ReplicationMetricsSnapshot {
            strict_total: 2,
            enqueue_total: 3,
            executor_enqueue_total: 3,
            rate_limited_total: 1,
        };
        assert_eq!(
            replication_depth_stub(Some(&full), 100),
            ReplicationDepth::FullProduction
        );
        assert_eq!(
            replication_depth_wire_label(ReplicationDepth::FullProduction),
            "full_production"
        );

        reset_replication_strict_metrics_for_test();
    }
}
