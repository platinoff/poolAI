//! Monitoring docs-canon band depth (PH-S1609…S1618, band 97 — project completion).
//!
//! Consolidates band 91–95 `MONITORING_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// Monitoring docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringDocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand97,
}

/// Band 91–95 Monitoring canon doc filenames covered by aggregate (PH-S1610).
pub const MONITORING_DOCS_CANON_SLICES: &[&str] = &[
    "MONITORING_DEPTH.md",
    "MONITORING_STORE.md",
    "MONITORING_API.md",
    "MONITORING_ADMIN_OPS.md",
    "MONITORING_STAND_SMOKE.md",
    "MONITORING_LOC_AUDIT.md",
];

/// Monitoring docs-canon criteria registry (PH-S1609): id · marker · doc path.
pub const MONITORING_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_docs_canon_depth",
        "MonitoringDocsCanonDepth",
        "crates/poolai-ui-core/src/monitoring_docs_canon_depth.rs",
    ),
    (
        "doc_depth",
        "MONITORING_DEPTH.md",
        "docs/development/MONITORING_DEPTH.md",
    ),
    (
        "doc_store",
        "MONITORING_STORE.md",
        "docs/development/MONITORING_STORE.md",
    ),
    (
        "doc_api",
        "MONITORING_API.md",
        "docs/development/MONITORING_API.md",
    ),
    (
        "doc_admin_ops",
        "MONITORING_ADMIN_OPS.md",
        "docs/development/MONITORING_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "MONITORING_STAND_SMOKE.md",
        "docs/development/MONITORING_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "MONITORING_LOC_AUDIT.md",
        "docs/development/MONITORING_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--monitoring-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1609_integration",
        "tests/galaxy_horizon_s1609_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-docs-canon` case names (PH-S1614).
pub const MONITORING_DOCS_CANON_CASES: &[&str] = &[
    "monitoring_docs_canon_depth",
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

/// FM §5.78 band-97 marker rows.
pub const FM_BAND97_ROWS: &[&str] = &[
    "5.78",
    "Monitoring docs canon",
    "PH-S1609…S1618",
    "monitoring_docs_canon_depth",
];

/// Monitoring docs-canon adoption markers for band 97.
pub const MONITORING_DOCS_CANON_BAND97_ROWS: &[&str] = &[
    "PH-S1609",
    "monitoring_docs_canon_depth",
    "PH-S1610",
    "MONITORING_DOCS_CANON_SLICES",
    "PH-S1611",
    "monitoring_docs_canon_integration",
    "PH-S1612",
    "VERIFY_MONITORING_DOCS_CANON",
    "PH-S1614",
    "--monitoring-docs-canon",
    "PH-S1618",
];

/// Production-verify stub: how many of the six Monitoring canon docs are referenced (PH-S1610).
pub fn monitoring_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = MONITORING_DOCS_CANON_SLICES.len();
    let met = MONITORING_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Monitoring docs-canon band depth from optional feature stub (PH-S1609).
pub fn monitoring_docs_canon_depth_stub(features: Option<&Value>) -> MonitoringDocsCanonDepth {
    let Some(f) = features else {
        return MonitoringDocsCanonDepth::None;
    };
    let depth = f
        .get("monitoring_docs_canon_depth")
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
        .get("monitoring_docs_canon_docs")
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
        return MonitoringDocsCanonDepth::FullBand97;
    }
    if close || ratio {
        return MonitoringDocsCanonDepth::RatioHold;
    }
    if docs {
        return MonitoringDocsCanonDepth::DocsCanon;
    }
    if loc {
        return MonitoringDocsCanonDepth::LocAuditFlag;
    }
    if export {
        return MonitoringDocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringDocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return MonitoringDocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return MonitoringDocsCanonDepth::SliceAggregate;
    }
    if depth {
        return MonitoringDocsCanonDepth::DepthModule;
    }
    MonitoringDocsCanonDepth::None
}

/// Total Monitoring docs-canon criteria in registry (PH-S1609).
pub fn monitoring_docs_canon_criteria_total() -> usize {
    MONITORING_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_docs_canon_depth_stub_ph_s1609() {
        assert_eq!(
            monitoring_docs_canon_depth_stub(None),
            MonitoringDocsCanonDepth::None
        );
        assert_eq!(
            monitoring_docs_canon_depth_stub(Some(&json!({"monitoring_docs_canon_depth": true}))),
            MonitoringDocsCanonDepth::DepthModule
        );
        assert_eq!(
            monitoring_docs_canon_depth_stub(Some(&json!({
                "monitoring_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringDocsCanonDepth::FullBand97
        );
        assert_eq!(MONITORING_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(monitoring_docs_canon_criteria_total(), 10);
        assert_eq!(MONITORING_DOCS_CANON_SLICES.len(), 6);
        assert!(FM_BAND97_ROWS.contains(&"PH-S1609…S1618"));
    }

    #[test]
    fn monitoring_docs_canon_slices_met_ph_s1610() {
        let src = "MONITORING_DEPTH.md MONITORING_STORE.md MONITORING_API.md MONITORING_ADMIN_OPS.md MONITORING_STAND_SMOKE.md MONITORING_LOC_AUDIT.md";
        assert_eq!(monitoring_docs_canon_slices_met(src), (6, 6));
        assert_eq!(
            monitoring_docs_canon_slices_met("MONITORING_DEPTH.md"),
            (1, 6)
        );
    }
}
