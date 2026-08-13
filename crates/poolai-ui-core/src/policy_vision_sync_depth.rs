//! Policies vision-sync band depth (PH-S1519…S1528, band 88 — enterprise phase D).
//!
//! Consolidates Policies phase-D docs-canon + `GSV/docs/vision/*` under one vision-sync gate.

use serde_json::Value;

/// Policies vision-sync depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyVisionSyncDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand88,
}

/// Vision-sync slice filenames covered by aggregate (PH-S1520).
pub const POLICY_VISION_SYNC_SLICES: &[&str] = &[
    "manifest.json",
    "extensions.json",
    "README.md",
    "vision.svg",
    "index.html",
    "POLICIES_DOCS_CANON.md",
];

/// Policies vision-sync criteria registry (PH-S1519): id · marker · doc path.
pub const POLICY_VISION_SYNC_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_vision_sync_depth",
        "PolicyVisionSyncDepth",
        "crates/poolai-ui-core/src/policy_vision_sync_depth.rs",
    ),
    (
        "vision_manifest",
        "\"revision\"",
        "GSV/docs/vision/manifest.json",
    ),
    (
        "vision_extensions",
        "active_sprint",
        "GSV/docs/vision/extensions.json",
    ),
    ("vision_readme", "docs/vision", "GSV/docs/vision/README.md"),
    ("vision_svg", "svg", "GSV/docs/vision/vision.svg"),
    ("vision_index", "manifest", "GSV/docs/vision/index.html"),
    (
        "doc_docs_canon",
        "POLICIES_DOCS_CANON.md",
        "docs/development/POLICIES_DOCS_CANON.md",
    ),
    (
        "aggregate_flag",
        "--policy-vision-sync",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_VISION_SYNC",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1519_integration",
        "tests/galaxy_horizon_s1519_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-vision-sync` case names (PH-S1524).
pub const POLICY_VISION_SYNC_CASES: &[&str] = &[
    "policy_vision_sync_depth",
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

/// FM §5.69 band-88 marker rows.
pub const FM_BAND88_ROWS: &[&str] = &[
    "5.69",
    "Policies vision sync",
    "PH-S1519…S1528",
    "policy_vision_sync_depth",
];

/// Policies vision-sync adoption markers for band 88.
pub const POLICY_VISION_SYNC_BAND88_ROWS: &[&str] = &[
    "PH-S1519",
    "policy_vision_sync_depth",
    "PH-S1520",
    "POLICY_VISION_SYNC_SLICES",
    "PH-S1521",
    "policy_vision_sync_integration",
    "PH-S1522",
    "VERIFY_POLICY_VISION_SYNC",
    "PH-S1524",
    "--policy-vision-sync",
    "PH-S1528",
];

/// Production-verify stub: how many vision-sync slices are referenced (PH-S1520).
pub fn policy_vision_sync_slices_met(canon_src: &str) -> (usize, usize) {
    let total = POLICY_VISION_SYNC_SLICES.len();
    let met = POLICY_VISION_SYNC_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Policies vision-sync band depth from optional feature stub (PH-S1519).
pub fn policy_vision_sync_depth_stub(features: Option<&Value>) -> PolicyVisionSyncDepth {
    let Some(f) = features else {
        return PolicyVisionSyncDepth::None;
    };
    let depth = f
        .get("policy_vision_sync_depth")
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
        .get("policy_vision_sync_docs")
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
        return PolicyVisionSyncDepth::FullBand88;
    }
    if close || ratio {
        return PolicyVisionSyncDepth::RatioHold;
    }
    if docs {
        return PolicyVisionSyncDepth::DocsCanon;
    }
    if loc {
        return PolicyVisionSyncDepth::LocAuditFlag;
    }
    if export {
        return PolicyVisionSyncDepth::StandSmokeExport;
    }
    if verify {
        return PolicyVisionSyncDepth::VerifyDevStandHook;
    }
    if contracts {
        return PolicyVisionSyncDepth::CriteriaContracts;
    }
    if slices {
        return PolicyVisionSyncDepth::SliceAggregate;
    }
    if depth {
        return PolicyVisionSyncDepth::DepthModule;
    }
    PolicyVisionSyncDepth::None
}

/// Total Policies vision-sync criteria in registry (PH-S1519).
pub fn policy_vision_sync_criteria_total() -> usize {
    POLICY_VISION_SYNC_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_vision_sync_depth_stub_ph_s1519() {
        assert_eq!(
            policy_vision_sync_depth_stub(None),
            PolicyVisionSyncDepth::None
        );
        assert_eq!(
            policy_vision_sync_depth_stub(Some(&json!({"policy_vision_sync_depth": true}))),
            PolicyVisionSyncDepth::DepthModule
        );
        assert_eq!(
            policy_vision_sync_depth_stub(Some(&json!({
                "policy_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyVisionSyncDepth::FullBand88
        );
        assert_eq!(POLICY_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(policy_vision_sync_criteria_total(), 10);
        assert_eq!(POLICY_VISION_SYNC_SLICES.len(), 6);
        assert!(FM_BAND88_ROWS.contains(&"PH-S1519…S1528"));
    }

    #[test]
    fn policy_vision_sync_slices_met_ph_s1520() {
        let src =
            "manifest.json extensions.json README.md vision.svg index.html POLICIES_DOCS_CANON.md";
        assert_eq!(policy_vision_sync_slices_met(src), (6, 6));
        assert_eq!(policy_vision_sync_slices_met("manifest.json"), (1, 6));
    }
}
