//! PH-S1388: Galaxy horizon close band 74 — Audit admin/ops glue.
//! Suite: `galaxy_horizon_s1379_integration`.

use poolai_ui_core::audit_admin_ops_depth::{
    audit_admin_ops_criteria_total, audit_admin_ops_depth_stub, AuditAdminOpsDepth,
    AUDIT_ADMIN_OPS_BAND74_ROWS, AUDIT_ADMIN_OPS_CASES, AUDIT_ADMIN_OPS_CRITERIA, FM_BAND74_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1379_band_audit_admin_ops_close_ph_s1388() {
    assert_eq!(
        audit_admin_ops_depth_stub(Some(&json!({"audit_admin_ops_depth": true}))),
        AuditAdminOpsDepth::DepthModule
    );
    assert_eq!(
        audit_admin_ops_depth_stub(Some(&json!({
            "audit_admin_ops_depth": true,
            "store_strip": true,
            "query_ops_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_admin_ops_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditAdminOpsDepth::FullBand74
    );

    assert_eq!(AUDIT_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(audit_admin_ops_criteria_total(), 10);
    assert!(AUDIT_ADMIN_OPS_CASES.contains(&"audit_admin_ops_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND74_ROWS {
        assert!(fm.contains(row), "FM missing band-74 row {row}");
    }
    assert!(fm.contains("PH-S1388"));
    assert!(fm.contains("5.55"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1379") || handoff.contains("band 74"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 75"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-admin-ops"));
    assert!(run_local.contains("VERIFY_AUDIT_ADMIN_OPS"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_admin_ops") || strategy.contains("band 74"));

    let audit_doc = include_str!("../docs/development/AUDIT_ADMIN_OPS.md");
    assert!(audit_doc.contains("audit-store-badge"));
    assert!(audit_doc.contains("/api/enterprise/audit/store"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1379") || roadmap.contains("Audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_ADMIN_OPS"));
    assert!(verify.contains("--audit-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_admin_ops_mode"));
    assert!(loc_audit.contains("audit_admin_ops_criteria_met_count"));

    let audit_ui = include_str!("../src/ui/admin/audit.rs");
    assert!(audit_ui.contains("audit-store-badge"));
    assert!(audit_ui.contains("refreshAuditEvents"));

    for marker in AUDIT_ADMIN_OPS_BAND74_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || audit_ui.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-74 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/audit_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_admin_ops_mode").is_some());
    assert!(ratio.get("audit_admin_ops_criteria_total").is_some());
}
