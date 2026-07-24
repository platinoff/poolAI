//! Audit admin/ops glue band depth (PH-S1379…S1388, band 74 — enterprise phase C).

use serde_json::Value;

/// Audit admin/ops glue depth flags (store strip / query refresh / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAdminOpsDepth {
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
    FullBand74,
}

/// Audit admin/ops criteria registry (PH-S1379): id · marker · doc path.
pub const AUDIT_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_admin_ops_depth",
        "AuditAdminOpsDepth",
        "crates/poolai-ui-core/src/audit_admin_ops_depth.rs",
    ),
    ("store_strip", "audit-store-badge", "src/ui/admin/audit.rs"),
    (
        "query_ops_glue",
        "refreshAuditEvents",
        "src/ui/admin/audit.rs",
    ),
    (
        "html_contracts",
        "audit_admin_ops_integration",
        "tests/audit_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "audit_admin_ops_band74_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--audit-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "audit_admin_ops_docs",
        "AUDIT_ADMIN_OPS.md",
        "docs/development/AUDIT_ADMIN_OPS.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1379_integration",
        "tests/galaxy_horizon_s1379_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-admin-ops` case names (PH-S1385).
pub const AUDIT_ADMIN_OPS_CASES: &[&str] = &[
    "audit_admin_ops_depth",
    "store_strip",
    "query_ops_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "audit_admin_ops_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.55 band-74 marker rows.
pub const FM_BAND74_ROWS: &[&str] = &[
    "5.55",
    "Audit admin/ops glue",
    "PH-S1379…S1388",
    "audit_admin_ops_depth",
];

/// Audit admin/ops adoption markers for band 74.
pub const AUDIT_ADMIN_OPS_BAND74_ROWS: &[&str] = &[
    "PH-S1379",
    "audit_admin_ops_depth",
    "PH-S1380",
    "audit-store-badge",
    "PH-S1381",
    "refreshAuditEvents",
    "PH-S1384",
    "VERIFY_AUDIT_ADMIN_OPS",
    "PH-S1385",
    "--audit-admin-ops",
    "PH-S1388",
];

/// Classify audit admin/ops band depth from optional feature stub (PH-S1379).
pub fn audit_admin_ops_depth_stub(features: Option<&Value>) -> AuditAdminOpsDepth {
    let Some(f) = features else {
        return AuditAdminOpsDepth::None;
    };
    let depth = f
        .get("audit_admin_ops_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_strip")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let query = f
        .get("query_ops_glue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let html = f
        .get("html_contracts")
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
        .get("audit_admin_ops_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_hold")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let close = f
        .get("band_close")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && query && html && verify && smoke && loc && docs && ratio && close {
        return AuditAdminOpsDepth::FullBand74;
    }
    if close || ratio {
        return AuditAdminOpsDepth::RatioHold;
    }
    if docs {
        return AuditAdminOpsDepth::DocsCanon;
    }
    if loc {
        return AuditAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return AuditAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return AuditAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return AuditAdminOpsDepth::HtmlContracts;
    }
    if query {
        return AuditAdminOpsDepth::QueryOpsGlue;
    }
    if store {
        return AuditAdminOpsDepth::StoreStrip;
    }
    if depth {
        return AuditAdminOpsDepth::DepthModule;
    }
    AuditAdminOpsDepth::None
}

/// Total audit admin/ops criteria in registry (PH-S1379).
pub fn audit_admin_ops_criteria_total() -> usize {
    AUDIT_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_admin_ops_depth_stub_ph_s1379() {
        assert_eq!(audit_admin_ops_depth_stub(None), AuditAdminOpsDepth::None);
        assert_eq!(
            audit_admin_ops_depth_stub(Some(&json!({"audit_admin_ops_depth": true}))),
            AuditAdminOpsDepth::DepthModule
        );
        assert_eq!(
            audit_admin_ops_depth_stub(Some(&json!({
                "audit_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditAdminOpsDepth::FullBand74
        );
        assert_eq!(AUDIT_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(audit_admin_ops_criteria_total(), 10);
        assert!(FM_BAND74_ROWS.contains(&"PH-S1379…S1388"));
    }
}
