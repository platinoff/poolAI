//! Minimal Prometheus text exposition parser for admin glue (PH-S672).

/// Parse the last gauge sample value for `metric_name` from Prometheus text (PH-S672).
pub fn parse_prometheus_gauge(metrics_text: &str, metric_name: &str) -> u64 {
    let needle = format!("{metric_name} ");
    for line in metrics_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || !trimmed.starts_with(&needle) {
            continue;
        }
        if let Some(value) = trimmed.strip_prefix(&needle) {
            if let Ok(parsed) = value.trim().parse::<f64>() {
                return parsed.max(0.0) as u64;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prometheus_gauge_ph_s672() {
        let body = r#"
# HELP galaxy_verification_checker_pending_total Pending checker tasks
# TYPE galaxy_verification_checker_pending_total gauge
galaxy_verification_checker_pending_total 3
galaxy_verification_sample_total 12
"#;
        assert_eq!(
            parse_prometheus_gauge(body, "galaxy_verification_checker_pending_total"),
            3
        );
        assert_eq!(parse_prometheus_gauge(body, "missing_metric"), 0);
    }
}
