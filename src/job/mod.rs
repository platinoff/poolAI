//! Job / mining layer wire types (P6 / Horizon S38).

mod domain_events;
mod lifecycle;
mod map;
mod onchain;
pub mod scheduler;
mod store;
#[cfg(feature = "job-store-sqlite")]
mod store_sqlite;
mod types;

pub use domain_events::{
    DomainEvent, DomainEventEnvelope, JobCompletedEvent, MemoryUpdatedEvent, SeedProvidedEvent,
};
pub use lifecycle::allows_transition;
pub use onchain::{
    emit_job_completed_if_anchor, emit_memory_updated, emit_seed_provided, events_dir_from_env,
    memory_content_digest,
};

pub use map::{
    envelope_from_job_spec, grid_result_from_status, job_spec_from_envelope,
    job_spec_from_grid_job, job_spec_to_grid_job, job_status_from_grid_result,
};
pub use scheduler::{
    schedule_from_context, schedule_pending, schedule_with_grid_peer, schedule_with_workers,
    ScheduleOutcome, VmCandidate, WorkerCandidate,
};
pub use store::{data_dir_from_env, JobStore};
pub use types::{
    check_patch_lease_epoch, JobId, JobKind, JobRecord, JobResources, JobScheduleBinding, JobSpec,
    JobStatus, PatchLeaseEpochError,
};
