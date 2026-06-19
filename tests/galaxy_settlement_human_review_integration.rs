//! PH-S560: human-review settlement hold on semantic_hash mismatch.

use poolai::grid::galaxy_settlement_metrics::{
    evaluate_semantic_hash_human_review_hold, record_settlement_human_review,
    reset_settlement_metrics_for_test, settlement_human_review_total,
};
use serde_json::json;

#[test]
fn human_review_hold_on_semantic_hash_mismatch_ph_s560() {
    reset_settlement_metrics_for_test();
    let metrics = json!({
        "task_profile": "non_deterministic",
        "expected_semantic_hash": "abc",
        "semantic_hash": "other"
    });
    assert!(evaluate_semantic_hash_human_review_hold(Some(&metrics)));
    record_settlement_human_review();
    assert_eq!(settlement_human_review_total(), 1);
    reset_settlement_metrics_for_test();
}

#[test]
fn human_review_hold_without_semantic_hash_ph_s560() {
    let metrics = json!({
        "task_profile": "llm"
    });
    assert!(evaluate_semantic_hash_human_review_hold(Some(&metrics)));
}

#[test]
fn no_human_review_when_hash_matches_ph_s560() {
    let metrics = json!({
        "task_profile": "non_deterministic",
        "expected_semantic_hash": "abc",
        "semantic_hash": "abc"
    });
    assert!(!evaluate_semantic_hash_human_review_hold(Some(&metrics)));
}
