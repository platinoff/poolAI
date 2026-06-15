//! Galaxy updates & compatibility display helpers (PH-S197).

/// Human label for wire `compat_status` / negotiation status (`accepted`, `upgrade_required`, …).
pub fn compat_status_label(status: &str) -> &'static str {
    match status.trim().to_ascii_lowercase().as_str() {
        "accepted" => "Accepted",
        "upgrade_required" => "Upgrade required",
        "unsupported" => "Unsupported",
        _ => "—",
    }
}

/// Normalize coordinator/worker protocol version for admin display.
pub fn protocol_version_label(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "—".to_string();
    }
    let core = trimmed.split_whitespace().next().unwrap_or(trimmed);
    let version_part = core.split('-').next().unwrap_or(core);
    if version_part.contains('.') {
        version_part.to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compat_status_label_maps_known_values() {
        assert_eq!(compat_status_label("accepted"), "Accepted");
        assert_eq!(compat_status_label("upgrade_required"), "Upgrade required");
        assert_eq!(compat_status_label("unsupported"), "Unsupported");
        assert_eq!(compat_status_label("unknown"), "—");
    }

    #[test]
    fn protocol_version_label_normalizes() {
        assert_eq!(protocol_version_label("1.2"), "1.2");
        assert_eq!(protocol_version_label("  1.2  "), "1.2");
        assert_eq!(protocol_version_label("1.2-beta"), "1.2");
        assert_eq!(protocol_version_label(""), "—");
    }
}
