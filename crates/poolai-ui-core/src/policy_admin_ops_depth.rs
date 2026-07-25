//! Policies admin/ops glue band depth (PH-S1479…S1488, band 84 — enterprise phase D).

use serde_json::Value;

/// Policies admin/ops glue depth flags (store strip / query refresh / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyAdminOpsDepth {
    None,
    DepthModule,
    StoreStrip,
    QueryOpsGlue,
    HtmlContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand84,
}

/// Policies admin/ops criteria registry (PH-S1479): id · marker · doc path.
pub const POLICY_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_admin_ops_depth",
        "PolicyAdminOpsDepth",
        "crates/poolai-ui-core/src/policy_admin_ops_depth.rs",
    ),
    (
        "store_strip",
        "policy-store-badge",
        "src/ui/admin/security.rs",
    ),
    (
        "query_ops_glue",
        "refreshSecurityPolicies",
        "src/ui/admin/security.rs",
    ),
    (
        "html_contracts",
        "policy_admin_ops_integration",
        "tests/policy_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "policy_admin_ops_band84_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--policy-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "policy_admin_ops_docs",
        "POLICIES_ADMIN_OPS.md",
        "docs/development/POLICIES_ADMIN_OPS.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1479_integration",
        "tests/galaxy_horizon_s1479_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-admin-ops` case names (PH-S1485).
pub const POLICY_ADMIN_OPS_CASES: &[&str] = &[
    "policy_admin_ops_depth",
    "store_strip",
    "query_ops_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "policy_admin_ops_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.65 band-84 marker rows.
pub const FM_BAND84_ROWS: &[&str] = &[
    "5.65",
    "Policies admin/ops glue",
    "PH-S1479…S1488",
    "policy_admin_ops_depth",
];

/// Policies admin/ops adoption markers for band 84.
pub const POLICY_ADMIN_OPS_BAND84_ROWS: &[&str] = &[
    "PH-S1479",
    "policy_admin_ops_depth",
    "PH-S1480",
    "policy-store-badge",
    "PH-S1481",
    "refreshSecurityPolicies",
    "PH-S1484",
    "VERIFY_POLICY_ADMIN_OPS",
    "PH-S1485",
    "--policy-admin-ops",
    "PH-S1488",
];

/// Classify policies admin/ops band depth from optional feature stub (PH-S1479).
pub fn policy_admin_ops_depth_stub(features: Option<&Value>) -> PolicyAdminOpsDepth {
    let Some(f) = features else {
        return PolicyAdminOpsDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("policy_admin_ops_depth");
    let store = enabled("store_strip");
    let query = enabled("query_ops_glue");
    let html = enabled("html_contracts");
    let verify = enabled("verify_dev_stand_hook");
    let smoke = enabled("stand_smoke_export");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("policy_admin_ops_docs");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && store && query && html && verify && smoke && loc && docs && ratio && close {
        return PolicyAdminOpsDepth::FullBand84;
    }
    if close || ratio {
        return PolicyAdminOpsDepth::RatioHold;
    }
    if docs {
        return PolicyAdminOpsDepth::DocsCanon;
    }
    if loc {
        return PolicyAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return PolicyAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return PolicyAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return PolicyAdminOpsDepth::HtmlContracts;
    }
    if query {
        return PolicyAdminOpsDepth::QueryOpsGlue;
    }
    if store {
        return PolicyAdminOpsDepth::StoreStrip;
    }
    if depth {
        return PolicyAdminOpsDepth::DepthModule;
    }
    PolicyAdminOpsDepth::None
}

/// Total policies admin/ops criteria in registry (PH-S1479).
pub fn policy_admin_ops_criteria_total() -> usize {
    POLICY_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_admin_ops_depth_stub_ph_s1479() {
        assert_eq!(policy_admin_ops_depth_stub(None), PolicyAdminOpsDepth::None);
        assert_eq!(
            policy_admin_ops_depth_stub(Some(&json!({"policy_admin_ops_depth": true}))),
            PolicyAdminOpsDepth::DepthModule
        );
        assert_eq!(
            policy_admin_ops_depth_stub(Some(&json!({
                "policy_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyAdminOpsDepth::FullBand84
        );
        assert_eq!(POLICY_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(policy_admin_ops_criteria_total(), 10);
        assert!(FM_BAND84_ROWS.contains(&"PH-S1479…S1488"));
    }
}
