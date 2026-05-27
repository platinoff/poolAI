use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use super::error::VerifyReleaseError;

#[derive(Debug, Clone, Deserialize)]
pub struct TrustRoot {
    #[serde(default)]
    pub maintainer_keys: Vec<MaintainerKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaintainerKey {
    pub key_id: String,
    pub public_key_hex: String,
}

impl TrustRoot {
    pub fn parse_json(bytes: &[u8]) -> Result<Self, VerifyReleaseError> {
        serde_json::from_slice(bytes)
            .map_err(|e| VerifyReleaseError::InvalidTrustRootJson(e.to_string()))
    }

    pub fn load(path: &Path) -> Result<Self, VerifyReleaseError> {
        if !path.exists() {
            return Err(VerifyReleaseError::TrustRootNotFound(path.to_path_buf()));
        }
        let bytes = std::fs::read(path).map_err(|e| VerifyReleaseError::IoRead {
            path: path.to_path_buf(),
            source: e,
        })?;
        Self::parse_json(&bytes)
    }

    pub fn key_map(&self) -> HashMap<&str, &str> {
        self.maintainer_keys
            .iter()
            .map(|k| (k.key_id.as_str(), k.public_key_hex.as_str()))
            .collect()
    }
}
