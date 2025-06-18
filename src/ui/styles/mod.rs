//! UI Styles - Стили и темы для пользовательского интерфейса
//! 
//! Этот модуль предоставляет:
//! - Стили и темы
//! - CSS компоненты
//! - Адаптивный дизайн

pub mod theme;
pub mod layout;
pub mod components;

use serde::{Deserialize, Serialize};

/// Основная тема приложения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTheme {
    pub name: String,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub breakpoints: Breakpoints,
    pub shadows: Shadows,
    pub borders: Borders,
}

/// Цветовая палитра
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub border: Color,
    pub divider: Color,
}

/// Цвет
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub light: String,
    pub main: String,
    pub dark: String,
    pub contrast: String,
}

/// Типографика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: String,
    pub font_weight_light: u32,
    pub font_weight_normal: u32,
    pub font_weight_medium: u32,
    pub font_weight_bold: u32,
    pub line_height: f32,
    pub letter_spacing: String,
}

/// Отступы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
}

/// Точки перелома для адаптивного дизайна
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoints {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

/// Тени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadows {
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

/// Границы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Borders {
    pub radius_sm: String,
    pub radius_md: String,
    pub radius_lg: String,
    pub width_sm: String,
    pub width_md: String,
    pub width_lg: String,
}

/// Светлая тема
pub fn light_theme() -> AppTheme {
    AppTheme {
        name: "light".to_string(),
        colors: ColorPalette {
            primary: Color {
                light: "#42a5f5".to_string(),
                main: "#1976d2".to_string(),
                dark: "#1565c0".to_string(),
                contrast: "#ffffff".to_string(),
            },
            secondary: Color {
                light: "#ba68c8".to_string(),
                main: "#9c27b0".to_string(),
                dark: "#7b1fa2".to_string(),
                contrast: "#ffffff".to_string(),
            },
            success: Color {
                light: "#66bb6a".to_string(),
                main: "#4caf50".to_string(),
                dark: "#388e3c".to_string(),
                contrast: "#ffffff".to_string(),
            },
            warning: Color {
                light: "#ffb74d".to_string(),
                main: "#ff9800".to_string(),
                dark: "#f57c00".to_string(),
                contrast: "#000000".to_string(),
            },
            error: Color {
                light: "#ef5350".to_string(),
                main: "#f44336".to_string(),
                dark: "#d32f2f".to_string(),
                contrast: "#ffffff".to_string(),
            },
            info: Color {
                light: "#29b6f6".to_string(),
                main: "#03a9f4".to_string(),
                dark: "#0288d1".to_string(),
                contrast: "#ffffff".to_string(),
            },
            background: Color {
                light: "#ffffff".to_string(),
                main: "#fafafa".to_string(),
                dark: "#f5f5f5".to_string(),
                contrast: "#000000".to_string(),
            },
            surface: Color {
                light: "#ffffff".to_string(),
                main: "#ffffff".to_string(),
                dark: "#fafafa".to_string(),
                contrast: "#000000".to_string(),
            },
            text: Color {
                light: "#757575".to_string(),
                main: "#212121".to_string(),
                dark: "#000000".to_string(),
                contrast: "#ffffff".to_string(),
            },
            text_secondary: Color {
                light: "#bdbdbd".to_string(),
                main: "#757575".to_string(),
                dark: "#424242".to_string(),
                contrast: "#ffffff".to_string(),
            },
            border: Color {
                light: "#e0e0e0".to_string(),
                main: "#bdbdbd".to_string(),
                dark: "#9e9e9e".to_string(),
                contrast: "#000000".to_string(),
            },
            divider: Color {
                light: "#f5f5f5".to_string(),
                main: "#e0e0e0".to_string(),
                dark: "#bdbdbd".to_string(),
                contrast: "#000000".to_string(),
            },
        },
        typography: Typography {
            font_family: "'Roboto', 'Helvetica', 'Arial', sans-serif".to_string(),
            font_size_base: "16px".to_string(),
            font_weight_light: 300,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
            line_height: 1.5,
            letter_spacing: "0.00938em".to_string(),
        },
        spacing: Spacing {
            xs: "4px".to_string(),
            sm: "8px".to_string(),
            md: "16px".to_string(),
            lg: "24px".to_string(),
            xl: "32px".to_string(),
            xxl: "48px".to_string(),
        },
        breakpoints: Breakpoints {
            xs: "0px".to_string(),
            sm: "600px".to_string(),
            md: "960px".to_string(),
            lg: "1280px".to_string(),
            xl: "1920px".to_string(),
        },
        shadows: Shadows {
            sm: "0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)".to_string(),
            md: "0 3px 6px rgba(0,0,0,0.16), 0 3px 6px rgba(0,0,0,0.23)".to_string(),
            lg: "0 10px 20px rgba(0,0,0,0.19), 0 6px 6px rgba(0,0,0,0.23)".to_string(),
            xl: "0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22)".to_string(),
        },
        borders: Borders {
            radius_sm: "4px".to_string(),
            radius_md: "8px".to_string(),
            radius_lg: "12px".to_string(),
            width_sm: "1px".to_string(),
            width_md: "2px".to_string(),
            width_lg: "3px".to_string(),
        },
    }
}

