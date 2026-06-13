//! ML pipeline metric helpers — parity with `admin_charts.js`.

/// Metric keys collected for sparklines / summary ( `POOLAI_ML_METRIC_KEYS` ).
pub const ML_METRIC_KEYS: &[&str] = &[
    "accuracy",
    "loss",
    "f1_score",
    "precision",
    "recall",
    "auc",
    "rmse",
    "mae",
    "r2",
    "epochs",
    "pruned_count",
    "sparsity_ratio",
    "feature_dim",
    "sample_count",
];

/// Mirrors `poolaiParseMlNumeric(val)`.
pub fn parse_ml_numeric(value: Option<&str>) -> Option<f64> {
    let raw = value.filter(|s| !s.is_empty())?;
    let n: f64 = raw.parse().ok()?;
    if n.is_finite() {
        Some(n)
    } else {
        None
    }
}

/// Mirrors `poolaiFormatMlMetricSummary(output)` — up to 4 `key=value` pairs.
pub fn format_ml_metric_summary(output: &[(String, String)]) -> String {
    let mut parts = Vec::new();
    for key in ML_METRIC_KEYS {
        if let Some((_, v)) = output.iter().find(|(k, v)| k == key && !v.is_empty()) {
            parts.push(format!("{key}={v}"));
        }
        if parts.len() >= 4 {
            break;
        }
    }
    if parts.is_empty() {
        if let Some((_, status)) = output.iter().find(|(k, _)| k == "status") {
            if !status.is_empty() {
                return format!("status={status}");
            }
        }
        return "—".to_string();
    }
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ml_numeric_cases() {
        assert_eq!(parse_ml_numeric(None), None);
        assert_eq!(parse_ml_numeric(Some("")), None);
        assert_eq!(parse_ml_numeric(Some("0.95")), Some(0.95));
        assert_eq!(parse_ml_numeric(Some("nan")), None);
    }

    #[test]
    fn format_ml_metric_summary_limits_four() {
        let output = vec![
            ("accuracy".into(), "0.9".into()),
            ("loss".into(), "0.1".into()),
            ("f1_score".into(), "0.88".into()),
            ("precision".into(), "0.87".into()),
            ("recall".into(), "0.86".into()),
        ];
        let s = format_ml_metric_summary(&output);
        assert_eq!(s.matches(',').count(), 3);
        assert!(s.contains("accuracy=0.9"));
    }

    #[test]
    fn format_ml_metric_summary_status_fallback() {
        let output = vec![("status".into(), "ok".into())];
        assert_eq!(format_ml_metric_summary(&output), "status=ok");
        assert_eq!(format_ml_metric_summary(&[]), "—");
    }
}
