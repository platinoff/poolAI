//! Ratio 96% stretch band depth classification (PH-S944, band 29).

use serde_json::Value;

/// Stretch spirit band depth flags (e2e scope / ratio / ops shell).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StretchDepth {
    None,
    E2eScope,
    RatioStretch,
    OpsShell,
    E2eRatioOps,
}

/// Classify stretch band depth from optional feature stub (PH-S944).
pub fn stretch_depth_stub(features: Option<&Value>) -> StretchDepth {
    let Some(f) = features else {
        return StretchDepth::None;
    };
    let e2e = f
        .get("e2e_scope_audit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_stretch_spirit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ops = f
        .get("ops_shell_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = e2e as u8 + ratio as u8 + ops as u8;
    match flags {
        0 => StretchDepth::None,
        1 if e2e => StretchDepth::E2eScope,
        1 if ratio => StretchDepth::RatioStretch,
        1 if ops => StretchDepth::OpsShell,
        _ => StretchDepth::E2eRatioOps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stretch_depth_stub_ph_s944() {
        assert_eq!(stretch_depth_stub(None), StretchDepth::None);
        assert_eq!(
            stretch_depth_stub(Some(&json!({"e2e_scope_audit": true}))),
            StretchDepth::E2eScope
        );
        assert_eq!(
            stretch_depth_stub(Some(&json!({"ratio_stretch_spirit": true}))),
            StretchDepth::RatioStretch
        );
        assert_eq!(
            stretch_depth_stub(Some(&json!({"ops_shell_canon": true}))),
            StretchDepth::OpsShell
        );
        assert_eq!(
            stretch_depth_stub(Some(&json!({
                "e2e_scope_audit": true,
                "ratio_stretch_spirit": true,
                "ops_shell_canon": true
            }))),
            StretchDepth::E2eRatioOps
        );
    }
}