/// Темная тема
pub fn dark_theme() -> AppTheme {
    AppTheme {
        name: "dark".to_string(),
        colors: ColorPalette {
            primary: Color {
                light: "#82b1ff".to_string(),
                main: "#448aff".to_string(),
                dark: "#2962ff".to_string(),
                contrast: "#000000".to_string(),
            },
            secondary: Color {
                light: "#e1bee7".to_string(),
                main: "#ce93d8".to_string(),
                dark: "#ba68c8".to_string(),
                contrast: "#000000".to_string(),
            },
            success: Color {
                light: "#a5d6a7".to_string(),
                main: "#81c784".to_string(),
                dark: "#66bb6a".to_string(),
                contrast: "#000000".to_string(),
            },
            warning: Color {
                light: "#ffcc02".to_string(),
                main: "#ffb300".to_string(),
                dark: "#ff8f00".to_string(),
                contrast: "#000000".to_string(),
            },
            error: Color {
                light: "#ff8a80".to_string(),
                main: "#ff5252".to_string(),
                dark: "#d32f2f".to_string(),
                contrast: "#ffffff".to_string(),
            },
            info: Color {
                light: "#80d8ff".to_string(),
                main: "#40c4ff".to_string(),
                dark: "#0091ea".to_string(),
                contrast: "#000000".to_string(),
            },
            background: Color {
                light: "#424242".to_string(),
                main: "#303030".to_string(),
                dark: "#212121".to_string(),
                contrast: "#ffffff".to_string(),
            },
            surface: Color {
                light: "#424242".to_string(),
                main: "#424242".to_string(),
                dark: "#303030".to_string(),
                contrast: "#ffffff".to_string(),
            },
            text: Color {
                light: "#ffffff".to_string(),
                main: "#ffffff".to_string(),
                dark: "#ffffff".to_string(),
                contrast: "#000000".to_string(),
            },
            text_secondary: Color {
                light: "#b3b3b3".to_string(),
                main: "#b3b3b3".to_string(),
                dark: "#808080".to_string(),
                contrast: "#000000".to_string(),
            },
            border: Color {
                light: "#616161".to_string(),
                main: "#424242".to_string(),
                dark: "#212121".to_string(),
                contrast: "#ffffff".to_string(),
            },
            divider: Color {
                light: "#424242".to_string(),
                main: "#424242".to_string(),
                dark: "#212121".to_string(),
                contrast: "#ffffff".to_string(),
            },
        },
        typography: Typography {
            font_family: "'Roboto', 'Helvetica', 'Arial', sans-serif".to_string(),
            font_size_base: "16px".to_string(),
            font_weight_light: 300,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
            line_height: 1.5,
            letter_spacing: "0.00938em".to_string(),
        },
        spacing: Spacing {
            xs: "4px".to_string(),
            sm: "8px".to_string(),
            md: "16px".to_string(),
            lg: "24px".to_string(),
            xl: "32px".to_string(),
            xxl: "48px".to_string(),
        },
        breakpoints: Breakpoints {
            xs: "0px".to_string(),
            sm: "600px".to_string(),
            md: "960px".to_string(),
            lg: "1280px".to_string(),
            xl: "1920px".to_string(),
        },
        shadows: Shadows {
            sm: "0 1px 3px rgba(0,0,0,0.3), 0 1px 2px rgba(0,0,0,0.4)".to_string(),
            md: "0 3px 6px rgba(0,0,0,0.4), 0 3px 6px rgba(0,0,0,0.5)".to_string(),
            lg: "0 10px 20px rgba(0,0,0,0.5), 0 6px 6px rgba(0,0,0,0.6)".to_string(),
            xl: "0 14px 28px rgba(0,0,0,0.6), 0 10px 10px rgba(0,0,0,0.7)".to_string(),
        },
        borders: Borders {
            radius_sm: "4px".to_string(),
            radius_md: "8px".to_string(),
            radius_lg: "12px".to_string(),
            width_sm: "1px".to_string(),
            width_md: "2px".to_string(),
            width_lg: "3px".to_string(),
        },
    }
}

