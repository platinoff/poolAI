//! Grid protocol wire types (FM-009 / Horizon S36).
//!
//! JSON **GridEnvelope** v1 unifies logical messages (Job, Result, MemoryShard, PeerStatus)
//! for transport over HTTP/WebSocket or future QUIC. See
//! `docs/development/GRID_PROTOCOL_CONCEPT_2026-04-06.md`.

pub mod dispatch;
mod envelope;
pub mod galaxy_capability_admission;
pub mod galaxy_capability_admission_depth;
pub mod galaxy_capability_admission_metrics;
pub mod galaxy_capability_doc;
pub mod galaxy_fee_split;
pub mod galaxy_fee_split_metrics;
pub mod galaxy_fraud_proof;
pub mod galaxy_governance_metrics;
pub mod galaxy_locality;
pub mod galaxy_locality_hot_tier_depth;
pub mod galaxy_locality_metrics;
pub mod galaxy_network_profile;
pub mod galaxy_network_profile_depth;
pub mod galaxy_network_profile_store;
pub mod galaxy_prefetch_depth;
pub mod galaxy_prefetch_metrics;
pub mod galaxy_prefetch_peer_pull;
pub mod galaxy_pricing_metrics;
pub mod galaxy_pricing_oracle;
pub mod galaxy_pricing_provider_metrics;
pub mod galaxy_protocol_negotiation_metrics;
pub mod galaxy_re_migrate_policy;
pub mod galaxy_replay_jobs;
pub mod galaxy_replay_metrics;
pub mod galaxy_replication;
pub mod galaxy_replication_metrics;
pub mod galaxy_replication_quorum_gate;
pub mod galaxy_routing_policy;
pub mod galaxy_security_advisory;
pub mod galaxy_settlement;
pub mod galaxy_settlement_metrics;
pub mod galaxy_settlement_mode;
pub mod galaxy_settlement_onchain;
pub mod galaxy_trust_score;
pub mod galaxy_trust_score_store;
pub mod galaxy_update_policy;
pub mod galaxy_verification_checker_jobs;
pub mod galaxy_verification_metrics;
pub mod galaxy_verification_replay;
pub mod galaxy_verify_sampling;
pub mod galaxy_worker_dto;
pub mod galaxy_worker_health;
mod map;
pub mod protocol_compat;
pub mod stand_smoke_metrics_parity;

pub use dispatch::{
    all_required_shards_hot, check_strict_locality_gate, co_access_graph_from_env,
    complete_prefetch_hook, coordinator_merged_seed_inventory, coordinator_seed_inventory_snapshot,
    default_co_access_graph, enqueue_prefetch_hook, evaluate_strict_prefetch_timeout,
    fetch_seed_shards_from_raid_hook, fetch_seed_shards_hook, ingest_envelope,
    ingest_job_locality_rank_stub, ingest_job_prefetch_stub, lease_acquire_prefetch_stub,
    locality_workers_from_seed_snapshots, noop_prefetch_hook, parse_prefetch_policy_mode,
    plan_co_access_prefetch, plan_prefetch, prefetch_backpressure_skip,
    prefetch_topology_admission_blocked_skip, re_migrate_prefetch_stub, resolve_seed_pull_shards,
    seed_pull_hook, wait_prefetch_hook, with_prefetch_peer, GridIngestKind, GridIngestOutcome,
    PrefetchPlan, PrefetchPolicyConfig, PrefetchPolicyMode, PrefetchTrigger, SeedInventoryEntry,
    SeedInventoryPeerSnapshot, DEFAULT_PREFETCH_DEADLINE_MS, ENV_CO_ACCESS_GRAPH_JSON,
    ENV_LOCALITY_MODE, ENV_PREFETCH_COORDINATOR_REGION, ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING,
    ENV_PREFETCH_DEADLINE_MS, ENV_PREFETCH_MIN_BANDWIDTH_MBPS, ENV_PREFETCH_PEER_BANDWIDTH_MBPS,
};
pub use envelope::{
    GridEnvelope, GridEnvelopeError, GridJobBody, GridMemoryShardBody, GridMessage,
    GridPeerStatusBody, GridResultBody, GridResultStatus, GRID_ENVELOPE_VERSION,
};
pub use galaxy_capability_doc::{
    capability_signing_message, parse_capability_document, validate_capability_document,
    verify_capability_signature_stub, CapabilityDocParseError, GalaxyCapabilityDocument,
    DEV_CAPABILITY_VERIFY_PK_HEX,
};
pub use galaxy_network_profile::{
    normalize_register_metadata, parse_network_profile_value, GalaxyEgressPolicy,
    GalaxyNetworkProfile, NetworkProfileParseError,
};
pub use galaxy_protocol_negotiation_metrics::{
    protocol_negotiation_accepted_total, protocol_negotiation_rejected_total,
    record_protocol_negotiation_accepted, record_protocol_negotiation_rejected,
    METRIC_PROTOCOL_NEGOTIATION_ACCEPTED_TOTAL, METRIC_PROTOCOL_NEGOTIATION_REJECTED_TOTAL,
};
pub use galaxy_verification_replay::{
    build_verification_replay_record, GalaxyVerificationReplayRecord,
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
