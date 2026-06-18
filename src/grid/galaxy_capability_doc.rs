//! Signed capability document parse/validate stub (PH-S439, Galaxy §6.6/§9).
//!
//! JSON schema parse for edge worker capability documents; ed25519 verify stub (PH-S466).

use ed25519_dalek::{Signature, VerifyingKey};
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

/// Env: dev fixture verifying key hex (PH-S466; matches `tests/fixtures/capability/dev_pubkey.hex`).
pub const DEV_CAPABILITY_VERIFY_PK_HEX: &str =
    "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";

/// Env: override capability verify public key hex (PH-S476).
pub const ENV_CAPABILITY_VERIFY_PK_HEX: &str = "POOLAI_GALAXY_CAPABILITY_VERIFY_PK_HEX";

/// Canonical signing message for capability documents (PH-S466 stub).
pub fn capability_signing_message(doc: &GalaxyCapabilityDocument) -> String {
    format!("{}:{}", doc.peer_id.trim(), doc.capabilities.join(","))
}

/// Ed25519 signature verify stub for dev fixtures (PH-S466).
pub fn verify_capability_signature_stub(
    doc: &GalaxyCapabilityDocument,
) -> Result<(), CapabilityDocParseError> {
    let Some(sig_hex) = doc.signature.as_ref() else {
        return Ok(());
    };
    if sig_hex.trim().is_empty() {
        return Err(CapabilityDocParseError::new("signature must not be empty"));
    }
    let pk_hex = std::env::var(ENV_CAPABILITY_VERIFY_PK_HEX)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| DEV_CAPABILITY_VERIFY_PK_HEX.to_string());
    let pk_bytes = hex::decode(pk_hex.trim())
        .map_err(|e| CapabilityDocParseError::new(format!("capability verify pk decode: {e}")))?;
    let pk_array: [u8; 32] = pk_bytes
        .try_into()
        .map_err(|_| CapabilityDocParseError::new("dev capability verify pk length"))?;
    let verifying_key = VerifyingKey::from_bytes(&pk_array)
        .map_err(|e| CapabilityDocParseError::new(format!("capability verify pk: {e}")))?;
    let sig_bytes = hex::decode(sig_hex.trim()).map_err(|e| {
        CapabilityDocParseError::new(format!("capability signature hex decode: {e}"))
    })?;
    let sig_array: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| CapabilityDocParseError::new("capability signature length"))?;
    let signature = Signature::from_bytes(&sig_array);
    verifying_key
        .verify_strict(capability_signing_message(doc).as_bytes(), &signature)
        .map_err(|_| CapabilityDocParseError::new("capability signature invalid"))
}

/// Validate required fields and optional dev signature (PH-S466).
pub fn validate_capability_document(
    doc: &GalaxyCapabilityDocument,
) -> Result<(), CapabilityDocParseError> {
    if doc.peer_id.trim().is_empty() {
        return Err(CapabilityDocParseError::new("peer_id required"));
    }
    verify_capability_signature_stub(doc)
}

/// `telegram_edge` register-remote requires signed capability document (PH-S504, Galaxy §6.6).
pub fn validate_telegram_edge_capability(
    is_telegram_edge: bool,
    doc: Option<&GalaxyCapabilityDocument>,
) -> Result<(), CapabilityDocParseError> {
    if !is_telegram_edge {
        return Ok(());
    }
    let doc = doc.ok_or_else(|| {
        CapabilityDocParseError::new("capability_document required for telegram_edge origin")
    })?;
    if doc
        .signature
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        return Err(CapabilityDocParseError::new(
            "signed capability_document required for telegram_edge origin",
        ));
    }
    validate_capability_document(doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;
    use serde_json::json;

    fn dev_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[7u8; 32])
    }

    #[test]
    fn parse_capability_document_ph_s439() {
        let doc = parse_capability_document(&json!({
            "peer_id": "edge-worker-1",
            "capabilities": ["inference:gpu", "prefetch:ram"],
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

    #[test]
    fn verify_capability_signature_stub_ph_s466() {
        let sk = dev_signing_key();
        let pk_hex = hex::encode(sk.verifying_key().to_bytes());
        assert_eq!(pk_hex, DEV_CAPABILITY_VERIFY_PK_HEX);
        let unsigned = GalaxyCapabilityDocument {
            peer_id: "edge-worker-1".into(),
            capabilities: vec!["inference:gpu".into()],
            signature: None,
            expires_at: None,
        };
        let msg = capability_signing_message(&unsigned);
        let doc = GalaxyCapabilityDocument {
            signature: Some(hex::encode(sk.sign(msg.as_bytes()).to_bytes())),
            ..unsigned
        };
        validate_capability_document(&doc).unwrap();
        let mut bad = doc.clone();
        bad.signature = Some("00".repeat(64));
        assert!(validate_capability_document(&bad).is_err());
    }
}
