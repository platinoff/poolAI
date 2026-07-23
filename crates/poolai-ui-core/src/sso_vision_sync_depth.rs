//! SSO vision-sync band depth (PH-S1319…S1328, band 68 — enterprise phase B).
//!
//! Consolidates SSO phase-B canon + `docs/vision/*` under one vision-sync gate.

use serde_json::Value;

/// SSO vision-sync depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoVisionSyncDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand68,
}

/// Vision-sync slice filenames covered by aggregate (PH-S1320).
pub const SSO_VISION_SYNC_SLICES: &[&str] = &[
    "manifest.json",
    "extensions.json",
    "README.md",
    "vision.svg",
    "index.html",
    "SSO_DOCS_CANON.md",
];

/// SSO vision-sync criteria registry (PH-S1319): id · marker · doc path.
pub const SSO_VISION_SYNC_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_vision_sync_depth",
        "SsoVisionSyncDepth",
        "crates/poolai-ui-core/src/sso_vision_sync_depth.rs",
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
        "SSO_DOCS_CANON.md",
        "docs/development/SSO_DOCS_CANON.md",
    ),
    (
        "aggregate_flag",
        "--sso-vision-sync",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_VISION_SYNC",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1319_integration",
        "tests/galaxy_horizon_s1319_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-vision-sync` case names (PH-S1324).
pub const SSO_VISION_SYNC_CASES: &[&str] = &[
    "sso_vision_sync_depth",
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

/// FM §5.49 band-68 marker rows.
pub const FM_BAND68_ROWS: &[&str] = &[
    "5.49",
    "SSO vision sync",
    "PH-S1319…S1328",
    "sso_vision_sync_depth",
];

/// SSO vision-sync adoption markers for band 68.
pub const SSO_VISION_SYNC_BAND68_ROWS: &[&str] = &[
    "PH-S1319",
    "sso_vision_sync_depth",
    "PH-S1320",
    "SSO_VISION_SYNC_SLICES",
    "PH-S1321",
    "sso_vision_sync_integration",
    "PH-S1322",
    "VERIFY_SSO_VISION_SYNC",
    "PH-S1324",
    "--sso-vision-sync",
    "PH-S1328",
];

/// Production-verify stub: how many vision-sync slices are referenced (PH-S1320).
pub fn sso_vision_sync_slices_met(canon_src: &str) -> (usize, usize) {
    let total = SSO_VISION_SYNC_SLICES.len();
    let met = SSO_VISION_SYNC_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify SSO vision-sync band depth from optional feature stub (PH-S1319).
pub fn sso_vision_sync_depth_stub(features: Option<&Value>) -> SsoVisionSyncDepth {
    let Some(f) = features else {
        return SsoVisionSyncDepth::None;
    };
    let depth = f
        .get("sso_vision_sync_depth")
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
        .get("sso_vision_sync_docs")
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
        return SsoVisionSyncDepth::FullBand68;
    }
    if close || ratio {
        return SsoVisionSyncDepth::RatioHold;
    }
    if docs {
        return SsoVisionSyncDepth::DocsCanon;
    }
    if loc {
        return SsoVisionSyncDepth::LocAuditFlag;
    }
    if export {
        return SsoVisionSyncDepth::StandSmokeExport;
    }
    if verify {
        return SsoVisionSyncDepth::VerifyDevStandHook;
    }
    if contracts {
        return SsoVisionSyncDepth::CriteriaContracts;
    }
    if slices {
        return SsoVisionSyncDepth::SliceAggregate;
    }
    if depth {
        return SsoVisionSyncDepth::DepthModule;
    }
    SsoVisionSyncDepth::None
}

/// Total SSO vision-sync criteria in registry (PH-S1319).
pub fn sso_vision_sync_criteria_total() -> usize {
    SSO_VISION_SYNC_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_vision_sync_depth_stub_ph_s1319() {
        assert_eq!(sso_vision_sync_depth_stub(None), SsoVisionSyncDepth::None);
        assert_eq!(
            sso_vision_sync_depth_stub(Some(&json!({"sso_vision_sync_depth": true}))),
            SsoVisionSyncDepth::DepthModule
        );
        assert_eq!(
            sso_vision_sync_depth_stub(Some(&json!({
                "sso_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoVisionSyncDepth::FullBand68
        );
        assert_eq!(SSO_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(sso_vision_sync_criteria_total(), 10);
        assert_eq!(SSO_VISION_SYNC_SLICES.len(), 6);
        assert!(FM_BAND68_ROWS.contains(&"PH-S1319…S1328"));
    }

    #[test]
    fn sso_vision_sync_slices_met_ph_s1320() {
        let src = "manifest.json extensions.json README.md vision.svg index.html SSO_DOCS_CANON.md";
        assert_eq!(sso_vision_sync_slices_met(src), (6, 6));
        assert_eq!(sso_vision_sync_slices_met("manifest.json"), (1, 6));
    }
}
