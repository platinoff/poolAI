//! Tenant vision-sync band depth (PH-S1219…S1228, band 58 — enterprise phase A).
//!
//! Consolidates tenant phase-A canon + `GSV/docs/vision/*` under one vision-sync gate.

use serde_json::Value;

/// Tenant vision-sync depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantVisionSyncDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand58,
}

/// Vision-sync slice filenames covered by aggregate (PH-S1220).
pub const TENANT_VISION_SYNC_SLICES: &[&str] = &[
    "manifest.json",
    "extensions.json",
    "README.md",
    "vision.svg",
    "index.html",
    "TENANT_DOCS_CANON.md",
];

/// Tenant vision-sync criteria registry (PH-S1219): id · marker · doc path.
pub const TENANT_VISION_SYNC_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_vision_sync_depth",
        "TenantVisionSyncDepth",
        "crates/poolai-ui-core/src/tenant_vision_sync_depth.rs",
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
        "TENANT_DOCS_CANON.md",
        "docs/development/TENANT_DOCS_CANON.md",
    ),
    (
        "aggregate_flag",
        "--tenant-vision-sync",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_VISION_SYNC",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1219_integration",
        "tests/galaxy_horizon_s1219_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-vision-sync` case names (PH-S1224).
pub const TENANT_VISION_SYNC_CASES: &[&str] = &[
    "tenant_vision_sync_depth",
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

/// FM §5.39 band-58 marker rows.
pub const FM_BAND58_ROWS: &[&str] = &[
    "5.39",
    "Tenant vision sync",
    "PH-S1219…S1228",
    "tenant_vision_sync_depth",
];

/// Tenant vision-sync adoption markers for band 58.
pub const TENANT_VISION_SYNC_BAND58_ROWS: &[&str] = &[
    "PH-S1219",
    "tenant_vision_sync_depth",
    "PH-S1220",
    "TENANT_VISION_SYNC_SLICES",
    "PH-S1221",
    "tenant_vision_sync_integration",
    "PH-S1222",
    "VERIFY_TENANT_VISION_SYNC",
    "PH-S1224",
    "--tenant-vision-sync",
    "PH-S1228",
];

/// Production-verify stub: how many vision-sync slices are referenced (PH-S1220).
pub fn tenant_vision_sync_slices_met(canon_src: &str) -> (usize, usize) {
    let total = TENANT_VISION_SYNC_SLICES.len();
    let met = TENANT_VISION_SYNC_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify tenant vision-sync band depth from optional feature stub (PH-S1219).
pub fn tenant_vision_sync_depth_stub(features: Option<&Value>) -> TenantVisionSyncDepth {
    let Some(f) = features else {
        return TenantVisionSyncDepth::None;
    };
    let depth = f
        .get("tenant_vision_sync_depth")
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
        .get("tenant_vision_sync_docs")
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
        return TenantVisionSyncDepth::FullBand58;
    }
    if close || ratio {
        return TenantVisionSyncDepth::RatioHold;
    }
    if docs {
        return TenantVisionSyncDepth::DocsCanon;
    }
    if loc {
        return TenantVisionSyncDepth::LocAuditFlag;
    }
    if export {
        return TenantVisionSyncDepth::StandSmokeExport;
    }
    if verify {
        return TenantVisionSyncDepth::VerifyDevStandHook;
    }
    if contracts {
        return TenantVisionSyncDepth::CriteriaContracts;
    }
    if slices {
        return TenantVisionSyncDepth::SliceAggregate;
    }
    if depth {
        return TenantVisionSyncDepth::DepthModule;
    }
    TenantVisionSyncDepth::None
}

/// Total tenant vision-sync criteria in registry (PH-S1219).
pub fn tenant_vision_sync_criteria_total() -> usize {
    TENANT_VISION_SYNC_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_vision_sync_depth_stub_ph_s1219() {
        assert_eq!(
            tenant_vision_sync_depth_stub(None),
            TenantVisionSyncDepth::None
        );
        assert_eq!(
            tenant_vision_sync_depth_stub(Some(&json!({"tenant_vision_sync_depth": true}))),
            TenantVisionSyncDepth::DepthModule
        );
        assert_eq!(
            tenant_vision_sync_depth_stub(Some(&json!({
                "tenant_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantVisionSyncDepth::FullBand58
        );
        assert_eq!(TENANT_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(tenant_vision_sync_criteria_total(), 10);
        assert_eq!(TENANT_VISION_SYNC_SLICES.len(), 6);
        assert!(FM_BAND58_ROWS.contains(&"PH-S1219…S1228"));
    }

    #[test]
    fn tenant_vision_sync_slices_met_ph_s1220() {
        let src =
            "manifest.json extensions.json README.md vision.svg index.html TENANT_DOCS_CANON.md";
        assert_eq!(tenant_vision_sync_slices_met(src), (6, 6));
        assert_eq!(tenant_vision_sync_slices_met("manifest.json"), (1, 6));
    }
}
