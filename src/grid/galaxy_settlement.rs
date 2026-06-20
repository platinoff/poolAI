//! Galaxy Grid settlement status stub (PH-S170, §6.3–6.5).
//!
//! Derives `pending_verification` on the grid result path from trust gate + verification sampling.

use crate::grid::galaxy_trust_score::SettlementGateVerdict;
use crate::grid::galaxy_verify_sampling::VerifySamplingVerdict;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    /// Worker/operator pool remainder after fee split (PH-S616, Galaxy §8.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_lamports: Option<u64>,
    /// Resolved Solana wallet for offline batch payout (PH-S538, Galaxy §8.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payout_pubkey: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_user_id: Option<String>,
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
            worker_lamports: None,
            payout_pubkey: None,
            telegram_user_id: None,
        }
    }
}

/// Resolve payout wallet from telegram user id (PH-S538).
pub fn resolve_payout_pubkey(telegram_user_id: Option<&str>) -> Option<String> {
    let uid = telegram_user_id?.trim();
    if uid.is_empty() {
        return None;
    }
    crate::services::virtual_node_telegram_wallet_service::VirtualNodeTelegramWalletService::lookup(
        uid,
    )
    .map(|b| b.payout_pubkey)
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
    if verification_sample == VerifySamplingVerdict::SampleScheduled
        || verification_sample == VerifySamplingVerdict::VerificationInconclusive
    {
        return SettlementStatus::PendingVerification;
    }
    match settlement_gate {
        SettlementGateVerdict::NotApplicable => SettlementStatus::NotApplicable,
        SettlementGateVerdict::PayoutEligible => SettlementStatus::Cleared,
        SettlementGateVerdict::PayoutHeld => SettlementStatus::PendingVerification,
    }
}

/// Settlement depth hint from grid result metrics (PH-S684, Galaxy §6.4–§6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementGateDepth {
    NotApplicable,
    Cleared,
    PendingVerification,
}

/// Classify settlement depth from grid result metrics stub fields (PH-S684).
pub fn settlement_gate_depth_stub(metrics: Option<&Value>) -> SettlementGateDepth {
    let settlement_gate = metrics
        .and_then(|m| m.get("settlement_gate_verdict"))
        .and_then(|v| v.as_str())
        .map(parse_settlement_gate_verdict)
        .unwrap_or(SettlementGateVerdict::NotApplicable);
    let verification_sample = metrics
        .and_then(|m| m.get("verification_sample"))
        .and_then(|v| v.as_str())
        .map(parse_verification_sample)
        .unwrap_or(VerifySamplingVerdict::NotApplicable);
    match resolve_settlement_status(settlement_gate, verification_sample) {
        SettlementStatus::NotApplicable => SettlementGateDepth::NotApplicable,
        SettlementStatus::Cleared => SettlementGateDepth::Cleared,
        SettlementStatus::PendingVerification => SettlementGateDepth::PendingVerification,
    }
}

fn parse_settlement_gate_verdict(raw: &str) -> SettlementGateVerdict {
    match raw.to_ascii_lowercase().as_str() {
        "payout_eligible" | "eligible" => SettlementGateVerdict::PayoutEligible,
        "payout_held" | "held" => SettlementGateVerdict::PayoutHeld,
        _ => SettlementGateVerdict::NotApplicable,
    }
}

fn parse_verification_sample(raw: &str) -> VerifySamplingVerdict {
    match raw.to_ascii_lowercase().as_str() {
        "sample_scheduled" | "scheduled" => VerifySamplingVerdict::SampleScheduled,
        "verification_inconclusive" | "inconclusive" => {
            VerifySamplingVerdict::VerificationInconclusive
        }
        "not_selected" => VerifySamplingVerdict::NotSelected,
        _ => VerifySamplingVerdict::NotApplicable,
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

    #[test]
    fn settlement_gate_depth_stub_ph_s684() {
        use serde_json::json;

        assert_eq!(
            settlement_gate_depth_stub(Some(&json!({
                "settlement_gate_verdict": "payout_held",
                "verification_sample": "not_selected",
            }))),
            SettlementGateDepth::PendingVerification
        );
        assert_eq!(
            settlement_gate_depth_stub(Some(&json!({
                "settlement_gate_verdict": "payout_eligible",
                "verification_sample": "not_selected",
            }))),
            SettlementGateDepth::Cleared
        );
    }
}
