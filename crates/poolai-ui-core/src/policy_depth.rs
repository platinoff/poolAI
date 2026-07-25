//! Policies depth scaffold (PH-S1449…S1458, band 81 — enterprise phase D).

use serde_json::Value;

/// Policy depth flags (production verify stub + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand81,
}

/// Policy criteria registry (PH-S1449): id · marker · doc path.
pub const POLICY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_depth",
        "PolicyDepth",
        "crates/poolai-ui-core/src/policy_depth.rs",
    ),
    (
        "store_wire",
        "POOLAI_POLICY_STORE",
        "src/enterprise/security.rs",
    ),
    (
        "api_contracts",
        "policy_depth_audit",
        "tests/policy_depth_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY",
        "bin/verify-dev-stand.sh",
    ),
    ("quick_flag", "--policy", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "policy_band81_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    ("loc_audit_flag", "--policy", "src/bin/poolai_loc_audit.rs"),
    (
        "policy_docs",
        "POLICIES_DEPTH.md",
        "docs/development/POLICIES_DEPTH.md",
    ),
];

/// `poolai-loc-audit --policy` case names (PH-S1454).
pub const POLICY_CASES: &[&str] = &[
    "policy_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "quick_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "policy_docs",
];

/// FM §5.62 band-81 marker rows.
pub const FM_BAND81_ROWS: &[&str] = &["5.62", "Policies depth", "PH-S1449…S1458", "policy_depth"];

/// Policy adoption markers for band 81.
pub const POLICY_BAND81_ROWS: &[&str] = &[
    "PH-S1449",
    "policy_depth",
    "PH-S1450",
    "POOLAI_POLICY_STORE",
    "PH-S1452",
    "VERIFY_POLICY",
    "PH-S1454",
    "--policy",
    "PH-S1458",
];

/// Classify policy band depth from optional feature stub (PH-S1449).
pub fn policy_depth_stub(features: Option<&Value>) -> PolicyDepth {
    let Some(f) = features else {
        return PolicyDepth::None;
    };
    let depth = f
        .get("policy_depth")
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
        .get("policy_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return PolicyDepth::FullBand81;
    }
    if docs {
        return PolicyDepth::DocsCanon;
    }
    if loc {
        return PolicyDepth::LocAuditFlag;
    }
    if smoke {
        return PolicyDepth::StandSmokeExport;
    }
    if verify {
        return PolicyDepth::VerifyDevStandHook;
    }
    if api {
        return PolicyDepth::ApiContracts;
    }
    if store {
        return PolicyDepth::StoreWire;
    }
    if depth {
        return PolicyDepth::DepthModule;
    }
    PolicyDepth::None
}

/// Total policy criteria in registry (PH-S1449).
pub fn policy_criteria_total() -> usize {
    POLICY_CRITERIA.len()
}

/// Env key for future durable security policy store (PH-S1449 scaffold).
pub const POLICY_STORE_ENV: &str = "POOLAI_POLICY_STORE";

/// Canonical store modes for band 81+ (memory default; sqlite horizon band 82+).
pub const POLICY_STORE_MODES: &[&str] = &["memory", "sqlite"];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_depth_stub_ph_s1449() {
        assert_eq!(policy_depth_stub(None), PolicyDepth::None);
        assert_eq!(
            policy_depth_stub(Some(&json!({"policy_depth": true}))),
            PolicyDepth::DepthModule
        );
        assert_eq!(
            policy_depth_stub(Some(&json!({
                "policy_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_docs": true,
            }))),
            PolicyDepth::FullBand81
        );
        assert_eq!(POLICY_CRITERIA.len(), 8);
        assert_eq!(policy_criteria_total(), 8);
        assert_eq!(POLICY_STORE_ENV, "POOLAI_POLICY_STORE");
        assert!(POLICY_STORE_MODES.contains(&"sqlite"));
        assert!(FM_BAND81_ROWS.contains(&"PH-S1449…S1458"));
    }
}
