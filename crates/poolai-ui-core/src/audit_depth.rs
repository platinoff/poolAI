//! Audit depth scaffold (PH-S1349…S1358, band 71 — enterprise phase C).

use serde_json::Value;

/// Audit depth flags (production verify stub + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand71,
}

/// Audit criteria registry (PH-S1349): id · marker · doc path.
pub const AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_depth",
        "AuditDepth",
        "crates/poolai-ui-core/src/audit_depth.rs",
    ),
    (
        "store_wire",
        "POOLAI_AUDIT_STORE",
        "src/enterprise/audit.rs",
    ),
    (
        "api_contracts",
        "audit_depth_audit",
        "tests/audit_depth_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    ("quick_flag", "--audit", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "audit_band71_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    ("loc_audit_flag", "--audit", "src/bin/poolai_loc_audit.rs"),
    (
        "audit_docs",
        "AUDIT_DEPTH.md",
        "docs/development/AUDIT_DEPTH.md",
    ),
];

/// `poolai-loc-audit --audit` case names (PH-S1354).
pub const AUDIT_CASES: &[&str] = &[
    "audit_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "quick_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "audit_docs",
];

/// FM §5.52 band-71 marker rows.
pub const FM_BAND71_ROWS: &[&str] = &["5.52", "Audit depth", "PH-S1349…S1358", "audit_depth"];

/// Audit adoption markers for band 71.
pub const AUDIT_BAND71_ROWS: &[&str] = &[
    "PH-S1349",
    "audit_depth",
    "PH-S1350",
    "POOLAI_AUDIT_STORE",
    "PH-S1352",
    "VERIFY_AUDIT",
    "PH-S1354",
    "--audit",
    "PH-S1358",
];

/// Classify audit band depth from optional feature stub (PH-S1349).
pub fn audit_depth_stub(features: Option<&Value>) -> AuditDepth {
    let Some(f) = features else {
        return AuditDepth::None;
    };
    let depth = f
        .get("audit_depth")
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
        .get("audit_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return AuditDepth::FullBand71;
    }
    if docs {
        return AuditDepth::DocsCanon;
    }
    if loc {
        return AuditDepth::LocAuditFlag;
    }
    if smoke {
        return AuditDepth::StandSmokeExport;
    }
    if verify {
        return AuditDepth::VerifyDevStandHook;
    }
    if api {
        return AuditDepth::ApiContracts;
    }
    if store {
        return AuditDepth::StoreWire;
    }
    if depth {
        return AuditDepth::DepthModule;
    }
    AuditDepth::None
}

/// Total audit criteria in registry (PH-S1349).
pub fn audit_criteria_total() -> usize {
    AUDIT_CRITERIA.len()
}

/// Env key for future durable audit event store (PH-S1349 scaffold).
pub const AUDIT_STORE_ENV: &str = "POOLAI_AUDIT_STORE";

/// Canonical store modes for band 71+ (file default; sqlite horizon band 72+).
pub const AUDIT_STORE_MODES: &[&str] = &["file", "sqlite"];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_depth_stub_ph_s1349() {
        assert_eq!(audit_depth_stub(None), AuditDepth::None);
        assert_eq!(
            audit_depth_stub(Some(&json!({"audit_depth": true}))),
            AuditDepth::DepthModule
        );
        assert_eq!(
            audit_depth_stub(Some(&json!({
                "audit_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_docs": true,
            }))),
            AuditDepth::FullBand71
        );
        assert_eq!(AUDIT_CRITERIA.len(), 8);
        assert_eq!(audit_criteria_total(), 8);
        assert_eq!(AUDIT_STORE_ENV, "POOLAI_AUDIT_STORE");
        assert!(AUDIT_STORE_MODES.contains(&"sqlite"));
        assert!(FM_BAND71_ROWS.contains(&"PH-S1349…S1358"));
    }
}
