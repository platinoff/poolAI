//! Galaxy governance depth classification stub (PH-S794, §9.5–9.6).

use crate::grid::galaxy_governance_metrics::GovernanceMetricsSnapshot;
use crate::grid::galaxy_update_policy::{update_policy_from_env, UpdatePolicyMode};

/// Governance wire depth (Galaxy §9.5–9.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceDepth {
    None,
    PolicyConfigured,
    MetricsLive,
    FullDepth,
}

/// Classify governance depth from optional metrics snapshot (PH-S794).
pub fn governance_depth_stub(snapshot: Option<&GovernanceMetricsSnapshot>) -> GovernanceDepth {
    let Some(s) = snapshot else {
        return if update_policy_from_env() == UpdatePolicyMode::Notify {
            GovernanceDepth::PolicyConfigured
        } else {
            GovernanceDepth::None
        };
    };
    let has_verify = s.release_verify_total > 0 || s.release_verify_fail_total > 0;
    let has_notify = s.update_notify_pending > 0;
    let has_advisory = s.advisory_acknowledged_total > 0;
    if has_verify && (has_notify || has_advisory) {
        GovernanceDepth::FullDepth
    } else if has_verify || has_notify || has_advisory {
        GovernanceDepth::MetricsLive
    } else if update_policy_from_env() != UpdatePolicyMode::Never {
        GovernanceDepth::PolicyConfigured
    } else {
        GovernanceDepth::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_governance_metrics::{
        governance_metrics_snapshot, record_release_verify_success,
        reset_governance_metrics_for_test, set_update_notify_pending,
    };
    use crate::grid::galaxy_security_advisory::{
        acknowledge_security_advisory, reset_security_advisory_for_test,
    };

    #[test]
    fn governance_depth_stub_ph_s794() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_governance_metrics_for_test();
        reset_security_advisory_for_test();
        std::env::set_var(
            crate::grid::galaxy_update_policy::ENV_UPDATE_POLICY,
            "notify",
        );

        assert_eq!(
            governance_depth_stub(None),
            GovernanceDepth::PolicyConfigured
        );

        record_release_verify_success();
        let live = governance_metrics_snapshot();
        assert_eq!(
            governance_depth_stub(Some(&live)),
            GovernanceDepth::MetricsLive
        );

        set_update_notify_pending(1);
        acknowledge_security_advisory("CVE-2026-0001");
        let full = governance_metrics_snapshot();
        assert_eq!(
            governance_depth_stub(Some(&full)),
            GovernanceDepth::FullDepth
        );

        std::env::remove_var(crate::grid::galaxy_update_policy::ENV_UPDATE_POLICY);
        reset_governance_metrics_for_test();
        reset_security_advisory_for_test();
    }
}
