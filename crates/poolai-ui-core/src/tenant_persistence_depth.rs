//! Tenant persistence band depth (PH-S1149…S1158, band 51 — enterprise phase A).

use serde_json::Value;

/// Tenant persistence depth flags (durable store scaffold + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantPersistenceDepth {
    None,
    DepthModule,
    LocAuditFlag,
    AuditTest,
    VerifyDevStandHook,
    QuickFlag,
    StandSmokeExport,
    DocsCanon,
    FullBand51,
}

/// Tenant persist criteria registry (PH-S1151): id · marker · doc path.
pub const TENANT_PERSIST_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_persistence_depth",
        "TenantPersistenceDepth",
        "crates/poolai-ui-core/src/tenant_persistence_depth.rs",
    ),
    (
        "loc_audit_flag",
        "--tenant-persist",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "audit_test",
        "tenant_persistence_audit",
        "tests/tenant_persistence_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_PERSIST",
        "bin/verify-dev-stand.sh",
    ),
    ("quick_flag", "--tenant-persist", "bin/run-poolai.sh"),
    (
        "tenant_persist_docs",
        "TENANT_PERSIST.md",
        "docs/development/TENANT_PERSIST.md",
    ),
    (
        "multi_tenancy_store_hint",
        "POOLAI_TENANT_STORE",
        "src/enterprise/multi_tenancy.rs",
    ),
];

/// `poolai-loc-audit --tenant-persist` case names (PH-S1150).
pub const TENANT_PERSIST_CASES: &[&str] = &[
    "tenant_persistence_depth",
    "loc_audit_flag",
    "audit_test",
    "verify_dev_stand_hook",
    "quick_flag",
    "tenant_persist_docs",
    "multi_tenancy_store_hint",
];

/// FM §5.32 band-51 marker rows.
pub const FM_BAND51_ROWS: &[&str] = &[
    "5.32",
    "Tenant persistence",
    "PH-S1149…S1158",
    "tenant_persistence_depth",
];

/// Tenant persist adoption markers for band 51.
pub const TENANT_PERSIST_BAND51_ROWS: &[&str] = &[
    "PH-S1149",
    "tenant_persistence_depth",
    "PH-S1150",
    "--tenant-persist",
    "PH-S1153",
    "VERIFY_TENANT_PERSIST",
    "PH-S1154",
    "--tenant-persist",
    "PH-S1158",
];

/// Classify tenant persistence band depth from optional feature stub (PH-S1149).
pub fn tenant_persistence_depth_stub(features: Option<&Value>) -> TenantPersistenceDepth {
    let Some(f) = features else {
        return TenantPersistenceDepth::None;
    };
    let depth = f
        .get("tenant_persistence_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let audit = f
        .get("audit_test")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let quick = f
        .get("quick_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("tenant_persist_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && loc && audit && verify && quick && smoke && docs {
        return TenantPersistenceDepth::FullBand51;
    }
    if docs {
        return TenantPersistenceDepth::DocsCanon;
    }
    if smoke {
        return TenantPersistenceDepth::StandSmokeExport;
    }
    if quick {
        return TenantPersistenceDepth::QuickFlag;
    }
    if verify {
        return TenantPersistenceDepth::VerifyDevStandHook;
    }
    if audit {
        return TenantPersistenceDepth::AuditTest;
    }
    if loc {
        return TenantPersistenceDepth::LocAuditFlag;
    }
    if depth {
        return TenantPersistenceDepth::DepthModule;
    }
    TenantPersistenceDepth::None
}

/// Total tenant persist criteria in registry (PH-S1151).
pub fn tenant_persist_criteria_total() -> usize {
    TENANT_PERSIST_CRITERIA.len()
}

/// Env key for future durable tenant store backend (PH-S1149 scaffold).
pub const TENANT_STORE_ENV: &str = "POOLAI_TENANT_STORE";

/// Canonical store modes for band 51+ (memory default; sqlite horizon).
pub const TENANT_STORE_MODES: &[&str] = &["memory", "sqlite"];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_persistence_depth_stub_ph_s1149() {
        assert_eq!(
            tenant_persistence_depth_stub(None),
            TenantPersistenceDepth::None
        );
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({"tenant_persistence_depth": true}))),
            TenantPersistenceDepth::DepthModule
        );
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({
                "tenant_persistence_depth": true,
                "loc_audit_flag": true,
                "audit_test": true,
                "verify_dev_stand_hook": true,
                "quick_flag": true,
                "stand_smoke_export": true,
                "tenant_persist_docs": true,
            }))),
            TenantPersistenceDepth::FullBand51
        );
        assert_eq!(TENANT_PERSIST_CRITERIA.len(), 7);
        assert_eq!(tenant_persist_criteria_total(), 7);
        assert_eq!(TENANT_STORE_ENV, "POOLAI_TENANT_STORE");
        assert!(TENANT_STORE_MODES.contains(&"sqlite"));
        assert!(FM_BAND51_ROWS.contains(&"PH-S1149…S1158"));
    }
}
