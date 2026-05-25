# Admin design system (PH-S09 / FM-045)

**Status:** Shared structural tokens + unified admin tables/forms/buttons.

## Token source

| File | Role |
|------|------|
| `src/ui/design_tokens.css` | Canonical spacing, typography, radius, shadows, z-index |
| `src/ui/admin_styles.css` | Admin colors, layout, components |
| `src/ui/themes.rs` | Dashboard theme colors + includes `design_tokens.css` |

Admin pages load tokens via `admin_layout`:

```rust
concat!(include_str!("../design_tokens.css"), include_str!("../admin_styles.css"))
```

## CSS classes

| Class | Use |
|-------|-----|
| `admin-table` + `admin-table--striped` | Data tables |
| `admin-table-container` | Horizontal scroll wrapper |
| `admin-form` | Vertical form stack |
| `form-group` | Label + control |
| `form-actions` | Button row |
| `btn-primary` / `btn-secondary` / `btn-danger` / `btn-ghost` | Actions |
| `admin-card` | Section panel |

## JavaScript helpers

### `admin_common.js`

| Function | Purpose |
|----------|---------|
| `poolaiApplyTheme` | Applies dark / light / **high-contrast** CSS variables (PH-S12 / PH-S14) |
| `poolaiNormalizeTheme` | Maps stored `poolai_theme` to supported admin theme id |
| `adminApplyDesignSystem(root)` | Adds table/form classes to dynamic admin DOM |
| `adminRenderTable(headers, rows, options?)` | HTML for striped table; empty rows → `adminEmptyStateHtml` |
| `adminEmptyStateHtml(message, options?)` | Centered empty state (PH-S42) |
| `adminEnhanceAdminTable(table, options?)` | Sortable headers, filter toolbar, CSV/JSON export |
| `adminInitTablesIn(root?)` | Auto-enhance all `.admin-table` in admin content |
| `adminBindTableSearch(input, table)` | Wire page-level search input to a table |
| `adminExportTableCsv` / `adminExportTableJson` | Export visible (filtered) rows |
| `adminFormFieldHtml(spec)` | One `form-group` field (`type`: text, select, textarea) |

Called from `adminObserveDynamicA11y()` on each admin page load and DOM mutations (includes `adminInitTablesIn`).

### `admin_charts.js` (PH-S10)

Loaded after `admin_common.js` in `admin_layout`. SVG-only charts; data from `GET /api/enterprise/monitoring/metrics`.

| Function | Purpose |
|----------|---------|
| `poolaiFetchMetricHistory(name, { hours, limit })` | Single-metric time series |
| `poolaiFetchMetricsWindow({ hours, limit })` | All metrics in a window |
| `poolaiGroupMetricsByName(metrics)` | Group API rows by `metric` |
| `poolaiRenderLineChart(name, data, opts)` | Full chart (`metric-chart-container`) |
| `poolaiRenderSparkline(label, values, opts)` | Compact dashboard sparkline |
| `poolaiRenderMetricsChartGrid(names, opts)` | Async card + grid for monitoring |
| `poolaiStartMetricsPolling(fn, ms)` | `setInterval` wrapper; returns `stop()` |

CSS: `.metric-chart-container`, `.metrics-charts-grid`, `.metrics-sparklines-grid`, `.metric-sparkline-card` in `admin_styles.css`.

## Example

```javascript
document.getElementById('list').innerHTML = adminRenderTable(
  ['Name', 'Status'],
  [['node-a', 'healthy'], ['node-b', 'degraded']]
);
```

**Last updated:** 2026-05-25 (PH-S42 admin table sort/filter/export + empty states).

## Themes (PH-S14)

| Theme | Admin (`admin_common.js`) | Dashboard (`mod.rs` / `themes.rs`) |
|-------|---------------------------|-------------------------------------|
| `dark` | default | default |
| `light` | ✅ | ✅ |
| `high-contrast` | ✅ PH-S14 | ✅ |

E2E: `e2e/tests/a11y.spec.ts` — axe `color-contrast` with `localStorage.poolai_theme=high-contrast` on login + admin routes.
