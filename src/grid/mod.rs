//! Grid protocol wire types (FM-009 / Horizon S36).
//!
//! JSON **GridEnvelope** v1 unifies logical messages (Job, Result, MemoryShard, PeerStatus)
//! for transport over HTTP/WebSocket or future QUIC. See
//! `docs/development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`.

pub mod dispatch;
mod envelope;
pub mod galaxy_fee_split;
mod map;

pub use dispatch::{ingest_envelope, GridIngestKind, GridIngestOutcome};
pub use envelope::{
    GridEnvelope, GridEnvelopeError, GridJobBody, GridMemoryShardBody, GridMessage,
    GridPeerStatusBody, GridResultBody, GridResultStatus, GRID_ENVELOPE_VERSION,
};
pub use map::{
    envelope_from_peer_info, envelope_from_put_artifact, memory_shard_from_put_artifact,
    peer_info_from_envelope, put_artifact_from_memory_shard,
};
