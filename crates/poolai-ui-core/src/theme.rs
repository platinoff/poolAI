//! Admin theme normalize + color token map (PH-S160).
//!
//! Parity: `src/ui/admin_theme.js`, `src/ui/themes.rs` browser palette.

use serde::Serialize;
use std::collections::BTreeMap;

/// Browser admin theme color tokens (camelCase JSON keys).
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeTokens {
    pub bg: &'static str,
    pub surface: &'static str,
    pub surface_secondary: &'static str,
    pub text: &'static str,
    pub text_muted: &'static str,
    pub border: &'static str,
    pub primary: &'static str,
    pub primary_hover: &'static str,
    pub secondary: &'static str,
    pub secondary_hover: &'static str,
    pub danger: &'static str,
    pub danger_hover: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub success: &'static str,
    pub link: &'static str,
    pub link_hover: &'static str,
}

pub const DARK: ThemeTokens = ThemeTokens {
    bg: "#0f1216",
    surface: "#171b22",
    surface_secondary: "#1e2329",
    text: "#e8e8e8",
    text_muted: "#a8b0bf",
    border: "#262b36",
    primary: "#67e480",
    primary_hover: "#50fa7b",
    secondary: "#6272a4",
    secondary_hover: "#7a8bc4",
    danger: "#c62828",
    danger_hover: "#e53935",
    warning: "#ffb86c",
    info: "#8be9fd",
    success: "#50fa7b",
    link: "#77c7ff",
    link_hover: "#8bd5ff",
};

pub const LIGHT: ThemeTokens = ThemeTokens {
    bg: "#ffffff",
    surface: "#f5f5f5",
    surface_secondary: "#e8e8e8",
    text: "#1a1a1a",
    text_muted: "#666666",
    border: "#d0d0d0",
    primary: "#00a86b",
    primary_hover: "#00c47a",
    secondary: "#6c757d",
    secondary_hover: "#5a6268",
    danger: "#dc3545",
    danger_hover: "#c82333",
    warning: "#ffc107",
    info: "#17a2b8",
    success: "#28a745",
    link: "#007bff",
    link_hover: "#0056b3",
};

pub const HIGH_CONTRAST: ThemeTokens = ThemeTokens {
    bg: "#000000",
    surface: "#1a1a1a",
    surface_secondary: "#000000",
    text: "#ffffff",
    text_muted: "#e0e0e0",
    border: "#ffffff",
    primary: "#00ff00",
    primary_hover: "#00cc00",
    secondary: "#ffff00",
    secondary_hover: "#cccc00",
    danger: "#ff0000",
    danger_hover: "#cc0000",
    warning: "#ffff00",
    info: "#00ffff",
    success: "#00ff00",
    link: "#00aaff",
    link_hover: "#0088cc",
};

/// Maps stored `poolai_theme` to supported admin theme id (PH-S14 high-contrast).
pub fn normalize_theme(name: &str) -> &'static str {
    if name == "light" {
        "light"
    } else if name == "high-contrast" {
        "high-contrast"
    } else {
        "dark"
    }
}

/// Token map for a normalized theme id.
pub fn theme_tokens(normalized: &str) -> &'static ThemeTokens {
    match normalized {
        "light" => &LIGHT,
        "high-contrast" => &HIGH_CONTRAST,
        _ => &DARK,
    }
}

/// Full `POOLAI_UI_THEMES` object for admin layout injection.
pub fn admin_themes_patch() -> BTreeMap<&'static str, ThemeTokens> {
    let mut map = BTreeMap::new();
    map.insert("dark", DARK);
    map.insert("light", LIGHT);
    map.insert("high-contrast", HIGH_CONTRAST);
    map
}

/// JSON literal assigned to `window.__poolaiAdminThemesRust`.
pub fn admin_theme_patch_json() -> String {
    serde_json::to_string(&admin_themes_patch()).expect("admin theme patch serializes")
}

/// Inline script body (no `<script>` wrapper) for admin layout injection.
pub fn admin_theme_patch_script() -> String {
    format!(
        "window.__poolaiAdminThemesRust={};",
        admin_theme_patch_json()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_theme_maps_unknown_to_dark() {
        assert_eq!(normalize_theme("dark"), "dark");
        assert_eq!(normalize_theme("light"), "light");
        assert_eq!(normalize_theme("high-contrast"), "high-contrast");
        assert_eq!(normalize_theme("dracula"), "dark");
        assert_eq!(normalize_theme(""), "dark");
    }

    #[test]
    fn patch_json_contains_high_contrast_primary() {
        let json = admin_theme_patch_json();
        assert!(json.contains(r#""high-contrast""#));
        assert!(json.contains("\"primary\":\"#00ff00\""));
        assert!(json.contains("\"surfaceSecondary\":\"#1e2329\""));
    }

    #[test]
    fn script_assigns_window_patch() {
        let script = admin_theme_patch_script();
        assert!(script.starts_with("window.__poolaiAdminThemesRust="));
        assert!(script.ends_with(';'));
    }

    #[test]
    fn theme_tokens_follows_normalize() {
        assert_eq!(theme_tokens(normalize_theme("hc")).bg, DARK.bg);
        assert_eq!(
            theme_tokens(normalize_theme("light")).primary,
            LIGHT.primary
        );
    }
}
