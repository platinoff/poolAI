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

/// Dashboard `last_updated` clock — UTC `HH:MM:SS` from RFC3339 `now` (PH-S193).
pub fn format_locale_time_hms(now_rfc3339: Option<&str>) -> String {
    let now = now_rfc3339
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);
    now.format("%H:%M:%S").to_string()
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
    fn format_locale_time_hms_parses() {
        assert_eq!(
            format_locale_time_hms(Some("2026-06-15T14:05:07Z")),
            "14:05:07"
        );
    }
}
