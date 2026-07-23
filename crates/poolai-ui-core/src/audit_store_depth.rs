//! Audit store-wire band depth (PH-S1359…S1368, band 72 — enterprise phase C).

use serde_json::Value;

/// Audit store-wire depth flags (durable path wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditStoreDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand72,
}

/// Audit store-wire criteria registry (PH-S1359): id · marker · doc path.
pub const AUDIT_STORE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_store_depth",
        "AuditStoreDepth",
        "crates/poolai-ui-core/src/audit_store_depth.rs",
    ),
    ("store_wire", "audit_store_wire", "src/enterprise/audit.rs"),
    (
        "api_contracts",
        "audit_store_wire_integration",
        "tests/audit_store_wire_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_STORE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "audit_store_band72_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--audit-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "audit_store_docs",
        "AUDIT_STORE.md",
        "docs/development/AUDIT_STORE.md",
    ),
];

/// `poolai-loc-audit --audit-store` case names (PH-S1364).
pub const AUDIT_STORE_CASES: &[&str] = &[
    "audit_store_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "audit_store_docs",
];

/// FM §5.53 band-72 marker rows.
pub const FM_BAND72_ROWS: &[&str] = &[
    "5.53",
    "Audit store wire",
    "PH-S1359…S1368",
    "audit_store_depth",
];

/// Audit store-wire adoption markers for band 72.
pub const AUDIT_BAND72_ROWS: &[&str] = &[
    "PH-S1359",
    "audit_store_depth",
    "PH-S1360",
    "audit_store_wire",
    "PH-S1362",
    "VERIFY_AUDIT_STORE",
    "PH-S1364",
    "--audit-store",
    "PH-S1368",
];

/// Classify audit store-wire band depth from optional feature stub (PH-S1359).
pub fn audit_store_depth_stub(features: Option<&Value>) -> AuditStoreDepth {
    let Some(f) = features else {
        return AuditStoreDepth::None;
    };
    let depth = f
        .get("audit_store_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let api = f
        .get("api_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("audit_store_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return AuditStoreDepth::FullBand72;
    }
    if docs {
        return AuditStoreDepth::DocsCanon;
    }
    if loc {
        return AuditStoreDepth::LocAuditFlag;
    }
    if smoke {
        return AuditStoreDepth::StandSmokeExport;
    }
    if verify {
        return AuditStoreDepth::VerifyDevStandHook;
    }
    if api {
        return AuditStoreDepth::ApiContracts;
    }
    if store {
        return AuditStoreDepth::StoreWire;
    }
    if depth {
        return AuditStoreDepth::DepthModule;
    }
    AuditStoreDepth::None
}

/// Total audit store-wire criteria in registry (PH-S1359).
pub fn audit_store_criteria_total() -> usize {
    AUDIT_STORE_CRITERIA.len()
}

/// Env key for audit store backend (shared with band 71 scaffold).
pub const AUDIT_STORE_ENV: &str = "POOLAI_AUDIT_STORE";

/// Env key for durable audit data directory (band 72 wire).
pub const AUDIT_DATA_DIR_ENV: &str = "POOLAI_AUDIT_DATA_DIR";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_store_depth_stub_ph_s1359() {
        assert_eq!(audit_store_depth_stub(None), AuditStoreDepth::None);
        assert_eq!(
            audit_store_depth_stub(Some(&json!({"audit_store_depth": true}))),
            AuditStoreDepth::DepthModule
        );
        assert_eq!(
            audit_store_depth_stub(Some(&json!({
                "audit_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_store_docs": true,
            }))),
            AuditStoreDepth::FullBand72
        );
        assert_eq!(AUDIT_STORE_CRITERIA.len(), 7);
        assert_eq!(audit_store_criteria_total(), 7);
        assert_eq!(AUDIT_STORE_ENV, "POOLAI_AUDIT_STORE");
        assert_eq!(AUDIT_DATA_DIR_ENV, "POOLAI_AUDIT_DATA_DIR");
        assert!(FM_BAND72_ROWS.contains(&"PH-S1359…S1368"));
    }
}
