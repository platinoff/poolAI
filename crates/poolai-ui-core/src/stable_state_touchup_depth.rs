//! STABLE state touch-up band depth (PH-S1109…S1118, band 47).

use serde_json::Value;

/// STABLE maintenance touch-up depth flags (criteria registry / docs canon / ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StableStateTouchupDepth {
    None,
    CriteriaRegistry,
    StableSummary,
    IndexCanon,
    HandoffZriz,
    LocAuditTouchup,
    VerifyDevStandHook,
    QuickTouchup,
    DocsCanon,
    FullBand47,
}

/// Maintenance-mode STABLE criteria registry (PH-S1112): id · marker · doc path.
pub const STABLE_TOUCHUP_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "product_complete",
        "§5.15",
        "docs/catalog/FUNCTION_MANAGEMENT.md",
    ),
    (
        "openapi_gap_zero",
        "poolai-openapi-gap-audit",
        "docs/openapi.yaml",
    ),
    (
        "integration_coverage",
        "cargo test-ci",
        ".cargo/config.toml",
    ),
    (
        "rust_ratio_formal",
        "ratio_95_formal_gate_met",
        "docs/development/rust_ratio.json",
    ),
    (
        "docs_canon",
        "STABLE_STATE_SUMMARY",
        "docs/status/STABLE_STATE_SUMMARY.md",
    ),
    (
        "vision_sync",
        "poolai-vision-sync",
        "docs/vision/manifest.json",
    ),
    ("ops_test_ci", "test-ci", "bin/verify-dev-stand.sh"),
];

/// `poolai-loc-audit --stable-touchup` case names (PH-S1110).
pub const STABLE_TOUCHUP_CASES: &[&str] = &[
    "product_complete",
    "openapi_gap_zero",
    "integration_coverage",
    "rust_ratio_formal",
    "docs_canon",
    "vision_sync",
    "ops_test_ci",
];

/// FM §5.28 band-47 marker rows.
pub const FM_BAND47_ROWS: &[&str] = &[
    "5.28",
    "STABLE touch-up",
    "PH-S1109…S1118",
    "stable_state_touchup_depth",
];

/// STABLE touch-up adoption markers for band 47.
pub const STABLE_TOUCHUP_BAND47_ROWS: &[&str] = &[
    "PH-S1109",
    "stable_state_touchup_depth",
    "PH-S1110",
    "--stable-touchup",
    "PH-S1113",
    "VERIFY_STABLE_TOUCHUP",
    "PH-S1114",
    "--stable-touchup",
    "PH-S1118",
];

/// Classify STABLE touch-up band depth from optional feature stub (PH-S1109).
pub fn stable_state_touchup_depth_stub(features: Option<&Value>) -> StableStateTouchupDepth {
    let Some(f) = features else {
        return StableStateTouchupDepth::None;
    };
    let criteria = f
        .get("criteria_registry")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let stable = f
        .get("stable_summary")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let index = f
        .get("index_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let handoff = f
        .get("handoff_zriz")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_touchup")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let quick = f
        .get("quick_touchup")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("docs_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if criteria && stable && index && handoff && loc && verify && quick && docs {
        return StableStateTouchupDepth::FullBand47;
    }
    if quick {
        return StableStateTouchupDepth::QuickTouchup;
    }
    if verify {
        return StableStateTouchupDepth::VerifyDevStandHook;
    }
    if loc {
        return StableStateTouchupDepth::LocAuditTouchup;
    }
    if docs {
        return StableStateTouchupDepth::DocsCanon;
    }
    if handoff {
        return StableStateTouchupDepth::HandoffZriz;
    }
    if index {
        return StableStateTouchupDepth::IndexCanon;
    }
    if stable {
        return StableStateTouchupDepth::StableSummary;
    }
    if criteria {
        return StableStateTouchupDepth::CriteriaRegistry;
    }
    StableStateTouchupDepth::None
}

/// Total STABLE maintenance criteria in registry (PH-S1112).
pub fn stable_criteria_total() -> usize {
    STABLE_TOUCHUP_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stable_state_touchup_depth_stub_ph_s1109() {
        assert_eq!(
            stable_state_touchup_depth_stub(None),
            StableStateTouchupDepth::None
        );
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({"criteria_registry": true}))),
            StableStateTouchupDepth::CriteriaRegistry
        );
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({
                "criteria_registry": true,
                "stable_summary": true,
                "index_canon": true,
                "handoff_zriz": true,
                "loc_audit_touchup": true,
                "verify_dev_stand_hook": true,
                "quick_touchup": true,
                "docs_canon": true,
            }))),
            StableStateTouchupDepth::FullBand47
        );
        assert_eq!(STABLE_TOUCHUP_CRITERIA.len(), 7);
        assert_eq!(stable_criteria_total(), 7);
        assert!(!STABLE_TOUCHUP_CASES.is_empty());
        assert!(FM_BAND47_ROWS.contains(&"PH-S1109…S1118"));
    }
}
