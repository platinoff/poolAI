//! Galaxy concept implemented-marker band depth (PH-S970, band 32).

use serde_json::Value;

/// Concept marker band depth flags (sections §1–§9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptMarkersDepth {
    None,
    Sections1To3,
    Sections4To6,
    Sections7To9,
    FullConcept,
}

/// Canonical section marker headers in POOLAI_GALAXY_GRID.md (band 32).
pub const CONCEPT_MARKER_BAND32_HEADERS: &[&str] = &[
    "band 32 PH-S970 — §1",
    "band 32 PH-S970 — §2",
    "band 32 PH-S970 — §3",
    "band 32 PH-S971 — §4",
    "band 32 PH-S971 — §5",
    "band 32 PH-S971 — §6",
    "band 22 PH-S870…S879",
    "band 32 PH-S972 — §9",
];

/// Roadmap horizon rows closed or BLOCKED in band 32 (PH-S974).
pub const CONCEPT_ROADMAP_BAND32_ROWS: &[&str] = &[
    "band 32 ✅ PH-S970…S979",
    "BLOCKED",
    "LAN replication benchmarks",
];

/// §8 TBD closure notes (PH-S973).
pub const CONCEPT_TBD_BAND32_NOTES: &[&str] = &[
    "offline batch queue stub ✅",
    "on-chain gate via `POOLAI_SETTLEMENT_ON_CHAIN`",
    "LAN replication",
    "BLOCKED",
    "FM-003",
];

/// Classify concept marker band depth from optional feature stub (PH-S970).
pub fn concept_markers_depth_stub(features: Option<&Value>) -> ConceptMarkersDepth {
    let Some(f) = features else {
        return ConceptMarkersDepth::None;
    };
    let s13 = f
        .get("sections_1_3")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let s46 = f
        .get("sections_4_6")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let s79 = f
        .get("sections_7_9")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = s13 as u8 + s46 as u8 + s79 as u8;
    match flags {
        0 => ConceptMarkersDepth::None,
        1 if s13 => ConceptMarkersDepth::Sections1To3,
        1 if s46 => ConceptMarkersDepth::Sections4To6,
        1 if s79 => ConceptMarkersDepth::Sections7To9,
        3 => ConceptMarkersDepth::FullConcept,
        _ => ConceptMarkersDepth::FullConcept,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn concept_markers_depth_stub_ph_s970() {
        assert_eq!(concept_markers_depth_stub(None), ConceptMarkersDepth::None);
        assert_eq!(
            concept_markers_depth_stub(Some(&json!({"sections_1_3": true}))),
            ConceptMarkersDepth::Sections1To3
        );
        assert_eq!(
            concept_markers_depth_stub(Some(&json!({
                "sections_1_3": true,
                "sections_4_6": true,
                "sections_7_9": true
            }))),
            ConceptMarkersDepth::FullConcept
        );
        assert_eq!(CONCEPT_MARKER_BAND32_HEADERS.len(), 8);
        assert_eq!(CONCEPT_ROADMAP_BAND32_ROWS.len(), 3);
        assert_eq!(CONCEPT_TBD_BAND32_NOTES.len(), 5);
    }
}
