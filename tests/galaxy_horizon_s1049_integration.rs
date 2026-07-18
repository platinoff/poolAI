//! PH-S1058: Galaxy horizon close band 41 — E2E visual/axe regression.

use poolai_ui_core::e2e_visual_axe_depth::{
    e2e_visual_axe_depth_stub, E2eVisualAxeDepth, E2E_VISUAL_AXE_BAND41_ROWS, FM_BAND41_ROWS,
    VISUAL_ADMIN_ROUTES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1049_band_e2e_visual_axe_close_ph_s1058() {
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

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND41_ROWS {
        assert!(fm.contains(row), "FM missing band-41 row {row}");
    }
    assert!(fm.contains("PH-S1058"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1049") || handoff.contains("band 41"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let visual = include_str!("../e2e/tests/visual.spec.ts");
    let a11y = include_str!("../e2e/tests/a11y.spec.ts");
    let helpers = include_str!("../e2e/tests/helpers.ts");
    assert!(helpers.contains("waitForVisualSnapshotReady"));
    assert!(a11y.contains("axe vision map (PH-S1051)"));
    assert!(visual.contains("waitForVisualSnapshotReady"));

    for route in VISUAL_ADMIN_ROUTES {
        assert!(
            visual.contains(route) && a11y.contains(route),
            "visual/axe parity missing route {route}"
        );
    }

    for marker in E2E_VISUAL_AXE_BAND41_ROWS {
        assert!(
            visual.contains(marker)
                || a11y.contains(marker)
                || helpers.contains(marker)
                || fm.contains(marker),
            "band-41 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/e2e_visual_axe_depth.rs").exists());
    assert!(Path::new("tests/e2e_scope_audit.rs").exists());
}
