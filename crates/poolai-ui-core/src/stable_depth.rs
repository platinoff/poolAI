//! STABLE / INDEX product-complete band depth (PH-S980, band 33).

use serde_json::Value;

/// STABLE product-complete band depth flags (stable / index / readme / handoff / progress).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StableDepth {
    None,
    StableDraft,
    IndexZriz,
    ReadmeMaintenance,
    HandoffTemplate,
    ProgressScope,
    FullStable,
}

/// Canonical STABLE_STATE markers for band 33 (PH-S980).
pub const STABLE_BAND33_HEADERS: &[&str] = &[
    "band 33 PH-S980",
    "## Development complete (draft, band 33 PH-S980)",
    "product-complete",
];

/// INDEX step 1–12 product-complete rows (PH-S981).
pub const INDEX_BAND33_ROWS: &[&str] =
    &["band 33", "PH-S981", "product-complete", "§5.14", "§5.15"];

/// HANDOFF maintenance-mode template markers (PH-S983).
pub const HANDOFF_MAINTENANCE_MARKERS: &[&str] =
    &["Maintenance mode (template", "PH-S983", "post-S1010"];

/// Classify STABLE product-complete band depth from optional feature stub (PH-S980).
pub fn stable_depth_stub(features: Option<&Value>) -> StableDepth {
    let Some(f) = features else {
        return StableDepth::None;
    };
    let stable = f
        .get("stable_draft")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let index = f
        .get("index_zriz")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let readme = f
        .get("readme_maintenance")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let handoff = f
        .get("handoff_template")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let progress = f
        .get("progress_scope")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = stable as u8 + index as u8 + readme as u8 + handoff as u8 + progress as u8;
    match flags {
        0 => StableDepth::None,
        1 if stable => StableDepth::StableDraft,
        1 if index => StableDepth::IndexZriz,
        1 if readme => StableDepth::ReadmeMaintenance,
        1 if handoff => StableDepth::HandoffTemplate,
        1 if progress => StableDepth::ProgressScope,
        5 => StableDepth::FullStable,
        _ => StableDepth::FullStable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stable_depth_stub_ph_s980() {
        assert_eq!(stable_depth_stub(None), StableDepth::None);
        assert_eq!(
            stable_depth_stub(Some(&json!({"stable_draft": true}))),
            StableDepth::StableDraft
        );
        assert_eq!(
            stable_depth_stub(Some(&json!({
                "stable_draft": true,
                "index_zriz": true,
                "readme_maintenance": true,
                "handoff_template": true,
                "progress_scope": true
            }))),
            StableDepth::FullStable
        );
        assert_eq!(STABLE_BAND33_HEADERS.len(), 3);
        assert_eq!(INDEX_BAND33_ROWS.len(), 5);
        assert_eq!(HANDOFF_MAINTENANCE_MARKERS.len(), 3);
    }
}
