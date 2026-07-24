//! Audit vision-sync band depth (PH-S1419…S1428, band 78 — enterprise phase C).
//!
//! Consolidates Audit phase-C docs-canon + `docs/vision/*` under one vision-sync gate.

use serde_json::Value;

/// Audit vision-sync depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditVisionSyncDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand78,
}

/// Vision-sync slice filenames covered by aggregate (PH-S1420).
pub const AUDIT_VISION_SYNC_SLICES: &[&str] = &[
    "manifest.json",
    "extensions.json",
    "README.md",
    "vision.svg",
    "index.html",
    "AUDIT_DOCS_CANON.md",
];

/// Audit vision-sync criteria registry (PH-S1419): id · marker · doc path.
pub const AUDIT_VISION_SYNC_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_vision_sync_depth",
        "AuditVisionSyncDepth",
        "crates/poolai-ui-core/src/audit_vision_sync_depth.rs",
    ),
    (
        "vision_manifest",
        "\"revision\"",
        "docs/vision/manifest.json",
    ),
    (
        "vision_extensions",
        "active_sprint",
        "docs/vision/extensions.json",
    ),
    ("vision_readme", "docs/vision", "docs/vision/README.md"),
    ("vision_svg", "svg", "docs/vision/vision.svg"),
    ("vision_index", "manifest", "docs/vision/index.html"),
    (
        "doc_docs_canon",
        "AUDIT_DOCS_CANON.md",
        "docs/development/AUDIT_DOCS_CANON.md",
    ),
    (
        "aggregate_flag",
        "--audit-vision-sync",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_VISION_SYNC",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1419_integration",
        "tests/galaxy_horizon_s1419_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-vision-sync` case names (PH-S1424).
pub const AUDIT_VISION_SYNC_CASES: &[&str] = &[
    "audit_vision_sync_depth",
    "vision_manifest",
    "vision_extensions",
    "vision_readme",
    "vision_svg",
    "vision_index",
    "doc_docs_canon",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "band_close",
];

/// FM §5.59 band-78 marker rows.
pub const FM_BAND78_ROWS: &[&str] = &[
    "5.59",
    "Audit vision sync",
    "PH-S1419…S1428",
    "audit_vision_sync_depth",
];

/// Audit vision-sync adoption markers for band 78.
pub const AUDIT_VISION_SYNC_BAND78_ROWS: &[&str] = &[
    "PH-S1419",
    "audit_vision_sync_depth",
    "PH-S1420",
    "AUDIT_VISION_SYNC_SLICES",
    "PH-S1421",
    "audit_vision_sync_integration",
    "PH-S1422",
    "VERIFY_AUDIT_VISION_SYNC",
    "PH-S1424",
    "--audit-vision-sync",
    "PH-S1428",
];

/// Production-verify stub: how many vision-sync slices are referenced (PH-S1420).
pub fn audit_vision_sync_slices_met(canon_src: &str) -> (usize, usize) {
    let total = AUDIT_VISION_SYNC_SLICES.len();
    let met = AUDIT_VISION_SYNC_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Audit vision-sync band depth from optional feature stub (PH-S1419).
pub fn audit_vision_sync_depth_stub(features: Option<&Value>) -> AuditVisionSyncDepth {
    let Some(f) = features else {
        return AuditVisionSyncDepth::None;
    };
    let depth = f
        .get("audit_vision_sync_depth")
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
        .get("audit_vision_sync_docs")
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
        return AuditVisionSyncDepth::FullBand78;
    }
    if close || ratio {
        return AuditVisionSyncDepth::RatioHold;
    }
    if docs {
        return AuditVisionSyncDepth::DocsCanon;
    }
    if loc {
        return AuditVisionSyncDepth::LocAuditFlag;
    }
    if export {
        return AuditVisionSyncDepth::StandSmokeExport;
    }
    if verify {
        return AuditVisionSyncDepth::VerifyDevStandHook;
    }
    if contracts {
        return AuditVisionSyncDepth::CriteriaContracts;
    }
    if slices {
        return AuditVisionSyncDepth::SliceAggregate;
    }
    if depth {
        return AuditVisionSyncDepth::DepthModule;
    }
    AuditVisionSyncDepth::None
}

/// Total Audit vision-sync criteria in registry (PH-S1419).
pub fn audit_vision_sync_criteria_total() -> usize {
    AUDIT_VISION_SYNC_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_vision_sync_depth_stub_ph_s1419() {
        assert_eq!(
            audit_vision_sync_depth_stub(None),
            AuditVisionSyncDepth::None
        );
        assert_eq!(
            audit_vision_sync_depth_stub(Some(&json!({"audit_vision_sync_depth": true}))),
            AuditVisionSyncDepth::DepthModule
        );
        assert_eq!(
            audit_vision_sync_depth_stub(Some(&json!({
                "audit_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditVisionSyncDepth::FullBand78
        );
        assert_eq!(AUDIT_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(audit_vision_sync_criteria_total(), 10);
        assert_eq!(AUDIT_VISION_SYNC_SLICES.len(), 6);
        assert!(FM_BAND78_ROWS.contains(&"PH-S1419…S1428"));
    }

    #[test]
    fn audit_vision_sync_slices_met_ph_s1420() {
        let src =
            "manifest.json extensions.json README.md vision.svg index.html AUDIT_DOCS_CANON.md";
        assert_eq!(audit_vision_sync_slices_met(src), (6, 6));
        assert_eq!(audit_vision_sync_slices_met("manifest.json"), (1, 6));
    }
}
