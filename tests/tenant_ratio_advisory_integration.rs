//! PH-S1231: Tenant ratio-advisory contracts (band 59).
//! Marker: tenant_ratio_advisory_integration
//!
//! Verifies prior tenant slices + sqlite CRUD markers and criteria totals.

use poolai_ui_core::tenant_ratio_advisory_depth::{
    tenant_ratio_advisory_criteria_total, tenant_ratio_advisory_depth_stub,
    tenant_ratio_advisory_slices_met, TenantRatioAdvisoryDepth, TENANT_RATIO_ADVISORY_CASES,
    TENANT_RATIO_ADVISORY_CRITERIA, TENANT_RATIO_ADVISORY_SLICES,
};
use poolai_ui_core::tenant_vision_sync_depth::tenant_vision_sync_criteria_total;
use serde_json::json;

#[test]
fn tenant_ratio_advisory_depth_registry_ph_s1229() {
    assert_eq!(TENANT_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
    assert!(TENANT_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
    assert!(TENANT_RATIO_ADVISORY_CASES.contains(&"sqlite_restart_safe"));
    assert_eq!(
        tenant_ratio_advisory_depth_stub(Some(&json!({"slice_aggregate": true}))),
        TenantRatioAdvisoryDepth::SliceAggregate
    );
}

#[test]
fn tenant_ratio_advisory_slice_docs_present_ph_s1230() {
    let canon = include_str!("../docs/development/TENANT_RATIO_ADVISORY.md");
    let (met, total) = tenant_ratio_advisory_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all ratio-advisory slices must be listed");
    for name in TENANT_RATIO_ADVISORY_SLICES {
        assert!(canon.contains(name), "missing ratio-advisory slice {name}");
    }
    assert!(std::path::Path::new("docs/development/TENANT_STORE.md").exists());
    assert!(std::path::Path::new("docs/development/TENANT_VISION_SYNC.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--tenant-ratio-advisory"));
    let multi = include_str!("../src/enterprise/multi_tenancy.rs");
    assert!(multi.contains("persist_tenant_to_sqlite"));
}

#[test]
fn tenant_ratio_advisory_criteria_totals_consistent_ph_s1231() {
    assert_eq!(tenant_vision_sync_criteria_total(), 10);
    assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
    assert_eq!(TENANT_RATIO_ADVISORY_SLICES.len(), 6);

    assert_eq!(
        tenant_ratio_advisory_depth_stub(Some(&json!({
            "tenant_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantRatioAdvisoryDepth::FullBand59
    );
}
