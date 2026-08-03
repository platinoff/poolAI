//! Ratio96 admin/ops glue band depth (PH-S1679…S1688, band 104 — phase F).
//!
//! Mirrors the band-94 monitoring admin/ops slice: a dashboard store strip that reads the
//! durable ratio store (`docs/development/rust_ratio.json`) via `GET /api/v1/ops/ratio96`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// Ratio96 admin/ops glue depth flags (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio96AdminOpsDepth {
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
    FullBand104,
}

/// Ratio96 admin/ops criteria registry (PH-S1679): id · marker · doc path.
pub const RATIO96_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "ratio96_admin_ops_depth",
        "Ratio96AdminOpsDepth",
        "crates/poolai-ui-core/src/ratio96_admin_ops_depth.rs",
    ),
    (
        "store_strip",
        "ratio96-store-badge",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "query_ops_glue",
        "refreshRatio96",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "html_contracts",
        "ratio96_admin_ops_integration",
        "tests/ratio96_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_RATIO96_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "ratio96_admin_ops_band104_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--ratio96-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "RATIO96_ADMIN_OPS.md",
        "docs/development/RATIO96_ADMIN_OPS.md",
    ),
    (
        "ratio_hold",
        "ratio96-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1679_integration",
        "tests/galaxy_horizon_s1679_integration.rs",
    ),
];

/// `poolai-loc-audit --ratio96-admin-ops` case names (PH-S1684).
pub const RATIO96_ADMIN_OPS_CASES: &[&str] = &[
    "ratio96_admin_ops_depth",
    "store_strip",
    "query_ops_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "ratio_hold",
    "band_close",
];

/// FM §5.85 band-104 marker rows.
pub const FM_BAND104_ROWS: &[&str] = &[
    "5.85",
    "Ratio96 admin/ops glue",
    "PH-S1679…S1688",
    "ratio96_admin_ops_depth",
];

/// Ratio96 admin/ops adoption markers for band 104.
pub const RATIO96_ADMIN_OPS_BAND104_ROWS: &[&str] = &[
    "PH-S1679",
    "ratio96_admin_ops_depth",
    "PH-S1680",
    "ratio96_store_wire_json",
    "PH-S1681",
    "ratio96_admin_ops_contracts",
    "PH-S1682",
    "ratio96-store-badge",
    "PH-S1684",
    "--ratio96-admin-ops",
    "PH-S1688",
];

/// Classify ratio96 admin/ops band depth from optional feature stub (PH-S1679).
pub fn ratio96_admin_ops_depth_stub(features: Option<&Value>) -> Ratio96AdminOpsDepth {
    let Some(f) = features else {
        return Ratio96AdminOpsDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("ratio96_admin_ops_depth");
    let store = enabled("store_strip");
    let query = enabled("query_ops_glue");
    let html = enabled("html_contracts");
    let verify = enabled("verify_dev_stand_hook");
    let smoke = enabled("stand_smoke_export");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("docs_canon");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && store && query && html && verify && smoke && loc && docs && ratio && close {
        return Ratio96AdminOpsDepth::FullBand104;
    }
    if close || ratio {
        return Ratio96AdminOpsDepth::RatioHold;
    }
    if docs {
        return Ratio96AdminOpsDepth::DocsCanon;
    }
    if loc {
        return Ratio96AdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return Ratio96AdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return Ratio96AdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return Ratio96AdminOpsDepth::HtmlContracts;
    }
    if query {
        return Ratio96AdminOpsDepth::QueryOpsGlue;
    }
    if store {
        return Ratio96AdminOpsDepth::StoreStrip;
    }
    if depth {
        return Ratio96AdminOpsDepth::DepthModule;
    }
    Ratio96AdminOpsDepth::None
}

/// Total ratio96 admin/ops criteria in registry (PH-S1679).
pub fn ratio96_admin_ops_criteria_total() -> usize {
    RATIO96_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_admin_ops_depth_stub_ph_s1679() {
        assert_eq!(
            ratio96_admin_ops_depth_stub(None),
            Ratio96AdminOpsDepth::None
        );
        assert_eq!(
            ratio96_admin_ops_depth_stub(Some(&json!({"ratio96_admin_ops_depth": true}))),
            Ratio96AdminOpsDepth::DepthModule
        );
        assert_eq!(
            ratio96_admin_ops_depth_stub(Some(&json!({
                "ratio96_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "docs_canon": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            Ratio96AdminOpsDepth::FullBand104
        );
        assert_eq!(RATIO96_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(ratio96_admin_ops_criteria_total(), 10);
        assert!(FM_BAND104_ROWS.contains(&"PH-S1679…S1688"));
    }
}
