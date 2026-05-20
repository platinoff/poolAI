//! Job / mining layer wire types (P6 / Horizon S38).

mod lifecycle;
mod map;
pub mod scheduler;
mod store;
#[cfg(feature = "job-store-sqlite")]
mod store_sqlite;
mod types;

pub use lifecycle::allows_transition;

pub use map::{
    envelope_from_job_spec, grid_result_from_status, job_spec_from_envelope,
    job_spec_from_grid_job, job_spec_to_grid_job, job_status_from_grid_result,
};
pub use scheduler::schedule_pending;
pub use store::{data_dir_from_env, JobStore};
pub use types::{JobId, JobKind, JobRecord, JobResources, JobSpec, JobStatus};
