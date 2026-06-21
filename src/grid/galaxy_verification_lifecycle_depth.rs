//! Galaxy verification checker lifecycle depth classification (PH-S884, §6.2).

use crate::grid::galaxy_verification_checker_jobs::verification_checker_job_submit_total;
use crate::grid::galaxy_verification_metrics::{
    verification_metrics_snapshot, VerificationMetricsSnapshot,
};

/// Verification checker lifecycle wire depth (Galaxy §6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationLifecycleDepth {
    None,
    EnqueueStub,
    TaskQueue,
    ShadowJobSubmit,
    DrainOnVerdict,
    FullLifecycle,
}

/// Classify verification lifecycle depth from metrics snapshot + shadow submit counter (PH-S884).
pub fn verification_lifecycle_depth_stub(
    snapshot: Option<&VerificationMetricsSnapshot>,
    checker_job_submit_total: u64,
) -> VerificationLifecycleDepth {
    let Some(s) = snapshot else {
        return VerificationLifecycleDepth::None;
    };
    let has_enqueue = s.checker_enqueue_total > 0;
    let has_pending = s.checker_pending_total > 0;
    let has_shadow = checker_job_submit_total > 0;
    let has_verdict = s.match_total > 0 || s.mismatch_total > 0;

    if has_enqueue && has_shadow && has_verdict && !has_pending {
        VerificationLifecycleDepth::FullLifecycle
    } else if has_verdict && has_enqueue {
        VerificationLifecycleDepth::DrainOnVerdict
    } else if has_shadow && (has_pending || has_enqueue) {
        VerificationLifecycleDepth::ShadowJobSubmit
    } else if has_pending {
        VerificationLifecycleDepth::TaskQueue
    } else if has_enqueue {
        VerificationLifecycleDepth::EnqueueStub
    } else {
        VerificationLifecycleDepth::None
    }
}

/// Wire label for verification-metrics / stand smoke (PH-S884).
pub fn verification_lifecycle_depth_wire_label(depth: VerificationLifecycleDepth) -> &'static str {
    match depth {
        VerificationLifecycleDepth::None => "none",
        VerificationLifecycleDepth::EnqueueStub => "enqueue_stub",
        VerificationLifecycleDepth::TaskQueue => "task_queue",
        VerificationLifecycleDepth::ShadowJobSubmit => "shadow_job_submit",
        VerificationLifecycleDepth::DrainOnVerdict => "drain_on_verdict",
        VerificationLifecycleDepth::FullLifecycle => "full_lifecycle",
    }
}

/// Runtime verification lifecycle depth from in-process counters.
pub fn current_verification_lifecycle_depth() -> VerificationLifecycleDepth {
    let snap = verification_metrics_snapshot();
    verification_lifecycle_depth_stub(Some(&snap), verification_checker_job_submit_total())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_verification_checker_jobs::reset_verification_checker_job_submit_for_test;
    use crate::grid::galaxy_verification_metrics::reset_verification_metrics_for_test;

    #[test]
    fn verification_lifecycle_depth_stub_ph_s884() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        reset_verification_metrics_for_test();
        reset_verification_checker_job_submit_for_test();

        assert_eq!(
            verification_lifecycle_depth_stub(None, 0),
            VerificationLifecycleDepth::None
        );

        let empty = VerificationMetricsSnapshot {
            sample_total: 0,
            mismatch_total: 0,
            match_total: 0,
            sample_completed_total: 0,
            checker_enqueue_total: 0,
            checker_pending_total: 0,
        };
        let enqueue_only = VerificationMetricsSnapshot {
            checker_enqueue_total: 1,
            ..empty
        };
        assert_eq!(
            verification_lifecycle_depth_stub(Some(&enqueue_only), 0),
            VerificationLifecycleDepth::EnqueueStub
        );

        let queued = VerificationMetricsSnapshot {
            checker_enqueue_total: 1,
            checker_pending_total: 2,
            ..empty
        };
        assert_eq!(
            verification_lifecycle_depth_stub(Some(&queued), 0),
            VerificationLifecycleDepth::TaskQueue
        );

        let shadow = VerificationMetricsSnapshot {
            checker_enqueue_total: 1,
            checker_pending_total: 1,
            ..empty
        };
        assert_eq!(
            verification_lifecycle_depth_stub(Some(&shadow), 1),
            VerificationLifecycleDepth::ShadowJobSubmit
        );

        let drained = VerificationMetricsSnapshot {
            checker_enqueue_total: 2,
            match_total: 1,
            checker_pending_total: 0,
            ..empty
        };
        assert_eq!(
            verification_lifecycle_depth_stub(Some(&drained), 1),
            VerificationLifecycleDepth::FullLifecycle
        );
        assert_eq!(
            verification_lifecycle_depth_wire_label(VerificationLifecycleDepth::FullLifecycle),
            "full_lifecycle"
        );

        reset_verification_metrics_for_test();
        reset_verification_checker_job_submit_for_test();
    }
}
