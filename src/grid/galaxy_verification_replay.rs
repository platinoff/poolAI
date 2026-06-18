//! Verification replay wire DTO (PH-S447, Galaxy §6.3).

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Structured replay verification record emitted on mismatch enqueue (Galaxy §6.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyVerificationReplayRecord {
    pub verification_id: String,
    pub primary_job_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_job_id: Option<String>,
    pub verdict: String,
    pub observed_at: String,
}

/// Build replay record from grid result metrics stub path (PH-S447).
pub fn build_verification_replay_record(
    primary_job_id: &str,
    metrics: Option<&serde_json::Value>,
) -> GalaxyVerificationReplayRecord {
    let verification_id = metrics
        .and_then(|m| m.get("verification_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| format!("verify:{primary_job_id}"));
    let replay_job_id = metrics
        .and_then(|m| m.get("replay_job_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let verdict = metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .unwrap_or("mismatch")
        .to_string();
    GalaxyVerificationReplayRecord {
        verification_id,
        primary_job_id: primary_job_id.to_string(),
        replay_job_id,
        verdict,
        observed_at: Utc::now().to_rfc3339(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_verification_replay_record_ph_s447() {
        let rec = build_verification_replay_record(
            "job-1",
            Some(&json!({
                "verification_id": "v-99",
                "replay_job_id": "job-1-replay",
                "verification_verdict": "mismatch"
            })),
        );
        assert_eq!(rec.verification_id, "v-99");
        assert_eq!(rec.primary_job_id, "job-1");
        assert_eq!(rec.replay_job_id.as_deref(), Some("job-1-replay"));
        assert_eq!(rec.verdict, "mismatch");
        assert!(!rec.observed_at.is_empty());
    }
}
