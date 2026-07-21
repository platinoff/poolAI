//! PH-S1218: Galaxy horizon close band 57 — tenant docs canon.
//! Suite: `galaxy_horizon_s1209_integration`.

use poolai_ui_core::tenant_docs_canon_depth::{
    tenant_docs_canon_criteria_total, tenant_docs_canon_depth_stub, tenant_docs_canon_slices_met,
    TenantDocsCanonDepth, FM_BAND57_ROWS, TENANT_DOCS_CANON_BAND57_ROWS, TENANT_DOCS_CANON_CASES,
    TENANT_DOCS_CANON_CRITERIA, TENANT_DOCS_CANON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1209_band_tenant_docs_canon_close_ph_s1218() {
    assert_eq!(
        tenant_docs_canon_depth_stub(Some(&json!({"tenant_docs_canon_depth": true}))),
        TenantDocsCanonDepth::DepthModule
    );
    assert_eq!(
        tenant_docs_canon_depth_stub(Some(&json!({
            "tenant_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantDocsCanonDepth::FullBand57
    );

    assert_eq!(TENANT_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(tenant_docs_canon_criteria_total(), 10);
    assert!(TENANT_DOCS_CANON_CASES.contains(&"doc_loc_audit"));
    assert_eq!(TENANT_DOCS_CANON_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_docs_canon_mode"));
    assert!(loc_audit.contains("tenant_docs_canon_criteria_met_count"));
    assert!(loc_audit.contains("--tenant-docs-canon"));

    let tenant_doc = include_str!("../docs/development/TENANT_DOCS_CANON.md");
    assert_eq!(tenant_docs_canon_slices_met(tenant_doc), (6, 6));
    assert!(tenant_doc.contains("--tenant-docs-canon"));
    assert!(
        tenant_doc.contains("TENANT_DOCS_CANON_SLICES") || tenant_doc.contains("TENANT_PERSIST.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND57_ROWS {
        assert!(fm.contains(row), "FM missing band-57 row {row}");
    }
    assert!(fm.contains("PH-S1218"));
    assert!(fm.contains("5.38"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1209") || handoff.contains("band 57"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 58"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-docs-canon"));
    assert!(run_local.contains("VERIFY_TENANT_DOCS_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_docs_canon_depth") || strategy.contains("band 57"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1209") || roadmap.contains("docs canon"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_DOCS_CANON"));
    assert!(verify.contains("--tenant-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-docs-canon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("tenant_docs_canon_band57_export_shape"));

    for marker in TENANT_DOCS_CANON_BAND57_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || tenant_doc.contains(marker),
            "band-57 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_DOCS_CANON.md").exists());
    assert!(Path::new("tests/tenant_docs_canon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_docs_canon_mode").is_some());
}
