//! PH-S1108: Galaxy horizon close band 46 — ratio/rust migration advisory.

use poolai_ui_core::rust_migration_advisory_depth::{
    migration_registry_total, rust_migration_advisory_depth_stub, RustMigrationAdvisoryDepth,
    ADMIN_JS_MIGRATION_CANDIDATES, ARCHIVED_E2E_MIGRATION_CANON, FM_BAND46_ROWS,
    MIGRATION_ADVISORY_CASES, RUST_MIGRATION_BAND46_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1099_band_rust_migration_advisory_close_ph_s1108() {
    assert_eq!(
        rust_migration_advisory_depth_stub(Some(&json!({"loc_audit_advisory": true}))),
        RustMigrationAdvisoryDepth::LocAuditAdvisory
    );
    assert_eq!(
        rust_migration_advisory_depth_stub(Some(&json!({
            "ui_js_candidates": true,
            "e2e_archived_canon": true,
            "loc_audit_advisory": true,
            "ops_shell_canon": true,
        }))),
        RustMigrationAdvisoryDepth::FullBand46
    );

    assert_eq!(ADMIN_JS_MIGRATION_CANDIDATES.len(), 6);
    assert_eq!(ARCHIVED_E2E_MIGRATION_CANON.len(), 8);
    assert!(migration_registry_total() >= 14);
    assert!(MIGRATION_ADVISORY_CASES.contains(&"ui_js_candidates"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND46_ROWS {
        assert!(fm.contains(row), "FM missing band-46 row {row}");
    }
    assert!(fm.contains("PH-S1108"));
    assert!(fm.contains("5.27"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1099") || handoff.contains("band 46"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 47"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--migration-advisory"));
    assert!(run_local.contains("VERIFY_MIGRATION_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("rust_migration_advisory_depth"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MIGRATION_ADVISORY"));
    assert!(verify.contains("--migration-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--migration-advisory"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("migration_advisory_mode"));
    assert!(loc_audit.contains("migration_candidate_total"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("rust_migration_advisory_band46_export_shape_ph_s1104"));

    for marker in RUST_MIGRATION_BAND46_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-46 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/rust_migration_advisory_depth.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("migration_candidate_total").is_some());
    assert!(ratio.get("migration_advisory_mode").is_some());
}
