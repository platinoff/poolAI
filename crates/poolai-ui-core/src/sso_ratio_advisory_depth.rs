//! SSO ratio-advisory band depth (PH-S1329…S1338, band 69 — enterprise phase B).
//!
//! Aggregates prior `--sso*` loc-audit slices + vision-sync under one ratio-advisory gate.

use serde_json::Value;

/// SSO ratio-advisory depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoRatioAdvisoryDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand69,
}

/// Prior SSO loc-audit / canon slices covered by aggregate (PH-S1330).
pub const SSO_RATIO_ADVISORY_SLICES: &[&str] = &[
    "--sso-store",
    "--sso-api",
    "--sso-admin-ops",
    "--sso-docs-canon",
    "--sso-vision-sync",
    "SSO_VISION_SYNC.md",
];

/// SSO ratio-advisory criteria registry (PH-S1329): id · marker · doc path.
pub const SSO_RATIO_ADVISORY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_ratio_advisory_depth",
        "SsoRatioAdvisoryDepth",
        "crates/poolai-ui-core/src/sso_ratio_advisory_depth.rs",
    ),
    (
        "prior_sso_store",
        "--sso-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    ("prior_sso_api", "--sso-api", "src/bin/poolai_loc_audit.rs"),
    (
        "doc_ratio_advisory",
        "SSO_RATIO_ADVISORY.md",
        "docs/development/SSO_RATIO_ADVISORY.md",
    ),
    (
        "doc_vision_sync",
        "SSO_VISION_SYNC.md",
        "docs/development/SSO_VISION_SYNC.md",
    ),
    (
        "aggregate_flag",
        "--sso-ratio-advisory",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_RATIO_ADVISORY",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "sso_ratio_advisory_integration",
        "tests/sso_ratio_advisory_integration.rs",
    ),
    (
        "doc_docs_canon",
        "SSO_DOCS_CANON.md",
        "docs/development/SSO_DOCS_CANON.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1329_integration",
        "tests/galaxy_horizon_s1329_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-ratio-advisory` case names (PH-S1334).
pub const SSO_RATIO_ADVISORY_CASES: &[&str] = &[
    "sso_ratio_advisory_depth",
    "prior_sso_store",
    "prior_sso_api",
    "doc_ratio_advisory",
    "doc_vision_sync",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "doc_docs_canon",
    "band_close",
];

/// FM §5.50 band-69 marker rows.
pub const FM_BAND69_ROWS: &[&str] = &[
    "5.50",
    "SSO ratio advisory",
    "PH-S1329…S1338",
    "sso_ratio_advisory_depth",
];

/// SSO ratio-advisory adoption markers for band 69.
pub const SSO_RATIO_ADVISORY_BAND69_ROWS: &[&str] = &[
    "PH-S1329",
    "sso_ratio_advisory_depth",
    "PH-S1330",
    "SSO_RATIO_ADVISORY_SLICES",
    "PH-S1331",
    "sso_ratio_advisory_integration",
    "PH-S1332",
    "VERIFY_SSO_RATIO_ADVISORY",
    "PH-S1334",
    "--sso-ratio-advisory",
    "PH-S1338",
];

/// Production-verify stub: how many ratio-advisory slices are referenced (PH-S1330).
pub fn sso_ratio_advisory_slices_met(canon_src: &str) -> (usize, usize) {
    let total = SSO_RATIO_ADVISORY_SLICES.len();
    let met = SSO_RATIO_ADVISORY_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify SSO ratio-advisory band depth from optional feature stub (PH-S1329).
pub fn sso_ratio_advisory_depth_stub(features: Option<&Value>) -> SsoRatioAdvisoryDepth {
    let Some(f) = features else {
        return SsoRatioAdvisoryDepth::None;
    };
    let depth = f
        .get("sso_ratio_advisory_depth")
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
        .get("sso_ratio_advisory_docs")
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
        return SsoRatioAdvisoryDepth::FullBand69;
    }
    if close || ratio {
        return SsoRatioAdvisoryDepth::RatioHold;
    }
    if docs {
        return SsoRatioAdvisoryDepth::DocsCanon;
    }
    if loc {
        return SsoRatioAdvisoryDepth::LocAuditFlag;
    }
    if export {
        return SsoRatioAdvisoryDepth::StandSmokeExport;
    }
    if verify {
        return SsoRatioAdvisoryDepth::VerifyDevStandHook;
    }
    if contracts {
        return SsoRatioAdvisoryDepth::CriteriaContracts;
    }
    if slices {
        return SsoRatioAdvisoryDepth::SliceAggregate;
    }
    if depth {
        return SsoRatioAdvisoryDepth::DepthModule;
    }
    SsoRatioAdvisoryDepth::None
}

/// Total SSO ratio-advisory criteria in registry (PH-S1329).
pub fn sso_ratio_advisory_criteria_total() -> usize {
    SSO_RATIO_ADVISORY_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_ratio_advisory_depth_stub_ph_s1329() {
        assert_eq!(
            sso_ratio_advisory_depth_stub(None),
            SsoRatioAdvisoryDepth::None
        );
        assert_eq!(
            sso_ratio_advisory_depth_stub(Some(&json!({
                "sso_ratio_advisory_depth": true
            }))),
            SsoRatioAdvisoryDepth::DepthModule
        );
        assert_eq!(
            sso_ratio_advisory_depth_stub(Some(&json!({
                "sso_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoRatioAdvisoryDepth::FullBand69
        );
        assert_eq!(SSO_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(sso_ratio_advisory_criteria_total(), 10);
        assert_eq!(SSO_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(FM_BAND69_ROWS.contains(&"PH-S1329…S1338"));
    }

    #[test]
    fn sso_ratio_advisory_slices_met_ph_s1330() {
        let src = "--sso-store --sso-api --sso-admin-ops --sso-docs-canon --sso-vision-sync SSO_VISION_SYNC.md";
        assert_eq!(sso_ratio_advisory_slices_met(src), (6, 6));
        assert_eq!(sso_ratio_advisory_slices_met("--sso-store"), (1, 6));
    }
}
