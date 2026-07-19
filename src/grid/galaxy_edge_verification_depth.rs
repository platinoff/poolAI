//! Galaxy edge verification horizon depth wire stub (PH-S1123).

use crate::grid::galaxy_edge_verification_metrics::{
    edge_verification_metrics_snapshot, EdgeVerificationMetricsSnapshot,
};

/// Edge verification horizon wire depth (Galaxy §6.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeVerificationHorizonDepth {
    None,
    FraudProofOnly,
    CapabilityWire,
    NetworkProfileAdvisory,
    FullHorizon,
}

/// Classify edge verification horizon depth from metrics snapshot (PH-S1123).
pub fn edge_verification_horizon_depth_stub(
    snapshot: Option<&EdgeVerificationMetricsSnapshot>,
) -> EdgeVerificationHorizonDepth {
    let Some(s) = snapshot else {
        return EdgeVerificationHorizonDepth::None;
    };
    let has_fraud = s.fraud_proof_pending > 0;
    let has_capability = s.capability_unsigned_rejected > 0 || s.capability_signed_accepted > 0;
    let has_network = s.network_profile_stale > 0;
    let tee = s.tee_attestation_required;

    if has_fraud && has_capability && (has_network || tee) {
        EdgeVerificationHorizonDepth::FullHorizon
    } else if has_network || tee {
        EdgeVerificationHorizonDepth::NetworkProfileAdvisory
    } else if has_capability {
        EdgeVerificationHorizonDepth::CapabilityWire
    } else if has_fraud {
        EdgeVerificationHorizonDepth::FraudProofOnly
    } else {
        EdgeVerificationHorizonDepth::None
    }
}

/// Wire label for edge-verification-metrics / stand smoke (PH-S1123).
pub fn edge_verification_horizon_depth_wire_label(
    depth: EdgeVerificationHorizonDepth,
) -> &'static str {
    match depth {
        EdgeVerificationHorizonDepth::None => "none",
        EdgeVerificationHorizonDepth::FraudProofOnly => "fraud_proof_only",
        EdgeVerificationHorizonDepth::CapabilityWire => "capability_wire",
        EdgeVerificationHorizonDepth::NetworkProfileAdvisory => "network_profile_advisory",
        EdgeVerificationHorizonDepth::FullHorizon => "full_horizon",
    }
}

/// Runtime edge verification horizon depth from in-process counters.
pub fn current_edge_verification_horizon_depth() -> EdgeVerificationHorizonDepth {
    edge_verification_horizon_depth_stub(Some(&edge_verification_metrics_snapshot()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_verification_horizon_depth_stub_ph_s1123() {
        assert_eq!(
            edge_verification_horizon_depth_stub(None),
            EdgeVerificationHorizonDepth::None
        );
        let snap = EdgeVerificationMetricsSnapshot {
            fraud_proof_pending: 1,
            capability_unsigned_rejected: 0,
            capability_signed_accepted: 0,
            network_profile_stale: 0,
            tee_attestation_required: false,
        };
        assert_eq!(
            edge_verification_horizon_depth_stub(Some(&snap)),
            EdgeVerificationHorizonDepth::FraudProofOnly
        );
        assert_eq!(
            edge_verification_horizon_depth_wire_label(EdgeVerificationHorizonDepth::FullHorizon),
            "full_horizon"
        );
    }
}
