//! SSO admin/ops glue band depth (PH-S1279…S1288, band 64 — enterprise phase B).

use serde_json::Value;

/// SSO admin/ops glue depth flags (store strip / provider refresh / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoAdminOpsDepth {
    None,
    DepthModule,
    StoreStrip,
    ProvidersGlue,
    HtmlContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand64,
}

/// SSO admin/ops criteria registry (PH-S1279): id · marker · doc path.
pub const SSO_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_admin_ops_depth",
        "SsoAdminOpsDepth",
        "crates/poolai-ui-core/src/sso_admin_ops_depth.rs",
    ),
    ("store_strip", "sso-store-badge", "src/ui/admin/security.rs"),
    (
        "providers_glue",
        "refreshOAuth2Providers",
        "src/ui/admin/security.rs",
    ),
    (
        "html_contracts",
        "sso_admin_ops_integration",
        "tests/sso_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "sso_admin_ops_band64_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--sso-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "sso_admin_ops_docs",
        "SSO_ADMIN_OPS.md",
        "docs/development/SSO_ADMIN_OPS.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1279_integration",
        "tests/galaxy_horizon_s1279_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-admin-ops` case names (PH-S1285).
pub const SSO_ADMIN_OPS_CASES: &[&str] = &[
    "sso_admin_ops_depth",
    "store_strip",
    "providers_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "sso_admin_ops_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.45 band-64 marker rows.
pub const FM_BAND64_ROWS: &[&str] = &[
    "5.45",
    "SSO admin/ops glue",
    "PH-S1279…S1288",
    "sso_admin_ops_depth",
];

/// SSO admin/ops adoption markers for band 64.
pub const SSO_ADMIN_OPS_BAND64_ROWS: &[&str] = &[
    "PH-S1279",
    "sso_admin_ops_depth",
    "PH-S1280",
    "sso-store-badge",
    "PH-S1281",
    "refreshOAuth2Providers",
    "PH-S1284",
    "VERIFY_SSO_ADMIN_OPS",
    "PH-S1285",
    "--sso-admin-ops",
    "PH-S1288",
];

/// Classify SSO admin/ops band depth from optional feature stub (PH-S1279).
pub fn sso_admin_ops_depth_stub(features: Option<&Value>) -> SsoAdminOpsDepth {
    let Some(f) = features else {
        return SsoAdminOpsDepth::None;
    };
    let depth = f
        .get("sso_admin_ops_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_strip")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let providers = f
        .get("providers_glue")
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
        .get("sso_admin_ops_docs")
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

    if depth && store && providers && html && verify && smoke && loc && docs && ratio && close {
        return SsoAdminOpsDepth::FullBand64;
    }
    if close || ratio {
        return SsoAdminOpsDepth::RatioHold;
    }
    if docs {
        return SsoAdminOpsDepth::DocsCanon;
    }
    if loc {
        return SsoAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return SsoAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return SsoAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return SsoAdminOpsDepth::HtmlContracts;
    }
    if providers {
        return SsoAdminOpsDepth::ProvidersGlue;
    }
    if store {
        return SsoAdminOpsDepth::StoreStrip;
    }
    if depth {
        return SsoAdminOpsDepth::DepthModule;
    }
    SsoAdminOpsDepth::None
}

/// Total SSO admin/ops criteria in registry (PH-S1279).
pub fn sso_admin_ops_criteria_total() -> usize {
    SSO_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_admin_ops_depth_stub_ph_s1279() {
        assert_eq!(sso_admin_ops_depth_stub(None), SsoAdminOpsDepth::None);
        assert_eq!(
            sso_admin_ops_depth_stub(Some(&json!({"sso_admin_ops_depth": true}))),
            SsoAdminOpsDepth::DepthModule
        );
        assert_eq!(
            sso_admin_ops_depth_stub(Some(&json!({
                "sso_admin_ops_depth": true,
                "store_strip": true,
                "providers_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoAdminOpsDepth::FullBand64
        );
        assert_eq!(SSO_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(sso_admin_ops_criteria_total(), 10);
        assert!(FM_BAND64_ROWS.contains(&"PH-S1279…S1288"));
    }
}
