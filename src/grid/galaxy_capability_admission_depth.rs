//! Galaxy signed capability admission depth classification stub (PH-S744, §6.6).

use crate::grid::galaxy_capability_doc::GalaxyCapabilityDocument;

/// Signed capability admission telemetry depth (Galaxy §6.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAdmissionDepth {
    None,
    Unsigned,
    SignedDevFixture,
    SignedWithExpiry,
    SignedWithTee,
}

/// Classify capability document depth from optional wire stub (PH-S744).
pub fn capability_admission_depth_stub(
    doc: Option<&GalaxyCapabilityDocument>,
) -> CapabilityAdmissionDepth {
    let Some(doc) = doc else {
        return CapabilityAdmissionDepth::None;
    };
    let has_sig = doc
        .signature
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if !has_sig {
        return CapabilityAdmissionDepth::Unsigned;
    }
    let has_tee = doc
        .tee_attestation
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    let has_expiry = doc
        .expires_at
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if has_tee {
        CapabilityAdmissionDepth::SignedWithTee
    } else if has_expiry {
        CapabilityAdmissionDepth::SignedWithExpiry
    } else {
        CapabilityAdmissionDepth::SignedDevFixture
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_admission_depth_stub_ph_s744() {
        assert_eq!(
            capability_admission_depth_stub(None),
            CapabilityAdmissionDepth::None
        );
        assert_eq!(
            capability_admission_depth_stub(Some(&GalaxyCapabilityDocument {
                peer_id: "edge-1".into(),
                capabilities: vec![],
                signature: None,
                expires_at: None,
                tee_attestation: None,
            })),
            CapabilityAdmissionDepth::Unsigned
        );
        assert_eq!(
            capability_admission_depth_stub(Some(&GalaxyCapabilityDocument {
                peer_id: "edge-1".into(),
                capabilities: vec!["inference:edge".into()],
                signature: Some("aa".repeat(64)),
                expires_at: None,
                tee_attestation: None,
            })),
            CapabilityAdmissionDepth::SignedDevFixture
        );
        assert_eq!(
            capability_admission_depth_stub(Some(&GalaxyCapabilityDocument {
                peer_id: "edge-1".into(),
                capabilities: vec!["inference:edge".into()],
                signature: Some("aa".repeat(64)),
                expires_at: Some("2027-12-31T00:00:00Z".into()),
                tee_attestation: None,
            })),
            CapabilityAdmissionDepth::SignedWithExpiry
        );
        assert_eq!(
            capability_admission_depth_stub(Some(&GalaxyCapabilityDocument {
                peer_id: "edge-1".into(),
                capabilities: vec!["inference:gpu".into()],
                signature: Some("aa".repeat(64)),
                expires_at: Some("2027-12-31T00:00:00Z".into()),
                tee_attestation: Some("attest-blob".into()),
            })),
            CapabilityAdmissionDepth::SignedWithTee
        );
    }
}
