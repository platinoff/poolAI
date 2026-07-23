//! PH-S1341: SSO horizon-close contracts (band 70).
//! Marker: sso_horizon_integration
//!
//! Verifies phase-B SSO slices + horizon criteria totals.

use poolai_ui_core::sso_horizon_depth::{
    sso_horizon_criteria_total, sso_horizon_depth_stub, sso_horizon_slices_met, SsoHorizonDepth,
    SSO_HORIZON_CASES, SSO_HORIZON_CRITERIA, SSO_HORIZON_SLICES,
};
use poolai_ui_core::sso_ratio_advisory_depth::sso_ratio_advisory_criteria_total;
use serde_json::json;

#[test]
fn sso_horizon_depth_registry_ph_s1339() {
    assert_eq!(SSO_HORIZON_CRITERIA.len(), 10);
    assert_eq!(sso_horizon_criteria_total(), 10);
    assert!(SSO_HORIZON_CASES.contains(&"aggregate_flag"));
    assert!(SSO_HORIZON_CASES.contains(&"phase_b_slices"));
    assert_eq!(
        sso_horizon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        SsoHorizonDepth::SliceAggregate
    );
}

#[test]
fn sso_horizon_slice_docs_present_ph_s1340() {
    let canon = include_str!("../docs/development/SSO_HORIZON.md");
    let (met, total) = sso_horizon_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all horizon slices must be listed");
    for name in SSO_HORIZON_SLICES {
        assert!(canon.contains(name), "missing horizon slice {name}");
    }
    assert!(std::path::Path::new("docs/development/SSO_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/SSO_STORE.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--sso-horizon"));
    assert!(loc_audit.contains("--sso-ratio-advisory"));
}

#[test]
fn sso_horizon_criteria_totals_consistent_ph_s1341() {
    assert_eq!(sso_ratio_advisory_criteria_total(), 10);
    assert_eq!(sso_horizon_criteria_total(), 10);
    assert_eq!(SSO_HORIZON_SLICES.len(), 10);

    assert_eq!(
        sso_horizon_depth_stub(Some(&json!({
            "sso_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoHorizonDepth::FullBand70
    );
}
