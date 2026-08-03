//! PH-S1688: Galaxy horizon close band 104 — Ratio96 admin/ops glue.
//! Suite: `galaxy_horizon_s1679_integration`.

use poolai_ui_core::ratio96_admin_ops_depth::{
    ratio96_admin_ops_criteria_total, ratio96_admin_ops_depth_stub, Ratio96AdminOpsDepth,
    FM_BAND104_ROWS, RATIO96_ADMIN_OPS_BAND104_ROWS, RATIO96_ADMIN_OPS_CASES,
    RATIO96_ADMIN_OPS_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1679_band_ratio96_admin_ops_close_ph_s1688() {
    assert_eq!(
        ratio96_admin_ops_depth_stub(Some(&json!({"ratio96_admin_ops_depth": true}))),
        Ratio96AdminOpsDepth::DepthModule
    );
    assert_eq!(
        ratio96_admin_ops_depth_stub(Some(&json!({
            "ratio96_admin_ops_depth": true,
            "store_strip": true,
            "query_ops_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "docs_canon": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        Ratio96AdminOpsDepth::FullBand104
    );

    assert_eq!(RATIO96_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(ratio96_admin_ops_criteria_total(), 10);
    assert!(RATIO96_ADMIN_OPS_CASES.contains(&"docs_canon"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND104_ROWS {
        assert!(fm.contains(row), "FM missing band-104 row {row}");
    }
    assert!(fm.contains("PH-S1688"));
    assert!(fm.contains("5.85"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1679") || handoff.contains("band 104"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 105"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ratio96-admin-ops"));
    assert!(run_local.contains("VERIFY_RATIO96_ADMIN_OPS"));

    let ratio_doc = include_str!("../docs/development/RATIO96_ADMIN_OPS.md");
    assert!(ratio_doc.contains("ratio96-store-badge"));
    assert!(ratio_doc.contains("/api/v1/ops/ratio96"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_RATIO96_ADMIN_OPS"));
    assert!(verify.contains("--ratio96-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ratio96-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ratio96_admin_ops_mode"));
    assert!(loc_audit.contains("ratio96_admin_ops_criteria_met_count"));

    let dash = include_str!("../src/ui/admin/dashboard.rs");
    assert!(dash.contains("ratio96-store-badge"));
    assert!(dash.contains("refreshRatio96"));

    for marker in RATIO96_ADMIN_OPS_BAND104_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || dash.contains(marker)
                || verify.contains(marker)
                || ratio_doc.contains(marker),
            "band-104 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ratio96_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/RATIO96_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/ratio96_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ratio96_admin_ops_mode").is_some());
    assert!(ratio.get("ratio96_admin_ops_criteria_total").is_some());
}
