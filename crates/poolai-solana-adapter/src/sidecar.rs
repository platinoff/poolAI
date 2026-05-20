//! Sidecar: validate NDJSON domain events and optional mock RPC submit (FM-024).

use crate::config::AdapterConfig;
use crate::events::DomainEventEnvelope;
use crate::rpc::{MockRpcClient, MockSubmitResult, RpcSubmitStatus};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc: Option<RpcAck>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RpcAck {
    pub status: RpcSubmitStatus,
    pub signature: String,
    pub slot: u64,
    pub cluster: String,
    pub rpc_url: String,
}

impl From<MockSubmitResult> for RpcAck {
    fn from(r: MockSubmitResult) -> Self {
        Self {
            status: r.status,
            signature: r.signature,
            slot: r.slot,
            cluster: r.cluster.as_wire_str().to_string(),
            rpc_url: r.rpc_url,
        }
    }
}

/// Stateful processor: schema validation + mock RPC when enabled in config.
#[derive(Debug)]
pub struct SidecarProcessor {
    config: AdapterConfig,
    rpc: MockRpcClient,
}

impl SidecarProcessor {
    pub fn new(config: AdapterConfig) -> Self {
        Self {
            config,
            rpc: MockRpcClient::new(),
        }
    }

    pub fn with_devnet_defaults() -> Self {
        Self::new(AdapterConfig::devnet_defaults())
    }

    pub fn config(&self) -> &AdapterConfig {
        &self.config
    }

    pub fn process_line(&mut self, line: &str) -> SidecarAck {
        let mut ack = process_event_line(line);
        if ack.status != SidecarAckStatus::Acked {
            return ack;
        }

        if !self.config.mock_rpc {
            return ack;
        }

        let trimmed = line.trim();
        let Ok(env) = DomainEventEnvelope::from_json(trimmed) else {
            return ack;
        };

        match self.rpc.submit_event(&self.config, &env) {
            Ok(result) => {
                ack.rpc = Some(result.into());
            }
            Err(_) => {
                // mock disabled despite config — leave ack without rpc block
            }
        }
        ack
    }
}

/// Parse one NDJSON line and produce an acknowledgment (schema only; no RPC state).
pub fn process_event_line(line: &str) -> SidecarAck {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return SidecarAck {
            status: SidecarAckStatus::Rejected,
            event_id: String::new(),
            error: Some("empty line".into()),
            rpc: None,
        };
    }
    match DomainEventEnvelope::from_json(trimmed) {
        Ok(env) => SidecarAck {
            status: SidecarAckStatus::Acked,
            event_id: env.event_id,
            error: None,
            rpc: None,
        },
        Err(e) => SidecarAck {
            status: SidecarAckStatus::Rejected,
            event_id: extract_event_id_fallback(trimmed).unwrap_or_default(),
            error: Some(e.to_string()),
            rpc: None,
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

    fn job_completed_line(event_id: &str) -> String {
        DomainEventEnvelope::new(
            event_id,
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        )
        .to_json()
        .unwrap()
    }

    #[test]
    fn acks_valid_job_completed_line() {
        let line = job_completed_line("id-99");
        let ack = process_event_line(&line);
        assert_eq!(ack.status, SidecarAckStatus::Acked);
        assert_eq!(ack.event_id, "id-99");
        assert!(ack.rpc.is_none());
    }

    #[test]
    fn rejects_invalid_json() {
        let ack = process_event_line("{not json");
        assert_eq!(ack.status, SidecarAckStatus::Rejected);
        assert!(ack.error.is_some());
    }

    #[test]
    fn processor_attaches_mock_rpc_on_devnet() {
        let line = job_completed_line("rpc-1");
        let mut proc = SidecarProcessor::with_devnet_defaults();
        let ack = proc.process_line(&line);
        assert_eq!(ack.status, SidecarAckStatus::Acked);
        let rpc = ack.rpc.expect("mock rpc block");
        assert_eq!(rpc.status, RpcSubmitStatus::Submitted);
        assert!(rpc.signature.starts_with("mocksig"));
    }

    #[test]
    fn processor_rpc_duplicate_on_replay() {
        let line = job_completed_line("rpc-dup");
        let mut proc = SidecarProcessor::with_devnet_defaults();
        let first = proc.process_line(&line);
        let second = proc.process_line(&line);
        assert_eq!(
            first.rpc.as_ref().map(|r| r.signature.as_str()),
            second.rpc.as_ref().map(|r| r.signature.as_str())
        );
        assert_eq!(
            second.rpc.map(|r| r.status),
            Some(RpcSubmitStatus::Duplicate)
        );
    }
}
