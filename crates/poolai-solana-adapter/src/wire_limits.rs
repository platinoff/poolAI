//! Wire validation for domain events and instructions (PH-S46).

mod limits {
    include!("../wire/limits.rs");
}
pub use limits::*;

use crate::events::{DomainEvent, DomainEventEnvelope};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireValidationError {
    EmptyEventId,
    FieldTooLong {
        field: &'static str,
        max: usize,
        len: usize,
    },
    InstructionDataTooLarge {
        len: usize,
        max: usize,
    },
}

impl fmt::Display for WireValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEventId => write!(f, "event_id must not be empty"),
            Self::FieldTooLong { field, max, len } => {
                write!(f, "{field} length {len} exceeds max {max}")
            }
            Self::InstructionDataTooLarge { len, max } => {
                write!(f, "instruction data length {len} exceeds max {max}")
            }
        }
    }
}

impl std::error::Error for WireValidationError {}

pub fn check_field(
    field: &'static str,
    value: &str,
    max: usize,
) -> Result<(), WireValidationError> {
    let len = value.len();
    if len > max {
        return Err(WireValidationError::FieldTooLong { field, max, len });
    }
    Ok(())
}

pub fn validate_envelope(env: &DomainEventEnvelope) -> Result<(), WireValidationError> {
    if env.event_id.trim().is_empty() {
        return Err(WireValidationError::EmptyEventId);
    }
    check_field("event_id", &env.event_id, MAX_EVENT_ID_LEN)?;
    match &env.event {
        DomainEvent::JobCompleted(e) => {
            check_field("job_id", &e.job_id, MAX_DOMAIN_ID_LEN)?;
            check_field("executor_peer_id", &e.executor_peer_id, MAX_PEER_ID_LEN)?;
            if let Some(d) = &e.verification_digest {
                check_field("verification_digest", d, MAX_DIGEST_LEN)?;
            }
        }
        DomainEvent::SeedProvided(e) => {
            check_field("shard_id", &e.shard_id, MAX_DOMAIN_ID_LEN)?;
            check_field("provider_peer_id", &e.provider_peer_id, MAX_PEER_ID_LEN)?;
            check_field("artifact_id", &e.artifact_id, MAX_DOMAIN_ID_LEN)?;
        }
        DomainEvent::MemoryUpdated(e) => {
            check_field("artifact_id", &e.artifact_id, MAX_DOMAIN_ID_LEN)?;
            check_field("version", &e.version, MAX_DOMAIN_ID_LEN)?;
            check_field("content_digest", &e.content_digest, MAX_DIGEST_LEN)?;
            if let Some(name) = &e.raid_logical_name {
                check_field("raid_logical_name", name, MAX_RAID_NAME_LEN)?;
            }
        }
    }
    Ok(())
}

pub fn validate_instruction_data(data: &[u8]) -> Result<(), WireValidationError> {
    if data.len() > MAX_INSTRUCTION_DATA {
        return Err(WireValidationError::InstructionDataTooLarge {
            len: data.len(),
            max: MAX_INSTRUCTION_DATA,
        });
    }
    Ok(())
}

pub fn memo_anchor_len(event_id: &str, type_tag: &str) -> usize {
    format!("poolai:v1:{event_id}:{type_tag}").len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{JobCompletedEvent, MemoryUpdatedEvent};

    #[test]
    fn rejects_empty_event_id() {
        let env = DomainEventEnvelope::new(
            "   ",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        assert_eq!(
            validate_envelope(&env).unwrap_err(),
            WireValidationError::EmptyEventId
        );
    }

    #[test]
    fn rejects_oversized_job_id() {
        let env = DomainEventEnvelope::new(
            "evt",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "x".repeat(MAX_DOMAIN_ID_LEN + 1),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        assert!(matches!(
            validate_envelope(&env).unwrap_err(),
            WireValidationError::FieldTooLong {
                field: "job_id",
                ..
            }
        ));
    }

    #[test]
    fn accepts_typical_envelope() {
        let env = DomainEventEnvelope::new(
            "evt-ok",
            DomainEvent::MemoryUpdated(MemoryUpdatedEvent {
                artifact_id: "art-1".into(),
                version: "1.0".into(),
                content_digest: "sha256:abc".into(),
                raid_logical_name: Some("weights".into()),
            }),
        );
        validate_envelope(&env).unwrap();
    }

    #[test]
    fn memo_anchor_within_limit() {
        let id = "e".repeat(120);
        assert!(memo_anchor_len(&id, "job_completed") <= MAX_MEMO_ANCHOR_LEN);
    }
}
