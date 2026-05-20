//! Job lifecycle transitions (JOB_LAYER_CONCEPT §2.2, FM-021).

use crate::job::JobStatus;

/// Whether `to` is allowed from `from` (idempotent when equal).
pub fn allows_transition(from: JobStatus, to: JobStatus) -> bool {
    if from == to {
        return true;
    }
    match from {
        JobStatus::Submitted => matches!(to, JobStatus::Scheduled | JobStatus::Failed),
        JobStatus::Scheduled => matches!(to, JobStatus::Executing | JobStatus::Failed),
        JobStatus::Executing => matches!(to, JobStatus::Verifying | JobStatus::Failed),
        JobStatus::Verifying => {
            matches!(
                to,
                JobStatus::Rewarded | JobStatus::Completed | JobStatus::Failed
            )
        }
        JobStatus::Rewarded => matches!(to, JobStatus::Completed | JobStatus::Failed),
        JobStatus::Completed | JobStatus::Failed => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_path_and_failed_escape() {
        assert!(allows_transition(
            JobStatus::Submitted,
            JobStatus::Scheduled
        ));
        assert!(allows_transition(
            JobStatus::Scheduled,
            JobStatus::Executing
        ));
        assert!(allows_transition(
            JobStatus::Executing,
            JobStatus::Verifying
        ));
        assert!(allows_transition(JobStatus::Verifying, JobStatus::Rewarded));
        assert!(allows_transition(JobStatus::Rewarded, JobStatus::Completed));
        assert!(allows_transition(
            JobStatus::Verifying,
            JobStatus::Completed
        ));
        assert!(allows_transition(JobStatus::Executing, JobStatus::Failed));
    }

    #[test]
    fn rejects_skip_and_terminal() {
        assert!(!allows_transition(
            JobStatus::Submitted,
            JobStatus::Executing
        ));
        assert!(!allows_transition(JobStatus::Completed, JobStatus::Failed));
        assert!(!allows_transition(JobStatus::Failed, JobStatus::Submitted));
    }
}
