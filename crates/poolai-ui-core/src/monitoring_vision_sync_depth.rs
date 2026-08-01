//! Monitoring vision-sync band depth (PH-S1619…S1628, band 98 — enterprise phase E).
//!
//! Consolidates Monitoring docs-canon + `docs/vision/*` under one vision-sync gate.

use serde_json::Value;

/// Monitoring vision-sync depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringVisionSyncDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand98,
}

/// Vision-sync slice filenames covered by aggregate (PH-S1620).
pub const MONITORING_VISION_SYNC_SLICES: &[&str] = &[
    "manifest.json",
    "extensions.json",
    "README.md",
    "vision.svg",
    "index.html",
    "MONITORING_DOCS_CANON.md",
];

/// Monitoring vision-sync criteria registry (PH-S1619): id · marker · doc path.
pub const MONITORING_VISION_SYNC_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_vision_sync_depth",
        "MonitoringVisionSyncDepth",
        "crates/poolai-ui-core/src/monitoring_vision_sync_depth.rs",
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
        "MONITORING_DOCS_CANON.md",
        "docs/development/MONITORING_DOCS_CANON.md",
    ),
    (
        "aggregate_flag",
        "--monitoring-vision-sync",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_VISION_SYNC",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1619_integration",
        "tests/galaxy_horizon_s1619_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-vision-sync` case names (PH-S1624).
pub const MONITORING_VISION_SYNC_CASES: &[&str] = &[
    "monitoring_vision_sync_depth",
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

/// FM §5.79 band-98 marker rows.
pub const FM_BAND98_ROWS: &[&str] = &[
    "5.79",
    "Monitoring vision sync",
    "PH-S1619…S1628",
    "monitoring_vision_sync_depth",
];

/// Monitoring vision-sync adoption markers for band 98.
pub const MONITORING_VISION_SYNC_BAND98_ROWS: &[&str] = &[
    "PH-S1619",
    "monitoring_vision_sync_depth",
    "PH-S1620",
    "MONITORING_VISION_SYNC_SLICES",
    "PH-S1621",
    "monitoring_vision_sync_integration",
    "PH-S1622",
    "VERIFY_MONITORING_VISION_SYNC",
    "PH-S1624",
    "--monitoring-vision-sync",
    "PH-S1628",
];

/// Production-verify stub: how many vision-sync slices are referenced (PH-S1620).
pub fn monitoring_vision_sync_slices_met(canon_src: &str) -> (usize, usize) {
    let total = MONITORING_VISION_SYNC_SLICES.len();
    let met = MONITORING_VISION_SYNC_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Monitoring vision-sync band depth from optional feature stub (PH-S1619).
pub fn monitoring_vision_sync_depth_stub(features: Option<&Value>) -> MonitoringVisionSyncDepth {
    let Some(f) = features else {
        return MonitoringVisionSyncDepth::None;
    };
    let depth = f
        .get("monitoring_vision_sync_depth")
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
        .get("monitoring_vision_sync_docs")
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
        return MonitoringVisionSyncDepth::FullBand98;
    }
    if close || ratio {
        return MonitoringVisionSyncDepth::RatioHold;
    }
    if docs {
        return MonitoringVisionSyncDepth::DocsCanon;
    }
    if loc {
        return MonitoringVisionSyncDepth::LocAuditFlag;
    }
    if export {
        return MonitoringVisionSyncDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringVisionSyncDepth::VerifyDevStandHook;
    }
    if contracts {
        return MonitoringVisionSyncDepth::CriteriaContracts;
    }
    if slices {
        return MonitoringVisionSyncDepth::SliceAggregate;
    }
    if depth {
        return MonitoringVisionSyncDepth::DepthModule;
    }
    MonitoringVisionSyncDepth::None
}

/// Total Monitoring vision-sync criteria in registry (PH-S1619).
pub fn monitoring_vision_sync_criteria_total() -> usize {
    MONITORING_VISION_SYNC_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_vision_sync_depth_stub_ph_s1619() {
        assert_eq!(
            monitoring_vision_sync_depth_stub(None),
            MonitoringVisionSyncDepth::None
        );
        assert_eq!(
            monitoring_vision_sync_depth_stub(Some(&json!({"monitoring_vision_sync_depth": true}))),
            MonitoringVisionSyncDepth::DepthModule
        );
        assert_eq!(
            monitoring_vision_sync_depth_stub(Some(&json!({
                "monitoring_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringVisionSyncDepth::FullBand98
        );
        assert_eq!(MONITORING_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(monitoring_vision_sync_criteria_total(), 10);
        assert_eq!(MONITORING_VISION_SYNC_SLICES.len(), 6);
        assert!(FM_BAND98_ROWS.contains(&"PH-S1619…S1628"));
    }

    #[test]
    fn monitoring_vision_sync_slices_met_ph_s1620() {
        let src =
            "manifest.json extensions.json README.md vision.svg index.html MONITORING_DOCS_CANON.md";
        assert_eq!(monitoring_vision_sync_slices_met(src), (6, 6));
        assert_eq!(monitoring_vision_sync_slices_met("manifest.json"), (1, 6));
    }
}
