//! PH-S1268: Galaxy horizon close band 62 — SSO store wire.

use poolai_ui_core::sso_store_depth::{
    sso_store_criteria_total, sso_store_depth_stub, SsoStoreDepth, FM_BAND62_ROWS, SSO_BAND62_ROWS,
    SSO_STORE_CASES, SSO_STORE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1259_band_sso_store_close_ph_s1268() {
    assert_eq!(
        sso_store_depth_stub(Some(&json!({"sso_store_depth": true}))),
        SsoStoreDepth::DepthModule
    );
    assert_eq!(
        sso_store_depth_stub(Some(&json!({
            "sso_store_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_store_docs": true,
        }))),
        SsoStoreDepth::FullBand62
    );

    assert_eq!(SSO_STORE_CRITERIA.len(), 7);
    assert_eq!(sso_store_criteria_total(), 7);
    assert!(SSO_STORE_CASES.contains(&"sso_store_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND62_ROWS {
        assert!(fm.contains(row), "FM missing band-62 row {row}");
    }
    assert!(fm.contains("PH-S1268"));
    assert!(fm.contains("5.43"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1259") || handoff.contains("band 62"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 63"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-store"));
    assert!(run_local.contains("VERIFY_SSO_STORE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_store") || strategy.contains("band 62"));

    let sso_doc = include_str!("../docs/development/SSO_STORE.md");
    assert!(sso_doc.contains("POOLAI_SSO_STORE"));
    assert!(sso_doc.contains("sso_store_wire"));
    assert!(sso_doc.contains("POOLAI_SSO_DATA_DIR"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1259") || roadmap.contains("SSO"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_STORE"));
    assert!(verify.contains("--sso-store"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-store"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_store_mode"));
    assert!(loc_audit.contains("sso_store_criteria_met_count"));

    let security = include_str!("../src/enterprise/security.rs");
    assert!(security.contains("POOLAI_SSO_STORE"));
    assert!(security.contains("sso_store_wire"));
    assert!(security.contains("POOLAI_SSO_DATA_DIR"));

    for marker in SSO_BAND62_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-62 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_store_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_STORE.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_store_mode").is_some());
    assert!(ratio.get("sso_store_criteria_total").is_some());
}
