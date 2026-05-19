//! Sidecar stub: validate NDJSON domain events (no RPC / no solana-sdk).

use crate::events::DomainEventEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SidecarAckStatus {
    Acked,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SidecarAck {
    pub status: SidecarAckStatus,
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parse one NDJSON line and produce an acknowledgment (MVP: always ack on valid schema).
pub fn process_event_line(line: &str) -> SidecarAck {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return SidecarAck {
            status: SidecarAckStatus::Rejected,
            event_id: String::new(),
            error: Some("empty line".into()),
        };
    }
    match DomainEventEnvelope::from_json(trimmed) {
        Ok(env) => SidecarAck {
            status: SidecarAckStatus::Acked,
            event_id: env.event_id,
            error: None,
        },
        Err(e) => SidecarAck {
            status: SidecarAckStatus::Rejected,
            event_id: extract_event_id_fallback(trimmed).unwrap_or_default(),
            error: Some(e.to_string()),
        },
    }
}

fn extract_event_id_fallback(json: &str) -> Option<String> {
    #[derive(Deserialize)]
    struct Partial {
        event_id: Option<String>,
    }
    serde_json::from_str::<Partial>(json)
        .ok()
        .and_then(|p| p.event_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{DomainEvent, JobCompletedEvent};

    #[test]
    fn acks_valid_job_completed_line() {
        let line = DomainEventEnvelope::new(
            "id-99",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        )
        .to_json()
        .unwrap();
        let ack = process_event_line(&line);
        assert_eq!(ack.status, SidecarAckStatus::Acked);
        assert_eq!(ack.event_id, "id-99");
    }

    #[test]
    fn rejects_invalid_json() {
        let ack = process_event_line("{not json");
        assert_eq!(ack.status, SidecarAckStatus::Rejected);
        assert!(ack.error.is_some());
    }
}
