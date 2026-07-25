//! Policies store-wire band depth (PH-S1459…S1468, band 82 — enterprise phase D).

use serde_json::Value;

/// Policies store-wire depth flags (durable path wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyStoreDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand82,
}

/// Policies store-wire criteria registry (PH-S1459): id · marker · doc path.
pub const POLICY_STORE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_store_depth",
        "PolicyStoreDepth",
        "crates/poolai-ui-core/src/policy_store_depth.rs",
    ),
    (
        "store_wire",
        "policy_store_wire",
        "src/enterprise/security.rs",
    ),
    (
        "api_contracts",
        "policy_store_wire_integration",
        "tests/policy_store_wire_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_STORE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "policy_store_band82_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--policy-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "policy_store_docs",
        "POLICIES_STORE.md",
        "docs/development/POLICIES_STORE.md",
    ),
];

/// `poolai-loc-audit --policy-store` case names (PH-S1464).
pub const POLICY_STORE_CASES: &[&str] = &[
    "policy_store_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "policy_store_docs",
];

/// FM §5.63 band-82 marker rows.
pub const FM_BAND82_ROWS: &[&str] = &[
    "5.63",
    "Policies store wire",
    "PH-S1459…S1468",
    "policy_store_depth",
];

/// Policies store-wire adoption markers for band 82.
pub const POLICY_BAND82_ROWS: &[&str] = &[
    "PH-S1459",
    "policy_store_depth",
    "PH-S1460",
    "policy_store_wire",
    "PH-S1462",
    "VERIFY_POLICY_STORE",
    "PH-S1464",
    "--policy-store",
    "PH-S1468",
];

/// Classify policies store-wire band depth from optional feature stub (PH-S1459).
pub fn policy_store_depth_stub(features: Option<&Value>) -> PolicyStoreDepth {
    let Some(f) = features else {
        return PolicyStoreDepth::None;
    };
    let depth = f
        .get("policy_store_depth")
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
        .get("policy_store_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return PolicyStoreDepth::FullBand82;
    }
    if docs {
        return PolicyStoreDepth::DocsCanon;
    }
    if loc {
        return PolicyStoreDepth::LocAuditFlag;
    }
    if smoke {
        return PolicyStoreDepth::StandSmokeExport;
    }
    if verify {
        return PolicyStoreDepth::VerifyDevStandHook;
    }
    if api {
        return PolicyStoreDepth::ApiContracts;
    }
    if store {
        return PolicyStoreDepth::StoreWire;
    }
    if depth {
        return PolicyStoreDepth::DepthModule;
    }
    PolicyStoreDepth::None
}

/// Total policies store-wire criteria in registry (PH-S1459).
pub fn policy_store_criteria_total() -> usize {
    POLICY_STORE_CRITERIA.len()
}

/// Env key for policy store backend (shared with band 81 scaffold).
pub const POLICY_STORE_ENV: &str = "POOLAI_POLICY_STORE";

/// Env key for durable policy data directory (band 82 wire).
pub const POLICY_DATA_DIR_ENV: &str = "POOLAI_POLICY_DATA_DIR";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_store_depth_stub_ph_s1459() {
        assert_eq!(policy_store_depth_stub(None), PolicyStoreDepth::None);
        assert_eq!(
            policy_store_depth_stub(Some(&json!({"policy_store_depth": true}))),
            PolicyStoreDepth::DepthModule
        );
        assert_eq!(
            policy_store_depth_stub(Some(&json!({
                "policy_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_store_docs": true,
            }))),
            PolicyStoreDepth::FullBand82
        );
        assert_eq!(POLICY_STORE_CRITERIA.len(), 7);
        assert_eq!(policy_store_criteria_total(), 7);
        assert_eq!(POLICY_STORE_ENV, "POOLAI_POLICY_STORE");
        assert_eq!(POLICY_DATA_DIR_ENV, "POOLAI_POLICY_DATA_DIR");
        assert!(FM_BAND82_ROWS.contains(&"PH-S1459…S1468"));
    }
}
