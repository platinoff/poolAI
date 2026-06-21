//! Admin jobs panel helpers (PH-S852 store backend badge wasm glue).

use crate::format::escape_html;

/// Normalize `store_backend` wire value for i18n key lookup.
pub fn normalize_store_backend_key(backend: &str) -> &'static str {
    match backend.trim().to_ascii_lowercase().as_str() {
        "raid" => "raid",
        "sqlite" => "sqlite",
        "json" | "" => "json",
        _ => "json",
    }
}

/// Store backend badge HTML for admin jobs panel (PH-S852).
pub fn render_jobs_store_badge_html(
    backend: &str,
    store_label: &str,
    store_hint: &str,
    backend_display: &str,
) -> String {
    let _ = normalize_store_backend_key(backend);
    format!(
        r#"<span class="status-badge active" title="{hint}">{label} {backend}</span>"#,
        hint = escape_html(store_hint),
        label = escape_html(store_label),
        backend = escape_html(backend_display),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_store_backend_key_ph_s852() {
        assert_eq!(normalize_store_backend_key("RAID"), "raid");
        assert_eq!(normalize_store_backend_key("sqlite"), "sqlite");
        assert_eq!(normalize_store_backend_key(""), "json");
        assert_eq!(normalize_store_backend_key("  json  "), "json");
    }

    #[test]
    fn render_jobs_store_badge_html_ph_s852() {
        let html =
            render_jobs_store_badge_html("raid", "Store:", "Job persistence backend", "RAID");
        assert!(html.contains("status-badge active"));
        assert!(html.contains("Store:"));
        assert!(html.contains("RAID"));
        assert!(html.contains("Job persistence backend"));
    }
}
