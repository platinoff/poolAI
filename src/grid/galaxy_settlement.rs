//! Galaxy Grid settlement status stub (PH-S170, §6.3–6.5).
//!
//! Derives `pending_verification` on the grid result path from trust gate + verification sampling.

use crate::grid::galaxy_trust_score::SettlementGateVerdict;
use crate::grid::galaxy_verify_sampling::VerifySamplingVerdict;
use serde::{Deserialize, Serialize};

/// Offline payout batch ledger entry stub (PH-S436, Galaxy §8.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutBatchLedgerEntry {
    pub job_id: String,
    pub cleared_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_usd_micro: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_lamports: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_dev_lamports: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_admin_lamports: Option<u64>,
}

impl PayoutBatchLedgerEntry {
    /// Ledger row with only required fields (fee-split fields optional).
    pub fn minimal(job_id: impl Into<String>, cleared_at: impl Into<String>) -> Self {
        Self {
            job_id: job_id.into(),
            cleared_at: cleared_at.into(),
            gross_usd_micro: None,
            gross_lamports: None,
            primary_dev_lamports: None,
            secondary_admin_lamports: None,
        }
    }
}

/// Coordinator settlement status on grid result ingest (stub — no payout wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementStatus {
    /// Gate not applied (`local_srv` origin).
    NotApplicable,
    /// Settlement may proceed (edge trust gate passed, no verification hold).
    Cleared,
    /// Settlement held pending verification (§6.3 replay / trust hold / sampled verification).
    PendingVerification,
}

/// Resolve settlement status from trust gate and verification sampling verdicts (PH-S170).
#[inline]
pub fn resolve_settlement_status(
    settlement_gate: SettlementGateVerdict,
    verification_sample: VerifySamplingVerdict,
) -> SettlementStatus {
    if settlement_gate == SettlementGateVerdict::PayoutHeld {
        return SettlementStatus::PendingVerification;
    }
    if verification_sample == VerifySamplingVerdict::SampleScheduled {
        return SettlementStatus::PendingVerification;
    }
    match settlement_gate {
        SettlementGateVerdict::NotApplicable => SettlementStatus::NotApplicable,
        SettlementGateVerdict::PayoutEligible => SettlementStatus::Cleared,
        SettlementGateVerdict::PayoutHeld => SettlementStatus::PendingVerification,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_settlement_status_local_not_applicable() {
        assert_eq!(
            resolve_settlement_status(
                SettlementGateVerdict::NotApplicable,
                VerifySamplingVerdict::NotApplicable,
            ),
            SettlementStatus::NotApplicable
        );
    }

    #[test]
    fn resolve_settlement_status_edge_cleared() {
        assert_eq!(
            resolve_settlement_status(
                SettlementGateVerdict::PayoutEligible,
                VerifySamplingVerdict::NotSelected,
            ),
            SettlementStatus::Cleared
        );
    }

    #[test]
    fn resolve_settlement_status_trust_hold_pending_ph_s170() {
        assert_eq!(
            resolve_settlement_status(
                SettlementGateVerdict::PayoutHeld,
                VerifySamplingVerdict::NotSelected,
            ),
            SettlementStatus::PendingVerification
        );
    }

    #[test]
    fn resolve_settlement_status_sample_scheduled_pending_ph_s170() {
        assert_eq!(
            resolve_settlement_status(
                SettlementGateVerdict::PayoutEligible,
                VerifySamplingVerdict::SampleScheduled,
            ),
            SettlementStatus::PendingVerification
        );
    }
}
