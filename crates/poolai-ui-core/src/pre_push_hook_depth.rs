//! Pre-push vision canon gate band depth (PH-S1129…S1138, band 49).

use serde_json::Value;

/// Pre-push hook / vision canon gate depth flags (git hook + poolai-vision-sync canon docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrePushHookDepth {
    None,
    HookScript,
    InstallScript,
    VisionSyncCanon,
    VisionSyncCheck,
    CargoFmtGate,
    HookDocs,
    VerifyDevStandHook,
    FullBand49,
}

/// Pre-push canon gate criteria registry (PH-S1131): id · marker · doc path.
pub const PRE_PUSH_HOOK_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "pre_push_hook_script",
        "poolai-vision-sync",
        "bin/pre-push-hook.sh",
    ),
    (
        "install_hook",
        "install-pre-push-hook.sh",
        "bin/install-pre-push-hook.sh",
    ),
    (
        "vision_sync_canon",
        "sync_vision_canon_docs",
        "src/bin/poolai_vision_sync.rs",
    ),
    (
        "vision_sync_check",
        "collect_canon_docs_drift",
        "src/bin/poolai_vision_sync.rs",
    ),
    (
        "cargo_fmt_gate",
        "cargo fmt --all --check",
        "bin/pre-push-hook.sh",
    ),
    (
        "pre_push_hook_docs",
        "PRE_PUSH_HOOK.md",
        "docs/development/PRE_PUSH_HOOK.md",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_PRE_PUSH_CANON",
        "bin/verify-dev-stand.sh",
    ),
];

/// `poolai-loc-audit --pre-push-canon` case names (PH-S1130).
pub const PRE_PUSH_HOOK_CASES: &[&str] = &[
    "pre_push_hook_script",
    "install_hook",
    "vision_sync_canon",
    "vision_sync_check",
    "cargo_fmt_gate",
    "pre_push_hook_docs",
    "verify_dev_stand_hook",
];

/// FM §5.30 band-49 marker rows.
pub const FM_BAND49_ROWS: &[&str] = &[
    "5.30",
    "Pre-push vision canon gate",
    "PH-S1129…S1138",
    "pre_push_hook_depth",
];

/// Pre-push canon gate adoption markers for band 49.
pub const PRE_PUSH_HOOK_BAND49_ROWS: &[&str] = &[
    "PH-S1129",
    "pre_push_hook_depth",
    "PH-S1130",
    "--pre-push-canon",
    "PH-S1134",
    "VERIFY_PRE_PUSH_CANON",
    "PH-S1134",
    "--pre-push-canon",
    "PH-S1138",
];

/// Classify pre-push hook band depth from optional feature stub (PH-S1129).
pub fn pre_push_hook_depth_stub(features: Option<&Value>) -> PrePushHookDepth {
    let Some(f) = features else {
        return PrePushHookDepth::None;
    };
    let hook = f
        .get("pre_push_hook_script")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let install = f
        .get("install_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let canon = f
        .get("vision_sync_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let check = f
        .get("vision_sync_check")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fmt = f
        .get("cargo_fmt_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("pre_push_hook_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if hook && install && canon && check && fmt && docs && verify {
        return PrePushHookDepth::FullBand49;
    }
    if verify {
        return PrePushHookDepth::VerifyDevStandHook;
    }
    if docs {
        return PrePushHookDepth::HookDocs;
    }
    if fmt {
        return PrePushHookDepth::CargoFmtGate;
    }
    if check {
        return PrePushHookDepth::VisionSyncCheck;
    }
    if canon {
        return PrePushHookDepth::VisionSyncCanon;
    }
    if install {
        return PrePushHookDepth::InstallScript;
    }
    if hook {
        return PrePushHookDepth::HookScript;
    }
    PrePushHookDepth::None
}

/// Total pre-push canon criteria in registry (PH-S1131).
pub fn pre_push_hook_criteria_total() -> usize {
    PRE_PUSH_HOOK_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn pre_push_hook_depth_stub_ph_s1129() {
        assert_eq!(pre_push_hook_depth_stub(None), PrePushHookDepth::None);
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({"pre_push_hook_script": true}))),
            PrePushHookDepth::HookScript
        );
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({
                "pre_push_hook_script": true,
                "install_hook": true,
                "vision_sync_canon": true,
                "vision_sync_check": true,
                "cargo_fmt_gate": true,
                "pre_push_hook_docs": true,
                "verify_dev_stand_hook": true,
            }))),
            PrePushHookDepth::FullBand49
        );
        assert_eq!(PRE_PUSH_HOOK_CRITERIA.len(), 7);
        assert_eq!(pre_push_hook_criteria_total(), 7);
        assert!(!PRE_PUSH_HOOK_CASES.is_empty());
        assert!(FM_BAND49_ROWS.contains(&"PH-S1129…S1138"));
    }
}
