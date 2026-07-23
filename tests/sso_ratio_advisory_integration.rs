//! PH-S1331: SSO ratio-advisory contracts (band 69).
//! Marker: sso_ratio_advisory_integration
//!
//! Verifies prior SSO + vision-sync slices are present and criteria totals are consistent.

use poolai_ui_core::sso_ratio_advisory_depth::{
    sso_ratio_advisory_criteria_total, sso_ratio_advisory_depth_stub,
    sso_ratio_advisory_slices_met, SsoRatioAdvisoryDepth, SSO_RATIO_ADVISORY_CASES,
    SSO_RATIO_ADVISORY_CRITERIA, SSO_RATIO_ADVISORY_SLICES,
};
use poolai_ui_core::sso_vision_sync_depth::sso_vision_sync_criteria_total;
use serde_json::json;

#[test]
fn sso_ratio_advisory_depth_registry_ph_s1329() {
    assert_eq!(SSO_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(sso_ratio_advisory_criteria_total(), 10);
    assert!(SSO_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
    assert!(SSO_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(
        sso_ratio_advisory_depth_stub(Some(&json!({"slice_aggregate": true}))),
        SsoRatioAdvisoryDepth::SliceAggregate
    );
}

#[test]
fn sso_ratio_advisory_slice_docs_present_ph_s1330() {
    let canon = include_str!("../docs/development/SSO_RATIO_ADVISORY.md");
    let (met, total) = sso_ratio_advisory_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all ratio-advisory slices must be listed");
    for name in SSO_RATIO_ADVISORY_SLICES {
        assert!(canon.contains(name), "missing ratio-advisory slice {name}");
    }
    assert!(std::path::Path::new("docs/development/SSO_VISION_SYNC.md").exists());
    assert!(std::path::Path::new("docs/development/SSO_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--sso-ratio-advisory"));
    assert!(loc_audit.contains("--sso-vision-sync"));
}

#[test]
fn sso_ratio_advisory_criteria_totals_consistent_ph_s1331() {
    assert_eq!(sso_vision_sync_criteria_total(), 10);
    assert_eq!(sso_ratio_advisory_criteria_total(), 10);
    assert_eq!(SSO_RATIO_ADVISORY_SLICES.len(), 6);

    assert_eq!(
        sso_ratio_advisory_depth_stub(Some(&json!({
            "sso_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoRatioAdvisoryDepth::FullBand69
    );
}
