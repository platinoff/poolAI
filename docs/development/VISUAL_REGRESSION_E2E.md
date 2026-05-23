# Visual regression E2E (PH-S11 / PH-S12)

**Status:** Playwright `toHaveScreenshot` baselines in `e2e/tests/visual.spec.ts` (no Percy/Chromatic — local/CI snapshots only).

## Scope

| Spec | Screenshots |
|------|-------------|
| **PH-S11** | Login (`login.png`); admin `main.admin-main` for dashboard + 9 admin routes (default dark + EN) |
| **PH-S12** | Theme × i18n matrix: `login-{dark\|light}-{en\|uk}.png`, `users-*`, `dashboard-*` (12 baselines) |
| Masks | Live metrics charts (`#metrics-chart`, `.metrics-charts-grid`, `.metric-chart-svg`); RAID dynamic panels |

Topology is **out of scope** (force-layout SVG is non-deterministic across runs).

## Theme + i18n (PH-S12)

| Mechanism | Storage / API |
|-----------|----------------|
| Theme | `localStorage.poolai_theme` → `poolaiApplyTheme` in `admin_common.js`; login uses `applyTheme(getTheme())` from dashboard `common_js` |
| Locale | `localStorage.poolai_ui_lang` (`en` \| `uk`) → `PoolAiI18n.setLang` / `getLang` (`i18n_core.js`) |

E2E helpers: `primeUiPrefs`, `matrixSnapshotName`, `expectVisualLang` in `e2e/tests/helpers.ts`.

## Baselines

- Path: `e2e/tests/visual.spec.ts-snapshots/*.png`
- Viewport: **1280×720**, `deviceScaleFactor: 1`
- Threshold: `maxDiffPixelRatio: 0.02` (`playwright.config.ts`)

Regenerate after intentional UI/CSS changes:

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
cd /s/rust/poolAI
bash bin/e2e-playwright.sh --start --update-snapshots
# or visual only:
cd e2e && npm run test:visual:update
```

**CI:** baselines are generated on **Linux** (GitHub `ubuntu-latest`). Refresh on Windows only for local debugging; commit Linux snapshots for `main`.

## Local

Same prerequisites as [E2E_PLAYWRIGHT.md](./E2E_PLAYWRIGHT.md):

```bash
bash bin/e2e-playwright.sh --start
cd e2e && npm run test:visual
```

## CI

Included in the existing **Playwright admin** job ([`e2e.yml`](../../.github/workflows/e2e.yml)) via `bin/e2e-playwright.sh --start` → full `npm test` (smoke + admin + a11y + visual).

**Last updated:** 2026-05-23 (PH-S12 theme/i18n matrix).
