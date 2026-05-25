//! Minimal PoolAI on-chain program — anchors domain event metadata (FM-033 / PH-S46).
//!
//! Deploy with Solana CLI / `cargo build-sbf` (see `program/README.md`).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

mod wire_limits {
    include!("../../../wire/limits.rs");
}
use wire_limits::*;

entrypoint!(process_instruction);

/// Wire format shared with `poolai-solana-adapter` (`instruction.rs`).
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PoolAiInstruction {
    /// Anchor a completed job (`JobCompleted` domain event).
    AnchorJobCompleted { event_id: String, job_id: String },
    /// Anchor seed provision (`SeedProvided`).
    AnchorSeedProvided { event_id: String, shard_id: String },
    /// Anchor memory update (`MemoryUpdated`).
    AnchorMemoryUpdated {
        event_id: String,
        artifact_id: String,
    },
}

fn check_len(field: &'static str, value: &str, max: usize) -> Result<(), ProgramError> {
    if value.trim().is_empty() {
        msg!("poolai: empty {field}");
        return Err(ProgramError::InvalidInstructionData);
    }
    if value.len() > max {
        msg!("poolai: {field} too long ({} > {max})", value.len());
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

fn validate_instruction(ix: &PoolAiInstruction) -> Result<(), ProgramError> {
    match ix {
        PoolAiInstruction::AnchorJobCompleted { event_id, job_id } => {
            check_len("event_id", event_id, MAX_EVENT_ID_LEN)?;
            check_len("job_id", job_id, MAX_DOMAIN_ID_LEN)?;
        }
        PoolAiInstruction::AnchorSeedProvided { event_id, shard_id } => {
            check_len("event_id", event_id, MAX_EVENT_ID_LEN)?;
            check_len("shard_id", shard_id, MAX_DOMAIN_ID_LEN)?;
        }
        PoolAiInstruction::AnchorMemoryUpdated {
            event_id,
            artifact_id,
        } => {
            check_len("event_id", event_id, MAX_EVENT_ID_LEN)?;
            check_len("artifact_id", artifact_id, MAX_DOMAIN_ID_LEN)?;
        }
    }
    Ok(())
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() > MAX_INSTRUCTION_DATA {
        msg!(
            "poolai: instruction data too large ({} > {MAX_INSTRUCTION_DATA})",
            instruction_data.len()
        );
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix = PoolAiInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    validate_instruction(&ix)?;
    match ix {
        PoolAiInstruction::AnchorJobCompleted { event_id, job_id } => {
            msg!(
                "poolai:job_completed event_id={} job_id={}",
                event_id,
                job_id
            );
        }
        PoolAiInstruction::AnchorSeedProvided { event_id, shard_id } => {
            msg!(
                "poolai:seed_provided event_id={} shard_id={}",
                event_id,
                shard_id
            );
        }
        PoolAiInstruction::AnchorMemoryUpdated {
            event_id,
            artifact_id,
        } => {
            msg!(
                "poolai:memory_updated event_id={} artifact_id={}",
                event_id,
                artifact_id
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_pubkey::Pubkey;

    #[test]
    fn round_trip_instruction_bytes() {
        let ix = PoolAiInstruction::AnchorJobCompleted {
            event_id: "evt-1".into(),
            job_id: "job-42".into(),
        };
        let data = borsh::to_vec(&ix).unwrap();
        let parsed = PoolAiInstruction::try_from_slice(&data).unwrap();
        assert_eq!(parsed, ix);
        let program_id = Pubkey::new_unique();
        assert!(process_instruction(&program_id, &[], &data).is_ok());
    }

    #[test]
    fn rejects_empty_event_id() {
        let ix = PoolAiInstruction::AnchorJobCompleted {
            event_id: "  ".into(),
            job_id: "j".into(),
        };
        let data = borsh::to_vec(&ix).unwrap();
        let program_id = Pubkey::new_unique();
        assert_eq!(
            process_instruction(&program_id, &[], &data),
            Err(ProgramError::InvalidInstructionData)
        );
    }

    #[test]
    fn rejects_oversized_job_id() {
        let ix = PoolAiInstruction::AnchorJobCompleted {
            event_id: "e".into(),
            job_id: "x".repeat(MAX_DOMAIN_ID_LEN + 1),
        };
        let data = borsh::to_vec(&ix).unwrap();
        let program_id = Pubkey::new_unique();
        assert_eq!(
            process_instruction(&program_id, &[], &data),
            Err(ProgramError::InvalidInstructionData)
        );
    }

    #[test]
    fn rejects_instruction_data_over_cap() {
        let program_id = Pubkey::new_unique();
        let oversized = vec![0u8; MAX_INSTRUCTION_DATA + 1];
        assert_eq!(
            process_instruction(&program_id, &[], &oversized),
            Err(ProgramError::InvalidInstructionData)
        );
    }
}
