// Shared on-chain wire limits (PH-S46).
// Included by `poolai-solana-adapter` and `poolai-events` — keep both in sync.

/// Idempotency key (`event_id`).
pub const MAX_EVENT_ID_LEN: usize = 128;
/// `job_id`, `shard_id`, `artifact_id`, `version`.
pub const MAX_DOMAIN_ID_LEN: usize = 256;
/// `executor_peer_id`, `provider_peer_id`.
pub const MAX_PEER_ID_LEN: usize = 128;
/// `verification_digest`, `content_digest`.
pub const MAX_DIGEST_LEN: usize = 128;
/// Optional `raid_logical_name`.
pub const MAX_RAID_NAME_LEN: usize = 64;
/// Memo fallback anchor payload (Solana Memo v2 practical limit).
pub const MAX_MEMO_ANCHOR_LEN: usize = 566;
/// Borsh instruction data cap (below Solana packet limits).
pub const MAX_INSTRUCTION_DATA: usize = 1024;
