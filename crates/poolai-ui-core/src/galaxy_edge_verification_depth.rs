//! Galaxy edge verification horizon band depth (PH-S1119…S1128, band 48).

use serde_json::Value;

/// Edge verification horizon depth flags (§6.6 fraud-proof / capability / TEE wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalaxyEdgeVerificationDepth {
    None,
    FraudProofStub,
    CapabilityAdmission,
    NetworkProfileStale,
    TeeAttestation,
    MetricsHttp,
    StandSmokeParity,
    FullBand48,
}

/// Edge verification maintenance criteria registry (PH-S1121): id · marker · doc path.
pub const EDGE_VERIFICATION_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "fraud_proof_stub",
        "galaxy_fraud_proof_pending_total",
        "src/grid/galaxy_fraud_proof.rs",
    ),
    (
        "capability_admission",
        "galaxy_capability_unsigned_rejected_total",
        "src/grid/galaxy_capability_admission_metrics.rs",
    ),
    (
        "network_profile_stale",
        "galaxy_network_profile_stale_total",
        "src/grid/galaxy_locality.rs",
    ),
    (
        "tee_attestation",
        "POOLAI_TEE_ATTEST_REQUIRED",
        "src/grid/galaxy_capability_doc.rs",
    ),
    (
        "metrics_http",
        "edge-verification-metrics",
        "src/network/api/grid.rs",
    ),
    (
        "stand_smoke_parity",
        "validate_band6_metrics_parity_v4",
        "src/grid/stand_smoke_metrics_parity.rs",
    ),
    (
        "openapi_wire",
        "getGridEdgeVerificationMetrics",
        "docs/openapi.yaml",
    ),
];

/// `poolai-loc-audit --edge-verification-advisory` case names (PH-S1120).
pub const EDGE_VERIFICATION_CASES: &[&str] = &[
    "fraud_proof_stub",
    "capability_admission",
    "network_profile_stale",
    "tee_attestation",
    "metrics_http",
    "stand_smoke_parity",
    "openapi_wire",
];

/// FM §5.29 band-48 marker rows.
pub const FM_BAND48_ROWS: &[&str] = &[
    "5.29",
    "Galaxy edge verification horizon",
    "PH-S1119…S1128",
    "galaxy_edge_verification_depth",
];

/// Edge verification adoption markers for band 48.
pub const EDGE_VERIFICATION_BAND48_ROWS: &[&str] = &[
    "PH-S1119",
    "galaxy_edge_verification_depth",
    "PH-S1120",
    "--edge-verification-advisory",
    "PH-S1125",
    "VERIFY_EDGE_VERIFICATION",
    "PH-S1125",
    "--edge-verification",
    "PH-S1128",
];

/// Classify edge verification band depth from optional feature stub (PH-S1119).
pub fn galaxy_edge_verification_depth_stub(
    features: Option<&Value>,
) -> GalaxyEdgeVerificationDepth {
    let Some(f) = features else {
        return GalaxyEdgeVerificationDepth::None;
    };
    let fraud = f
        .get("fraud_proof_stub")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let capability = f
        .get("capability_admission")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let network = f
        .get("network_profile_stale")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tee = f
        .get("tee_attestation")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let http = f
        .get("metrics_http")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let parity = f
        .get("stand_smoke_parity")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if fraud && capability && network && tee && http && parity {
        return GalaxyEdgeVerificationDepth::FullBand48;
    }
    if parity {
        return GalaxyEdgeVerificationDepth::StandSmokeParity;
    }
    if http {
        return GalaxyEdgeVerificationDepth::MetricsHttp;
    }
    if tee {
        return GalaxyEdgeVerificationDepth::TeeAttestation;
    }
    if network {
        return GalaxyEdgeVerificationDepth::NetworkProfileStale;
    }
    if capability {
        return GalaxyEdgeVerificationDepth::CapabilityAdmission;
    }
    if fraud {
        return GalaxyEdgeVerificationDepth::FraudProofStub;
    }
    GalaxyEdgeVerificationDepth::None
}

/// Total edge verification criteria in registry (PH-S1121).
pub fn edge_verification_criteria_total() -> usize {
    EDGE_VERIFICATION_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn galaxy_edge_verification_depth_stub_ph_s1119() {
        assert_eq!(
            galaxy_edge_verification_depth_stub(None),
            GalaxyEdgeVerificationDepth::None
        );
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({"fraud_proof_stub": true}))),
            GalaxyEdgeVerificationDepth::FraudProofStub
        );
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({
                "fraud_proof_stub": true,
                "capability_admission": true,
                "network_profile_stale": true,
                "tee_attestation": true,
                "metrics_http": true,
                "stand_smoke_parity": true,
            }))),
            GalaxyEdgeVerificationDepth::FullBand48
        );
        assert_eq!(EDGE_VERIFICATION_CRITERIA.len(), 7);
        assert_eq!(edge_verification_criteria_total(), 7);
        assert!(!EDGE_VERIFICATION_CASES.is_empty());
        assert!(FM_BAND48_ROWS.contains(&"PH-S1119…S1128"));
    }
}
