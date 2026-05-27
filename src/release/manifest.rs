use serde::Deserialize;

/// Galaxy §9.2 release manifest (subset implemented for verify-release).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub version: String,
    #[serde(default)]
    pub git_tag: Option<String>,
    #[serde(default)]
    pub protocol_min: Option<String>,
    #[serde(default)]
    pub protocol_max: Option<String>,
    pub artifacts: Vec<ReleaseArtifact>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ReleaseArtifact {
    pub name: String,
    #[serde(default)]
    pub path: Option<String>,
    pub sha256: String,
    #[serde(default)]
    pub sig_ref: Option<String>,
}

impl ReleaseManifest {
    pub fn parse_json(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn find_artifact(&self, name: &str) -> Option<&ReleaseArtifact> {
        self.artifacts
            .iter()
            .find(|a| a.name == name || a.path.as_deref() == Some(name))
    }
}
