//! Policies ratio-advisory band depth (PH-S1529…S1538, band 89 — enterprise phase D).
//!
//! Aggregates prior `--policy*` loc-audit slices + vision-sync under one ratio-advisory gate.

use serde_json::Value;

/// Policies ratio-advisory depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRatioAdvisoryDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand89,
}

/// Prior Policies loc-audit / canon slices covered by aggregate (PH-S1530).
pub const POLICY_RATIO_ADVISORY_SLICES: &[&str] = &[
    "--policy-store",
    "--policy-api",
    "--policy-admin-ops",
    "--policy-docs-canon",
    "--policy-vision-sync",
    "POLICIES_VISION_SYNC.md",
];

/// Policies ratio-advisory criteria registry (PH-S1529): id · marker · doc path.
pub const POLICY_RATIO_ADVISORY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_ratio_advisory_depth",
        "PolicyRatioAdvisoryDepth",
        "crates/poolai-ui-core/src/policy_ratio_advisory_depth.rs",
    ),
    (
        "prior_policy_store",
        "--policy-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "prior_policy_api",
        "--policy-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_ratio_advisory",
        "POLICIES_RATIO_ADVISORY.md",
        "docs/development/POLICIES_RATIO_ADVISORY.md",
    ),
    (
        "doc_vision_sync",
        "POLICIES_VISION_SYNC.md",
        "docs/development/POLICIES_VISION_SYNC.md",
    ),
    (
        "aggregate_flag",
        "--policy-ratio-advisory",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_RATIO_ADVISORY",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "policy_ratio_advisory_integration",
        "tests/policy_ratio_advisory_integration.rs",
    ),
    (
        "doc_docs_canon",
        "POLICIES_DOCS_CANON.md",
        "docs/development/POLICIES_DOCS_CANON.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1529_integration",
        "tests/galaxy_horizon_s1529_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-ratio-advisory` case names (PH-S1534).
pub const POLICY_RATIO_ADVISORY_CASES: &[&str] = &[
    "policy_ratio_advisory_depth",
    "prior_policy_store",
    "prior_policy_api",
    "doc_ratio_advisory",
    "doc_vision_sync",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "doc_docs_canon",
    "band_close",
];

/// FM §5.70 band-89 marker rows.
pub const FM_BAND89_ROWS: &[&str] = &[
    "5.70",
    "Policies ratio advisory",
    "PH-S1529…S1538",
    "policy_ratio_advisory_depth",
];

/// Policies ratio-advisory adoption markers for band 89.
pub const POLICY_RATIO_ADVISORY_BAND89_ROWS: &[&str] = &[
    "PH-S1529",
    "policy_ratio_advisory_depth",
    "PH-S1530",
    "POLICY_RATIO_ADVISORY_SLICES",
    "PH-S1531",
    "policy_ratio_advisory_integration",
    "PH-S1532",
    "VERIFY_POLICY_RATIO_ADVISORY",
    "PH-S1534",
    "--policy-ratio-advisory",
    "PH-S1538",
];

/// Production-verify stub: how many ratio-advisory slices are referenced (PH-S1530).
pub fn policy_ratio_advisory_slices_met(canon_src: &str) -> (usize, usize) {
    let total = POLICY_RATIO_ADVISORY_SLICES.len();
    let met = POLICY_RATIO_ADVISORY_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Policies ratio-advisory band depth from optional feature stub (PH-S1529).
pub fn policy_ratio_advisory_depth_stub(features: Option<&Value>) -> PolicyRatioAdvisoryDepth {
    let Some(f) = features else {
        return PolicyRatioAdvisoryDepth::None;
    };
    let depth = f
        .get("policy_ratio_advisory_depth")
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
        .get("policy_ratio_advisory_docs")
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
        return PolicyRatioAdvisoryDepth::FullBand89;
    }
    if close || ratio {
        return PolicyRatioAdvisoryDepth::RatioHold;
    }
    if docs {
        return PolicyRatioAdvisoryDepth::DocsCanon;
    }
    if loc {
        return PolicyRatioAdvisoryDepth::LocAuditFlag;
    }
    if export {
        return PolicyRatioAdvisoryDepth::StandSmokeExport;
    }
    if verify {
        return PolicyRatioAdvisoryDepth::VerifyDevStandHook;
    }
    if contracts {
        return PolicyRatioAdvisoryDepth::CriteriaContracts;
    }
    if slices {
        return PolicyRatioAdvisoryDepth::SliceAggregate;
    }
    if depth {
        return PolicyRatioAdvisoryDepth::DepthModule;
    }
    PolicyRatioAdvisoryDepth::None
}

/// Total Policies ratio-advisory criteria in registry (PH-S1529).
pub fn policy_ratio_advisory_criteria_total() -> usize {
    POLICY_RATIO_ADVISORY_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_ratio_advisory_depth_stub_ph_s1529() {
        assert_eq!(
            policy_ratio_advisory_depth_stub(None),
            PolicyRatioAdvisoryDepth::None
        );
        assert_eq!(
            policy_ratio_advisory_depth_stub(Some(&json!({
                "policy_ratio_advisory_depth": true
            }))),
            PolicyRatioAdvisoryDepth::DepthModule
        );
        assert_eq!(
            policy_ratio_advisory_depth_stub(Some(&json!({
                "policy_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyRatioAdvisoryDepth::FullBand89
        );
        assert_eq!(POLICY_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(policy_ratio_advisory_criteria_total(), 10);
        assert_eq!(POLICY_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(FM_BAND89_ROWS.contains(&"PH-S1529…S1538"));
    }

    #[test]
    fn policy_ratio_advisory_slices_met_ph_s1530() {
        let src = "--policy-store --policy-api --policy-admin-ops --policy-docs-canon --policy-vision-sync POLICIES_VISION_SYNC.md";
        assert_eq!(policy_ratio_advisory_slices_met(src), (6, 6));
        assert_eq!(policy_ratio_advisory_slices_met("--policy-store"), (1, 6));
    }
}
