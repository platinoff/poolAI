//! UI Themes module
//!
//! Provides theme customization support for the PoolAI dashboard.
//! Supports light/dark themes and custom color schemes.
//!
//! Structural design tokens (spacing, typography, shadows) from [`poolai_ui_core::design_tokens`] (PH-S166).

/// Shared structural CSS variables (PH-S166 — generated in poolai-ui-core).
pub fn design_tokens_css() -> String {
    poolai_ui_core::design_tokens::design_tokens_css()
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub background: &'static str,
    pub surface: &'static str,
    pub surface_secondary: &'static str,
    pub text: &'static str,
    pub text_muted: &'static str,
    pub border: &'static str,
    pub primary: &'static str,
    pub primary_hover: &'static str,
    pub danger: &'static str,
    pub danger_hover: &'static str,
    pub secondary: &'static str,
    pub secondary_hover: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub link: &'static str,
    pub link_hover: &'static str,
}

/// Dark theme (Dracula-inspired) - default
pub const DARK_THEME: Theme = Theme {
    name: "dark",
    background: "#0f1216",
    surface: "#171b22",
    surface_secondary: "#0f1216",
    text: "#e8e8e8",
    text_muted: "#a8b0bf",
    border: "#262b36",
    primary: "#50fa7b",
    primary_hover: "#67e480",
    danger: "#c62828",
    danger_hover: "#e53935",
    secondary: "#6272a4",
    secondary_hover: "#7a8bc4",
    success: "#50fa7b",
    warning: "#f1fa8c",
    info: "#8be9fd",
    link: "#77c7ff",
    link_hover: "#8bd5ff",
};

/// Light theme
pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    background: "#ffffff",
    surface: "#f5f5f5",
    surface_secondary: "#e8e8e8",
    text: "#1a1a1a",
    text_muted: "#666666",
    border: "#d0d0d0",
    primary: "#00a86b",
    primary_hover: "#00c47a",
    danger: "#dc3545",
    danger_hover: "#c82333",
    secondary: "#6c757d",
    secondary_hover: "#5a6268",
    success: "#28a745",
    warning: "#ffc107",
    info: "#17a2b8",
    link: "#007bff",
    link_hover: "#0056b3",
};

/// High contrast theme (accessibility)
pub const HIGH_CONTRAST_THEME: Theme = Theme {
    name: "high-contrast",
    background: "#000000",
    surface: "#1a1a1a",
    surface_secondary: "#000000",
    text: "#ffffff",
    text_muted: "#e0e0e0",
    border: "#ffffff",
    primary: "#00ff00",
    primary_hover: "#00cc00",
    danger: "#ff0000",
    danger_hover: "#cc0000",
    secondary: "#ffff00",
    secondary_hover: "#cccc00",
    success: "#00ff00",
    warning: "#ffff00",
    info: "#00ffff",
    link: "#00aaff",
    link_hover: "#0088cc",
};

impl Theme {
    /// Generate CSS variables for the theme with comprehensive design tokens
    pub fn to_css_variables(&self) -> String {
        format!(
            r#"
  :root {{
    /* Theme Identity */
    --theme-name: "{}";
    
    /* Color System */
    --bg: {};
    --surface: {};
    --surface-secondary: {};
    --text: {};
    --text-muted: {};
    --border: {};
    --primary: {};
    --primary-hover: {};
    --danger: {};
    --danger-hover: {};
    --secondary: {};
    --secondary-hover: {};
    --success: {};
    --warning: {};
    --info: {};
    --link: {};
    --link-hover: {};
  }}
"#,
            self.name,
            self.background,
            self.surface,
            self.surface_secondary,
            self.text,
            self.text_muted,
            self.border,
            self.primary,
            self.primary_hover,
            self.danger,
            self.danger_hover,
            self.secondary,
            self.secondary_hover,
            self.success,
            self.warning,
            self.info,
            self.link,
            self.link_hover
        )
    }

    /// Generate CSS with theme variables applied
    pub fn to_css(&self) -> String {
        format!(
            r#"
{design_tokens}
{theme_vars}
  body {{
    background: var(--bg);
    color: var(--text);
  }}
  .card {{
    background: var(--surface);
    border-color: var(--border);
  }}
  .btn-primary {{
    background: var(--primary);
    border-color: var(--primary);
    color: var(--bg);
  }}
  [data-poolai-theme="high-contrast"] .btn-primary {{
    background: #004400;
    border-color: #00ff00;
    color: #ffffff;
  }}
  [data-poolai-theme="high-contrast"] .btn-primary:hover {{
    background: #006600;
    border-color: #00ff00;
    color: #ffffff;
  }}
  [data-poolai-theme="high-contrast"] .btn-danger {{
    background: #cc0000;
    border-color: #ff0000;
    color: #ffffff;
  }}
  .btn-primary:hover {{
    background: var(--primary-hover);
  }}
  .btn-danger {{
    background: var(--danger);
    border-color: var(--danger);
    color: #fff;
  }}
  .btn-danger:hover {{
    background: var(--danger-hover);
  }}
  .btn-secondary {{
    background: var(--secondary);
    border-color: var(--secondary);
    color: var(--bg);
  }}
  .btn-secondary:hover {{
    background: var(--secondary-hover);
  }}
  .pill-success {{
    background: var(--success);
    color: var(--bg);
    border-color: var(--success);
  }}
  .pill-error {{
    background: var(--danger);
    color: #fff;
    border-color: var(--danger);
  }}
  .pill-warning {{
    background: var(--warning);
    color: var(--bg);
    border-color: var(--warning);
  }}
  .pill-info {{
    background: var(--info);
    color: var(--bg);
    border-color: var(--info);
  }}
  a {{
    color: var(--link);
  }}
  a:hover {{
    color: var(--link-hover);
  }}
  .muted {{
    color: var(--text-muted);
  }}
  .topbar {{
    background: var(--surface);
    border-color: var(--border);
  }}
  .brand h1 {{
    color: var(--primary);
  }}
"#,
            design_tokens = design_tokens_css(),
            theme_vars = self.to_css_variables(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_tokens_include_spacing_scale() {
        let css = design_tokens_css();
        assert!(css.contains("--spacing-1: 4px"));
        assert!(css.contains("--font-size-base: 16px"));
    }

    #[test]
    fn theme_css_includes_tokens_and_colors() {
        let css = DARK_THEME.to_css();
        assert!(css.contains("--spacing-4: 16px"));
        assert!(css.contains("--bg: #0f1216"));
    }
}

/// Get theme by name
pub fn get_theme(name: &str) -> &'static Theme {
    match name {
        "light" => &LIGHT_THEME,
        "high-contrast" => &HIGH_CONTRAST_THEME,
        _ => &DARK_THEME, // default
    }
}

/// Get all available themes
pub fn get_all_themes() -> Vec<&'static Theme> {
    vec![&DARK_THEME, &LIGHT_THEME, &HIGH_CONTRAST_THEME]
}
