/** PH-S160: tokens from poolai-ui-core (`window.__poolaiAdminThemesRust`); DOM glue only. */

function poolaiThemeMap() {
  if (window.__poolaiAdminThemesRust) {
    return window.__poolaiAdminThemesRust;
  }
  return {
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
  };
}

function poolaiNormalizeTheme(name) {
  const wasm = window.poolaiUiWasm;
  if (wasm && wasm.ready && typeof wasm.normalizeTheme === 'function') {
    return wasm.normalizeTheme(name);
  }
  if (name === 'light' || name === 'high-contrast') return name;
  return 'dark';
}

function poolaiApplyTheme(themeName) {
  const normalized = poolaiNormalizeTheme(themeName);
  const themes = poolaiThemeMap();
  const theme = themes[normalized] || themes.dark;
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
