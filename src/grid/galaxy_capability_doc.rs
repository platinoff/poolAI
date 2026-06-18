//! Signed capability document parse/validate stub (PH-S439, Galaxy §6.6/§9).
//!
//! JSON schema parse for edge worker capability documents; no live signature verify wire.

use serde::{Deserialize, Serialize};

/// Wire DTO for signed capability documents (Galaxy §6.6 roadmap stub).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyCapabilityDocument {
    pub peer_id: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Parse/validation failure for capability documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDocParseError {
    pub message: String,
}

impl CapabilityDocParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CapabilityDocParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Parse capability document from JSON value.
pub fn parse_capability_document(
    value: &serde_json::Value,
) -> Result<GalaxyCapabilityDocument, CapabilityDocParseError> {
    serde_json::from_value(value.clone())
        .map_err(|e| CapabilityDocParseError::new(format!("capability document parse failed: {e}")))
}

/// Validate required fields (no cryptographic verify).
pub fn validate_capability_document(
    doc: &GalaxyCapabilityDocument,
) -> Result<(), CapabilityDocParseError> {
    if doc.peer_id.trim().is_empty() {
        return Err(CapabilityDocParseError::new("peer_id required"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_capability_document_ph_s439() {
        let doc = parse_capability_document(&json!({
            "peer_id": "edge-worker-1",
            "capabilities": ["inference:gpu", "prefetch:ram"],
            "signature": "stub-sig",
            "expires_at": "2026-12-31T00:00:00Z"
        }))
        .unwrap();
        assert_eq!(doc.peer_id, "edge-worker-1");
        assert_eq!(doc.capabilities.len(), 2);
        validate_capability_document(&doc).unwrap();
    }

    #[test]
    fn validate_capability_document_rejects_empty_peer_ph_s439() {
        let doc = GalaxyCapabilityDocument {
            peer_id: "  ".into(),
            capabilities: vec![],
            signature: None,
            expires_at: None,
        };
        assert!(validate_capability_document(&doc).is_err());
    }
}
