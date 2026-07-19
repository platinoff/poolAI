//! PH-S1101: Rust migration advisory audit — ui_js candidates + archived e2e canon.

use poolai_ui_core::rust_migration_advisory_depth::{
    rust_migration_advisory_depth_stub, RustMigrationAdvisoryDepth, ADMIN_JS_MIGRATION_CANDIDATES,
    ARCHIVED_E2E_MIGRATION_CANON, MIGRATION_ADVISORY_CASES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn rust_migration_advisory_audit_ph_s1101() {
    assert_eq!(
        rust_migration_advisory_depth_stub(Some(&json!({"e2e_archived_canon": true}))),
        RustMigrationAdvisoryDepth::E2eArchivedCanon
    );

    for (js_path, rust_target) in ADMIN_JS_MIGRATION_CANDIDATES {
        assert!(
            Path::new(js_path).is_file(),
            "ui_js candidate missing: {js_path}"
        );
        assert!(!rust_target.is_empty(), "rust target for {js_path}");
    }

    for (archived, rust_canon) in ARCHIVED_E2E_MIGRATION_CANON {
        let archive_path = format!("e2e/archive/api-smoke/{archived}");
        assert!(
            Path::new(&archive_path).is_file(),
            "archived API spec missing: {archive_path}"
        );
        assert!(
            Path::new(rust_canon).is_file(),
            "Rust wire canon missing for {archived}: {rust_canon}"
        );
    }

    assert_eq!(MIGRATION_ADVISORY_CASES.len(), 5);
    assert!(MIGRATION_ADVISORY_CASES.contains(&"ratio_95_formal_gate"));
}
