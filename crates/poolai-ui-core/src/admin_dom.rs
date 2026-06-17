//! Admin loading/error DOM snippets — parity with `admin_common.js` (PH-S274).

use crate::format::escape_html;

/// `adminShowLoading` inner HTML (`<div class="muted">…</div>`).
pub fn admin_loading_html(text: &str) -> String {
    let display = if text.trim().is_empty() {
        "Loading…"
    } else {
        text
    };
    format!(r#"<div class="muted">{}</div>"#, escape_html(display))
}

/// `adminShowInlineError` inner HTML (`role="alert"` fetch error block).
pub fn admin_inline_error_html(message: &str) -> String {
    format!(
        r#"<div class="admin-fetch-error" role="alert">{}</div>"#,
        escape_html(message)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_loading_html_escapes_and_wraps() {
        assert_eq!(
            admin_loading_html("Load <x>"),
            r#"<div class="muted">Load &lt;x&gt;</div>"#
        );
    }

    #[test]
    fn admin_loading_html_empty_defaults() {
        assert_eq!(
            admin_loading_html(""),
            r#"<div class="muted">Loading…</div>"#
        );
    }

    #[test]
    fn admin_inline_error_html_role_alert() {
        let html = admin_inline_error_html("boom & bust");
        assert!(html.contains(r#"role="alert""#));
        assert!(html.contains("admin-fetch-error"));
        assert!(html.contains("boom &amp; bust"));
    }
}
