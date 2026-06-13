//! HTML escaping — parity with `escapeHtml` in `admin_common.js`.

/// Escape HTML special characters (`&`, `<`, `>`, `"`).
pub fn escape_html(s: impl AsRef<str>) -> String {
    s.as_ref()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
}
