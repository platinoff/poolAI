//! Structural + admin default color CSS variables (PH-S166).
//!
//! Parity: former `src/ui/design_tokens.css` and admin `:root` colors in `admin_styles.css`.

use crate::theme::DARK;

/// Canonical structural design token map (`:root` spacing, typography, shadows, z-index).
pub const STRUCTURAL: &[(&str, &str)] = &[
    ("font-size-xs", "12px"),
    ("font-size-sm", "14px"),
    ("font-size-base", "16px"),
    ("font-size-lg", "18px"),
    ("font-size-xl", "24px"),
    ("font-size-2xl", "32px"),
    ("line-height-tight", "1.2"),
    ("line-height-normal", "1.5"),
    ("line-height-relaxed", "1.75"),
    ("font-weight-normal", "400"),
    ("font-weight-medium", "500"),
    ("font-weight-semibold", "600"),
    ("font-weight-bold", "700"),
    ("spacing-1", "4px"),
    ("spacing-2", "8px"),
    ("spacing-3", "12px"),
    ("spacing-4", "16px"),
    ("spacing-5", "20px"),
    ("spacing-6", "24px"),
    ("spacing-8", "32px"),
    ("spacing-10", "40px"),
    ("spacing-12", "48px"),
    ("spacing-16", "64px"),
    ("radius-sm", "4px"),
    ("radius-md", "8px"),
    ("radius-lg", "12px"),
    ("radius-xl", "16px"),
    ("radius-full", "9999px"),
    ("shadow-sm", "0 1px 2px 0 rgba(0, 0, 0, 0.05)"),
    (
        "shadow-md",
        "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
    ),
    (
        "shadow-lg",
        "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
    ),
    (
        "shadow-xl",
        "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
    ),
    ("shadow-2xl", "0 25px 50px -12px rgba(0, 0, 0, 0.25)"),
    ("transition-fast", "0.15s ease"),
    ("transition-base", "0.2s ease"),
    ("transition-slow", "0.3s ease"),
    ("transition-slower", "0.5s ease"),
    ("z-dropdown", "1000"),
    ("z-sticky", "1020"),
    ("z-fixed", "1030"),
    ("z-modal-backdrop", "1040"),
    ("z-modal", "1050"),
    ("z-popover", "1060"),
    ("z-tooltip", "1070"),
];

/// Admin layout default dark palette + shell vars (PH-S160 `DARK` parity).
pub fn admin_color_root_css() -> String {
    let t = DARK;
    format!(
        r#":root {{
  --bg: {bg};
  --surface: {surface};
  --surface-secondary: {surface_secondary};
  --text: {text};
  --text-muted: {text_muted};
  --primary: {primary};
  --primary-hover: {primary_hover};
  --link: {link};
  --link-hover: {link_hover};
  --border: {border};
  --danger: {danger};
  --danger-hover: {danger_hover};
  --warning: {warning};
  --info: {info};
  --success: {success};
  --admin-sidebar-width: 260px;
  --admin-bg: var(--bg);
  --admin-surface: var(--surface);
  --admin-surface-secondary: var(--surface-secondary);
  --admin-border: var(--border);
  --admin-text: var(--text);
  --admin-text-muted: var(--text-muted);
  --admin-primary: var(--primary);
  --admin-primary-hover: var(--primary-hover);
  --admin-danger: var(--danger);
  --admin-warning: var(--warning);
  --admin-info: var(--info);
}}"#,
        bg = t.bg,
        surface = t.surface,
        surface_secondary = t.surface_secondary,
        text = t.text,
        text_muted = t.text_muted,
        primary = t.primary,
        primary_hover = t.primary_hover,
        link = t.link,
        link_hover = t.link_hover,
        border = t.border,
        danger = t.danger,
        danger_hover = t.danger_hover,
        warning = t.warning,
        info = t.info,
        success = t.success,
    )
}

/// Structural `:root` block formerly in `design_tokens.css`.
pub fn design_tokens_css() -> String {
    let mut css = String::from(
        "/* PoolAI design tokens (PH-S166) — poolai-ui-core/design_tokens.rs */\n\n:root {\n",
    );
    for (name, value) in STRUCTURAL {
        css.push_str(&format!("  --{name}: {value};\n"));
    }
    css.push_str("}\n");
    css
}

/// Admin `<style>` prefix: structural tokens + default color `:root`.
pub fn admin_base_css() -> String {
    format!("{}\n{}", design_tokens_css(), admin_color_root_css())
}

/// UI_UX plan parity note — structural token audit gate (PH-S1025).
pub const DESIGN_TOKENS_AUDIT_NOTE: &str =
    "PH-S1025: structural tokens in poolai-ui-core/design_tokens.rs; admin palette via DARK theme.";

/// Returns true when structural token map and CSS export are in sync.
pub fn design_tokens_parity_gate() -> bool {
    !STRUCTURAL.is_empty()
        && STRUCTURAL.iter().all(|(name, value)| {
            !name.is_empty()
                && !value.is_empty()
                && design_tokens_css().contains(&format!("--{name}:"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_css_includes_spacing_and_typography() {
        let css = design_tokens_css();
        assert!(css.contains("--spacing-1: 4px"));
        assert!(css.contains("--font-size-base: 16px"));
        assert!(css.contains("--z-modal: 1050"));
    }

    #[test]
    fn admin_color_root_matches_dark_theme() {
        let css = admin_color_root_css();
        assert!(css.contains(&format!("--bg: {};", DARK.bg)));
        assert!(css.contains("--admin-sidebar-width: 260px"));
        assert!(css.contains("--admin-primary: var(--primary)"));
    }

    #[test]
    fn admin_base_css_concatenates_blocks() {
        let css = admin_base_css();
        assert!(css.contains("--spacing-4: 16px"));
        assert!(css.contains("--primary: #67e480"));
    }

    #[test]
    fn design_tokens_parity_gate_ph_s1025() {
        assert!(design_tokens_parity_gate());
        assert!(DESIGN_TOKENS_AUDIT_NOTE.contains("PH-S1025"));
        assert!(design_tokens_css().contains("--font-size-base: 16px"));
    }
}
