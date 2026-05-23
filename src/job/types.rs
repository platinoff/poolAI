//! Job layer wire types (P6 / Horizon S38).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Opaque job identifier (string for Grid / HTTP interop).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobId(pub String);

impl JobId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Task category (JOB_LAYER_CONCEPT §2.1).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    Inference,
    Training,
    FineTune,
    Indexing,
    Embeddings,
    Memory,
    System,
}

impl JobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inference => "inference",
            Self::Training => "training",
            Self::FineTune => "fine_tune",
            Self::Indexing => "indexing",
            Self::Embeddings => "embeddings",
            Self::Memory => "memory",
            Self::System => "system",
        }
    }
}

/// Lifecycle state (JOB_LAYER_CONCEPT §2.2).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Submitted,
    Scheduled,
    Executing,
    Verifying,
    Rewarded,
    Completed,
    Failed,
}

/// Resource requirements (subset of JobSpec).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct JobResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_threads: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_memory_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<u64>,
}

/// Job specification for scheduling / Grid Job messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobSpec {
    pub id: JobId,
    pub kind: JobKind,
    #[serde(default)]
    pub resources: JobResources,
    #[serde(default)]
    pub priority: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_secs: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_artifact_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
}

/// Optional VM/worker target chosen by the scheduler (FM-034).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct JobScheduleBinding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

/// Stored job row for HTTP stub / metrics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobRecord {
    pub spec: JobSpec,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    /// Pool worker assigned at schedule time (FM-034).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    /// Running VM instance assigned at schedule time (FM-034).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}
