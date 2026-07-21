//! PH-S1238: Galaxy horizon close band 59 — tenant ratio advisory.
//! Suite: `galaxy_horizon_s1229_integration`.

use poolai_ui_core::tenant_ratio_advisory_depth::{
    tenant_ratio_advisory_criteria_total, tenant_ratio_advisory_depth_stub,
    tenant_ratio_advisory_slices_met, TenantRatioAdvisoryDepth, FM_BAND59_ROWS,
    TENANT_RATIO_ADVISORY_BAND59_ROWS, TENANT_RATIO_ADVISORY_CASES, TENANT_RATIO_ADVISORY_CRITERIA,
    TENANT_RATIO_ADVISORY_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1229_band_tenant_ratio_advisory_close_ph_s1238() {
    assert_eq!(
        tenant_ratio_advisory_depth_stub(Some(&json!({
            "tenant_ratio_advisory_depth": true
        }))),
        TenantRatioAdvisoryDepth::DepthModule
    );
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

    assert_eq!(TENANT_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
    assert!(TENANT_RATIO_ADVISORY_CASES.contains(&"sqlite_restart_safe"));
    assert_eq!(TENANT_RATIO_ADVISORY_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_ratio_advisory_mode"));
    assert!(loc_audit.contains("tenant_ratio_advisory_criteria_met_count"));
    assert!(loc_audit.contains("--tenant-ratio-advisory"));

    let tenant_doc = include_str!("../docs/development/TENANT_RATIO_ADVISORY.md");
    assert_eq!(tenant_ratio_advisory_slices_met(tenant_doc), (6, 6));
    assert!(tenant_doc.contains("--tenant-ratio-advisory"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND59_ROWS {
        assert!(fm.contains(row), "FM missing band-59 row {row}");
    }
    assert!(fm.contains("PH-S1238"));
    assert!(fm.contains("5.40"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1229") || handoff.contains("band 59"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 60"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-ratio-advisory"));
    assert!(run_local.contains("VERIFY_TENANT_RATIO_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_ratio_advisory_depth") || strategy.contains("band 59"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1229") || roadmap.contains("ratio advisory"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_RATIO_ADVISORY"));
    assert!(verify.contains("--tenant-ratio-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-ratio-advisory"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("tenant_ratio_advisory_band59_export_shape"));

    let multi = include_str!("../src/enterprise/multi_tenancy.rs");
    assert!(multi.contains("persist_tenant_to_sqlite"));

    for marker in TENANT_RATIO_ADVISORY_BAND59_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || tenant_doc.contains(marker)
                || multi.contains(marker),
            "band-59 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_ratio_advisory_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/tenant_ratio_advisory_integration.rs").exists());
    assert!(Path::new("tests/tenant_sqlite_durable_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_ratio_advisory_mode").is_some());
}
