//! Galaxy edge verification metrics snapshot (PH-S1122, Galaxy §6.6).

use crate::grid::galaxy_capability_admission_metrics::{
    capability_signed_accepted_total, capability_unsigned_rejected_total,
};
use crate::grid::galaxy_capability_doc::tee_attestation_required;
use crate::grid::galaxy_fraud_proof::fraud_proof_pending_total;
use crate::grid::galaxy_locality::network_profile_stale_total;

/// Read-only edge verification snapshot for `GET /api/v1/grid/edge-verification-metrics`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct EdgeVerificationMetricsSnapshot {
    pub fraud_proof_pending: u64,
    pub capability_unsigned_rejected: u64,
    pub capability_signed_accepted: u64,
    pub network_profile_stale: u64,
    pub tee_attestation_required: bool,
}

/// Coordinator edge verification metrics snapshot (PH-S1122).
pub fn edge_verification_metrics_snapshot() -> EdgeVerificationMetricsSnapshot {
    EdgeVerificationMetricsSnapshot {
        fraud_proof_pending: fraud_proof_pending_total(),
        capability_unsigned_rejected: capability_unsigned_rejected_total(),
        capability_signed_accepted: capability_signed_accepted_total(),
        network_profile_stale: network_profile_stale_total(),
        tee_attestation_required: tee_attestation_required(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_capability_admission_metrics::{
        record_capability_signed_accepted, record_capability_unsigned_rejected,
        reset_capability_admission_metrics_for_test,
    };
    use crate::grid::galaxy_fraud_proof::{
        record_fraud_proof_pending, reset_fraud_proof_metrics_for_test,
    };
    use crate::grid::galaxy_locality::reset_network_profile_stale_metrics_for_test;

    #[test]
    fn edge_verification_metrics_snapshot_ph_s1122() {
        reset_fraud_proof_metrics_for_test();
        reset_capability_admission_metrics_for_test();
        reset_network_profile_stale_metrics_for_test();
        record_fraud_proof_pending();
        record_capability_unsigned_rejected();
        record_capability_signed_accepted();
        let snap = edge_verification_metrics_snapshot();
        assert_eq!(snap.fraud_proof_pending, 1);
        assert_eq!(snap.capability_unsigned_rejected, 1);
        assert_eq!(snap.capability_signed_accepted, 1);
        reset_fraud_proof_metrics_for_test();
        reset_capability_admission_metrics_for_test();
        reset_network_profile_stale_metrics_for_test();
    }
}
