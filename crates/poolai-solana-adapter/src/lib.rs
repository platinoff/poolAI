//! PoolAI Solana adapter — domain event schema v1 (FM-010 / Horizon S37).
//!
//! Standalone crate: **no** `solana-sdk` dependency. Core (`poolai`) emits JSON events;
//! this sidecar validates and acknowledges them; FM-024 adds devnet config + mock RPC stub.

pub mod config;
pub mod events;
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
pub use rpc::{MockRpcClient, MockRpcError, MockSubmitResult, RpcSubmitStatus};
pub use sidecar::{process_event_line, RpcAck, SidecarAck, SidecarAckStatus, SidecarProcessor};
