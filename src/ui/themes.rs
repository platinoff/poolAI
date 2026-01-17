//! UI Themes module
//!
//! Provides theme customization support for the PoolAI dashboard.
//! Supports light/dark themes and custom color schemes.

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
    danger: "#ff5555",
    danger_hover: "#ff6e6e",
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
    text_muted: "#cccccc",
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
    
    /* Typography Scale (based on 16px base) */
    --font-size-xs: 12px;
    --font-size-sm: 14px;
    --font-size-base: 16px;
    --font-size-lg: 18px;
    --font-size-xl: 24px;
    --font-size-2xl: 32px;
    
    /* Line Heights */
    --line-height-tight: 1.2;
    --line-height-normal: 1.5;
    --line-height-relaxed: 1.75;
    
    /* Font Weights */
    --font-weight-normal: 400;
    --font-weight-medium: 500;
    --font-weight-semibold: 600;
    --font-weight-bold: 700;
    
    /* Spacing Scale (4px base unit) */
    --spacing-1: 4px;
    --spacing-2: 8px;
    --spacing-3: 12px;
    --spacing-4: 16px;
    --spacing-5: 20px;
    --spacing-6: 24px;
    --spacing-8: 32px;
    --spacing-10: 40px;
    --spacing-12: 48px;
    --spacing-16: 64px;
    
    /* Border Radius */
    --radius-sm: 4px;
    --radius-md: 8px;
    --radius-lg: 12px;
    --radius-xl: 16px;
    --radius-full: 9999px;
    
    /* Shadows (0-4 levels) */
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
    --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    --shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    --shadow-2xl: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
    
    /* Transitions */
    --transition-fast: 0.15s ease;
    --transition-base: 0.2s ease;
    --transition-slow: 0.3s ease;
    --transition-slower: 0.5s ease;
    
    /* Z-index Scale */
    --z-dropdown: 1000;
    --z-sticky: 1020;
    --z-fixed: 1030;
    --z-modal-backdrop: 1040;
    --z-modal: 1050;
    --z-popover: 1060;
    --z-tooltip: 1070;
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
{}
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
    color: #fff;
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
            self.to_css_variables()
        )
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
