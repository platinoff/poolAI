//! Borsh instruction encoding — must match `program/poolai-events/src/lib.rs`.

use crate::events::{DomainEvent, DomainEventEnvelope};
use crate::wire_limits::{self, validate_instruction_data, WireValidationError};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_instruction::{account_meta::AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::fmt;
use std::str::FromStr;

/// Solana Memo program (v2) — fallback anchor when custom program is not deployed.
pub const MEMO_PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

/// Bundled placeholder until `POOLAI_SOLANA_PROGRAM_ID` / deploy.
pub const PLACEHOLDER_PROGRAM_ID: &str = "11111111111111111111111111111111";

/// `true` when sidecar should use Memo fallback (no custom program deployed).
pub fn is_placeholder_program_id(program_id: &str) -> bool {
    program_id.trim() == PLACEHOLDER_PROGRAM_ID
}

/// Anchor mode for RPC ack metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorMode {
    Memo,
    Program,
}

impl AnchorMode {
    pub fn as_wire_str(self) -> &'static str {
        match self {
            Self::Memo => "memo",
            Self::Program => "program",
        }
    }

    pub fn for_program_id(program_id: &str) -> Self {
        if is_placeholder_program_id(program_id) {
            Self::Memo
        } else {
            Self::Program
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PoolAiInstruction {
    AnchorJobCompleted {
        event_id: String,
        job_id: String,
    },
    AnchorSeedProvided {
        event_id: String,
        shard_id: String,
    },
    AnchorMemoryUpdated {
        event_id: String,
        artifact_id: String,
    },
}

impl PoolAiInstruction {
    pub fn from_envelope(envelope: &DomainEventEnvelope) -> Self {
        match &envelope.event {
            DomainEvent::JobCompleted(e) => Self::AnchorJobCompleted {
                event_id: envelope.event_id.clone(),
                job_id: e.job_id.clone(),
            },
            DomainEvent::SeedProvided(e) => Self::AnchorSeedProvided {
                event_id: envelope.event_id.clone(),
                shard_id: e.shard_id.clone(),
            },
            DomainEvent::MemoryUpdated(e) => Self::AnchorMemoryUpdated {
                event_id: envelope.event_id.clone(),
                artifact_id: e.artifact_id.clone(),
            },
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, InstructionBuildError> {
        borsh::to_vec(self).map_err(|e| InstructionBuildError::Borsh(e.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionBuildError {
    Borsh(String),
    InvalidProgramId(String),
    InvalidPayer(String),
    WireValidation(WireValidationError),
    MemoTooLong { len: usize, max: usize },
}

impl std::fmt::Display for InstructionBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Borsh(e) => write!(f, "instruction borsh error: {e}"),
            Self::InvalidProgramId(e) => write!(f, "invalid program_id: {e}"),
            Self::InvalidPayer(e) => write!(f, "invalid payer pubkey: {e}"),
            Self::WireValidation(e) => write!(f, "wire validation: {e}"),
            Self::MemoTooLong { len, max } => {
                write!(f, "memo anchor length {len} exceeds max {max}")
            }
        }
    }
}

impl std::error::Error for InstructionBuildError {}

/// Build a single-instruction transaction payload for the configured program.
pub fn build_submit_instruction(
    program_id_str: &str,
    payer: &Pubkey,
    envelope: &DomainEventEnvelope,
) -> Result<Instruction, InstructionBuildError> {
    wire_limits::validate_envelope(envelope).map_err(InstructionBuildError::WireValidation)?;

    if is_placeholder_program_id(program_id_str) {
        return build_memo_instruction(payer, envelope);
    }

    let program_id = Pubkey::from_str(program_id_str.trim())
        .map_err(|e| InstructionBuildError::InvalidProgramId(e.to_string()))?;
    let pool_ix = PoolAiInstruction::from_envelope(envelope);
    let data = pool_ix.encode()?;
    validate_instruction_data(&data).map_err(InstructionBuildError::WireValidation)?;
    Ok(Instruction::new_with_bytes(
        program_id,
        &data,
        vec![AccountMeta::new_readonly(*payer, true)],
    ))
}

fn build_memo_instruction(
    payer: &Pubkey,
    envelope: &DomainEventEnvelope,
) -> Result<Instruction, InstructionBuildError> {
    let memo_program = Pubkey::from_str(MEMO_PROGRAM_ID)
        .map_err(|e| InstructionBuildError::InvalidProgramId(e.to_string()))?;
    let memo = format!(
        "poolai:v1:{}:{}",
        envelope.event_id,
        event_type_tag(&envelope.event)
    );
    let len = memo.len();
    if len > wire_limits::MAX_MEMO_ANCHOR_LEN {
        return Err(InstructionBuildError::MemoTooLong {
            len,
            max: wire_limits::MAX_MEMO_ANCHOR_LEN,
        });
    }
    Ok(Instruction::new_with_bytes(
        memo_program,
        memo.as_bytes(),
        vec![AccountMeta::new_readonly(*payer, true)],
    ))
}

fn event_type_tag(event: &DomainEvent) -> &'static str {
    match event {
        DomainEvent::JobCompleted(_) => "job_completed",
        DomainEvent::SeedProvided(_) => "seed_provided",
        DomainEvent::MemoryUpdated(_) => "memory_updated",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{JobCompletedEvent, SeedProvidedEvent};

    #[test]
    fn poolai_instruction_matches_program_crate_layout() {
        let ix = PoolAiInstruction::AnchorJobCompleted {
            event_id: "e".into(),
            job_id: "j".into(),
        };
        let bytes = ix.encode().unwrap();
        let back = PoolAiInstruction::try_from_slice(&bytes).unwrap();
        assert_eq!(back, ix);
    }

    #[test]
    fn memo_fallback_when_placeholder_program() {
        let payer = Pubkey::new_unique();
        let env = DomainEventEnvelope::new(
            "evt-memo",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "j1".into(),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        let ix = build_submit_instruction(PLACEHOLDER_PROGRAM_ID, &payer, &env).unwrap();
        assert_eq!(ix.program_id.to_string(), MEMO_PROGRAM_ID);
        let data = String::from_utf8_lossy(&ix.data);
        assert!(data.contains("evt-memo"));
    }

    #[test]
    fn rejects_oversized_envelope_before_submit() {
        let payer = Pubkey::new_unique();
        let env = DomainEventEnvelope::new(
            "evt",
            DomainEvent::JobCompleted(JobCompletedEvent {
                job_id: "x".repeat(wire_limits::MAX_DOMAIN_ID_LEN + 1),
                executor_peer_id: "p".into(),
                payout_lamports: None,
                verification_digest: None,
            }),
        );
        let err = build_submit_instruction(PLACEHOLDER_PROGRAM_ID, &payer, &env).unwrap_err();
        assert!(matches!(err, InstructionBuildError::WireValidation(_)));
    }

    #[test]
    fn anchor_mode_for_program_id() {
        assert_eq!(
            AnchorMode::for_program_id(PLACEHOLDER_PROGRAM_ID),
            AnchorMode::Memo
        );
        assert_eq!(
            AnchorMode::for_program_id(&Pubkey::new_unique().to_string()),
            AnchorMode::Program
        );
    }

    #[test]
    fn custom_program_instruction_when_program_id_set() {
        let payer = Pubkey::new_unique();
        let program = Pubkey::new_unique();
        let env = DomainEventEnvelope::new(
            "evt-p",
            DomainEvent::SeedProvided(SeedProvidedEvent {
                shard_id: "s".into(),
                provider_peer_id: "p".into(),
                artifact_id: "a".into(),
            }),
        );
        let ix = build_submit_instruction(&program.to_string(), &payer, &env).unwrap();
        assert_eq!(ix.program_id, program);
        assert!(!ix.data.is_empty());
    }
}
