//! Galaxy Grid verification metrics stubs (PH-S175 / PH-S177 / PH-S180, §6.2).
//!
//! Counters for grid result verification sample, mismatch, and match; checker task enqueue wire (PH-S488).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// In-process counter for verification samples scheduled (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_SAMPLE_TOTAL: &str = "galaxy_verification_sample_total";

/// In-process counter for verification digest mismatches (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_MISMATCH_TOTAL: &str = "galaxy_verification_mismatch_total";

/// In-process counter for verification digest matches (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_MATCH_TOTAL: &str = "galaxy_verification_match_total";

/// In-process counter for verification samples completed with verdict (PH-S343).
pub const METRIC_VERIFICATION_SAMPLE_COMPLETED_TOTAL: &str =
    "galaxy_verification_sample_completed_total";

/// Verification checker enqueue stub invocations (PH-S437).
pub const METRIC_VERIFICATION_CHECKER_ENQUEUE_TOTAL: &str =
    "galaxy_verification_checker_enqueue_total";
pub const METRIC_VERIFICATION_CHECKER_PENDING_TOTAL: &str =
    "galaxy_verification_checker_pending_total";

static SAMPLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static MISMATCH_TOTAL: AtomicU64 = AtomicU64::new(0);
static MATCH_TOTAL: AtomicU64 = AtomicU64::new(0);
static SAMPLE_COMPLETED_TOTAL: AtomicU64 = AtomicU64::new(0);
static CHECKER_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Stub verification checker task record (PH-S488).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct VerificationCheckerTask {
    pub job_id: String,
    pub task_type: String,
}

