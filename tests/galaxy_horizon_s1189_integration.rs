//! PH-S1198: Galaxy horizon close band 55 — tenant live stand smoke.
//! Suite: `galaxy_horizon_s1189_integration`.

use poolai_ui_core::tenant_stand_smoke_depth::{
    tenant_stand_smoke_criteria_total, tenant_stand_smoke_depth_stub, TenantStandSmokeDepth,
    FM_BAND55_ROWS, TENANT_STAND_SMOKE_BAND55_ROWS, TENANT_STAND_SMOKE_CASES,
    TENANT_STAND_SMOKE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1189_band_tenant_stand_smoke_close_ph_s1198() {
    assert_eq!(
        tenant_stand_smoke_depth_stub(Some(&json!({"tenant_stand_smoke_depth": true}))),
        TenantStandSmokeDepth::DepthModule
    );
    assert_eq!(
        tenant_stand_smoke_depth_stub(Some(&json!({
            "tenant_stand_smoke_depth": true,
            "live_store": true,
            "live_crud": true,
            "live_usage_quota": true,
            "cli_flag": true,
            "loc_audit_flag": true,
            "verify_dev_stand_hook": true,
            "tenant_stand_smoke_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantStandSmokeDepth::FullBand55
    );

    assert_eq!(TENANT_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(tenant_stand_smoke_criteria_total(), 10);
    assert!(TENANT_STAND_SMOKE_CASES.contains(&"tenant_stand_smoke_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND55_ROWS {
        assert!(fm.contains(row), "FM missing band-55 row {row}");
    }
    assert!(fm.contains("PH-S1198"));
    assert!(fm.contains("5.36"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1189") || handoff.contains("band 55"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 56"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-stand-smoke"));
    assert!(run_local.contains("VERIFY_TENANT_STAND_SMOKE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_stand_smoke_depth") || strategy.contains("band 55"));

    let tenant_doc = include_str!("../docs/development/TENANT_STAND_SMOKE.md");
    assert!(tenant_doc.contains("/api/enterprise/tenants/store"));
    assert!(
        tenant_doc.contains("smoke_tenants_crud_lifecycle")
            || tenant_doc.contains("--tenant-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1189") || roadmap.contains("stand smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_STAND_SMOKE"));
    assert!(verify.contains("--tenant-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_stand_smoke_mode"));
    assert!(loc_audit.contains("tenant_stand_smoke_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("smoke_tenants_store_wire"));
    assert!(smoke.contains("smoke_tenants_crud_lifecycle"));
    assert!(smoke.contains("smoke_tenants_usage_quota_isolation"));
    assert!(smoke.contains("tenant_stand_smoke_only"));

    for marker in TENANT_STAND_SMOKE_BAND55_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker),
            "band-55 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_STAND_SMOKE.md").exists());
    assert!(Path::new("tests/tenant_stand_smoke_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_stand_smoke_mode").is_some());
}
