//! PH-S1318: Galaxy horizon close band 67 — SSO docs canon.
//! Suite: `galaxy_horizon_s1309_integration`.

use poolai_ui_core::sso_docs_canon_depth::{
    sso_docs_canon_criteria_total, sso_docs_canon_depth_stub, sso_docs_canon_slices_met,
    SsoDocsCanonDepth, FM_BAND67_ROWS, SSO_DOCS_CANON_BAND67_ROWS, SSO_DOCS_CANON_CASES,
    SSO_DOCS_CANON_CRITERIA, SSO_DOCS_CANON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1309_band_sso_docs_canon_close_ph_s1318() {
    assert_eq!(
        sso_docs_canon_depth_stub(Some(&json!({"sso_docs_canon_depth": true}))),
        SsoDocsCanonDepth::DepthModule
    );
    assert_eq!(
        sso_docs_canon_depth_stub(Some(&json!({
            "sso_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoDocsCanonDepth::FullBand67
    );

    assert_eq!(SSO_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(sso_docs_canon_criteria_total(), 10);
    assert!(SSO_DOCS_CANON_CASES.contains(&"doc_loc_audit"));
    assert_eq!(SSO_DOCS_CANON_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_docs_canon_mode"));
    assert!(loc_audit.contains("sso_docs_canon_criteria_met_count"));
    assert!(loc_audit.contains("--sso-docs-canon"));

    let sso_doc = include_str!("../docs/development/SSO_DOCS_CANON.md");
    assert_eq!(sso_docs_canon_slices_met(sso_doc), (6, 6));
    assert!(sso_doc.contains("--sso-docs-canon"));
    assert!(sso_doc.contains("SSO_DOCS_CANON_SLICES") || sso_doc.contains("SSO_DEPTH.md"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND67_ROWS {
        assert!(fm.contains(row), "FM missing band-67 row {row}");
    }
    assert!(fm.contains("PH-S1318"));
    assert!(fm.contains("5.48"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1309") || handoff.contains("band 67"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 68"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-docs-canon"));
    assert!(run_local.contains("VERIFY_SSO_DOCS_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_docs_canon_depth") || strategy.contains("band 67"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1309") || roadmap.contains("docs canon"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_DOCS_CANON"));
    assert!(verify.contains("--sso-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-docs-canon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("sso_docs_canon_band67_export_shape"));

    for marker in SSO_DOCS_CANON_BAND67_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || sso_doc.contains(marker),
            "band-67 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_DOCS_CANON.md").exists());
    assert!(Path::new("tests/sso_docs_canon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_docs_canon_mode").is_some());
}
