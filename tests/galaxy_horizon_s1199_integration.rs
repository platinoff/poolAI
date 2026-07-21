//! PH-S1208: Galaxy horizon close band 56 — tenant loc-audit aggregate.
//! Suite: `galaxy_horizon_s1199_integration`.

use poolai_ui_core::tenant_loc_audit_depth::{
    tenant_loc_audit_criteria_total, tenant_loc_audit_depth_stub, tenant_loc_audit_slices_met,
    TenantLocAuditDepth, FM_BAND56_ROWS, TENANT_LOC_AUDIT_BAND56_ROWS, TENANT_LOC_AUDIT_CASES,
    TENANT_LOC_AUDIT_CRITERIA, TENANT_LOC_AUDIT_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1199_band_tenant_loc_audit_close_ph_s1208() {
    assert_eq!(
        tenant_loc_audit_depth_stub(Some(&json!({"tenant_loc_audit_depth": true}))),
        TenantLocAuditDepth::DepthModule
    );
    assert_eq!(
        tenant_loc_audit_depth_stub(Some(&json!({
            "tenant_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantLocAuditDepth::FullBand56
    );

    assert_eq!(TENANT_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(tenant_loc_audit_criteria_total(), 10);
    assert!(TENANT_LOC_AUDIT_CASES.contains(&"tenant_loc_audit_docs"));
    assert_eq!(TENANT_LOC_AUDIT_SLICES.len(), 5);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert_eq!(tenant_loc_audit_slices_met(loc_audit), (5, 5));
    assert!(loc_audit.contains("tenant_loc_audit_mode"));
    assert!(loc_audit.contains("tenant_loc_audit_criteria_met_count"));
    assert!(loc_audit.contains("--tenant-loc-audit"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND56_ROWS {
        assert!(fm.contains(row), "FM missing band-56 row {row}");
    }
    assert!(fm.contains("PH-S1208"));
    assert!(fm.contains("5.37"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1199") || handoff.contains("band 56"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 57"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-loc-audit"));
    assert!(run_local.contains("VERIFY_TENANT_LOC_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_loc_audit_depth") || strategy.contains("band 56"));

    let tenant_doc = include_str!("../docs/development/TENANT_LOC_AUDIT.md");
    assert!(tenant_doc.contains("--tenant-loc-audit"));
    assert!(
        tenant_doc.contains("TENANT_LOC_AUDIT_SLICES") || tenant_doc.contains("--tenant-persist")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1199") || roadmap.contains("loc-audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_LOC_AUDIT"));
    assert!(verify.contains("--tenant-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-loc-audit"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("tenant_loc_audit_band56_export_shape"));

    for marker in TENANT_LOC_AUDIT_BAND56_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || tenant_doc.contains(marker),
            "band-56 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_LOC_AUDIT.md").exists());
    assert!(Path::new("tests/tenant_loc_audit_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_loc_audit_mode").is_some());
}
