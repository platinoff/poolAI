//! DOCS_LEGACY audit band depth classification (PH-S960, band 31).

use serde_json::Value;

/// Docs legacy close band depth flags (audit / banners / concept / architect).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocsLegacyDepth {
    None,
    LegacyAudit,
    FlatBanners,
    ConceptDeHype,
    ArchitectSync,
    FullLegacy,
}

/// Flat session snapshots that received PH-S961 stale banners.
/// Paths reflect the 2026-11 archive sweep: 11 moved under `docs/archive/2026-11/`;
/// `QUICK_START.md` stays flat in `docs/`. Banner content verified for all 12.
pub const FLAT_LEGACY_DOC_SAMPLES: &[&str] = &[
    "archive/2026-11/EXECUTE_NOW.md",
    "archive/2026-11/GIT_PUSH_NOW_2026-01-22.md",
    "archive/2026-11/FIX_AND_PUSH_NOW.md",
    "archive/2026-11/AUTO_PUSH_EXECUTION.md",
    "archive/2026-11/FINAL_PUSH_READY.md",
    "archive/2026-11/AUTONOMOUS_SESSION_COMPLETE_2026-01-22.md",
    "archive/2026-11/CHAT_SUMMARY_2026-01-22.md",
    "archive/2026-11/CONTEXT_SNAPSHOT_2026-03-04.md",
    "archive/2026-11/DOCUMENTATION_CLEANUP_SUMMARY.md",
    "archive/2026-11/ROOT_CLEANUP.md",
    "archive/2026-11/WORKSPACE_CLEANUP.md",
    "QUICK_START.md",
];

/// Canonical DOCS_LEGACY audit table rows closed in band 31 (PH-S960).
pub const LEGACY_AUDIT_BAND31_ROWS: &[&str] = &[
    "NEXT_STEPS_ARCHITECT_2026-03-17.md",
    "poolAI_concept_root.txt",
    "PH-S961",
    "OPENAPI_GAP_AUDIT_2026-05-19.md",
];

/// Classify docs legacy band depth from optional feature stub (PH-S960).
pub fn docs_legacy_depth_stub(features: Option<&Value>) -> DocsLegacyDepth {
    let Some(f) = features else {
        return DocsLegacyDepth::None;
    };
    let audit = f
        .get("legacy_audit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let banners = f
        .get("flat_banners")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let concept = f
        .get("concept_dehype")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let architect = f
        .get("architect_sync")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = audit as u8 + banners as u8 + concept as u8 + architect as u8;
    match flags {
        0 => DocsLegacyDepth::None,
        1 if audit => DocsLegacyDepth::LegacyAudit,
        1 if banners => DocsLegacyDepth::FlatBanners,
        1 if concept => DocsLegacyDepth::ConceptDeHype,
        1 if architect => DocsLegacyDepth::ArchitectSync,
        4 => DocsLegacyDepth::FullLegacy,
        _ => DocsLegacyDepth::FullLegacy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn docs_legacy_depth_stub_ph_s960() {
        assert_eq!(docs_legacy_depth_stub(None), DocsLegacyDepth::None);
        assert_eq!(
            docs_legacy_depth_stub(Some(&json!({"legacy_audit": true}))),
            DocsLegacyDepth::LegacyAudit
        );
        assert_eq!(
            docs_legacy_depth_stub(Some(&json!({
                "legacy_audit": true,
                "flat_banners": true,
                "concept_dehype": true,
                "architect_sync": true
            }))),
            DocsLegacyDepth::FullLegacy
        );
        assert_eq!(FLAT_LEGACY_DOC_SAMPLES.len(), 12);
        assert_eq!(LEGACY_AUDIT_BAND31_ROWS.len(), 4);
    }
}
