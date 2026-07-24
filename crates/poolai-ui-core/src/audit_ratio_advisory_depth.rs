//! Audit ratio-advisory band depth (PH-S1429…S1438, band 79 — enterprise phase C).
//!
//! Aggregates prior `--audit*` loc-audit slices + vision-sync under one ratio-advisory gate.

use serde_json::Value;

/// Audit ratio-advisory depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditRatioAdvisoryDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand79,
}

/// Prior Audit loc-audit / canon slices covered by aggregate (PH-S1430).
pub const AUDIT_RATIO_ADVISORY_SLICES: &[&str] = &[
    "--audit-store",
    "--audit-api",
    "--audit-admin-ops",
    "--audit-docs-canon",
    "--audit-vision-sync",
    "AUDIT_VISION_SYNC.md",
];

/// Audit ratio-advisory criteria registry (PH-S1429): id · marker · doc path.
pub const AUDIT_RATIO_ADVISORY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_ratio_advisory_depth",
        "AuditRatioAdvisoryDepth",
        "crates/poolai-ui-core/src/audit_ratio_advisory_depth.rs",
    ),
    (
        "prior_audit_store",
        "--audit-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "prior_audit_api",
        "--audit-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_ratio_advisory",
        "AUDIT_RATIO_ADVISORY.md",
        "docs/development/AUDIT_RATIO_ADVISORY.md",
    ),
    (
        "doc_vision_sync",
        "AUDIT_VISION_SYNC.md",
        "docs/development/AUDIT_VISION_SYNC.md",
    ),
    (
        "aggregate_flag",
        "--audit-ratio-advisory",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_RATIO_ADVISORY",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "audit_ratio_advisory_integration",
        "tests/audit_ratio_advisory_integration.rs",
    ),
    (
        "doc_docs_canon",
        "AUDIT_DOCS_CANON.md",
        "docs/development/AUDIT_DOCS_CANON.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1429_integration",
        "tests/galaxy_horizon_s1429_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-ratio-advisory` case names (PH-S1434).
pub const AUDIT_RATIO_ADVISORY_CASES: &[&str] = &[
    "audit_ratio_advisory_depth",
    "prior_audit_store",
    "prior_audit_api",
    "doc_ratio_advisory",
    "doc_vision_sync",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "doc_docs_canon",
    "band_close",
];

/// FM §5.60 band-79 marker rows.
pub const FM_BAND79_ROWS: &[&str] = &[
    "5.60",
    "Audit ratio advisory",
    "PH-S1429…S1438",
    "audit_ratio_advisory_depth",
];

/// Audit ratio-advisory adoption markers for band 79.
pub const AUDIT_RATIO_ADVISORY_BAND79_ROWS: &[&str] = &[
    "PH-S1429",
    "audit_ratio_advisory_depth",
    "PH-S1430",
    "AUDIT_RATIO_ADVISORY_SLICES",
    "PH-S1431",
    "audit_ratio_advisory_integration",
    "PH-S1432",
    "VERIFY_AUDIT_RATIO_ADVISORY",
    "PH-S1434",
    "--audit-ratio-advisory",
    "PH-S1438",
];

/// Production-verify stub: how many ratio-advisory slices are referenced (PH-S1430).
pub fn audit_ratio_advisory_slices_met(canon_src: &str) -> (usize, usize) {
    let total = AUDIT_RATIO_ADVISORY_SLICES.len();
    let met = AUDIT_RATIO_ADVISORY_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Audit ratio-advisory band depth from optional feature stub (PH-S1429).
pub fn audit_ratio_advisory_depth_stub(features: Option<&Value>) -> AuditRatioAdvisoryDepth {
    let Some(f) = features else {
        return AuditRatioAdvisoryDepth::None;
    };
    let depth = f
        .get("audit_ratio_advisory_depth")
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
        .get("audit_ratio_advisory_docs")
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
        return AuditRatioAdvisoryDepth::FullBand79;
    }
    if close || ratio {
        return AuditRatioAdvisoryDepth::RatioHold;
    }
    if docs {
        return AuditRatioAdvisoryDepth::DocsCanon;
    }
    if loc {
        return AuditRatioAdvisoryDepth::LocAuditFlag;
    }
    if export {
        return AuditRatioAdvisoryDepth::StandSmokeExport;
    }
    if verify {
        return AuditRatioAdvisoryDepth::VerifyDevStandHook;
    }
    if contracts {
        return AuditRatioAdvisoryDepth::CriteriaContracts;
    }
    if slices {
        return AuditRatioAdvisoryDepth::SliceAggregate;
    }
    if depth {
        return AuditRatioAdvisoryDepth::DepthModule;
    }
    AuditRatioAdvisoryDepth::None
}

/// Total Audit ratio-advisory criteria in registry (PH-S1429).
pub fn audit_ratio_advisory_criteria_total() -> usize {
    AUDIT_RATIO_ADVISORY_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_ratio_advisory_depth_stub_ph_s1429() {
        assert_eq!(
            audit_ratio_advisory_depth_stub(None),
            AuditRatioAdvisoryDepth::None
        );
        assert_eq!(
            audit_ratio_advisory_depth_stub(Some(&json!({
                "audit_ratio_advisory_depth": true
            }))),
            AuditRatioAdvisoryDepth::DepthModule
        );
        assert_eq!(
            audit_ratio_advisory_depth_stub(Some(&json!({
                "audit_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditRatioAdvisoryDepth::FullBand79
        );
        assert_eq!(AUDIT_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(audit_ratio_advisory_criteria_total(), 10);
        assert_eq!(AUDIT_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(FM_BAND79_ROWS.contains(&"PH-S1429…S1438"));
    }

    #[test]
    fn audit_ratio_advisory_slices_met_ph_s1430() {
        let src = "--audit-store --audit-api --audit-admin-ops --audit-docs-canon --audit-vision-sync AUDIT_VISION_SYNC.md";
        assert_eq!(audit_ratio_advisory_slices_met(src), (6, 6));
        assert_eq!(audit_ratio_advisory_slices_met("--audit-store"), (1, 6));
    }
}
