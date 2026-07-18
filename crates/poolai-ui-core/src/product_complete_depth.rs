//! Product-complete closure band depth (PH-S1010, band 36).

use serde_json::Value;

/// Product-complete band depth flags (stable final / handoff maintenance / FM closure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductCompleteDepth {
    None,
    StableFinal,
    HandoffMaintenance,
    FmClosure,
    RatioFormal,
    FullProductComplete,
}

/// STABLE_STATE final markers (PH-S1010).
pub const STABLE_BAND36_HEADERS: &[&str] = &[
    "band 36 PH-S1010",
    "## Development complete (band 36 PH-S1010)",
    "maintenance mode",
];

/// HANDOFF active maintenance markers (PH-S1010).
pub const HANDOFF_MAINTENANCE_ACTIVE: &[&str] = &[
    "## Maintenance mode (PH-S1010)",
    "FM **§5.15** ✅",
    "product-complete",
];

/// FM §5.15 closure rows (PH-S1010).
pub const FM_BAND36_ROWS: &[&str] = &[
    "PH-S1010",
    "§5.15",
    "product-complete closure",
    "maintenance mode",
];

/// GALAXY_GRID_ROADMAP band-36 marker rows.
pub const GALAXY_BAND36_ROWS: &[&str] = &["band 36 PH-S1010", "product-complete", "FM **§5.15**"];

/// Classify product-complete band depth from optional feature stub (PH-S1010).
pub fn product_complete_depth_stub(features: Option<&Value>) -> ProductCompleteDepth {
    let Some(f) = features else {
        return ProductCompleteDepth::None;
    };
    let stable = f
        .get("stable_final")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let handoff = f
        .get("handoff_maintenance")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fm = f
        .get("fm_closure")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_formal")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = stable as u8 + handoff as u8 + fm as u8 + ratio as u8;
    match flags {
        0 => ProductCompleteDepth::None,
        1 if stable => ProductCompleteDepth::StableFinal,
        1 if handoff => ProductCompleteDepth::HandoffMaintenance,
        1 if fm => ProductCompleteDepth::FmClosure,
        1 if ratio => ProductCompleteDepth::RatioFormal,
        4 => ProductCompleteDepth::FullProductComplete,
        _ => ProductCompleteDepth::FullProductComplete,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn product_complete_depth_stub_ph_s1010() {
        assert_eq!(
            product_complete_depth_stub(None),
            ProductCompleteDepth::None
        );
        assert_eq!(
            product_complete_depth_stub(Some(&json!({"stable_final": true}))),
            ProductCompleteDepth::StableFinal
        );
        assert_eq!(
            product_complete_depth_stub(Some(&json!({
                "stable_final": true,
                "handoff_maintenance": true,
                "fm_closure": true,
                "ratio_formal": true
            }))),
            ProductCompleteDepth::FullProductComplete
        );
        assert_eq!(STABLE_BAND36_HEADERS.len(), 3);
        assert_eq!(HANDOFF_MAINTENANCE_ACTIVE.len(), 3);
        assert_eq!(FM_BAND36_ROWS.len(), 4);
        assert_eq!(GALAXY_BAND36_ROWS.len(), 3);
    }
}