/// Генерирует CSS для темы
pub fn generate_css(theme: &AppTheme) -> String {
    format!(
        r#"
        :root {{
            /* Colors */
            --color-primary-light: {};
            --color-primary-main: {};
            --color-primary-dark: {};
            --color-primary-contrast: {};
            
            --color-secondary-light: {};
            --color-secondary-main: {};
            --color-secondary-dark: {};
            --color-secondary-contrast: {};
            
            --color-success-light: {};
            --color-success-main: {};
            --color-success-dark: {};
            --color-success-contrast: {};
            
            --color-warning-light: {};
            --color-warning-main: {};
            --color-warning-dark: {};
            --color-warning-contrast: {};
            
            --color-error-light: {};
            --color-error-main: {};
            --color-error-dark: {};
            --color-error-contrast: {};
            
            --color-info-light: {};
            --color-info-main: {};
            --color-info-dark: {};
            --color-info-contrast: {};
            
            --color-background-light: {};
            --color-background-main: {};
            --color-background-dark: {};
            --color-background-contrast: {};
            
            --color-surface-light: {};
            --color-surface-main: {};
            --color-surface-dark: {};
            --color-surface-contrast: {};
            
            --color-text-light: {};
            --color-text-main: {};
            --color-text-dark: {};
            --color-text-contrast: {};
            
            --color-text-secondary-light: {};
            --color-text-secondary-main: {};
            --color-text-secondary-dark: {};
            --color-text-secondary-contrast: {};
            
            --color-border-light: {};
            --color-border-main: {};
            --color-border-dark: {};
            --color-border-contrast: {};
            
            --color-divider-light: {};
            --color-divider-main: {};
            --color-divider-dark: {};
            --color-divider-contrast: {};
            
            /* Typography */
            --font-family: {};
            --font-size-base: {};
            --font-weight-light: {};
            --font-weight-normal: {};
            --font-weight-medium: {};
            --font-weight-bold: {};
            --line-height: {};
            --letter-spacing: {};
            
            /* Spacing */
            --spacing-xs: {};
            --spacing-sm: {};
            --spacing-md: {};
            --spacing-lg: {};
            --spacing-xl: {};
            --spacing-xxl: {};
            
            /* Breakpoints */
            --breakpoint-xs: {};
            --breakpoint-sm: {};
            --breakpoint-md: {};
            --breakpoint-lg: {};
            --breakpoint-xl: {};
            
            /* Shadows */
            --shadow-sm: {};
            --shadow-md: {};
            --shadow-lg: {};
            --shadow-xl: {};
            
            /* Borders */
            --border-radius-sm: {};
            --border-radius-md: {};
            --border-radius-lg: {};
            --border-width-sm: {};
            --border-width-md: {};
            --border-width-lg: {};
        }}
        
        /* Global styles */
        * {{
            box-sizing: border-box;
        }}
        
        body {{
            font-family: var(--font-family);
            font-size: var(--font-size-base);
            font-weight: var(--font-weight-normal);
            line-height: var(--line-height);
            letter-spacing: var(--letter-spacing);
            background-color: var(--color-background-main);
            color: var(--color-text-main);
            margin: 0;
            padding: 0;
        }}
        
        /* Utility classes */
        .text-primary {{ color: var(--color-primary-main); }}
        .text-secondary {{ color: var(--color-secondary-main); }}
        .text-success {{ color: var(--color-success-main); }}
        .text-warning {{ color: var(--color-warning-main); }}
        .text-error {{ color: var(--color-error-main); }}
        .text-info {{ color: var(--color-info-main); }}
        
        .bg-primary {{ background-color: var(--color-primary-main); }}
        .bg-secondary {{ background-color: var(--color-secondary-main); }}
        .bg-success {{ background-color: var(--color-success-main); }}
        .bg-warning {{ background-color: var(--color-warning-main); }}
        .bg-error {{ background-color: var(--color-error-main); }}
        .bg-info {{ background-color: var(--color-info-main); }}
        
        .shadow-sm {{ box-shadow: var(--shadow-sm); }}
        .shadow-md {{ box-shadow: var(--shadow-md); }}
        .shadow-lg {{ box-shadow: var(--shadow-lg); }}
        .shadow-xl {{ box-shadow: var(--shadow-xl); }}
        
        .rounded-sm {{ border-radius: var(--border-radius-sm); }}
        .rounded-md {{ border-radius: var(--border-radius-md); }}
        .rounded-lg {{ border-radius: var(--border-radius-lg); }}
        
        .p-xs {{ padding: var(--spacing-xs); }}
        .p-sm {{ padding: var(--spacing-sm); }}
        .p-md {{ padding: var(--spacing-md); }}
        .p-lg {{ padding: var(--spacing-lg); }}
        .p-xl {{ padding: var(--spacing-xl); }}
        .p-xxl {{ padding: var(--spacing-xxl); }}
        
        .m-xs {{ margin: var(--spacing-xs); }}
        .m-sm {{ margin: var(--spacing-sm); }}
        .m-md {{ margin: var(--spacing-md); }}
        .m-lg {{ margin: var(--spacing-lg); }}
        .m-xl {{ margin: var(--spacing-xl); }}
        .m-xxl {{ margin: var(--spacing-xxl); }}
        "#,
        // Colors
        theme.colors.primary.light, theme.colors.primary.main, theme.colors.primary.dark, theme.colors.primary.contrast,
        theme.colors.secondary.light, theme.colors.secondary.main, theme.colors.secondary.dark, theme.colors.secondary.contrast,
        theme.colors.success.light, theme.colors.success.main, theme.colors.success.dark, theme.colors.success.contrast,
        theme.colors.warning.light, theme.colors.warning.main, theme.colors.warning.dark, theme.colors.warning.contrast,
        theme.colors.error.light, theme.colors.error.main, theme.colors.error.dark, theme.colors.error.contrast,
        theme.colors.info.light, theme.colors.info.main, theme.colors.info.dark, theme.colors.info.contrast,
        theme.colors.background.light, theme.colors.background.main, theme.colors.background.dark, theme.colors.background.contrast,
        theme.colors.surface.light, theme.colors.surface.main, theme.colors.surface.dark, theme.colors.surface.contrast,
        theme.colors.text.light, theme.colors.text.main, theme.colors.text.dark, theme.colors.text.contrast,
        theme.colors.text_secondary.light, theme.colors.text_secondary.main, theme.colors.text_secondary.dark, theme.colors.text_secondary.contrast,
        theme.colors.border.light, theme.colors.border.main, theme.colors.border.dark, theme.colors.border.contrast,
        theme.colors.divider.light, theme.colors.divider.main, theme.colors.divider.dark, theme.colors.divider.contrast,
        // Typography
        theme.typography.font_family, theme.typography.font_size_base,
        theme.typography.font_weight_light, theme.typography.font_weight_normal,
        theme.typography.font_weight_medium, theme.typography.font_weight_bold,
        theme.typography.line_height, theme.typography.letter_spacing,
        // Spacing
        theme.spacing.xs, theme.spacing.sm, theme.spacing.md, theme.spacing.lg, theme.spacing.xl, theme.spacing.xxl,
        // Breakpoints
        theme.breakpoints.xs, theme.breakpoints.sm, theme.breakpoints.md, theme.breakpoints.lg, theme.breakpoints.xl,
        // Shadows
        theme.shadows.sm, theme.shadows.md, theme.shadows.lg, theme.shadows.xl,
        // Borders
        theme.borders.radius_sm, theme.borders.radius_md, theme.borders.radius_lg,
        theme.borders.width_sm, theme.borders.width_md, theme.borders.width_lg,
    )
} 