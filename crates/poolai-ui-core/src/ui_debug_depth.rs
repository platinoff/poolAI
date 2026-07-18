//! UI/debug polish band depth (PH-S1019…S1028, band 38).

use serde_json::Value;

/// Band-38 UI/debug polish depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiDebugDepth {
    None,
    VisionPowerPolish,
    AdminPowerI18n,
    HomePowerShortcut,
    ClippyHygiene,
    ChronoUiCore,
    DesignTokensAudit,
    PowerFeedbackUx,
    MsysHardening,
    FullUiDebug,
}

/// FM §5.18 band-38 marker rows.
pub const FM_BAND38_ROWS: &[&str] = &["5.18", "UI/debug polish", "PH-S1019…S1028", "power UX"];

/// RUN_LOCAL / bin markers for band 38.
pub const UI_DEBUG_BAND38_ROWS: &[&str] = &[
    "PH-S1019",
    "PH-S1020",
    "PH-S1021",
    "poolai-msys.ps1",
    "PH-S1028",
];

pub fn ui_debug_depth_stub(features: Option<&Value>) -> UiDebugDepth {
    let Some(f) = features else {
        return UiDebugDepth::None;
    };
    let vision = f
        .get("vision_power_polish")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let admin_i18n = f
        .get("admin_power_i18n")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let home = f
        .get("home_power_shortcut")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let clippy = f
        .get("clippy_hygiene")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let chrono = f
        .get("chrono_ui_core")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tokens = f
        .get("design_tokens_audit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let feedback = f
        .get("power_feedback_ux")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let msys = f
        .get("msys_hardening")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [
        vision, admin_i18n, home, clippy, chrono, tokens, feedback, msys,
    ];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => UiDebugDepth::None,
        8 => UiDebugDepth::FullUiDebug,
        _ if vision && !admin_i18n => UiDebugDepth::VisionPowerPolish,
        _ if admin_i18n => UiDebugDepth::AdminPowerI18n,
        _ if home => UiDebugDepth::HomePowerShortcut,
        _ if clippy => UiDebugDepth::ClippyHygiene,
        _ if chrono => UiDebugDepth::ChronoUiCore,
        _ if tokens => UiDebugDepth::DesignTokensAudit,
        _ if feedback => UiDebugDepth::PowerFeedbackUx,
        _ if msys => UiDebugDepth::MsysHardening,
        _ => UiDebugDepth::FullUiDebug,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ui_debug_depth_stub_ph_s1028() {
        assert_eq!(ui_debug_depth_stub(None), UiDebugDepth::None);
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
    }
}
