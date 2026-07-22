//! PH-S1241: Tenant horizon-close contracts (band 60).
//! Marker: tenant_horizon_integration
//!
//! Verifies phase-A tenant slices + horizon criteria totals.

use poolai_ui_core::tenant_horizon_depth::{
    tenant_horizon_criteria_total, tenant_horizon_depth_stub, tenant_horizon_slices_met,
    TenantHorizonDepth, TENANT_HORIZON_CASES, TENANT_HORIZON_CRITERIA, TENANT_HORIZON_SLICES,
};
use poolai_ui_core::tenant_ratio_advisory_depth::tenant_ratio_advisory_criteria_total;
use serde_json::json;

#[test]
fn tenant_horizon_depth_registry_ph_s1239() {
    assert_eq!(TENANT_HORIZON_CRITERIA.len(), 10);
    assert_eq!(tenant_horizon_criteria_total(), 10);
    assert!(TENANT_HORIZON_CASES.contains(&"aggregate_flag"));
    assert!(TENANT_HORIZON_CASES.contains(&"phase_a_slices"));
    assert_eq!(
        tenant_horizon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        TenantHorizonDepth::SliceAggregate
    );
}

#[test]
fn tenant_horizon_slice_docs_present_ph_s1240() {
    let canon = include_str!("../docs/development/TENANT_HORIZON.md");
    let (met, total) = tenant_horizon_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all horizon slices must be listed");
    for name in TENANT_HORIZON_SLICES {
        assert!(canon.contains(name), "missing horizon slice {name}");
    }
    assert!(std::path::Path::new("docs/development/TENANT_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/TENANT_STORE.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--tenant-horizon"));
    let multi = include_str!("../src/enterprise/multi_tenancy.rs");
    assert!(multi.contains("persist_tenant_to_sqlite"));
}

#[test]
fn tenant_horizon_criteria_totals_consistent_ph_s1241() {
    assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
    assert_eq!(tenant_horizon_criteria_total(), 10);
    assert_eq!(TENANT_HORIZON_SLICES.len(), 10);

    assert_eq!(
        tenant_horizon_depth_stub(Some(&json!({
            "tenant_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantHorizonDepth::FullBand60
    );
}
