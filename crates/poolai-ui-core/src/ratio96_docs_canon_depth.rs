//! Ratio96 docs-canon band depth (PH-S1709…S1718, band 107 — enterprise phase F).
//!
//! Consolidates band 101–106 `RATIO96_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// Ratio96 docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio96DocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand107,
}

/// Band 101–106 ratio96 canon doc filenames covered by aggregate (PH-S1710).
pub const RATIO96_DOCS_CANON_SLICES: &[&str] = &[
    "RATIO96_DEPTH.md",
    "RATIO96_ADMIN_OPS.md",
    "RATIO96_STAND_SMOKE.md",
    "RATIO96_LOC_AUDIT.md",
];

/// Ratio96 docs-canon criteria registry (PH-S1709): id · marker · doc path.
pub const RATIO96_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "ratio96_docs_canon_depth",
        "Ratio96DocsCanonDepth",
        "crates/poolai-ui-core/src/ratio96_docs_canon_depth.rs",
    ),
    (
        "doc_depth",
        "RATIO96_DEPTH.md",
        "docs/development/RATIO96_DEPTH.md",
    ),
    (
        "doc_admin_ops",
        "RATIO96_ADMIN_OPS.md",
        "docs/development/RATIO96_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "RATIO96_STAND_SMOKE.md",
        "docs/development/RATIO96_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "RATIO96_LOC_AUDIT.md",
        "docs/development/RATIO96_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--ratio96-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_RATIO96_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "ratio96_docs_canon_band107_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "ratio_hold",
        "--ratio96-docs-canon --advisory --min-ratio 0.95",
        "docs/development/RATIO96_DOCS_CANON.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1709_integration",
        "tests/galaxy_horizon_s1709_integration.rs",
    ),
];

/// `poolai-loc-audit --ratio96-docs-canon` case names (PH-S1714).
pub const RATIO96_DOCS_CANON_CASES: &[&str] = &[
    "ratio96_docs_canon_depth",
    "doc_depth",
    "doc_admin_ops",
    "doc_stand_smoke",
    "doc_loc_audit",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "ratio_hold",
    "band_close",
];

/// FM §5.88 band-107 marker rows.
pub const FM_BAND107_ROWS: &[&str] = &[
    "5.88",
    "Ratio96 docs canon",
    "PH-S1709…S1718",
    "ratio96_docs_canon_depth",
];

/// Ratio96 docs-canon adoption markers for band 107.
pub const RATIO96_DOCS_CANON_BAND107_ROWS: &[&str] = &[
    "PH-S1709",
    "ratio96_docs_canon_depth",
    "PH-S1710",
    "RATIO96_DOCS_CANON_SLICES",
    "PH-S1711",
    "ratio96_docs_canon_integration",
    "PH-S1712",
    "VERIFY_RATIO96_DOCS_CANON",
    "PH-S1714",
    "--ratio96-docs-canon",
    "PH-S1718",
];

/// Production-verify stub: how many of the four ratio96 canon docs are referenced (PH-S1710).
pub fn ratio96_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = RATIO96_DOCS_CANON_SLICES.len();
    let met = RATIO96_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify ratio96 docs-canon band depth from optional feature stub (PH-S1709).
pub fn ratio96_docs_canon_depth_stub(features: Option<&Value>) -> Ratio96DocsCanonDepth {
    let Some(f) = features else {
        return Ratio96DocsCanonDepth::None;
    };
    let depth = f
        .get("ratio96_docs_canon_depth")
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
        .get("ratio96_docs_canon_docs")
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
        return Ratio96DocsCanonDepth::FullBand107;
    }
    if close || ratio {
        return Ratio96DocsCanonDepth::RatioHold;
    }
    if docs {
        return Ratio96DocsCanonDepth::DocsCanon;
    }
    if loc {
        return Ratio96DocsCanonDepth::LocAuditFlag;
    }
    if export {
        return Ratio96DocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return Ratio96DocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return Ratio96DocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return Ratio96DocsCanonDepth::SliceAggregate;
    }
    if depth {
        return Ratio96DocsCanonDepth::DepthModule;
    }
    Ratio96DocsCanonDepth::None
}

/// Total ratio96 docs-canon criteria in registry (PH-S1709).
pub fn ratio96_docs_canon_criteria_total() -> usize {
    RATIO96_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_docs_canon_depth_stub_ph_s1709() {
        assert_eq!(
            ratio96_docs_canon_depth_stub(None),
            Ratio96DocsCanonDepth::None
        );
        assert_eq!(
            ratio96_docs_canon_depth_stub(Some(&json!({"ratio96_docs_canon_depth": true}))),
            Ratio96DocsCanonDepth::DepthModule
        );
        assert_eq!(
            ratio96_docs_canon_depth_stub(Some(&json!({
                "ratio96_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "ratio96_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            Ratio96DocsCanonDepth::FullBand107
        );
        assert_eq!(RATIO96_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(ratio96_docs_canon_criteria_total(), 10);
        assert_eq!(RATIO96_DOCS_CANON_SLICES.len(), 4);
        assert!(FM_BAND107_ROWS.contains(&"PH-S1709…S1718"));
    }

    #[test]
    fn ratio96_docs_canon_slices_met_ph_s1710() {
        let src =
            "RATIO96_DEPTH.md RATIO96_ADMIN_OPS.md RATIO96_STAND_SMOKE.md RATIO96_LOC_AUDIT.md";
        assert_eq!(ratio96_docs_canon_slices_met(src), (4, 4));
        assert_eq!(ratio96_docs_canon_slices_met("RATIO96_DEPTH.md"), (1, 4));
    }
}
