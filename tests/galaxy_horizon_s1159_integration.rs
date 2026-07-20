//! PH-S1168: Galaxy horizon close band 52 — tenant store wire.

use poolai_ui_core::tenant_depth::{
    tenant_criteria_total, tenant_depth_stub, TenantDepth, FM_BAND52_ROWS, TENANT_BAND52_ROWS,
    TENANT_CASES, TENANT_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1159_band_tenant_store_close_ph_s1168() {
    assert_eq!(
        tenant_depth_stub(Some(&json!({"tenant_depth": true}))),
        TenantDepth::DepthModule
    );
    assert_eq!(
        tenant_depth_stub(Some(&json!({
            "tenant_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_store_docs": true,
        }))),
        TenantDepth::FullBand52
    );

    assert_eq!(TENANT_CRITERIA.len(), 7);
    assert_eq!(tenant_criteria_total(), 7);
    assert!(TENANT_CASES.contains(&"tenant_store_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND52_ROWS {
        assert!(fm.contains(row), "FM missing band-52 row {row}");
    }
    assert!(fm.contains("PH-S1168"));
    assert!(fm.contains("5.33"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1159") || handoff.contains("band 52"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 53"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-store"));
    assert!(run_local.contains("VERIFY_TENANT_STORE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_depth"));

    let tenant_doc = include_str!("../docs/development/TENANT_STORE.md");
    assert!(tenant_doc.contains("POOLAI_TENANT_DATA_DIR"));
    assert!(tenant_doc.contains("tenant_store_wire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1159") || roadmap.contains("enterprise"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_STORE"));
    assert!(verify.contains("--tenant-store"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-store"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_store_mode"));
    assert!(loc_audit.contains("tenant_store_criteria_met_count"));

    let multi = include_str!("../src/enterprise/multi_tenancy.rs");
    assert!(multi.contains("tenant_store_wire"));
    assert!(multi.contains("POOLAI_TENANT_DATA_DIR"));

    for marker in TENANT_BAND52_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-52 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_STORE.md").exists());
    assert!(Path::new("tests/tenant_store_wire_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_store_mode").is_some());
    assert!(ratio.get("tenant_store_criteria_total").is_some());
}
