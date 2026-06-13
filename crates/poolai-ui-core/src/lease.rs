//! Job lease display helpers — parity with embedded JS in `src/ui/admin/jobs.rs`.

use chrono::{DateTime, Utc};

/// Lease badge state derived from `lease_expires_at` (RFC3339 or ISO parseable).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseDisplayState {
    /// Missing or unparseable expiry.
    None,
    /// `now < expires_at`.
    Active,
    /// `now >= expires_at`.
    Expired,
}

impl LeaseDisplayState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Active => "active",
            Self::Expired => "expired",
        }
    }

    pub fn badge_css_class(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Active => Some("active"),
            Self::Expired => Some("warning"),
        }
    }
}

/// Mirrors `leaseState(expiresAt)` in admin jobs JS.
pub fn lease_state(expires_at: Option<&str>, now: DateTime<Utc>) -> LeaseDisplayState {
    let Some(raw) = expires_at.filter(|s| !s.is_empty()) else {
        return LeaseDisplayState::None;
    };
    let Ok(expires) = DateTime::parse_from_rfc3339(raw).map(|dt| dt.with_timezone(&Utc)) else {
        return LeaseDisplayState::None;
    };
    if now < expires {
        LeaseDisplayState::Active
    } else {
        LeaseDisplayState::Expired
    }
}

/// Mirrors `formatLeaseCell(value)`.
pub fn format_lease_cell(value: Option<&str>) -> String {
    match value.filter(|s| !s.is_empty()) {
        None => "—".to_string(),
        Some(v) => v.to_string(),
    }
}

/// Mirrors `leaseEpochCell` display text (without HTML wrapper).
pub fn format_lease_epoch_display(epoch: Option<&str>) -> String {
    match epoch.filter(|s| !s.is_empty()) {
        None => "—".to_string(),
        Some(raw) => {
            if let Ok(n) = raw.parse::<i64>() {
                format!("#{n}")
            } else if let Ok(n) = raw.parse::<f64>() {
                if n.is_finite() {
                    format!("#{n}")
                } else {
                    raw.to_string()
                }
            } else {
                raw.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn lease_state_none_for_empty() {
        let now = Utc.with_ymd_and_hms(2026, 6, 13, 12, 0, 0).unwrap();
        assert_eq!(lease_state(None, now), LeaseDisplayState::None);
        assert_eq!(lease_state(Some(""), now), LeaseDisplayState::None);
        assert_eq!(
            lease_state(Some("not-a-date"), now),
            LeaseDisplayState::None
        );
    }

    #[test]
    fn lease_state_active_and_expired() {
        let now = Utc.with_ymd_and_hms(2026, 6, 13, 12, 0, 0).unwrap();
        let future = "2026-06-13T13:00:00Z";
        let past = "2026-06-13T11:00:00Z";
        assert_eq!(lease_state(Some(future), now), LeaseDisplayState::Active);
        assert_eq!(lease_state(Some(past), now), LeaseDisplayState::Expired);
    }

    #[test]
    fn format_lease_cell_and_epoch() {
        assert_eq!(format_lease_cell(None), "—");
        assert_eq!(
            format_lease_cell(Some("2026-06-13T12:00:00Z")),
            "2026-06-13T12:00:00Z"
        );
        assert_eq!(format_lease_epoch_display(None), "—");
        assert_eq!(format_lease_epoch_display(Some("3")), "#3");
        assert_eq!(format_lease_epoch_display(Some("bad")), "bad");
    }
}
