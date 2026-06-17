# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S238 ✅ · vision **rev 187** · **1** відкритий (PH-S239) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S239** — Admin i18n slim config panel |
| **Відкритих** | **1** (PH-S239) |

---

## PH-S239 — scope

- `admin.cfg.*` → `poolai-ui-core` + `admin_config_patch` + `admin_layout_config`
- Remove `admin.cfg.*` + `admin.page.config` from `i18n_core.js`
- Pattern: PH-S236…S238 slim panels
- Acceptance: targeted `cargo test`; FM/HANDOFF/NEXT; push; replenish §5.12 when queue closes

---

## Copy-paste — PH-S239

```
PH-S239 — Admin i18n slim config panel (code)
Scope: admin.cfg.* → poolai-ui-core; slim layout; cargo test; FM/HANDOFF/NEXT; commit+push
```
