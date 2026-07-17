//! FUNCTIONALITY_DIGEST band depth classification (PH-S950, band 30).

use serde_json::Value;

/// Digest sync band depth flags (grid / job / ui-wasm / bins).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestDepth {
    None,
    Grid,
    Job,
    UiWasm,
    Bins,
    FullDigest,
}

/// Canonical `src/grid/` module stems from `grid::mod.rs` (PH-S950 inventory).
pub const GRID_MODULE_STEMS: &[&str] = &[
    "dispatch",
    "envelope",
    "galaxy_capability_admission",
    "galaxy_capability_admission_depth",
    "galaxy_capability_admission_metrics",
    "galaxy_capability_doc",
    "galaxy_fee_split",
    "galaxy_fee_split_depth",
    "galaxy_fee_split_metrics",
    "galaxy_fraud_proof",
    "galaxy_governance_depth",
    "galaxy_governance_metrics",
    "galaxy_locality",
    "galaxy_locality_hot_tier_depth",
    "galaxy_locality_metrics",
    "galaxy_network_profile",
    "galaxy_network_profile_depth",
    "galaxy_network_profile_store",
    "galaxy_prefetch_depth",
    "galaxy_prefetch_metrics",
    "galaxy_prefetch_peer_pull",
    "galaxy_pricing_depth",
    "galaxy_pricing_metrics",
    "galaxy_pricing_oracle",
    "galaxy_pricing_provider_metrics",
    "galaxy_protocol_negotiation_metrics",
    "galaxy_re_migrate_policy",
    "galaxy_replay_jobs",
    "galaxy_replay_metrics",
    "galaxy_replication",
    "galaxy_replication_depth",
    "galaxy_replication_metrics",
    "galaxy_replication_quorum_gate",
    "galaxy_routing_policy",
    "galaxy_security_advisory",
    "galaxy_settlement",
    "galaxy_settlement_metrics",
    "galaxy_settlement_mode",
    "galaxy_settlement_onchain",
    "galaxy_settlement_onchain_depth",
    "galaxy_settlement_payout_batch_queue",
    "galaxy_settlement_payout_depth",
    "galaxy_settlement_payout_metrics",
    "galaxy_trust_persist_depth",
    "galaxy_trust_score",
    "galaxy_trust_score_store",
    "galaxy_trust_score_store_sqlite",
    "galaxy_update_policy",
    "galaxy_verification_checker_jobs",
    "galaxy_verification_lifecycle_depth",
    "galaxy_verification_metrics",
    "galaxy_verification_replay",
    "galaxy_verify_sampling",
    "galaxy_worker_dto",
    "galaxy_worker_health",
    "map",
    "protocol_compat",
    "solana_depth",
    "stand_smoke_metrics_parity",
];

/// Canonical `src/job/` module stems from `job::mod.rs` (PH-S951 inventory).
pub const JOB_MODULE_STEMS: &[&str] = &[
    "domain_events",
    "lease_acquire",
    "lease_config",
    "lease_failover",
    "lifecycle",
    "map",
    "mod",
    "onchain",
    "scheduler",
    "store",
    "store_depth",
    "store_sqlite",
    "types",
];

/// Classify digest band depth from optional feature stub (PH-S950).
pub fn digest_depth_stub(features: Option<&Value>) -> DigestDepth {
    let Some(f) = features else {
        return DigestDepth::None;
    };
    let grid = f
        .get("grid_digest")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let job = f
        .get("job_digest")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ui = f
        .get("ui_wasm_digest")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let bins = f
        .get("bins_digest")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = grid as u8 + job as u8 + ui as u8 + bins as u8;
    match flags {
        0 => DigestDepth::None,
        1 if grid => DigestDepth::Grid,
        1 if job => DigestDepth::Job,
        1 if ui => DigestDepth::UiWasm,
        1 if bins => DigestDepth::Bins,
        4 => DigestDepth::FullDigest,
        _ => DigestDepth::FullDigest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn digest_depth_stub_ph_s950() {
        assert_eq!(digest_depth_stub(None), DigestDepth::None);
        assert_eq!(
            digest_depth_stub(Some(&json!({"grid_digest": true}))),
            DigestDepth::Grid
        );
        assert_eq!(
            digest_depth_stub(Some(&json!({
                "grid_digest": true,
                "job_digest": true,
                "ui_wasm_digest": true,
                "bins_digest": true
            }))),
            DigestDepth::FullDigest
        );
        assert_eq!(GRID_MODULE_STEMS.len(), 57);
        assert_eq!(JOB_MODULE_STEMS.len(), 13);
    }
}
