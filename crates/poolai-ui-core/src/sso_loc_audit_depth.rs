//! SSO loc-audit aggregate band depth (PH-S1299…S1308, band 66 — enterprise phase B).
//!
//! Consolidates band 61–65 `--sso*` loc-audit slices under one aggregate gate.

use serde_json::Value;

/// SSO loc-audit depth flags (aggregate / slices / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoLocAuditDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand66,
}

/// Band 61–65 loc-audit slice flags covered by aggregate (PH-S1300).
pub const SSO_LOC_AUDIT_SLICES: &[&str] = &[
    "--sso",
    "--sso-store",
    "--sso-api",
    "--sso-admin-ops",
    "--sso-stand-smoke",
];

/// SSO loc-audit criteria registry (PH-S1299): id · marker · doc path.
pub const SSO_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_loc_audit_depth",
        "SsoLocAuditDepth",
        "crates/poolai-ui-core/src/sso_loc_audit_depth.rs",
    ),
    ("slice_sso", "--sso", "src/bin/poolai_loc_audit.rs"),
    ("slice_store", "--sso-store", "src/bin/poolai_loc_audit.rs"),
    ("slice_api", "--sso-api", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_admin_ops",
        "--sso-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_stand_smoke",
        "--sso-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "aggregate_flag",
        "--sso-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "sso_loc_audit_docs",
        "SSO_LOC_AUDIT.md",
        "docs/development/SSO_LOC_AUDIT.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1299_integration",
        "tests/galaxy_horizon_s1299_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-loc-audit` case names (PH-S1304).
pub const SSO_LOC_AUDIT_CASES: &[&str] = &[
    "sso_loc_audit_depth",
    "slice_sso",
    "slice_store",
    "slice_api",
    "slice_admin_ops",
    "slice_stand_smoke",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "sso_loc_audit_docs",
    "band_close",
];

/// FM §5.47 band-66 marker rows.
pub const FM_BAND66_ROWS: &[&str] = &[
    "5.47",
    "SSO loc-audit",
    "PH-S1299…S1308",
    "sso_loc_audit_depth",
];

/// SSO loc-audit adoption markers for band 66.
pub const SSO_LOC_AUDIT_BAND66_ROWS: &[&str] = &[
    "PH-S1299",
    "sso_loc_audit_depth",
    "PH-S1300",
    "SSO_LOC_AUDIT_SLICES",
    "PH-S1301",
    "sso_loc_audit_integration",
    "PH-S1302",
    "VERIFY_SSO_LOC_AUDIT",
    "PH-S1304",
    "--sso-loc-audit",
    "PH-S1308",
];

/// Production-verify stub: report how many of the five slice flags are present (PH-S1300).
pub fn sso_loc_audit_slices_met(loc_audit_src: &str) -> (usize, usize) {
    let total = SSO_LOC_AUDIT_SLICES.len();
    let met = SSO_LOC_AUDIT_SLICES
        .iter()
        .filter(|flag| loc_audit_src.contains(*flag))
        .count();
    (met, total)
}

/// Classify SSO loc-audit band depth from optional feature stub (PH-S1299).
pub fn sso_loc_audit_depth_stub(features: Option<&Value>) -> SsoLocAuditDepth {
    let Some(f) = features else {
        return SsoLocAuditDepth::None;
    };
    let depth = f
        .get("sso_loc_audit_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let slices = f
        .get("slice_aggregate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let contracts = f
        .get("criteria_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let export = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("sso_loc_audit_docs")
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

    if depth && slices && contracts && verify && export && loc && docs && ratio && close {
        return SsoLocAuditDepth::FullBand66;
    }
    if close || ratio {
        return SsoLocAuditDepth::RatioHold;
    }
    if docs {
        return SsoLocAuditDepth::DocsCanon;
    }
    if loc {
        return SsoLocAuditDepth::LocAuditFlag;
    }
    if export {
        return SsoLocAuditDepth::StandSmokeExport;
    }
    if verify {
        return SsoLocAuditDepth::VerifyDevStandHook;
    }
    if contracts {
        return SsoLocAuditDepth::CriteriaContracts;
    }
    if slices {
        return SsoLocAuditDepth::SliceAggregate;
    }
    if depth {
        return SsoLocAuditDepth::DepthModule;
    }
    SsoLocAuditDepth::None
}

/// Total SSO loc-audit criteria in registry (PH-S1299).
pub fn sso_loc_audit_criteria_total() -> usize {
    SSO_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_loc_audit_depth_stub_ph_s1299() {
        assert_eq!(sso_loc_audit_depth_stub(None), SsoLocAuditDepth::None);
        assert_eq!(
            sso_loc_audit_depth_stub(Some(&json!({"sso_loc_audit_depth": true}))),
            SsoLocAuditDepth::DepthModule
        );
        assert_eq!(
            sso_loc_audit_depth_stub(Some(&json!({
                "sso_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoLocAuditDepth::FullBand66
        );
        assert_eq!(SSO_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(sso_loc_audit_criteria_total(), 10);
        assert_eq!(SSO_LOC_AUDIT_SLICES.len(), 5);
        assert!(FM_BAND66_ROWS.contains(&"PH-S1299…S1308"));
    }

    #[test]
    fn sso_loc_audit_slices_met_ph_s1300() {
        let src = "--sso --sso-store --sso-api --sso-admin-ops --sso-stand-smoke";
        assert_eq!(sso_loc_audit_slices_met(src), (5, 5));
        assert_eq!(sso_loc_audit_slices_met("--sso"), (1, 5));
    }
}
