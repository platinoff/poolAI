//! SSO horizon-close band depth (PH-S1339…S1348, band 70 — enterprise phase B close).
//!
//! Aggregates bands 61–69 `--sso*` loc-audit slices under one horizon gate,
//! closing SSO phase B before Audit (band 71).

use serde_json::Value;

/// SSO horizon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoHorizonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand70,
}

/// Phase-B SSO loc-audit / canon slices covered by horizon aggregate (PH-S1340).
pub const SSO_HORIZON_SLICES: &[&str] = &[
    "--sso",
    "--sso-store",
    "--sso-api",
    "--sso-admin-ops",
    "--sso-stand-smoke",
    "--sso-loc-audit",
    "--sso-docs-canon",
    "--sso-vision-sync",
    "--sso-ratio-advisory",
    "SSO_RATIO_ADVISORY.md",
];

/// SSO horizon criteria registry (PH-S1339): id · marker · doc path.
pub const SSO_HORIZON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_horizon_depth",
        "SsoHorizonDepth",
        "crates/poolai-ui-core/src/sso_horizon_depth.rs",
    ),
    (
        "phase_b_slices",
        "SSO_HORIZON_SLICES",
        "crates/poolai-ui-core/src/sso_horizon_depth.rs",
    ),
    (
        "prior_sso_store",
        "--sso-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_sso_horizon",
        "SSO_HORIZON.md",
        "docs/development/SSO_HORIZON.md",
    ),
    (
        "doc_ratio_advisory",
        "SSO_RATIO_ADVISORY.md",
        "docs/development/SSO_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--sso-horizon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_HORIZON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "sso_horizon_integration",
        "tests/sso_horizon_integration.rs",
    ),
    (
        "ratio_advisory_prior",
        "sso_ratio_advisory_depth",
        "crates/poolai-ui-core/src/sso_ratio_advisory_depth.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1339_integration",
        "tests/galaxy_horizon_s1339_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-horizon` case names (PH-S1344).
pub const SSO_HORIZON_CASES: &[&str] = &[
    "sso_horizon_depth",
    "phase_b_slices",
    "prior_sso_store",
    "doc_sso_horizon",
    "doc_ratio_advisory",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "ratio_advisory_prior",
    "band_close",
];

/// FM §5.51 band-70 marker rows.
pub const FM_BAND70_ROWS: &[&str] = &[
    "5.51",
    "SSO horizon close",
    "PH-S1339…S1348",
    "sso_horizon_depth",
];

/// SSO horizon adoption markers for band 70.
pub const SSO_HORIZON_BAND70_ROWS: &[&str] = &[
    "PH-S1339",
    "sso_horizon_depth",
    "PH-S1340",
    "SSO_HORIZON_SLICES",
    "PH-S1341",
    "sso_horizon_integration",
    "PH-S1342",
    "VERIFY_SSO_HORIZON",
    "PH-S1344",
    "--sso-horizon",
    "PH-S1348",
];

/// Production-verify stub: how many horizon slices are referenced (PH-S1340).
pub fn sso_horizon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = SSO_HORIZON_SLICES.len();
    let met = SSO_HORIZON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify SSO horizon band depth from optional feature stub (PH-S1339).
pub fn sso_horizon_depth_stub(features: Option<&Value>) -> SsoHorizonDepth {
    let Some(f) = features else {
        return SsoHorizonDepth::None;
    };
    let depth = f
        .get("sso_horizon_depth")
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
        .get("sso_horizon_docs")
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
        return SsoHorizonDepth::FullBand70;
    }
    if close || ratio {
        return SsoHorizonDepth::RatioHold;
    }
    if docs {
        return SsoHorizonDepth::DocsCanon;
    }
    if loc {
        return SsoHorizonDepth::LocAuditFlag;
    }
    if export {
        return SsoHorizonDepth::StandSmokeExport;
    }
    if verify {
        return SsoHorizonDepth::VerifyDevStandHook;
    }
    if contracts {
        return SsoHorizonDepth::CriteriaContracts;
    }
    if slices {
        return SsoHorizonDepth::SliceAggregate;
    }
    if depth {
        return SsoHorizonDepth::DepthModule;
    }
    SsoHorizonDepth::None
}

/// Total SSO horizon criteria in registry (PH-S1339).
pub fn sso_horizon_criteria_total() -> usize {
    SSO_HORIZON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_horizon_depth_stub_ph_s1339() {
        assert_eq!(sso_horizon_depth_stub(None), SsoHorizonDepth::None);
        assert_eq!(
            sso_horizon_depth_stub(Some(&json!({
                "sso_horizon_depth": true
            }))),
            SsoHorizonDepth::DepthModule
        );
        assert_eq!(
            sso_horizon_depth_stub(Some(&json!({
                "sso_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoHorizonDepth::FullBand70
        );
        assert_eq!(SSO_HORIZON_CRITERIA.len(), 10);
        assert_eq!(sso_horizon_criteria_total(), 10);
        assert_eq!(SSO_HORIZON_SLICES.len(), 10);
        assert!(FM_BAND70_ROWS.contains(&"PH-S1339…S1348"));
    }

    #[test]
    fn sso_horizon_slices_met_ph_s1340() {
        let src = "--sso --sso-store --sso-api --sso-admin-ops --sso-stand-smoke --sso-loc-audit --sso-docs-canon --sso-vision-sync --sso-ratio-advisory SSO_RATIO_ADVISORY.md";
        assert_eq!(sso_horizon_slices_met(src), (10, 10));
        assert_eq!(sso_horizon_slices_met("--sso"), (1, 10));
    }
}
