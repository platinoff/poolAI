//! Grid envelope v1 — JSON wire format.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported envelope version (JSON field `v`).
pub const GRID_ENVELOPE_VERSION: u32 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum GridEnvelopeError {
    UnsupportedVersion(u32),
    UnknownMessageType(String),
    Json(String),
}

impl fmt::Display for GridEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(v) => write!(f, "grid envelope: unsupported version {v}"),
            Self::UnknownMessageType(t) => write!(f, "grid envelope: unknown message type {t}"),
            Self::Json(e) => write!(f, "grid envelope: json error: {e}"),
        }
    }
}

impl std::error::Error for GridEnvelopeError {}

/// Top-level Grid wire container (v1).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridEnvelope {
    /// Protocol version; must be [`GRID_ENVELOPE_VERSION`].
    pub v: u32,
    /// UTC timestamp when the envelope was created (producer clock).
    pub sent_at: DateTime<Utc>,
    /// Optional originating peer id (routing / audit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_peer_id: Option<String>,
    #[serde(flatten)]
    pub msg: GridMessage,
}

/// Logical Grid message kinds (Priority 6 / GRID_PROTOCOL_CONCEPT).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GridMessage {
    Job(GridJobBody),
    Result(GridResultBody),
    MemoryShard(GridMemoryShardBody),
    PeerStatus(GridPeerStatusBody),
}

/// Job request on the grid plane.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridJobBody {
    pub job_id: String,
    /// Task kind: `inference`, `training`, `memory`, `system`, …
    pub task_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_artifact_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
}

/// Job execution result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridResultBody {
    pub job_id: String,
    pub status: GridResultStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_artifact_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GridResultStatus {
    Completed,
    Failed,
    Verified,
}

/// Memory shard descriptor (RAID artifact plane).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridMemoryShardBody {
    pub shard_id: String,
    pub artifact_id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_logical_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_hints: Option<Vec<String>>,
}

/// Peer health / capacity (discovery plane).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridPeerStatusBody {
    pub peer_id: String,
    pub address: String,
    pub port: u16,
    pub last_seen: DateTime<Utc>,
    pub cpu_cores: usize,
    pub memory_mb: usize,
    #[serde(default)]
    pub gpu_devices: Vec<usize>,
    #[serde(default)]
    pub current_load: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl GridEnvelope {
    pub fn new(msg: GridMessage, source_peer_id: Option<String>) -> Self {
        Self {
            v: GRID_ENVELOPE_VERSION,
            sent_at: Utc::now(),
            source_peer_id,
            msg,
        }
    }

    pub fn validate(&self) -> Result<(), GridEnvelopeError> {
        if self.v != GRID_ENVELOPE_VERSION {
            return Err(GridEnvelopeError::UnsupportedVersion(self.v));
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, GridEnvelopeError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|e| GridEnvelopeError::Json(e.to_string()))
    }

    pub fn from_json(s: &str) -> Result<Self, GridEnvelopeError> {
        let env: Self =
            serde_json::from_str(s).map_err(|e| GridEnvelopeError::Json(e.to_string()))?;
        env.validate()?;
        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_status_round_trip_json() {
        let env = GridEnvelope::new(
            GridMessage::PeerStatus(GridPeerStatusBody {
                peer_id: "node-1".into(),
                address: "127.0.0.1".into(),
                port: 8080,
                last_seen: Utc::now(),
                cpu_cores: 8,
                memory_mb: 16384,
                gpu_devices: vec![0],
                current_load: 0.25,
                role: Some("miner".into()),
            }),
            Some("coordinator".into()),
        );
        let json = env.to_json().unwrap();
        let back = GridEnvelope::from_json(&json).unwrap();
        assert_eq!(back.v, GRID_ENVELOPE_VERSION);
        assert_eq!(back.source_peer_id, env.source_peer_id);
        assert_eq!(back.msg, env.msg);
    }

    #[test]
    fn job_and_result_types_in_json() {
        let job = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "j-1".into(),
                task_kind: "inference".into(),
                verification_policy: Some("replicate-3".into()),
                input_artifact_ids: vec!["art-1".into()],
                deadline: None,
            }),
            None,
        );
        let parsed = GridEnvelope::from_json(&job.to_json().unwrap()).unwrap();
        assert!(matches!(parsed.msg, GridMessage::Job(_)));

        let result = GridEnvelope::new(
            GridMessage::Result(GridResultBody {
                job_id: "j-1".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec!["art-out".into()],
                proof: None,
                metrics: None,
            }),
            None,
        );
        assert!(matches!(
            GridEnvelope::from_json(&result.to_json().unwrap())
                .unwrap()
                .msg,
            GridMessage::Result(_)
        ));
    }

    #[test]
    fn rejects_unsupported_version() {
        let env = GridEnvelope {
            v: 99,
            sent_at: Utc::now(),
            source_peer_id: None,
            msg: GridMessage::PeerStatus(GridPeerStatusBody {
                peer_id: "x".into(),
                address: "h".into(),
                port: 1,
                last_seen: Utc::now(),
                cpu_cores: 1,
                memory_mb: 1024,
                gpu_devices: vec![],
                current_load: 0.0,
                role: None,
            }),
        };
        assert!(matches!(
            env.validate(),
            Err(GridEnvelopeError::UnsupportedVersion(99))
        ));
    }
}
