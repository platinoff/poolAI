//! Ratio / rust migration advisory band depth (PH-S1099…S1108, band 46).

use serde_json::Value;

/// Ratio/rust migration advisory depth flags (ui_js → wasm, archived e2e → Rust wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustMigrationAdvisoryDepth {
    None,
    UiJsCandidates,
    E2eArchivedCanon,
    LocAuditAdvisory,
    OpsShellCanon,
    FullBand46,
}

/// Admin JS glue files with Rust/wasm migration target (PH-S1102).
pub const ADMIN_JS_MIGRATION_CANDIDATES: &[(&str, &str)] = &[
    (
        "src/ui/admin_common.js",
        "poolai-ui-core/admin_common_depth",
    ),
    ("src/ui/admin_charts.js", "poolai-ui-core/charts_depth"),
    ("src/ui/i18n_core.js", "poolai-ui-core/i18n"),
    ("src/ui/admin_theme.js", "poolai-ui-core/theme"),
    ("src/ui/admin_modal_a11y.js", "poolai-ui-core/modal"),
    ("src/ui/topology_graph.js", "poolai-ui-core/topology"),
];

/// Archived Playwright API specs → Rust wire canon (PH-S1103).
pub const ARCHIVED_E2E_MIGRATION_CANON: &[(&str, &str)] = &[
    ("jobs_lease.spec.ts", "tests/jobs_api_contracts.rs"),
    ("jobs_migrating.spec.ts", "tests/jobs_api_contracts.rs"),
    (
        "protocol_middleware.spec.ts",
        "tests/protocol_middleware_integration.rs",
    ),
    (
        "telegram_wallet.spec.ts",
        "tests/telegram_wallet_integration.rs",
    ),
    ("grid_pricing.spec.ts", "tests/grid_pricing_integration.rs"),
    (
        "grid_job_lease.spec.ts",
        "tests/grid_envelope_lease_integration.rs",
    ),
    (
        "grid_result_lease.spec.ts",
        "tests/grid_envelope_lease_integration.rs",
    ),
    ("jobs_raid.spec.ts", "tests/job_store_raid_persistence.rs"),
];

/// `poolai-loc-audit --migration-advisory` case names (PH-S1100).
pub const MIGRATION_ADVISORY_CASES: &[&str] = &[
    "ui_js_candidates",
    "e2e_archived_canon",
    "ops_shell_canon",
    "stretch_spirit_hold",
    "ratio_95_formal_gate",
];

/// FM §5.27 band-46 marker rows.
pub const FM_BAND46_ROWS: &[&str] = &[
    "5.27",
    "ratio/rust migration advisory",
    "PH-S1099…S1108",
    "rust_migration_advisory_depth",
];

/// Rust migration advisory adoption markers for band 46.
pub const RUST_MIGRATION_BAND46_ROWS: &[&str] = &[
    "PH-S1099",
    "rust_migration_advisory_depth",
    "PH-S1100",
    "--migration-advisory",
    "PH-S1103",
    "VERIFY_MIGRATION_ADVISORY",
    "PH-S1104",
    "--migration-advisory",
    "PH-S1108",
];

/// Classify rust migration advisory band depth from optional feature stub (PH-S1099).
pub fn rust_migration_advisory_depth_stub(features: Option<&Value>) -> RustMigrationAdvisoryDepth {
    let Some(f) = features else {
        return RustMigrationAdvisoryDepth::None;
    };
    let ui_js = f
        .get("ui_js_candidates")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let e2e = f
        .get("e2e_archived_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_advisory")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ops = f
        .get("ops_shell_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if ui_js && e2e && loc && ops {
        return RustMigrationAdvisoryDepth::FullBand46;
    }
    if loc {
        return RustMigrationAdvisoryDepth::LocAuditAdvisory;
    }
    if ops {
        return RustMigrationAdvisoryDepth::OpsShellCanon;
    }
    if e2e {
        return RustMigrationAdvisoryDepth::E2eArchivedCanon;
    }
    if ui_js {
        return RustMigrationAdvisoryDepth::UiJsCandidates;
    }
    RustMigrationAdvisoryDepth::None
}

/// Total migration registry entries (ui_js + archived e2e).
pub fn migration_registry_total() -> usize {
    ADMIN_JS_MIGRATION_CANDIDATES.len() + ARCHIVED_E2E_MIGRATION_CANON.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rust_migration_advisory_depth_stub_ph_s1099() {
        assert_eq!(
            rust_migration_advisory_depth_stub(None),
            RustMigrationAdvisoryDepth::None
        );
        assert_eq!(
            rust_migration_advisory_depth_stub(Some(&json!({"ui_js_candidates": true}))),
            RustMigrationAdvisoryDepth::UiJsCandidates
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
        assert!(!MIGRATION_ADVISORY_CASES.is_empty());
        assert!(FM_BAND46_ROWS.contains(&"PH-S1099…S1108"));
        assert!(migration_registry_total() >= 14);
    }
}
