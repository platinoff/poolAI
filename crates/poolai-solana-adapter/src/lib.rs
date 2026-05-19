//! PoolAI Solana adapter — domain event schema v1 (FM-010 / Horizon S37).
//!
//! Standalone crate: **no** `solana-sdk` dependency. Core (`poolai`) emits JSON events;
//! this sidecar validates and acknowledges them before a future on-chain program is wired.

pub mod events;
pub mod sidecar;

pub use events::{
    DomainEvent, DomainEventEnvelope, EventParseError, JobCompletedEvent, MemoryUpdatedEvent,
    SeedProvidedEvent, EVENT_SCHEMA_VERSION,
};
pub use sidecar::{process_event_line, SidecarAck, SidecarAckStatus};
