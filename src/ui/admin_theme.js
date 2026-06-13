/** PH-S12 / PH-S14: dark + light + high-contrast tokens (aligned with themes.rs). */
const POOLAI_UI_THEMES = {
  dark: {
    bg: '#0f1216',
    surface: '#171b22',
    surfaceSecondary: '#1e2329',
    text: '#e8e8e8',
    textMuted: '#a8b0bf',
    border: '#262b36',
    primary: '#67e480',
    primaryHover: '#50fa7b',
    secondary: '#6272a4',
    secondaryHover: '#7a8bc4',
    danger: '#c62828',
    dangerHover: '#e53935',
    warning: '#ffb86c',
    info: '#8be9fd',
    success: '#50fa7b',
    link: '#77c7ff',
    linkHover: '#8bd5ff',
  },
  light: {
    bg: '#ffffff',
    surface: '#f5f5f5',
    surfaceSecondary: '#e8e8e8',
    text: '#1a1a1a',
    textMuted: '#666666',
    border: '#d0d0d0',
    primary: '#00a86b',
    primaryHover: '#00c47a',
    secondary: '#6c757d',
    secondaryHover: '#5a6268',
    danger: '#dc3545',
    dangerHover: '#c82333',
    warning: '#ffc107',
    info: '#17a2b8',
    success: '#28a745',
    link: '#007bff',
    linkHover: '#0056b3',
  },
  'high-contrast': {
    bg: '#000000',
    surface: '#1a1a1a',
    surfaceSecondary: '#000000',
    text: '#ffffff',
    textMuted: '#e0e0e0',
    border: '#ffffff',
    primary: '#00ff00',
    primaryHover: '#00cc00',
    secondary: '#ffff00',
    secondaryHover: '#cccc00',
    danger: '#ff0000',
    dangerHover: '#cc0000',
    warning: '#ffff00',
    info: '#00ffff',
    success: '#00ff00',
    link: '#00aaff',
    linkHover: '#0088cc',
  },
};

function poolaiNormalizeTheme(name) {
  if (name === 'light' || name === 'high-contrast') return name;
  return 'dark';
}

function poolaiApplyTheme(themeName) {
  const normalized = poolaiNormalizeTheme(themeName);
  const theme = POOLAI_UI_THEMES[normalized] || POOLAI_UI_THEMES.dark;
  const root = document.documentElement;
  root.style.setProperty('--bg', theme.bg);
  root.style.setProperty('--surface', theme.surface);
  root.style.setProperty('--surface-secondary', theme.surfaceSecondary);
  root.style.setProperty('--text', theme.text);
  root.style.setProperty('--text-muted', theme.textMuted);
  root.style.setProperty('--border', theme.border);
  root.style.setProperty('--primary', theme.primary);
  root.style.setProperty('--primary-hover', theme.primaryHover);
  root.style.setProperty('--secondary', theme.secondary);
  root.style.setProperty('--secondary-hover', theme.secondaryHover);
  root.style.setProperty('--danger', theme.danger);
  root.style.setProperty('--danger-hover', theme.dangerHover);
  root.style.setProperty('--warning', theme.warning);
  root.style.setProperty('--info', theme.info);
  root.style.setProperty('--success', theme.success);
  root.style.setProperty('--link', theme.link);
  root.style.setProperty('--link-hover', theme.linkHover);
  root.dataset.poolaiTheme = normalized;
}

function poolaiInitThemeFromStorage() {
  let name = 'dark';
  try {
    name = localStorage.getItem('poolai_theme') || 'dark';
  } catch (e) {
    name = 'dark';
  }
  poolaiApplyTheme(poolaiNormalizeTheme(name));
}

window.poolaiApplyTheme = poolaiApplyTheme;
window.poolaiInitThemeFromStorage = poolaiInitThemeFromStorage;
window.poolaiNormalizeTheme = poolaiNormalizeTheme;
