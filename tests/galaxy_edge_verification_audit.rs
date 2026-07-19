//! PH-S1121: Galaxy edge verification horizon audit — criteria registry + maintenance markers.

use poolai_ui_core::galaxy_edge_verification_depth::{
    edge_verification_criteria_total, galaxy_edge_verification_depth_stub,
    GalaxyEdgeVerificationDepth, EDGE_VERIFICATION_BAND48_ROWS, EDGE_VERIFICATION_CASES,
    EDGE_VERIFICATION_CRITERIA, FM_BAND48_ROWS,
};
use serde_json::json;

#[test]
fn galaxy_edge_verification_audit_ph_s1121() {
    assert_eq!(
        galaxy_edge_verification_depth_stub(Some(&json!({"capability_admission": true}))),
        GalaxyEdgeVerificationDepth::CapabilityAdmission
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
    assert!(EDGE_VERIFICATION_CASES.contains(&"metrics_http"));
    assert!(EDGE_VERIFICATION_CASES.contains(&"openapi_wire"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND48_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in EDGE_VERIFICATION_BAND48_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-48 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = EDGE_VERIFICATION_CRITERIA
        .iter()
        .map(|(id, _, _)| *id)
        .collect();
    assert!(criteria_ids.contains(&"fraud_proof_stub"));
    assert!(criteria_ids.contains(&"tee_attestation"));
}
