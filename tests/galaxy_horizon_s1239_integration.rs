//! PH-S1248: Galaxy horizon close band 60 — tenant phase A horizon close.
//! Suite: `galaxy_horizon_s1239_integration`.

use poolai_ui_core::tenant_horizon_depth::{
    tenant_horizon_criteria_total, tenant_horizon_depth_stub, tenant_horizon_slices_met,
    TenantHorizonDepth, FM_BAND60_ROWS, TENANT_HORIZON_BAND60_ROWS, TENANT_HORIZON_CASES,
    TENANT_HORIZON_CRITERIA, TENANT_HORIZON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1239_band_tenant_horizon_close_ph_s1248() {
    assert_eq!(
        tenant_horizon_depth_stub(Some(&json!({
            "tenant_horizon_depth": true
        }))),
        TenantHorizonDepth::DepthModule
    );
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

    assert_eq!(TENANT_HORIZON_CRITERIA.len(), 10);
    assert_eq!(tenant_horizon_criteria_total(), 10);
    assert!(TENANT_HORIZON_CASES.contains(&"sqlite_restart_safe"));
    assert_eq!(TENANT_HORIZON_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_horizon_mode"));
    assert!(loc_audit.contains("tenant_horizon_criteria_met_count"));
    assert!(loc_audit.contains("--tenant-horizon"));

    let tenant_doc = include_str!("../docs/development/TENANT_HORIZON.md");
    assert_eq!(tenant_horizon_slices_met(tenant_doc), (10, 10));
    assert!(tenant_doc.contains("--tenant-horizon"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND60_ROWS {
        assert!(fm.contains(row), "FM missing band-60 row {row}");
    }
    assert!(fm.contains("PH-S1248"));
    assert!(fm.contains("5.41"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1239") || handoff.contains("band 60"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 61"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-horizon"));
    assert!(run_local.contains("VERIFY_TENANT_HORIZON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_horizon_depth") || strategy.contains("band 60"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1239") || roadmap.contains("horizon close"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_HORIZON"));
    assert!(verify.contains("--tenant-horizon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-horizon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("tenant_horizon_band60_export_shape"));

    let multi = include_str!("../src/enterprise/multi_tenancy.rs");
    assert!(multi.contains("persist_tenant_to_sqlite"));

    for marker in TENANT_HORIZON_BAND60_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || tenant_doc.contains(marker)
                || multi.contains(marker),
            "band-60 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_horizon_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_HORIZON.md").exists());
    assert!(Path::new("tests/tenant_horizon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_horizon_mode").is_some());
}
