//! PH-S1028: Galaxy horizon close band 38 — UI/debug polish.

use poolai_ui_core::design_tokens::{design_tokens_parity_gate, DESIGN_TOKENS_AUDIT_NOTE};
use poolai_ui_core::owner_ops_depth::admin_power_panel_script;
use poolai_ui_core::ui_debug_depth::{
    ui_debug_depth_stub, UiDebugDepth, FM_BAND38_ROWS, UI_DEBUG_BAND38_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1019_band_ui_debug_close_ph_s1028() {
    assert_eq!(
        ui_debug_depth_stub(Some(&json!({"vision_power_polish": true}))),
        UiDebugDepth::VisionPowerPolish
    );
    assert_eq!(
        ui_debug_depth_stub(Some(&json!({
            "vision_power_polish": true,
            "admin_power_i18n": true,
            "home_power_shortcut": true,
            "clippy_hygiene": true,
            "chrono_ui_core": true,
            "design_tokens_audit": true,
            "power_feedback_ux": true,
            "msys_hardening": true
        }))),
        UiDebugDepth::FullUiDebug
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND38_ROWS {
        assert!(fm.contains(row), "FM missing band-38 row {row}");
    }
    assert!(fm.contains("PH-S1028"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    for row in UI_DEBUG_BAND38_ROWS {
        assert!(
            run_local.contains(row) || row.starts_with("PH-S"),
            "RUN_LOCAL missing band-38 marker {row}"
        );
    }
    assert!(run_local.contains("poolai-msys.ps1"));
    assert!(run_local.contains("-Command") || run_local.contains("-lc"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1019") || handoff.contains("band 38"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    assert!(design_tokens_parity_gate());
    assert!(DESIGN_TOKENS_AUDIT_NOTE.contains("PH-S1025"));

    let power_script = admin_power_panel_script();
    assert!(power_script.contains("adminAnnounceLive"));
    assert!(power_script.contains("admin.power.result"));

    let vision_js = include_str!("../docs/vision/vision.js");
    assert!(vision_js.contains("visionPowerMenuItems"));
    assert!(vision_js.contains("visionAnnouncePower"));
    assert!(vision_js.contains("ArrowDown"));

    let vision_html = include_str!("../docs/vision/index.html");
    assert!(vision_html.contains("superseded by GSV"));
    assert!(!vision_html.contains("vision-power-status"));

    let admin = include_str!("../src/ui/admin/mod.rs");
    assert!(admin.contains("admin.power.btn"));
    assert!(admin.contains("admin_power_patch"));

    let home = include_str!("../src/ui/mod.rs");
    assert!(home.contains("home-power-shutdown"));
    assert!(home.contains("home_power_shell_script"));

    let msys = include_str!("../bin/poolai-msys.ps1");
    assert!(msys.contains("[CmdletBinding()]"));
    assert!(msys.contains("-Command"));

    assert!(Path::new("crates/poolai-ui-core/src/ui_debug_depth.rs").exists());
}
