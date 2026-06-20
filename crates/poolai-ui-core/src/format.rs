//! HTML escaping and dashboard shell datetime formatters (PH-S146, PH-S193).

use chrono::{DateTime, Utc};

/// Escape HTML special characters (`&`, `<`, `>`, `"`).
pub fn escape_html(s: impl AsRef<str>) -> String {
    s.as_ref()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Parse ISO/RFC3339 (or legacy unix string) for dashboard RAID `stored_at` cells.
pub fn format_iso_datetime_display(raw: Option<&str>) -> String {
    let Some(s) = raw.map(str::trim).filter(|t| !t.is_empty()) else {
        return "—".to_string();
    };
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return dt
            .with_timezone(&Utc)
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();
    }
    if let Ok(secs) = s.parse::<i64>() {
        if let Some(dt) = DateTime::from_timestamp(secs, 0) {
            return dt.format("%Y-%m-%d %H:%M:%S UTC").to_string();
        }
    }
    s.to_string()
}

/// Dashboard uptime label — `Nd Nh Nm` from seconds (PH-S385 wasm glue).
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let mins = (seconds % 3_600) / 60;
    format!("{days}d {hours}h {mins}m")
}

/// Dashboard `last_updated` clock — UTC `HH:MM:SS` from RFC3339 `now` (PH-S193).
pub fn format_locale_time_hms(now_rfc3339: Option<&str>) -> String {
    let now = now_rfc3339
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);
    now.format("%H:%M:%S").to_string()
}

/// Dashboard alert severity → `status-badge` CSS class (PH-S406 wasm glue).
pub fn alert_severity_badge_class(severity: Option<&str>) -> String {
    let raw = severity
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("info");
    match raw.to_ascii_lowercase().as_str() {
        "critical" | "error" | "warning" | "info" => raw.to_ascii_lowercase(),
        _ => "info".to_string(),
    }
}

/// Dashboard quick-stat CPU percent label (PH-S428 wasm glue).
pub fn format_percent(value: f64) -> String {
    format!("{:.1}%", value)
}

/// Dashboard quick-stat memory MB label (PH-S428 wasm glue).
pub fn format_megabytes(value: f64) -> String {
    format!("{:.0} MB", value)
}

/// Human-readable byte size for RAID artifact tables (PH-S618 wasm glue).
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    const K: f64 = 1024.0;
    const SIZES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let bytes_f = bytes as f64;
    let i = (bytes_f.log(K).floor() as usize).min(SIZES.len() - 1);
    let scaled = bytes_f / K.powi(i as i32);
    format!("{:.2} {}", scaled, SIZES[i])
}

/// Secret rotation unix timestamp display (PH-S628 wasm glue).
pub fn format_unix_timestamp_display(secs: Option<i64>, never_label: &str) -> String {
    let Some(ts) = secs.filter(|&s| s > 0) else {
        return never_label.to_string();
    };
    if let Some(dt) = DateTime::from_timestamp(ts, 0) {
        return dt.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
    ts.to_string()
}

/// Secret rotation kind → default English label (PH-S628 wasm glue).
pub fn format_rotation_kind(kind: &str) -> String {
    match kind.trim() {
        "jwt" => "JWT signing secret".to_string(),
        "tls_certificate" => "TLS certificate".to_string(),
        "telegram_webhook" => "Telegram webhook secret".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_special_chars() {
        assert_eq!(escape_html("a&b<c>\"d\""), "a&amp;b&lt;c&gt;&quot;d&quot;");
    }

    #[test]
    fn escape_html_plain() {
        assert_eq!(escape_html("hello"), "hello");
    }

    #[test]
    fn format_iso_datetime_rfc3339() {
        assert_eq!(
            format_iso_datetime_display(Some("2026-06-15T12:34:56Z")),
            "2026-06-15 12:34:56 UTC"
        );
    }

    #[test]
    fn format_iso_datetime_empty() {
        assert_eq!(format_iso_datetime_display(None), "—");
    }

    #[test]
    fn format_uptime_days_hours_mins_ph_s385() {
        assert_eq!(format_uptime(0), "0d 0h 0m");
        assert_eq!(format_uptime(3_600), "0d 1h 0m");
        assert_eq!(format_uptime(90_061), "1d 1h 1m");
    }

    #[test]
    fn format_locale_time_hms_parses() {
        assert_eq!(
            format_locale_time_hms(Some("2026-06-15T14:05:07Z")),
            "14:05:07"
        );
    }

    #[test]
    fn alert_severity_badge_class_ph_s406() {
        assert_eq!(alert_severity_badge_class(Some("Critical")), "critical");
        assert_eq!(alert_severity_badge_class(Some("WARNING")), "warning");
        assert_eq!(alert_severity_badge_class(None), "info");
        assert_eq!(alert_severity_badge_class(Some("unknown")), "info");
    }

    #[test]
    fn format_percent_ph_s428() {
        assert_eq!(format_percent(12.345), "12.3%");
        assert_eq!(format_percent(0.0), "0.0%");
    }

    #[test]
    fn format_megabytes_ph_s428() {
        assert_eq!(format_megabytes(1024.6), "1025 MB");
        assert_eq!(format_megabytes(0.0), "0 MB");
    }

    #[test]
    fn format_bytes_ph_s618() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1_500_000), "1.43 MB");
    }

    #[test]
    fn format_unix_timestamp_display_ph_s628() {
        assert_eq!(format_unix_timestamp_display(None, "Never"), "Never");
        assert_eq!(
            format_unix_timestamp_display(Some(1_718_280_000), "Never"),
            "2024-04-13 09:00:00 UTC"
        );
    }

    #[test]
    fn format_rotation_kind_ph_s628() {
        assert_eq!(format_rotation_kind("jwt"), "JWT signing secret");
        assert_eq!(format_rotation_kind("custom"), "custom");
    }
}
