//! Vision map/a11y/perf band depth (PH-S1039…S1048, band 40).

use serde_json::Value;

/// Band-40 Vision map/a11y/perf depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisionMapDepth {
    None,
    SkipLinksLandmarks,
    IconAriaLabels,
    ExplorerTreeKeyboard,
    LinkGraphA11y,
    MapFilterIncremental,
    DenseMapLod,
    BackgroundTabPerf,
    FullVisionMap,
}

/// FM §5.21 band-40 marker rows.
pub const FM_BAND40_ROWS: &[&str] = &["5.21", "Vision map/a11y/perf", "PH-S1039…S1048", "vision"];

/// Vision UI adoption markers for band 40.
pub const VISION_MAP_BAND40_ROWS: &[&str] = &[
    "PH-S1039",
    "vision-skip-link",
    "role=\"tree\"",
    "link-neighbor",
    "updateMapSprintDim",
    "PH-S1048",
];

pub fn vision_map_depth_stub(features: Option<&Value>) -> VisionMapDepth {
    let Some(f) = features else {
        return VisionMapDepth::None;
    };
    let skip = f
        .get("skip_links_landmarks")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let icons = f
        .get("icon_aria_labels")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tree = f
        .get("explorer_tree_keyboard")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let links = f
        .get("link_graph_a11y")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let filters = f
        .get("map_filter_incremental")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let lod = f
        .get("dense_map_lod")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let perf = f
        .get("background_tab_perf")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [skip, icons, tree, links, filters, lod, perf];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => VisionMapDepth::None,
        7 => VisionMapDepth::FullVisionMap,
        _ if skip && !icons => VisionMapDepth::SkipLinksLandmarks,
        _ if icons => VisionMapDepth::IconAriaLabels,
        _ if tree => VisionMapDepth::ExplorerTreeKeyboard,
        _ if links => VisionMapDepth::LinkGraphA11y,
        _ if filters => VisionMapDepth::MapFilterIncremental,
        _ if lod => VisionMapDepth::DenseMapLod,
        _ if perf => VisionMapDepth::BackgroundTabPerf,
        _ => VisionMapDepth::FullVisionMap,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn vision_map_depth_stub_ph_s1048() {
        assert_eq!(vision_map_depth_stub(None), VisionMapDepth::None);
        assert_eq!(
            vision_map_depth_stub(Some(&json!({"skip_links_landmarks": true}))),
            VisionMapDepth::SkipLinksLandmarks
        );
        assert_eq!(
            vision_map_depth_stub(Some(&json!({
                "skip_links_landmarks": true,
                "icon_aria_labels": true,
                "explorer_tree_keyboard": true,
                "link_graph_a11y": true,
                "map_filter_incremental": true,
                "dense_map_lod": true,
                "background_tab_perf": true
            }))),
            VisionMapDepth::FullVisionMap
        );
    }
}
