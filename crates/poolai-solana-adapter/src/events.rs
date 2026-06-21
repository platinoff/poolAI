//! Domain event schema v1 — contract between PoolAI core and the Solana sidecar.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

pub const EVENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, PartialEq, Eq)]
pub enum EventParseError {
    UnsupportedSchemaVersion(u32),
    Json(String),
}

impl fmt::Display for EventParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(v) => {
                write!(f, "solana adapter: unsupported schema version {v}")
            }
            Self::Json(e) => write!(f, "solana adapter: json error: {e}"),
        }
    }
}

impl std::error::Error for EventParseError {}

/// Wire container for a single domain event (NDJSON line or HTTP body).
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

/// Job reached `verified` / `rewarded` — adapter may anchor payout intent on-chain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobCompletedEvent {
    pub job_id: String,
    pub executor_peer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_digest: Option<String>,
}

/// Peer provided seed / bandwidth for a memory shard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeedProvidedEvent {
    pub shard_id: String,
    pub provider_peer_id: String,
    pub artifact_id: String,
}

/// New memory / artifact version anchored (hash only on-chain).
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

    pub fn validate(&self) -> Result<(), EventParseError> {
        if self.schema_version != EVENT_SCHEMA_VERSION {
            return Err(EventParseError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }
        crate::wire_limits::validate_envelope(self)
            .map_err(|e| EventParseError::Json(e.to_string()))
    }

    pub fn to_json(&self) -> Result<String, EventParseError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|e| EventParseError::Json(e.to_string()))
    }

    pub fn from_json(s: &str) -> Result<Self, EventParseError> {
        let env: Self =
            serde_json::from_str(s).map_err(|e| EventParseError::Json(e.to_string()))?;
        env.validate()?;
        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_v1_core_wire_fixture_ph_s871() {
        let fixture = r#"{"schema_version":1,"emitted_at":"2026-06-21T12:00:00Z","event_id":"evt-s871","type":"job_completed","job_id":"j-s871","executor_peer_id":"peer-s871","payout_lamports":4200,"verification_digest":"sha256:fixture"}"#;
        let env = DomainEventEnvelope::from_json(fixture).expect("adapter parse");
        assert_eq!(env.schema_version, EVENT_SCHEMA_VERSION);
        assert_eq!(env.event_id, "evt-s871");
        assert!(matches!(env.event, DomainEvent::JobCompleted(_)));
        let round = env.to_json().expect("serialize");
        let back = DomainEventEnvelope::from_json(&round).expect("round trip");
        assert_eq!(back.event_id, "evt-s871");
    }

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
        let back = DomainEventEnvelope::from_json(&env.to_json().unwrap()).unwrap();
        assert_eq!(back.event_id, "evt-1");
        assert!(matches!(back.event, DomainEvent::JobCompleted(_)));
    }

    #[test]
    fn rejects_oversized_event_id_on_validate() {
        let env = DomainEventEnvelope::new(
            "x".repeat(crate::wire_limits::MAX_EVENT_ID_LEN + 1),
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        assert!(env.validate().is_err());
    }

    #[test]
    fn seed_and_memory_events_in_json() {
        let seed = DomainEventEnvelope::new(
            "evt-2",
            DomainEvent::SeedProvided(SeedProvidedEvent {
                shard_id: "weights:1.0".into(),
                provider_peer_id: "peer-b".into(),
                artifact_id: "art-1".into(),
            }),
        );
        assert_eq!(
            DomainEventEnvelope::from_json(&seed.to_json().unwrap())
                .unwrap()
                .event,
            seed.event
        );

        let mem = DomainEventEnvelope::new(
            "evt-3",
            DomainEvent::MemoryUpdated(MemoryUpdatedEvent {
                artifact_id: "art-1".into(),
                version: "1.0.1".into(),
                content_digest: "sha256:def".into(),
                raid_logical_name: Some("weights".into()),
            }),
        );
        assert!(matches!(
            DomainEventEnvelope::from_json(&mem.to_json().unwrap())
                .unwrap()
                .event,
            DomainEvent::MemoryUpdated(_)
        ));
    }
}
