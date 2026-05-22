//! Minimal PoolAI on-chain program — anchors domain event metadata (FM-033).
//!
//! Deploy with Solana CLI / `cargo build-sbf` (see `program/README.md`).

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

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

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = PoolAiInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
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
}
