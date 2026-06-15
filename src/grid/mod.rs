//! Grid protocol wire types (FM-009 / Horizon S36).
//!
//! JSON **GridEnvelope** v1 unifies logical messages (Job, Result, MemoryShard, PeerStatus)
//! for transport over HTTP/WebSocket or future QUIC. See
//! `docs/development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`.

pub mod dispatch;
mod envelope;
pub mod galaxy_fee_split;
pub mod galaxy_fee_split_metrics;
pub mod galaxy_locality;
pub mod galaxy_network_profile;
pub mod galaxy_prefetch_metrics;
pub mod galaxy_pricing_oracle;
pub mod galaxy_pricing_provider_metrics;
pub mod galaxy_replay_metrics;
pub mod galaxy_replication;
pub mod galaxy_replication_metrics;
pub mod galaxy_settlement;
pub mod galaxy_settlement_metrics;
pub mod galaxy_trust_score;
pub mod galaxy_verification_metrics;
pub mod galaxy_verify_sampling;
mod map;
pub mod protocol_compat;

pub use dispatch::{
    coordinator_seed_inventory_snapshot, ingest_envelope, noop_prefetch_hook, parse_prefetch_policy_mode,
    plan_prefetch, GridIngestKind, GridIngestOutcome, PrefetchPlan, PrefetchPolicyConfig,
    PrefetchPolicyMode, PrefetchTrigger, SeedInventoryEntry, SeedInventoryPeerSnapshot,
    DEFAULT_PREFETCH_DEADLINE_MS, ENV_LOCALITY_MODE, ENV_PREFETCH_DEADLINE_MS,
};
pub use envelope::{
    GridEnvelope, GridEnvelopeError, GridJobBody, GridMemoryShardBody, GridMessage,
    GridPeerStatusBody, GridResultBody, GridResultStatus, GRID_ENVELOPE_VERSION,
};
pub use galaxy_network_profile::{
    normalize_register_metadata, parse_network_profile_value, GalaxyEgressPolicy,
    GalaxyNetworkProfile, NetworkProfileParseError,
};
pub use galaxy_verify_sampling::{
    parse_verify_base_sample_rate, VerifySamplingConfig, DEFAULT_VERIFY_BASE_SAMPLE_RATE,
    ENV_VERIFY_BASE_SAMPLE_RATE,
};
pub use map::{
    envelope_from_peer_info, envelope_from_put_artifact, memory_shard_from_put_artifact,
    peer_info_from_envelope, put_artifact_from_memory_shard,
};
pub use protocol_compat::{
    negotiate, negotiate_with_coordinator, CompatStatus, ProtocolNegotiation, ProtocolVersion,
    DEFAULT_COORDINATOR_PROTOCOL, MIN_COORDINATOR_VERSION_DOCS_URL,
};
