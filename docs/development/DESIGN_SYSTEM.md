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

## JavaScript helpers (`admin_common.js`)

| Function | Purpose |
|----------|---------|
| `adminApplyDesignSystem(root)` | Adds table/form classes to dynamic admin DOM |
| `adminRenderTable(headers, rows)` | HTML for striped table |
| `adminFormFieldHtml(spec)` | One `form-group` field (`type`: text, select, textarea) |

Called from `adminObserveDynamicA11y()` on each admin page load and DOM mutations.

## Example

```javascript
document.getElementById('list').innerHTML = adminRenderTable(
  ['Name', 'Status'],
  [['node-a', 'healthy'], ['node-b', 'degraded']]
);
```

**Last updated:** 2026-05-23 (PH-S09).
