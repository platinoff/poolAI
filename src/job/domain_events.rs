//! Domain event schema v1 for Solana sidecar (PH-S38 / FM-010).
//!
//! JSON shape matches `crates/poolai-solana-adapter/src/events.rs` — **no** `solana-sdk` in `poolai`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

pub const EVENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum EventSerializeError {
    UnsupportedSchemaVersion(u32),
    Json(String),
}

impl fmt::Display for EventSerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(v) => {
                write!(f, "domain event: unsupported schema version {v}")
            }
            Self::Json(e) => write!(f, "domain event: json error: {e}"),
        }
    }
}

impl std::error::Error for EventSerializeError {}

/// Wire container (NDJSON line) consumed by `poolai-solana-adapter`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainEventEnvelope {
    pub schema_version: u32,
    pub emitted_at: DateTime<Utc>,
    /// Idempotency key (core-generated); sidecar deduplicates on this field.
    pub event_id: String,
    #[serde(flatten)]
    pub event: DomainEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEvent {
    JobCompleted(JobCompletedEvent),
    SeedProvided(SeedProvidedEvent),
    MemoryUpdated(MemoryUpdatedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobCompletedEvent {
    pub job_id: String,
    pub executor_peer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeedProvidedEvent {
    pub shard_id: String,
    pub provider_peer_id: String,
    pub artifact_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryUpdatedEvent {
    pub artifact_id: String,
    pub version: String,
    pub content_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_logical_name: Option<String>,
}

impl DomainEventEnvelope {
    pub fn new(event_id: impl Into<String>, event: DomainEvent) -> Self {
        Self {
            schema_version: EVENT_SCHEMA_VERSION,
            emitted_at: Utc::now(),
            event_id: event_id.into(),
            event,
        }
    }

    pub fn validate(&self) -> Result<(), EventSerializeError> {
        if self.schema_version != EVENT_SCHEMA_VERSION {
            return Err(EventSerializeError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }
        Ok(())
    }

    pub fn to_json_line(&self) -> Result<String, EventSerializeError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|e| EventSerializeError::Json(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_completed_round_trip() {
        let env = DomainEventEnvelope::new(
            "evt-1",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j-42".into(),
                executor_peer_id: "peer-a".into(),
                payout_lamports: Some(1000),
                verification_digest: Some("sha256:abc".into()),
            }),
        );
        let line = env.to_json_line().unwrap();
        let back: DomainEventEnvelope = serde_json::from_str(&line).unwrap();
        assert_eq!(back.event_id, "evt-1");
        assert!(matches!(back.event, DomainEvent::JobCompleted(_)));
        assert_eq!(back.schema_version, EVENT_SCHEMA_VERSION);
    }

    #[test]
    fn domain_events_ndjson_persist_depth_ph_s872() {
        let events = [
            DomainEventEnvelope::new(
                "evt-job",
                DomainEvent::JobCompleted(JobCompletedEvent {
                    job_id: "j-1".into(),
                    executor_peer_id: "peer-1".into(),
                    payout_lamports: Some(500),
                    verification_digest: None,
                }),
            ),
            DomainEventEnvelope::new(
                "evt-seed",
                DomainEvent::SeedProvided(SeedProvidedEvent {
                    shard_id: "shard-a".into(),
                    provider_peer_id: "peer-2".into(),
                    artifact_id: "art-1".into(),
                }),
            ),
            DomainEventEnvelope::new(
                "evt-mem",
                DomainEvent::MemoryUpdated(MemoryUpdatedEvent {
                    artifact_id: "art-1".into(),
                    version: "1.0.0".into(),
                    content_digest: "sha256:abc".into(),
                    raid_logical_name: Some("weights".into()),
                }),
            ),
        ];
        let mut lines = Vec::new();
        for env in &events {
            lines.push(env.to_json_line().expect("line"));
        }
        assert_eq!(lines.len(), 3);
        for (line, expected) in lines.iter().zip(events.iter()) {
            let back: DomainEventEnvelope = serde_json::from_str(line).expect("parse");
            assert_eq!(back.event_id, expected.event_id);
            assert_eq!(back.schema_version, EVENT_SCHEMA_VERSION);
        }
    }
}
