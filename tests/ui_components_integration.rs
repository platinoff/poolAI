//! UI Components Integration Tests
//!
//! Tests for UI components, themes, and layout functionality.

use poolai::ui::{get_all_themes, get_theme, DARK_THEME, HIGH_CONTRAST_THEME, LIGHT_THEME};

#[tokio::test]
async fn test_theme_get_all_themes() {
    let themes = get_all_themes();
    assert_eq!(themes.len(), 3);
    assert!(themes.contains(&&DARK_THEME));
    assert!(themes.contains(&&LIGHT_THEME));
    assert!(themes.contains(&&HIGH_CONTRAST_THEME));
}

#[tokio::test]
async fn test_theme_get_theme_dark() {
    let theme = get_theme("dark");
    assert_eq!(theme.name, "dark");
    assert_eq!(theme.background, "#0f1216");
    assert_eq!(theme.primary, "#50fa7b");
}

#[tokio::test]
async fn test_theme_get_theme_light() {
    let theme = get_theme("light");
    assert_eq!(theme.name, "light");
    assert_eq!(theme.background, "#ffffff");
    assert_eq!(theme.primary, "#00a86b");
}

#[tokio::test]
async fn test_theme_get_theme_high_contrast() {
    let theme = get_theme("high-contrast");
    assert_eq!(theme.name, "high-contrast");
    assert_eq!(theme.background, "#000000");
    assert_eq!(theme.primary, "#00ff00");
}

#[tokio::test]
async fn test_theme_get_theme_default() {
    let theme = get_theme("unknown");
    assert_eq!(theme.name, "dark"); // Default fallback
}

#[tokio::test]
async fn test_theme_to_css_variables() {
    let theme = get_theme("dark");
    let css = theme.to_css_variables();
    assert!(css.contains("--bg:"));
    assert!(css.contains("--primary:"));
    assert!(css.contains("--text:"));
}

#[tokio::test]
async fn test_theme_to_css() {
    let theme = get_theme("dark");
    let css = theme.to_css();
    assert!(css.contains("body {"));
    assert!(css.contains(".card {"));
    assert!(css.contains(".btn-primary {"));
}

#[tokio::test]
async fn test_dark_theme_colors() {
    assert_eq!(DARK_THEME.background, "#0f1216");
    assert_eq!(DARK_THEME.surface, "#171b22");
    assert_eq!(DARK_THEME.text, "#e8e8e8");
    assert_eq!(DARK_THEME.primary, "#50fa7b");
}

#[tokio::test]
async fn test_light_theme_colors() {
    assert_eq!(LIGHT_THEME.background, "#ffffff");
    assert_eq!(LIGHT_THEME.surface, "#f5f5f5");
    assert_eq!(LIGHT_THEME.text, "#1a1a1a");
    assert_eq!(LIGHT_THEME.primary, "#00a86b");
}

#[tokio::test]
async fn test_high_contrast_theme_colors() {
    assert_eq!(HIGH_CONTRAST_THEME.background, "#000000");
    assert_eq!(HIGH_CONTRAST_THEME.surface, "#1a1a1a");
    assert_eq!(HIGH_CONTRAST_THEME.text, "#ffffff");
    assert_eq!(HIGH_CONTRAST_THEME.primary, "#00ff00");
}

#[tokio::test]
async fn test_theme_css_variables_format() {
    let theme = get_theme("dark");
    let css = theme.to_css_variables();
    // Check that CSS variables are properly formatted
    assert!(css.contains(":root {"));
    assert!(css.contains("--bg:"));
    assert!(css.contains("--surface:"));
    assert!(css.contains("--text:"));
    assert!(css.contains("--primary:"));
}

#[tokio::test]
async fn test_theme_css_includes_all_elements() {
    let theme = get_theme("dark");
    let css = theme.to_css();
    // Check that CSS includes all major UI elements
    assert!(css.contains("body {"));
    assert!(css.contains(".card {"));
    assert!(css.contains(".btn-primary {"));
    assert!(css.contains(".btn-danger {"));
    assert!(css.contains(".pill-success {"));
    assert!(css.contains("a {"));
}
