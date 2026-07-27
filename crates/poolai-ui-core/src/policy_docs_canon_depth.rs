//! Policies docs-canon band depth (PH-S1509…S1518, band 87 — enterprise phase D).
//!
//! Consolidates band 81–86 `POLICIES_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// Policies docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand87,
}

/// Band 81–86 Policies canon doc filenames covered by aggregate (PH-S1510).
pub const POLICY_DOCS_CANON_SLICES: &[&str] = &[
    "POLICIES_DEPTH.md",
    "POLICIES_STORE.md",
    "POLICIES_API.md",
    "POLICIES_ADMIN_OPS.md",
    "POLICIES_STAND_SMOKE.md",
    "POLICIES_LOC_AUDIT.md",
];

/// Policies docs-canon criteria registry (PH-S1509): id · marker · doc path.
pub const POLICY_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_docs_canon_depth",
        "PolicyDocsCanonDepth",
        "crates/poolai-ui-core/src/policy_docs_canon_depth.rs",
    ),
    (
        "doc_depth",
        "POLICIES_DEPTH.md",
        "docs/development/POLICIES_DEPTH.md",
    ),
    (
        "doc_store",
        "POLICIES_STORE.md",
        "docs/development/POLICIES_STORE.md",
    ),
    (
        "doc_api",
        "POLICIES_API.md",
        "docs/development/POLICIES_API.md",
    ),
    (
        "doc_admin_ops",
        "POLICIES_ADMIN_OPS.md",
        "docs/development/POLICIES_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "POLICIES_STAND_SMOKE.md",
        "docs/development/POLICIES_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "POLICIES_LOC_AUDIT.md",
        "docs/development/POLICIES_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--policy-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1509_integration",
        "tests/galaxy_horizon_s1509_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-docs-canon` case names (PH-S1514).
pub const POLICY_DOCS_CANON_CASES: &[&str] = &[
    "policy_docs_canon_depth",
    "doc_depth",
    "doc_store",
    "doc_api",
    "doc_admin_ops",
    "doc_stand_smoke",
    "doc_loc_audit",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "band_close",
];

/// FM §5.68 band-87 marker rows.
pub const FM_BAND87_ROWS: &[&str] = &[
    "5.68",
    "Policies docs canon",
    "PH-S1509…S1518",
    "policy_docs_canon_depth",
];

/// Policies docs-canon adoption markers for band 87.
pub const POLICY_DOCS_CANON_BAND87_ROWS: &[&str] = &[
    "PH-S1509",
    "policy_docs_canon_depth",
    "PH-S1510",
    "POLICY_DOCS_CANON_SLICES",
    "PH-S1511",
    "policy_docs_canon_integration",
    "PH-S1512",
    "VERIFY_POLICY_DOCS_CANON",
    "PH-S1514",
    "--policy-docs-canon",
    "PH-S1518",
];

/// Production-verify stub: how many of the six Policies canon docs are referenced (PH-S1510).
pub fn policy_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = POLICY_DOCS_CANON_SLICES.len();
    let met = POLICY_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Policies docs-canon band depth from optional feature stub (PH-S1509).
pub fn policy_docs_canon_depth_stub(features: Option<&Value>) -> PolicyDocsCanonDepth {
    let Some(f) = features else {
        return PolicyDocsCanonDepth::None;
    };
    let depth = f
        .get("policy_docs_canon_depth")
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
        .get("policy_docs_canon_docs")
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
        return PolicyDocsCanonDepth::FullBand87;
    }
    if close || ratio {
        return PolicyDocsCanonDepth::RatioHold;
    }
    if docs {
        return PolicyDocsCanonDepth::DocsCanon;
    }
    if loc {
        return PolicyDocsCanonDepth::LocAuditFlag;
    }
    if export {
        return PolicyDocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return PolicyDocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return PolicyDocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return PolicyDocsCanonDepth::SliceAggregate;
    }
    if depth {
        return PolicyDocsCanonDepth::DepthModule;
    }
    PolicyDocsCanonDepth::None
}

/// Total Policies docs-canon criteria in registry (PH-S1509).
pub fn policy_docs_canon_criteria_total() -> usize {
    POLICY_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_docs_canon_depth_stub_ph_s1509() {
        assert_eq!(
            policy_docs_canon_depth_stub(None),
            PolicyDocsCanonDepth::None
        );
        assert_eq!(
            policy_docs_canon_depth_stub(Some(&json!({"policy_docs_canon_depth": true}))),
            PolicyDocsCanonDepth::DepthModule
        );
        assert_eq!(
            policy_docs_canon_depth_stub(Some(&json!({
                "policy_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyDocsCanonDepth::FullBand87
        );
        assert_eq!(POLICY_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(policy_docs_canon_criteria_total(), 10);
        assert_eq!(POLICY_DOCS_CANON_SLICES.len(), 6);
        assert!(FM_BAND87_ROWS.contains(&"PH-S1509…S1518"));
    }

    #[test]
    fn policy_docs_canon_slices_met_ph_s1510() {
        let src = "POLICIES_DEPTH.md POLICIES_STORE.md POLICIES_API.md POLICIES_ADMIN_OPS.md POLICIES_STAND_SMOKE.md POLICIES_LOC_AUDIT.md";
        assert_eq!(policy_docs_canon_slices_met(src), (6, 6));
        assert_eq!(policy_docs_canon_slices_met("POLICIES_DEPTH.md"), (1, 6));
    }
}
