//! PH-S1018: Galaxy horizon close band 37 — owner ops UX v2.

use poolai_ui_core::owner_ops_depth::{
    owner_ops_depth_stub, OwnerOpsDepth, FM_BAND37_ROWS, OWNER_OPS_BAND37_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1011_band_owner_ops_close_ph_s1018() {
    assert_eq!(
        owner_ops_depth_stub(Some(&json!({"light_launch": true}))),
        OwnerOpsDepth::LightLaunch
    );
    assert_eq!(
        owner_ops_depth_stub(Some(&json!({
            "light_launch": true,
            "quick_preset": true,
            "vision_launch": true,
            "last_run_persist": true,
            "admin_power_ui": true,
            "power_wire": true,
            "vision_power_ui": true
        }))),
        OwnerOpsDepth::FullOwnerOps
    );

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    for row in OWNER_OPS_BAND37_ROWS {
        assert!(
            run_local.contains(row) || row.starts_with("PH-S"),
            "RUN_LOCAL missing band-37 marker {row}"
        );
    }
    assert!(run_local.contains("quick"));
    assert!(run_local.contains("--light") || run_local.contains("Light"));

    let readme = include_str!("../README.md");
    assert!(readme.contains("open-docs-vision"));
    assert!(readme.contains("8765"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND37_ROWS {
        assert!(fm.contains(row), "FM missing band-37 row {row}");
    }
    assert!(fm.contains("PH-S1018") && fm.contains("**✅**"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1011"));
    assert!(handoff.contains("band 37"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    assert!(Path::new("src/ops/last_run.rs").exists());
    assert!(Path::new("src/ops/power.rs").exists());
    assert!(Path::new("tests/ops_power_integration.rs").exists());
    assert!(Path::new("bin/open-docs-vision.sh").exists());
    assert!(Path::new("crates/poolai-ui-core/src/owner_ops_depth.rs").exists());

    let vision_js = include_str!("../docs/vision/vision.js");
    assert!(vision_js.contains("bindVisionPowerMenu"));
    assert!(vision_js.contains("vision-power-menu"));

    let admin = include_str!("../src/ui/admin/mod.rs");
    assert!(admin.contains("poolaiOpenAdminPowerModal"));
    assert!(admin.contains("admin-power-btn"));
}
