//! PoolAI Solana adapter — domain event schema v1 (FM-010 / Horizon S37).
//!
//! Standalone workspace crate: **`solana-sdk` only here**, not in main `poolai`.
//! FM-024 mock RPC stub; FM-033 on-chain program prototype + devnet JSON-RPC submit.

pub mod config;
pub mod events;
pub mod instruction;
pub mod rpc;
pub mod sidecar;

pub use config::{
    AdapterConfig, CommitmentLevel, ConfigError, SolanaCluster, BUNDLED_DEVNET_TOML,
    DEFAULT_DEVNET_RPC_URL, ENV_CLUSTER, ENV_CONFIG_PATH, ENV_MOCK_RPC, ENV_RPC_URL,
};
pub use events::{
    DomainEvent, DomainEventEnvelope, EventParseError, JobCompletedEvent, MemoryUpdatedEvent,
    SeedProvidedEvent, EVENT_SCHEMA_VERSION,
};
pub use instruction::{PoolAiInstruction, MEMO_PROGRAM_ID, PLACEHOLDER_PROGRAM_ID};
pub use rpc::{
    DevnetRpcClient, DevnetRpcError, HttpRpcTransport, MockRpcClient, MockRpcError,
    MockSubmitResult, RpcSubmitStatus, RpcTransport, ENV_KEYPAIR_PATH, ENV_PROGRAM_ID,
};
pub use sidecar::{process_event_line, RpcAck, SidecarAck, SidecarAckStatus, SidecarProcessor};
