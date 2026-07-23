//! SSO docs-canon band depth (PH-S1309…S1318, band 67 — enterprise phase B).
//!
//! Consolidates band 61–66 `SSO_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// SSO docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoDocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand67,
}

/// Band 61–66 SSO canon doc filenames covered by aggregate (PH-S1310).
pub const SSO_DOCS_CANON_SLICES: &[&str] = &[
    "SSO_DEPTH.md",
    "SSO_STORE.md",
    "SSO_API.md",
    "SSO_ADMIN_OPS.md",
    "SSO_STAND_SMOKE.md",
    "SSO_LOC_AUDIT.md",
];

/// SSO docs-canon criteria registry (PH-S1309): id · marker · doc path.
pub const SSO_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_docs_canon_depth",
        "SsoDocsCanonDepth",
        "crates/poolai-ui-core/src/sso_docs_canon_depth.rs",
    ),
    ("doc_depth", "SSO_DEPTH.md", "docs/development/SSO_DEPTH.md"),
    ("doc_store", "SSO_STORE.md", "docs/development/SSO_STORE.md"),
    ("doc_api", "SSO_API.md", "docs/development/SSO_API.md"),
    (
        "doc_admin_ops",
        "SSO_ADMIN_OPS.md",
        "docs/development/SSO_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "SSO_STAND_SMOKE.md",
        "docs/development/SSO_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "SSO_LOC_AUDIT.md",
        "docs/development/SSO_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--sso-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1309_integration",
        "tests/galaxy_horizon_s1309_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-docs-canon` case names (PH-S1314).
pub const SSO_DOCS_CANON_CASES: &[&str] = &[
    "sso_docs_canon_depth",
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

/// FM §5.48 band-67 marker rows.
pub const FM_BAND67_ROWS: &[&str] = &[
    "5.48",
    "SSO docs canon",
    "PH-S1309…S1318",
    "sso_docs_canon_depth",
];

/// SSO docs-canon adoption markers for band 67.
pub const SSO_DOCS_CANON_BAND67_ROWS: &[&str] = &[
    "PH-S1309",
    "sso_docs_canon_depth",
    "PH-S1310",
    "SSO_DOCS_CANON_SLICES",
    "PH-S1311",
    "sso_docs_canon_integration",
    "PH-S1312",
    "VERIFY_SSO_DOCS_CANON",
    "PH-S1314",
    "--sso-docs-canon",
    "PH-S1318",
];

/// Production-verify stub: how many of the six SSO canon docs are referenced (PH-S1310).
pub fn sso_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = SSO_DOCS_CANON_SLICES.len();
    let met = SSO_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify SSO docs-canon band depth from optional feature stub (PH-S1309).
pub fn sso_docs_canon_depth_stub(features: Option<&Value>) -> SsoDocsCanonDepth {
    let Some(f) = features else {
        return SsoDocsCanonDepth::None;
    };
    let depth = f
        .get("sso_docs_canon_depth")
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
        .get("sso_docs_canon_docs")
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
        return SsoDocsCanonDepth::FullBand67;
    }
    if close || ratio {
        return SsoDocsCanonDepth::RatioHold;
    }
    if docs {
        return SsoDocsCanonDepth::DocsCanon;
    }
    if loc {
        return SsoDocsCanonDepth::LocAuditFlag;
    }
    if export {
        return SsoDocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return SsoDocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return SsoDocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return SsoDocsCanonDepth::SliceAggregate;
    }
    if depth {
        return SsoDocsCanonDepth::DepthModule;
    }
    SsoDocsCanonDepth::None
}

/// Total SSO docs-canon criteria in registry (PH-S1309).
pub fn sso_docs_canon_criteria_total() -> usize {
    SSO_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_docs_canon_depth_stub_ph_s1309() {
        assert_eq!(sso_docs_canon_depth_stub(None), SsoDocsCanonDepth::None);
        assert_eq!(
            sso_docs_canon_depth_stub(Some(&json!({"sso_docs_canon_depth": true}))),
            SsoDocsCanonDepth::DepthModule
        );
        assert_eq!(
            sso_docs_canon_depth_stub(Some(&json!({
                "sso_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoDocsCanonDepth::FullBand67
        );
        assert_eq!(SSO_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(sso_docs_canon_criteria_total(), 10);
        assert_eq!(SSO_DOCS_CANON_SLICES.len(), 6);
        assert!(FM_BAND67_ROWS.contains(&"PH-S1309…S1318"));
    }

    #[test]
    fn sso_docs_canon_slices_met_ph_s1310() {
        let src = "SSO_DEPTH.md SSO_STORE.md SSO_API.md SSO_ADMIN_OPS.md SSO_STAND_SMOKE.md SSO_LOC_AUDIT.md";
        assert_eq!(sso_docs_canon_slices_met(src), (6, 6));
        assert_eq!(sso_docs_canon_slices_met("SSO_DEPTH.md"), (1, 6));
    }
}
