//! PH-S1188: Galaxy horizon close band 54 — tenant admin/ops glue.
//! Suite: `galaxy_horizon_s1179_integration`.

use poolai_ui_core::tenant_admin_ops_depth::{
    tenant_admin_ops_criteria_total, tenant_admin_ops_depth_stub, TenantAdminOpsDepth,
    FM_BAND54_ROWS, TENANT_ADMIN_OPS_BAND54_ROWS, TENANT_ADMIN_OPS_CASES,
    TENANT_ADMIN_OPS_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1179_band_tenant_admin_ops_close_ph_s1188() {
    assert_eq!(
        tenant_admin_ops_depth_stub(Some(&json!({"tenant_admin_ops_depth": true}))),
        TenantAdminOpsDepth::DepthModule
    );
    assert_eq!(
        tenant_admin_ops_depth_stub(Some(&json!({
            "tenant_admin_ops_depth": true,
            "store_strip": true,
            "usage_quota_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_admin_ops_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantAdminOpsDepth::FullBand54
    );

    assert_eq!(TENANT_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(tenant_admin_ops_criteria_total(), 10);
    assert!(TENANT_ADMIN_OPS_CASES.contains(&"tenant_admin_ops_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND54_ROWS {
        assert!(fm.contains(row), "FM missing band-54 row {row}");
    }
    assert!(fm.contains("PH-S1188"));
    assert!(fm.contains("5.35"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1179") || handoff.contains("band 54"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 55"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-admin-ops"));
    assert!(run_local.contains("VERIFY_TENANT_ADMIN_OPS"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_admin_ops_depth") || strategy.contains("band 54"));

    let tenant_doc = include_str!("../docs/development/TENANT_ADMIN_OPS.md");
    assert!(tenant_doc.contains("/api/enterprise/tenants/store"));
    assert!(tenant_doc.contains("probeTenantQuota"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1179") || roadmap.contains("admin/ops glue"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_ADMIN_OPS"));
    assert!(verify.contains("--tenant-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_admin_ops_mode"));
    assert!(loc_audit.contains("tenant_admin_ops_criteria_met_count"));

    let tenants_ui = include_str!("../src/ui/admin/tenants.rs");
    assert!(tenants_ui.contains("tenant-store-badge"));
    assert!(tenants_ui.contains("probeTenantQuota"));

    for marker in TENANT_ADMIN_OPS_BAND54_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || tenants_ui.contains(marker)
                || verify.contains(marker),
            "band-54 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/tenant_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_admin_ops_mode").is_some());
}
