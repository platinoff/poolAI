//! Devnet-oriented RPC configuration for the sidecar (FM-024 / FM-033).
//!
//! **No mainnet** — `mainnet-beta` is rejected at load time.

use crate::instruction::{is_placeholder_program_id, PLACEHOLDER_PROGRAM_ID};
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::env;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DEFAULT_DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
pub const ENV_CONFIG_PATH: &str = "POOLAI_SOLANA_CONFIG";
pub const ENV_CLUSTER: &str = "POOLAI_SOLANA_CLUSTER";
pub const ENV_RPC_URL: &str = "POOLAI_SOLANA_RPC_URL";
pub const ENV_MOCK_RPC: &str = "POOLAI_SOLANA_MOCK_RPC";

/// Bundled default profile (see `config/devnet.toml` in this crate).
pub const BUNDLED_DEVNET_TOML: &str = include_str!("../config/devnet.toml");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SolanaCluster {
    Devnet,
    Localnet,
    #[serde(rename = "mainnet-beta")]
    MainnetBeta,
}

impl SolanaCluster {
    pub fn default_rpc_url(self) -> &'static str {
        match self {
            Self::Devnet => DEFAULT_DEVNET_RPC_URL,
            Self::Localnet => "http://127.0.0.1:8899",
            Self::MainnetBeta => "https://api.mainnet-beta.solana.com",
        }
    }

    pub fn allows_mock_stub(self) -> bool {
        !matches!(self, Self::MainnetBeta)
    }

    pub fn as_wire_str(self) -> &'static str {
        match self {
            Self::Devnet => "devnet",
            Self::Localnet => "localnet",
            Self::MainnetBeta => "mainnet-beta",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterConfig {
    pub cluster: SolanaCluster,
    pub rpc_url: String,
    pub commitment: CommitmentLevel,
    pub mock_rpc: bool,
    pub program_id: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MainnetNotAllowed,
    Io(String),
    Toml(String),
    InvalidProgramId(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainnetNotAllowed => {
                write!(
                    f,
                    "mainnet-beta is not allowed in FM-024 stub; use devnet or localnet"
                )
            }
            Self::Io(e) => write!(f, "config io error: {e}"),
            Self::Toml(e) => write!(f, "config toml error: {e}"),
            Self::InvalidProgramId(e) => write!(f, "invalid program_id: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl AdapterConfig {
    pub fn devnet_defaults() -> Self {
        Self::from_toml(BUNDLED_DEVNET_TOML).expect("bundled devnet.toml must parse")
    }

    pub fn from_toml(s: &str) -> Result<Self, ConfigError> {
        let cfg: Self = toml::from_str(s).map_err(|e| ConfigError::Toml(e.to_string()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let bytes =
            std::fs::read_to_string(path.as_ref()).map_err(|e| ConfigError::Io(e.to_string()))?;
        Self::from_toml(&bytes)
    }

    /// Load bundled devnet profile, then apply `POOLAI_SOLANA_*` overrides.
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = if let Ok(path) = env::var(ENV_CONFIG_PATH) {
            Self::from_file(path)?
        } else {
            Self::devnet_defaults()
        };

        if let Ok(cluster) = env::var(ENV_CLUSTER) {
            let parsed = parse_cluster_env(&cluster).map_err(|e| ConfigError::Toml(e))?;
            cfg.cluster = parsed;
            if cfg.rpc_url.is_empty() || cfg.rpc_url == DEFAULT_DEVNET_RPC_URL {
                cfg.rpc_url = parsed.default_rpc_url().to_string();
            }
        }

        if let Ok(url) = env::var(ENV_RPC_URL) {
            if !url.trim().is_empty() {
                cfg.rpc_url = url;
            }
        }

        if let Ok(mock) = env::var(ENV_MOCK_RPC) {
            cfg.mock_rpc = matches!(mock.trim(), "1" | "true" | "yes" | "on");
        }

        if let Ok(program_id) = env::var(crate::rpc::ENV_PROGRAM_ID) {
            let trimmed = program_id.trim();
            if !trimmed.is_empty() {
                cfg.program_id = trimmed.to_string();
            }
        }

        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if matches!(self.cluster, SolanaCluster::MainnetBeta) {
            return Err(ConfigError::MainnetNotAllowed);
        }
        if self.rpc_url.trim().is_empty() {
            return Err(ConfigError::Toml("rpc_url must not be empty".into()));
        }
        if !is_placeholder_program_id(&self.program_id)
            && Pubkey::from_str(self.program_id.trim()).is_err()
        {
            return Err(ConfigError::InvalidProgramId(format!(
                "program_id must be a valid base58 pubkey or placeholder {PLACEHOLDER_PROGRAM_ID}"
            )));
        }
        Ok(())
    }

    /// Resolved program id for submit (env `POOLAI_SOLANA_PROGRAM_ID` overrides TOML).
    pub fn resolved_program_id(&self) -> String {
        std::env::var(crate::rpc::ENV_PROGRAM_ID)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| self.program_id.clone())
    }

    /// `true` when a deployed `poolai-events` program id is configured (not Memo fallback).
    pub fn uses_custom_program(&self) -> bool {
        !is_placeholder_program_id(&self.resolved_program_id())
    }

    pub fn bundled_devnet_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/devnet.toml")
    }
}

fn parse_cluster_env(raw: &str) -> Result<SolanaCluster, String> {
    match raw.trim().to_lowercase().replace('_', "-").as_str() {
        "devnet" => Ok(SolanaCluster::Devnet),
        "localnet" => Ok(SolanaCluster::Localnet),
        "mainnet" | "mainnet-beta" => Ok(SolanaCluster::MainnetBeta),
        other => Err(format!("unknown cluster: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_devnet_parses() {
        let cfg = AdapterConfig::devnet_defaults();
        assert_eq!(cfg.cluster, SolanaCluster::Devnet);
        assert_eq!(cfg.rpc_url, DEFAULT_DEVNET_RPC_URL);
        assert!(!cfg.mock_rpc);
    }

    #[test]
    fn rejects_mainnet_in_toml() {
        let err = AdapterConfig::from_toml(
            r#"
cluster = "mainnet-beta"
rpc_url = "https://api.mainnet-beta.solana.com"
commitment = "confirmed"
mock_rpc = true
program_id = "11111111111111111111111111111111"
"#,
        )
        .unwrap_err();
        assert_eq!(err, ConfigError::MainnetNotAllowed);
    }

    #[test]
    fn rejects_invalid_deployed_program_id() {
        let err = AdapterConfig::from_toml(
            r#"
cluster = "devnet"
rpc_url = "https://api.devnet.solana.com"
commitment = "confirmed"
mock_rpc = false
program_id = "not-a-valid-pubkey"
"#,
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::InvalidProgramId(_)));
    }

    #[test]
    fn uses_custom_program_when_pubkey_set() {
        let cfg = AdapterConfig::from_toml(
            r#"
cluster = "devnet"
rpc_url = "https://api.devnet.solana.com"
commitment = "confirmed"
mock_rpc = false
program_id = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
"#,
        )
        .unwrap();
        assert!(cfg.uses_custom_program());
    }

    #[test]
    fn parse_cluster_env_localnet() {
        assert_eq!(
            parse_cluster_env("localnet").unwrap(),
            SolanaCluster::Localnet
        );
        assert_eq!(
            parse_cluster_env("mainnet-beta").unwrap(),
            SolanaCluster::MainnetBeta
        );
    }
}
