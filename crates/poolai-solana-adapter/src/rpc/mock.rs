//! In-process mock Solana RPC — records submit intent without network I/O.

use crate::config::{AdapterConfig, SolanaCluster};
use crate::events::DomainEventEnvelope;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RpcSubmitStatus {
    Submitted,
    Duplicate,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MockSubmitResult {
    pub status: RpcSubmitStatus,
    pub cluster: SolanaCluster,
    pub rpc_url: String,
    pub signature: String,
    pub slot: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MockRpcError {
    MockDisabled,
}

impl fmt::Display for MockRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MockDisabled => write!(f, "mock RPC is disabled in adapter config"),
        }
    }
}

impl std::error::Error for MockRpcError {}

/// Deterministic mock RPC client with idempotency on `event_id`.
#[derive(Debug, Default)]
pub struct MockRpcClient {
    by_event_id: HashMap<String, MockSubmitResult>,
    next_slot: u64,
}

impl MockRpcClient {
    pub fn new() -> Self {
        Self {
            by_event_id: HashMap::new(),
            next_slot: 1,
        }
    }

    pub fn submit_event(
        &mut self,
        config: &AdapterConfig,
        envelope: &DomainEventEnvelope,
    ) -> Result<MockSubmitResult, MockRpcError> {
        if !config.mock_rpc {
            return Err(MockRpcError::MockDisabled);
        }

        if let Some(existing) = self.by_event_id.get(&envelope.event_id) {
            return Ok(MockSubmitResult {
                status: RpcSubmitStatus::Duplicate,
                ..existing.clone()
            });
        }

        let slot = self.next_slot;
        self.next_slot = self.next_slot.saturating_add(1);

        let signature = mock_signature(&envelope.event_id, &config.program_id);
        let result = MockSubmitResult {
            status: RpcSubmitStatus::Submitted,
            cluster: config.cluster,
            rpc_url: config.rpc_url.clone(),
            signature,
            slot,
        };
        self.by_event_id
            .insert(envelope.event_id.clone(), result.clone());
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.by_event_id.len()
    }
}

/// Stable pseudo-signature for tests and idempotent replays (not a real ed25519 sig).
pub fn mock_signature(event_id: &str, program_id: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    event_id.hash(&mut hasher);
    program_id.hash(&mut hasher);
    let h = hasher.finish();
    format!("mocksig{:016x}", h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{DomainEvent, JobCompletedEvent};

    #[test]
    fn submit_is_idempotent_on_event_id() {
        let mut config = AdapterConfig::devnet_defaults();
        config.mock_rpc = true;
        let mut rpc = MockRpcClient::new();
        let env = DomainEventEnvelope::new(
            "evt-dup",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );

        let first = rpc.submit_event(&config, &env).unwrap();
        assert_eq!(first.status, RpcSubmitStatus::Submitted);

        let second = rpc.submit_event(&config, &env).unwrap();
        assert_eq!(second.status, RpcSubmitStatus::Duplicate);
        assert_eq!(second.signature, first.signature);
        assert_eq!(rpc.len(), 1);
    }

    #[test]
    fn mock_signature_stable() {
        let a = mock_signature("e1", "prog");
        let b = mock_signature("e1", "prog");
        assert_eq!(a, b);
        assert!(a.starts_with("mocksig"));
    }
}
