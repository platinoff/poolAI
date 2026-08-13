//! PH-S1221: Tenant vision-sync contracts (band 58).
//! Marker: tenant_vision_sync_integration
//!
//! Verifies vision slices + prior TENANT_DOCS_CANON are present and criteria totals are consistent.

use poolai_ui_core::tenant_docs_canon_depth::tenant_docs_canon_criteria_total;
use poolai_ui_core::tenant_vision_sync_depth::{
    tenant_vision_sync_criteria_total, tenant_vision_sync_depth_stub,
    tenant_vision_sync_slices_met, TenantVisionSyncDepth, TENANT_VISION_SYNC_CASES,
    TENANT_VISION_SYNC_CRITERIA, TENANT_VISION_SYNC_SLICES,
};
use serde_json::json;

#[test]
fn tenant_vision_sync_depth_registry_ph_s1219() {
    assert_eq!(TENANT_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(tenant_vision_sync_criteria_total(), 10);
    assert!(TENANT_VISION_SYNC_CASES.contains(&"aggregate_flag"));
    assert!(TENANT_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(
        tenant_vision_sync_depth_stub(Some(&json!({"slice_aggregate": true}))),
        TenantVisionSyncDepth::SliceAggregate
    );
}

#[test]
fn tenant_vision_sync_slice_docs_present_ph_s1220() {
    let canon = include_str!("../docs/development/TENANT_VISION_SYNC.md");
    let (met, total) = tenant_vision_sync_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all vision-sync slices must be listed");
    for name in TENANT_VISION_SYNC_SLICES {
        assert!(canon.contains(name), "missing vision-sync slice {name}");
    }
    assert!(std::path::Path::new("GSV/docs/vision/manifest.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/extensions.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/README.md").exists());
    assert!(std::path::Path::new("GSV/docs/vision/vision.svg").exists());
    assert!(std::path::Path::new("GSV/docs/vision/index.html").exists());
    assert!(std::path::Path::new("docs/development/TENANT_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--tenant-vision-sync"));
}

#[test]
fn tenant_vision_sync_criteria_totals_consistent_ph_s1221() {
    assert_eq!(tenant_docs_canon_criteria_total(), 10);
    assert_eq!(tenant_vision_sync_criteria_total(), 10);
    assert_eq!(TENANT_VISION_SYNC_SLICES.len(), 6);

    assert_eq!(
        tenant_vision_sync_depth_stub(Some(&json!({
            "tenant_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantVisionSyncDepth::FullBand58
    );
}
