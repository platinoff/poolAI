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
    /// Galaxy §4.3.1 lease holder (`srv` / worker id); unset for legacy rows (PH-S94).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_owner: Option<String>,
    /// Monotonic lease generation; CAS on `job_id + lease_epoch` (PH-S94 stub).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_epoch: Option<u64>,
    /// Lease expiry (RFC3339); worker must renew before this instant (PH-S94 stub).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_expires_at: Option<DateTime<Utc>>,
}

impl JobRecord {
    /// True when all lease wire fields are present.
    pub fn has_lease_fields(&self) -> bool {
        self.lease_owner.is_some() && self.lease_epoch.is_some() && self.lease_expires_at.is_some()
    }

    /// Active lease at `now`: owner + epoch set and not expired (Galaxy §4.3.1 stub).
    pub fn lease_active_at(&self, now: DateTime<Utc>) -> bool {
        let Some(expires) = self.lease_expires_at else {
            return false;
        };
        self.lease_owner.as_ref().is_some_and(|s| !s.is_empty())
            && self.lease_epoch.is_some()
            && now < expires
    }

    /// CAS helper stub: epoch matches and lease still active at `now`.
    pub fn lease_epoch_matches(&self, epoch: u64, now: DateTime<Utc>) -> bool {
        self.lease_epoch == Some(epoch) && self.lease_active_at(now)
    }
}

/// PATCH `/api/v1/jobs/{id}` lease epoch validation (PH-S95, Galaxy §4.3.1 CAS stub).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchLeaseEpochError {
    /// Client sent `lease_epoch` but the job row has no lease fields.
    NoLeaseOnJob,
    /// Epoch mismatch or lease expired at `now`.
    Rejected,
}

/// When `provided` is `None`, validation passes (legacy PATCH without CAS).
pub fn check_patch_lease_epoch(
    record: &JobRecord,
    provided: Option<u64>,
    now: DateTime<Utc>,
) -> Result<(), PatchLeaseEpochError> {
    let Some(epoch) = provided else {
        return Ok(());
    };
    if !record.has_lease_fields() {
        return Err(PatchLeaseEpochError::NoLeaseOnJob);
    }
    if record.lease_epoch_matches(epoch, now) {
        Ok(())
    } else {
        Err(PatchLeaseEpochError::Rejected)
    }
}

#[cfg(test)]
mod lease_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn job_record_deserializes_without_lease_fields() {
        let json = r#"{
            "spec": {
                "id": "legacy-1",
                "kind": "inference",
                "resources": {},
                "priority": 0,
                "input_artifact_ids": []
            },
            "status": "submitted",
            "created_at": "2026-05-27T12:00:00Z"
        }"#;
        let record: JobRecord = serde_json::from_str(json).expect("parse legacy job");
        assert!(!record.has_lease_fields());
        assert!(!record.lease_active_at(Utc::now()));
    }

    #[test]
    fn check_patch_lease_epoch_accepts_match_and_rejects_mismatch() {
        let expires = Utc.with_ymd_and_hms(2026, 5, 27, 13, 0, 0).unwrap();
        let before = expires - chrono::Duration::seconds(1);
        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("lease-patch"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap(),
            worker_id: None,
            vm_id: None,
            lease_owner: Some("worker-a".into()),
            lease_epoch: Some(5),
            lease_expires_at: Some(expires),
        };
        assert!(check_patch_lease_epoch(&record, None, before).is_ok());
        assert!(check_patch_lease_epoch(&record, Some(5), before).is_ok());
        assert_eq!(
            check_patch_lease_epoch(&record, Some(4), before),
            Err(PatchLeaseEpochError::Rejected)
        );
        let legacy = JobRecord {
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            ..record.clone()
        };
        assert_eq!(
            check_patch_lease_epoch(&legacy, Some(1), before),
            Err(PatchLeaseEpochError::NoLeaseOnJob)
        );
    }

    #[test]
    fn job_record_lease_roundtrip_and_active_window() {
        let expires = Utc.with_ymd_and_hms(2026, 5, 27, 13, 0, 0).unwrap();
        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("lease-1"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Submitted,
            created_at: Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap(),
            worker_id: None,
            vm_id: None,
            lease_owner: Some("worker-a".into()),
            lease_epoch: Some(3),
            lease_expires_at: Some(expires),
        };
        let json = serde_json::to_string(&record).expect("serialize");
        let back: JobRecord = serde_json::from_str(&json).expect("deserialize");
        assert!(back.has_lease_fields());
        let before = expires - chrono::Duration::seconds(1);
        let after = expires + chrono::Duration::seconds(1);
        assert!(back.lease_active_at(before));
        assert!(!back.lease_active_at(after));
        assert!(back.lease_epoch_matches(3, before));
        assert!(!back.lease_epoch_matches(2, before));
    }
}
