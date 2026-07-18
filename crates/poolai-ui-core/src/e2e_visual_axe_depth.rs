//! E2E visual/axe regression band depth (PH-S1049…S1058, band 41).

use serde_json::Value;

/// Band-41 E2E visual/axe regression depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum E2eVisualAxeDepth {
    None,
    VisualParityTier1,
    VisualParityTier2,
    VisionAxeSmoke,
    VisionVisualSnapshot,
    HighContrastAxeExtend,
    VisualSnapshotReadyHelper,
    E2eScopeParityGate,
    FullE2eVisualAxe,
}

/// FM §5.22 band-41 marker rows.
pub const FM_BAND41_ROWS: &[&str] = &["5.22", "E2E visual/axe", "PH-S1049…S1058", "Playwright"];

/// E2E visual/axe adoption markers for band 41.
pub const E2E_VISUAL_AXE_BAND41_ROWS: &[&str] = &[
    "PH-S1049",
    "waitForVisualSnapshotReady",
    "PH-S1051",
    "axe vision map",
    "PH-S1058",
];

/// Admin routes with visual snapshot coverage (PH-S1049/S1050).
pub const VISUAL_ADMIN_ROUTES: &[&str] = &[
    "/ui/admin/config",
    "/ui/admin/jobs",
    "/ui/admin/updates-compat",
    "/ui/admin/seed-inventory",
    "/ui/admin/security-advisories",
];

pub fn e2e_visual_axe_depth_stub(features: Option<&Value>) -> E2eVisualAxeDepth {
    let Some(f) = features else {
        return E2eVisualAxeDepth::None;
    };
    let tier1 = f
        .get("visual_parity_tier1")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tier2 = f
        .get("visual_parity_tier2")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision_axe = f
        .get("vision_axe_smoke")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision_visual = f
        .get("vision_visual_snapshot")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let hc = f
        .get("high_contrast_axe_extend")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let helper = f
        .get("visual_snapshot_ready_helper")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let scope = f
        .get("e2e_scope_parity_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [tier1, tier2, vision_axe, vision_visual, hc, helper, scope];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => E2eVisualAxeDepth::None,
        7 => E2eVisualAxeDepth::FullE2eVisualAxe,
        _ if tier1 && !tier2 => E2eVisualAxeDepth::VisualParityTier1,
        _ if tier2 => E2eVisualAxeDepth::VisualParityTier2,
        _ if vision_axe => E2eVisualAxeDepth::VisionAxeSmoke,
        _ if vision_visual => E2eVisualAxeDepth::VisionVisualSnapshot,
        _ if hc => E2eVisualAxeDepth::HighContrastAxeExtend,
        _ if helper => E2eVisualAxeDepth::VisualSnapshotReadyHelper,
        _ if scope => E2eVisualAxeDepth::E2eScopeParityGate,
        _ => E2eVisualAxeDepth::FullE2eVisualAxe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn e2e_visual_axe_depth_stub_ph_s1058() {
        assert_eq!(e2e_visual_axe_depth_stub(None), E2eVisualAxeDepth::None);
        assert_eq!(
            e2e_visual_axe_depth_stub(Some(&json!({"visual_parity_tier1": true}))),
            E2eVisualAxeDepth::VisualParityTier1
        );
        assert_eq!(
            e2e_visual_axe_depth_stub(Some(&json!({
                "visual_parity_tier1": true,
                "visual_parity_tier2": true,
                "vision_axe_smoke": true,
                "vision_visual_snapshot": true,
                "high_contrast_axe_extend": true,
                "visual_snapshot_ready_helper": true,
                "e2e_scope_parity_gate": true
            }))),
            E2eVisualAxeDepth::FullE2eVisualAxe
        );
    }
}