static CHECKER_TASKS: LazyLock<Mutex<Vec<VerificationCheckerTask>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Record one verification sample on the grid result path.
pub fn record_verification_sample() {
    SAMPLE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_sample_total() -> u64 {
    SAMPLE_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_sample_metrics_for_test() {
    SAMPLE_TOTAL.store(0, Ordering::Relaxed);
}

/// Record one verification mismatch on the grid result path.
pub fn record_verification_mismatch() {
    MISMATCH_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_mismatch_total() -> u64 {
    MISMATCH_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_mismatch_metrics_for_test() {
    MISMATCH_TOTAL.store(0, Ordering::Relaxed);
}

/// Record one verification match on the grid result path.
pub fn record_verification_match() {
    MATCH_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_match_total() -> u64 {
    MATCH_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_match_metrics_for_test() {
    MATCH_TOTAL.store(0, Ordering::Relaxed);
}

/// Record one verification sample completed with match or mismatch verdict.
pub fn record_verification_sample_completed() {
    SAMPLE_COMPLETED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_sample_completed_total() -> u64 {
    SAMPLE_COMPLETED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_sample_completed_metrics_for_test() {
    SAMPLE_COMPLETED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_metrics_for_test() {
    reset_verification_sample_metrics_for_test();
    reset_verification_mismatch_metrics_for_test();
    reset_verification_match_metrics_for_test();
    reset_verification_sample_completed_metrics_for_test();
    CHECKER_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    reset_verification_checker_tasks_for_test();
}

/// Record one verification checker enqueue stub (PH-S437).
pub fn record_verification_checker_enqueue() {
    CHECKER_ENQUEUE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_checker_enqueue_total() -> u64 {
    CHECKER_ENQUEUE_TOTAL.load(Ordering::Relaxed)
}

/// Enqueue verification checker stub when sample is scheduled (PH-S437); task wire (PH-S488).
pub fn enqueue_verification_checker(scheduled: bool) {
    if scheduled {
        record_verification_checker_enqueue();
    }
}

/// Enqueue shadow-checker stub task for a sampled job (PH-S488).
pub fn enqueue_verification_checker_task(job_id: &str) {
    if let Ok(mut tasks) = CHECKER_TASKS.lock() {
        tasks.push(VerificationCheckerTask {
            job_id: job_id.to_string(),
            task_type: "verification_checker".into(),
        });
    }
}

/// Pending checker stub tasks (in-process, PH-S488).
pub fn verification_checker_tasks() -> Vec<VerificationCheckerTask> {
    CHECKER_TASKS.lock().map(|g| g.clone()).unwrap_or_default()
}

/// Pending checker task count for Prometheus (PH-S496).
pub fn verification_checker_pending_total() -> u64 {
    CHECKER_TASKS.lock().map(|g| g.len() as u64).unwrap_or(0)
}

/// Remove pending checker task after verdict (PH-S495).
pub fn drain_verification_checker_task(job_id: &str) -> bool {
    if let Ok(mut tasks) = CHECKER_TASKS.lock() {
        let before = tasks.len();
        tasks.retain(|t| t.job_id != job_id);
        return tasks.len() < before;
    }
    false
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_checker_tasks_for_test() {
    if let Ok(mut tasks) = CHECKER_TASKS.lock() {
        tasks.clear();
    }
}

/// Grid result path stub: increment sample counter when stub selects edge sample or explicit flag.
pub fn evaluate_result_verification_sample(
    metrics: Option<&serde_json::Value>,
    sample_scheduled: bool,
) -> bool {
    let explicit = metrics
        .and_then(|m| m.get("verification_sample"))
        .and_then(|v| v.as_bool())
        == Some(true);
    let scheduled = sample_scheduled || explicit;
    if scheduled {
        record_verification_sample();
        enqueue_verification_checker(true);
    }
    scheduled
}

/// Grid result path stub: read optional `metrics.verification_verdict`; increment on `mismatch`.
pub fn evaluate_result_verification_mismatch(metrics: Option<&serde_json::Value>) -> bool {
    let is_mismatch = metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("mismatch"));
    if is_mismatch {
        record_verification_mismatch();
    }
    is_mismatch
}

/// Grid result path stub: read optional `metrics.verification_verdict`; increment on `match` (PH-S180).
pub fn evaluate_result_verification_match(metrics: Option<&serde_json::Value>) -> bool {
    let is_match = metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("match"));
    if is_match {
        record_verification_match();
    }
    is_match
}

/// Grid result path stub: non-deterministic `semantic_hash` compare (PH-S511, Galaxy §6.2).
/// Returns `Some(true)` on match, `Some(false)` on mismatch, `None` when not applicable.
pub fn evaluate_semantic_hash_verification(metrics: Option<&serde_json::Value>) -> Option<bool> {
    let m = metrics?;
    let task_profile = m.get("task_profile").and_then(|v| v.as_str())?;
    if !task_profile.eq_ignore_ascii_case("non_deterministic") {
        return None;
    }
    let expected = m.get("expected_semantic_hash").and_then(|v| v.as_str())?;
    let actual = m.get("semantic_hash").and_then(|v| v.as_str());
    let is_match = actual.is_some_and(|a| a == expected);
    if is_match {
        record_verification_match();
    } else {
        record_verification_mismatch();
    }
    Some(is_match)
}

/// Grid result path stub: increment when verification verdict is `match` or `mismatch` (PH-S343).
pub fn evaluate_result_verification_sample_completed(metrics: Option<&serde_json::Value>) -> bool {
    let completed = metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("match") || s.eq_ignore_ascii_case("mismatch"));
    if completed {
        record_verification_sample_completed();
    }
    completed
}

#[cfg(test)]
static VERIFICATION_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn verification_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    VERIFICATION_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn evaluate_result_verification_sample_increments_on_scheduled_ph_s177() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(!evaluate_result_verification_sample(None, false));
        assert_eq!(verification_sample_total(), 0);

        assert!(evaluate_result_verification_sample(None, true));
        assert_eq!(verification_sample_total(), 1);
        assert_eq!(verification_checker_enqueue_total(), 1);

        assert!(!evaluate_result_verification_sample(None, false));
        assert_eq!(verification_sample_total(), 1);

        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_sample_increments_on_explicit_flag_ph_s177() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(evaluate_result_verification_sample(
            Some(&json!({ "verification_sample": true })),
            false,
        ));
        assert_eq!(verification_sample_total(), 1);
        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_mismatch_increments_counter_ph_s175() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(!evaluate_result_verification_mismatch(None));
        assert_eq!(verification_mismatch_total(), 0);

        assert!(evaluate_result_verification_mismatch(Some(&json!({
            "verification_verdict": "mismatch"
        }))));
        assert_eq!(verification_mismatch_total(), 1);

        assert!(!evaluate_result_verification_mismatch(Some(&json!({
            "verification_verdict": "match"
        }))));
        assert_eq!(verification_mismatch_total(), 1);

        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_match_increments_counter_ph_s180() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(!evaluate_result_verification_match(None));
        assert_eq!(verification_match_total(), 0);

        assert!(evaluate_result_verification_match(Some(&json!({
            "verification_verdict": "match"
        }))));
        assert_eq!(verification_match_total(), 1);

        assert!(!evaluate_result_verification_match(Some(&json!({
            "verification_verdict": "mismatch"
        }))));
        assert_eq!(verification_match_total(), 1);

        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_sample_completed_increments_on_verdict_ph_s343() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(!evaluate_result_verification_sample_completed(None));
        assert_eq!(verification_sample_completed_total(), 0);

        assert!(evaluate_result_verification_sample_completed(Some(
            &json!({
                "verification_verdict": "match"
            })
        )));
        assert_eq!(verification_sample_completed_total(), 1);

        assert!(evaluate_result_verification_sample_completed(Some(
            &json!({
                "verification_verdict": "mismatch"
            })
        )));
        assert_eq!(verification_sample_completed_total(), 2);

        reset_verification_metrics_for_test();
    }

    #[test]
    fn enqueue_verification_checker_task_ph_s488() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        enqueue_verification_checker_task("job-vc-1");
        enqueue_verification_checker(true);
        assert_eq!(verification_checker_enqueue_total(), 1);
        let tasks = verification_checker_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].job_id, "job-vc-1");
        assert_eq!(tasks[0].task_type, "verification_checker");
        reset_verification_metrics_for_test();
    }

    #[test]
    fn drain_verification_checker_task_ph_s495() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        enqueue_verification_checker_task("job-drain-1");
        enqueue_verification_checker_task("job-drain-2");
        assert_eq!(verification_checker_pending_total(), 2);
        assert!(drain_verification_checker_task("job-drain-1"));
        assert_eq!(verification_checker_pending_total(), 1);
        assert!(!drain_verification_checker_task("job-missing"));
        drain_verification_checker_task("job-drain-2");
        assert_eq!(verification_checker_pending_total(), 0);
        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_semantic_hash_verification_ph_s511() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        let match_metrics = json!({
            "task_profile": "non_deterministic",
            "expected_semantic_hash": "abc123",
            "semantic_hash": "abc123"
        });
        assert_eq!(
            evaluate_semantic_hash_verification(Some(&match_metrics)),
            Some(true)
        );
        assert_eq!(verification_match_total(), 1);

        reset_verification_metrics_for_test();
        let mismatch_metrics = json!({
            "task_profile": "non_deterministic",
            "expected_semantic_hash": "abc123",
            "semantic_hash": "other"
        });
        assert_eq!(
            evaluate_semantic_hash_verification(Some(&mismatch_metrics)),
            Some(false)
        );
        assert_eq!(verification_mismatch_total(), 1);
        reset_verification_metrics_for_test();
    }
}
